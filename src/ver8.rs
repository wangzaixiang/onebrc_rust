use crate::MEASUREMENT_FILE;
use memchr::memchr2;
use memmap2::Mmap;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::intrinsics::{likely, unlikely};

use std::simd::cmp::SimdPartialEq;
use std::simd::u8x64;

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

#[inline(never)]
pub fn ver8() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;

    let mmap = unsafe { Mmap::map(&file)? };
    let buf = mmap.as_ref();

    struct Item {
        min: i16,
        max: i16,
        count: u32,
        sum: i32,
    }
    // let mut hash = HashMap::with_capacity_and_hasher(16384, fasthash::spooky::Hash64);
    let mut hash:FxHashMap<String, Item> = FxHashMap::with_capacity_and_hasher(16384, rustc_hash::FxBuildHasher::default());

    // let mut sum = 0;
    let mut callback = |name: &[u8], value: i16| {  // ~13.5s 60%
        match hash.get_mut(unsafe { core::str::from_utf8_unchecked(name) }) {
            Some(item) => {
                item.count += 1;
                item.sum += value as i32;
                item.min = item.min.min(value);
                item.max = item.max.max(value);
            }
            None => {
                let item = Item {
                    min: value,
                    max: value,
                    count: 1,
                    sum: value as i32,
                };
                hash.insert(unsafe { core::str::from_utf8_unchecked(name) }.to_string(), item);
            }
        }
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

    Ok(result)
}
