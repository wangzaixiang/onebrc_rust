use std::collections::HashMap;
use std::intrinsics::{likely, unlikely};
use memchr::{memchr2};
use memmap2::Mmap;
use crate::MEASUREMENT_FILE;

use std::simd::{u8x64};
use std::simd::cmp::SimdPartialEq;

#[inline]
fn parse_value(_buf: &[u8]) -> i16 {    // ~0.5s
    // return 0;
    use std::intrinsics::unlikely;
    let mut sign = 1;
    let mut value = 0;
    for b in _buf {
        if unlikely(*b == b'-') {
            sign = -1;
        }
        else if unlikely(*b == b'.') {
            continue;
        } else {
            value = value * 10 + (*b - b'0') as i32;
        }
    }
    (value * sign) as i16
}

struct Item {
    min: i16,
    max: i16,
    count: u32,
    sum: i32,
}
struct Entry {
    key: String,
    hash: u64,
    item: Item
}
struct SimpleHashMap {
    indexes: [u16; 32768+256],      // 32767: empty, others access keys[idx], 8K * 4
    values: Vec<Entry>,
    loses: u64
}


// #[test]
// fn test_hash(){
//
//     let str1 = "hello world this is an example";
//     let str2 = format!("{}{}", "12", str1);
//     let hash2 = SimpleHashMap::compute_hash(&str2.as_bytes()[2..]);
//
//     assert_eq!(hash1, hash2);
// }

impl SimpleHashMap {

    fn new() -> SimpleHashMap {
        SimpleHashMap {
            indexes: [ u16::MAX; 32768+256],
            values: Vec::with_capacity(8192),
            loses: 0
        }
    }

    fn compute_hash(key: &[u8]) -> u64 {
        // 5 * 12
        let count = key.len();
        let mut shift = 0;
        let mut result = 0u64;
        for i in 0..count {
            let b = (key[i] as u64) & 0x1F;
            result ^= b << shift;
            shift += 5;
            shift = shift % 64;
        }
        result
    }

    /// k[0..8] ^ k[8..16] ^ ..
    // #[inline]
    // fn compute_hash(key: &[u8]) -> u64 {
    //     let ptr = key.as_ptr() as usize;
    //     if unlikely(ptr % 4 == 0) {
    //         Self::compute_hash_0(key)
    //     }
    //     else {
    //         let count = 4 - ptr % 4; // 1,2,3 -> 3,2,1
    //         let mut shift = 0;
    //         let mut part0 = 0;
    //         for i in 0..count {
    //             let b = key[i] as u32;
    //             part0 += b << shift;
    //             shift += 8;
    //         }
    //         let part1 = Self::compute_hash_0(&key[count..]);
    //         let part1 = part1 << shift | (part1 >> (64 - shift));
    //         part1 ^ part0 as u64
    //     }
    // }
    // #[inline]
    // fn compute_hash_0(key: &[u8]) -> u64 {
    //     let ptr = key.as_ptr();
    //     let len = key.len();
    //
    //     if len < 4 { // 0..<4
    //         let b1 = if len >= 1 { key[0] } else { 0 };
    //         let b2 = if len >= 2 { key[1] } else { 0 };
    //         let b3 = if len >= 3 { key[2] } else { 0 };
    //         let b4 = 0;
    //         let int1 = ((b4 as u32) << 24) | ((b3 as u32) << 16) | ((b2 as u32) << 8) | (b1 as u32);
    //         return int1 as u64;
    //     }
    //
    //     let int1 = unsafe {  *(ptr as *const u32) };
    //     if len < 8 {  // 4..<8
    //         let b1 = if len >= 6 { key[4] } else { 0 };
    //         let b2 = if len >= 6 { key[5] } else { 0 };
    //         let b3 = if len >= 7 { key[6] } else { 0 };
    //         let b4 = if len >= 8 { key[7] } else { 0 };
    //         let int2 = ((b4 as u32) << 24) | ((b3 as u32) << 16) | ((b2 as u32) << 8) | (b1 as u32);
    //         return ((int2 as u64) << 32) | (int1 as u64);
    //     }
    //
    //     let int2 = unsafe { *(ptr.offset(4) as *const u32) };
    //     let int1_2 = ( ((int2 as u64) << 32) | (int1 as u64) );
    //
    //     if len < 12 { // 8..<12
    //         let b1 = if len >= 9 { key[8] } else { 0 };
    //         let b2 = if len >= 10 { key[9] } else { 0 };
    //         let b3 = if len >= 11 { key[10] } else { 0 };
    //         let b4 = if len >= 12 { key[11] } else { 0 };
    //         let int3 = ((b4 as u32) << 24) | ((b3 as u32) << 16) | ((b2 as u32) << 8) | (b1 as u32);
    //         return (int3 as u64) ^ int1_2 ;
    //     }
    //
    //     let int3 = unsafe { *(ptr.offset(8) as *const u32) };
    //     let int4 = if len < 16 { // 12..<16
    //         let b1 = if len >=13 { key[12] } else { 0 };
    //         let b2 = if len >= 14 { key[13] } else { 0 };
    //         let b3 = if len >= 15 { key[14] } else { 0 };
    //         let b4 = if len >= 16 { key[15] } else { 0 };
    //         ((b4 as u32) << 24) | ((b3 as u32) << 16) | ((b2 as u32) << 8) | (b1 as u32)
    //     }
    //     else {
    //         unsafe { *(ptr.offset(12) as *const u32) }
    //     };
    //     let int3_4 = ((int3 as u64) << 32) | (int4 as u64);
    //     int1_2 ^ int3_4
    //
    // }

    // #[inline]
    // fn compute_slot(hash: u64) -> u16 {
    //     let p: &[u8; 8] = unsafe { &*(&hash as *const u64 as *const [u8; 8]) };
    //
    //     let slot0 = ((p[0] ^ p[3] ^ p[6] ) & 0x1F) as u16;
    //     let slot1 = ((p[1] ^ p[4] ^ p[7] ) & 0x1F) as u16;
    //     let slot2 = ((p[2] ^ p[5]) & 0x1F) as u16;
    //
    //     slot0 | (slot1 << 5) | (slot2 << 10)
    // }

    fn get_or_insert(&mut self, key: &str) -> &mut Item {

        // let hash = Self::compute_hash(key.as_bytes());
        let hash = Self::compute_hash(key.as_bytes());

        let mut idx_in_slots: usize = ((hash % 8192) * 4) as usize;
        loop {
            let index: usize = self.indexes[idx_in_slots] as usize;
            if index == u16::MAX as usize {
                let index = self.values.len();
                self.indexes[idx_in_slots] = index as u16;
                let entry = Entry {
                    key: key.to_string(),
                    hash: hash,
                    item: Item {
                        min: 0,
                        max: 0,
                        count: 0,
                        sum: 0
                    }
                };
                self.values.push(entry);
                return &mut self.values[index].item;
            }
            else if self.values[index].hash == hash /* && self.values[index].key == key */{
                // if self.values[index].key != key {
                //     panic!("hash collision for {}, {}", self.values[index].key, key);
                // }
                return &mut self.values[index].item;
            }
            else {
                idx_in_slots += 1; // continue search
                self.loses += 1;
            }
        }
    }

    fn iter(&self) -> SimpleHashIterator {
        SimpleHashIterator {
            hashmap: self,
            idx: 0
        }
    }
}

struct SimpleHashIterator<'a> {
    hashmap: &'a SimpleHashMap,
    idx: usize
}

impl <'a> IntoIterator for &'a SimpleHashMap {
    type Item = (&'a String, &'a Item);
    type IntoIter = SimpleHashIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleHashIterator {
            hashmap: self,
            idx: 0
        }
    }
}

impl <'a> Iterator for SimpleHashIterator<'a> {
    type Item = (&'a String, &'a Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.hashmap.values.len() {
            let key = &self.hashmap.values[self.idx].key;
            let value = &self.hashmap.values[self.idx].item;
            self.idx += 1;
            Some((key, value))
        }
        else {
            None
        }
    }
}

#[inline(never)]
pub fn ver10() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;

    let mmap = unsafe { Mmap::map(&file)? };
    let buf = mmap.as_ref();


    let mut hash: SimpleHashMap = SimpleHashMap::new();
    // let mut hash:FxHashMap<String, Item> = FxHashMap::with_capacity_and_hasher(16384, rustc_hash::FxBuildHasher::default());

    // let mut sum = 0;
    let mut callback = |name: &[u8], value: i16| {  // ~13.5s 60%
        let item = hash.get_or_insert(unsafe { core::str::from_utf8_unchecked(name) });
        item.count += 1;
        item.sum += value as i32;
        item.min = item.min.min(value);
        item.max = item.max.max(value);
    };

    let v01 = u8x64::splat(b';');
    let v02 = u8x64::splat(b'\n');

    enum State {
        BEGIN, POS1
    }
    let mut state2: State = State::BEGIN;     // BEGIN, POS1
    let mut line_begin: usize = 0usize;  // always valid
    let mut pos1: usize = 0;        // when state2 is POS1
    let mut cursor: usize = 0;      // if block_is_tail, cursor can scroll forward, otherwise, cursor is always the head of the block
    let mut block_is_tail: bool = false;
    let mut simd_mask: u64 = {      // when block_is_tail == false, simd_mask is the search mask
        let v1: u8x64 = u8x64::from_slice(buf);        // 64 bytes
        (v1.simd_eq(v01) | v1.simd_eq(v02)).to_bitmask()
    };

    loop {

        let pos: usize =  loop {
            if likely(block_is_tail == false) {    // 1. simd_block
                let first = simd_mask.trailing_zeros(); // 0..64
                if likely(first < 64) {  // 1.1 having a match
                    simd_mask &= !(1 << first);
                    break cursor + first as usize;      // break result 1: from simd_block
                } else {  // 1.2 load next block and continue loop
                    cursor += 64;
                    if likely(cursor + 64 <= buf.len()) { // 1.2.1 load next u8x64 block
                        let v1 = u8x64::from_slice(&buf[cursor..cursor + 64]);
                        simd_mask = (v1.simd_eq(v01) | v1.simd_eq(v02)).to_bitmask();
                    } else {    // 1.2.2 load the tail block
                        block_is_tail = true;
                    }
                    continue;
                }
            } else {  // 2. tail block
                match memchr2(b';', b'\n', &buf[cursor..]) {
                    Some(index) => {
                        let result = cursor + index;
                        cursor += index + 1;
                        break result;   // break result 2: from tail block
                    }
                    None => {
                        unreachable!("tail block should always have a match");
                    }
                }
            }
        };

        match state2 {
            State::BEGIN => {
                pos1 = pos;
                state2 = State::POS1;
            }
            State::POS1 => {
                let pos2 = pos;
                callback(&buf[line_begin..pos1], parse_value(&buf[pos1+1..pos2]));
                state2 = State::BEGIN;
                line_begin = pos2 + 1;
            }
        }

        if unlikely( pos + 1 == buf.len() ) {
            break;
        }

    }

    let result = hash.iter().map(|(name, item)| {
        (name.clone(), (item.min as f32/ 10.0, item.max as f32 / 10.0, item.sum as f32 / item.count as f32 / 10.0))
    }).collect();

    println!("total loses: {}", hash.loses);
    Ok(result)
}
