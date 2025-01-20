use crate::MEASUREMENT_FILE;
use std::collections::HashMap;
use std::intrinsics::{likely, unlikely};
use std::mem::transmute;

use memmap2::{Mmap, MmapOptions};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdInt;
use std::simd::{i16x4, i8x4, u8x64};

unsafe fn parse_value_simd(val1: &[u8]) -> i16 {

    let pad_1 = 8 - val1.len() as isize;

    let ptr1 = val1.as_ptr().offset( -pad_1);

    let l1 = u64::from_be_bytes( *(ptr1 as *const [u8;8]) );

    // clear top pad_1 * 8 bits of l1
    let l1 = l1 & (u64::MAX >> (pad_1 * 8));

    let sign_1 = if val1[0] == b'-' { -1 } else { 1 };

    let v2: i8x4 = transmute(l1 as u32);

    let v2: i16x4  = v2.cast();

    let scale: i16x4 = i16x4::from_array([ 1, 0, 10, 100 ]);
    let mask = v2.simd_ge(i16x4::splat('0' as i16));
    let v2 = mask.select(v2, i16x4::splat(b'0' as i16));
    let sub = v2 - i16x4::splat(b'0' as i16);      // (c - '0')
    let mul = sub * scale;                                // (c - '0') * scale

    let mul_2 = mul.rotate_elements_right::<2>();       // 100 + 0, 10 + 1
    let sum = mul + mul_2;

    let sum_2 = sum.rotate_elements_right::<1>();       // 100 + 0 + 10 + 1
    let sum = sum + sum_2;

    let array: &[i16;4] = & transmute(sum);
    sign_1 * array[3]
}



struct FileReader {
    _mmap: Mmap,         // const
    // length: usize,      // const
    // buf: *const u8,     // const
}

impl FileReader {

    fn new(mmap: Mmap) -> FileReader {
        let length = mmap.len();
        let buf = mmap.as_ptr();
        let reader = FileReader {
            _mmap: mmap,
        };

        // preload each pages make code fast 1891ms - 677ms = ~ 1.2s TODO why?
        reader.preload_pages(buf, length);
        reader
    }

    #[inline(never)]
    fn preload_pages(&self, buf: *const u8, len: usize) -> i64 {
        let mut sum = 0i64;
        for i in (0..len).step_by(2048*1024) {
            sum ^=  unsafe { *(buf.add(i) as *const i64) };
        }
        if sum == 0 {   // avoid optimize
            println!("sum: {}", sum);
        }
        sum
    }

    /// load 128 bytes, 3 ~ 16 lines,
    /// (pos1: u128, pos2: u128, pos1_count: usize, pos2_count: usize)
    #[inline]
    fn load_current_64(&self, buf: *const u8, cursor: usize) -> (u64, u64) {
        let ptr = unsafe { buf.add(cursor) };
        let v1 = u8x64::from_slice(unsafe { std::slice::from_raw_parts(ptr, 64) });

        let pos1 = v1.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
        let pos2 = v1.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;
        (pos1, pos2)
    }

    #[inline]
    fn build_line(&self, buf: *const u8, line_begin: usize, pos1: usize, pos2: usize) -> (&[u8], &[u8]) {

        let start_ptr = unsafe { buf.add(line_begin ) };
        let pos1_ptr = unsafe { buf.add(pos1) };

        let key: &[u8] = unsafe { std::slice::from_raw_parts( start_ptr, pos1- line_begin) };  // pos1_ptr.sub_ptr(start_ptr)) };
        let value: &[u8] = unsafe { std::slice::from_raw_parts( pos1_ptr.add(1), pos2-pos1-1 ) }; //pos2_ptr.sub_ptr(pos1_ptr)-1) };
        (key, value)
    }

    #[inline]
    fn get_and_clear(pos: &mut u64) -> usize {
        let at = pos.trailing_zeros();
        *pos &= !1 << at;
        at as usize
    }

    #[inline(never)]
    fn scan_loop(&self, aggr: &mut AggrInfo) {
        let mut line_start = 0usize;    //
        let mut last_pos1: u64 = 0;

        let buffer = self._mmap.as_ptr();   // force to register
        let buffer_len = self._mmap.len();
        let mut cursor: usize = 0;                   // force to register
        let mut eof = false;

        // let hashes = &mut aggr.hashes;  // force to register
        let aggregator = aggr;    // force to register


        while !eof {
            if likely(cursor + 64 <= buffer_len)  {
                let (mut pos1, mut pos2) = self.load_current_64(buffer, cursor);    //  1 ~ 8 lines

                let mut lines = pos2.count_ones();
                while likely(lines >= 3) {
                    let l1_pos1 = if last_pos1 != 0 { cursor - 64 + Self::get_and_clear(&mut last_pos1) } else { cursor + Self::get_and_clear(&mut pos1) };
                    let l1_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    let l1_key = unsafe { std::slice::from_raw_parts( buffer.add(line_start), l1_pos1 - line_start) };
                    let l1_val = unsafe { std::slice::from_raw_parts( buffer.add(l1_pos1 + 1), l1_pos2 - l1_pos1 - 1) };

                    let l1_key_a = u64::from_le_bytes(unsafe { *(buffer.add(line_start) as *const [u8; 8]) });
                    let l1_key_b = u64::from_le_bytes(unsafe { *(buffer.add(line_start).add(8) as *const [u8; 8]) });

                    let l1_len = l1_pos1 - line_start;
                    let len_a = l1_len.min(8);
                    let len_b = (l1_len - len_a).min(8);
                    let l1_key_a = l1_key_a & MASKS[len_a];
                    let l1_key_b = l1_key_b & MASKS[len_b];
                    line_start = l1_pos2 + 1;

                    let l2_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    let l2_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    let l2_key = unsafe { std::slice::from_raw_parts( buffer.add(line_start), l2_pos1 - line_start) };
                    let l2_val = unsafe { std::slice::from_raw_parts( buffer.add(l2_pos1 + 1), l2_pos2 - l2_pos1 - 1) };

                    let l2_key_a = u64::from_le_bytes(unsafe { *(buffer.add(l1_pos2+1) as *const [u8; 8]) });
                    let l2_key_b = u64::from_le_bytes(unsafe { *(buffer.add(l1_pos2+1).add(8) as *const [u8; 8]) });

                    let l2_len = l2_pos1 - l1_pos2 - 1;
                    let len_a = l2_len.min(8);
                    let len_b = (l2_len - len_a).min(8);
                    let l2_key_a = l2_key_a & MASKS[len_a];
                    let l2_key_b = l2_key_b & MASKS[len_b];
                    line_start = l2_pos2 + 1;

                    let l3_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    let l3_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    let l3_key = unsafe { std::slice::from_raw_parts( buffer.add(line_start), l3_pos1 - line_start) };
                    let l3_val = unsafe { std::slice::from_raw_parts( buffer.add(l3_pos1 + 1), l3_pos2 - l3_pos1 - 1) };

                    let l3_key_a = u64::from_le_bytes(unsafe { *(buffer.add(l2_pos2+1) as *const [u8; 8]) });
                    let l3_key_b = u64::from_le_bytes(unsafe { *(buffer.add(l2_pos2+1).add(8) as *const [u8; 8]) });

                    let l3_len = l3_pos1 - l2_pos2 - 1;
                    let len_a = l3_len.min(8);
                    let len_b = (l3_len - len_a).min(8);
                    let l3_key_a = l3_key_a & MASKS[len_a];
                    let l3_key_b = l3_key_b & MASKS[len_b];
                    line_start = l3_pos2 + 1;

                    // let (key1, val1) = self.build_line(buffer, line_start, l1_pos1, l1_pos2);
                    // let (key2, val2) = self.build_line(buffer, l1_pos2 + 1, l2_pos1, l2_pos2);
                    // let (key3, val3) = self.build_line(buffer, l2_pos2 + 1, l3_pos1, l3_pos2);

                    // let (key_a_1, key_b_1, key_a_2, key_b_2, key_a_3, key_b_3) = str_to_hash_normal_3(key1, key2, key3);
                    // let (key_a_1, key_b_1) = str_to_hash_normal(key1);
                    // let (key_a_2, key_b_2) = str_to_hash_normal(key2);
                    // let (key_a_3, key_b_3) = str_to_hash_normal(key3);

                    // let (v1, v2, v3) = unsafe { parse_values_3(val1, val2, val3) };
                    let v1 = unsafe { parse_value_simd(l1_val) };
                    let v2 = unsafe { parse_value_simd(l2_val) };
                    let v3 = unsafe { parse_value_simd(l3_val) };

                    Self::save_item(aggregator, l1_key, l1_key_a, l1_key_b, v1);
                    Self::save_item(aggregator, l2_key, l2_key_a, l2_key_b, v2);
                    Self::save_item(aggregator, l3_key, l3_key_a, l3_key_b, v3);

                    lines -= 3;
                }

                while likely(lines > 0) {
                    let l1_pos1 = if last_pos1 != 0 { cursor - 64 + Self::get_and_clear(&mut last_pos1)  }  else { cursor + Self::get_and_clear(&mut pos1)};
                    let l1_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    let (key1, val1) = self.build_line(buffer, line_start, l1_pos1, l1_pos2);

                    let (key_a_1, key_b_1) = str_to_hash_normal(key1);

                    let v1 = unsafe { parse_value_simd(val1) };
                    Self::save_item(aggregator, key1, key_a_1, key_b_1, v1);
                    lines -= 1;
                    line_start = l1_pos2 + 1;
                }
                last_pos1 = pos1;
                cursor += 64;
            }
            else {
                println!("process last block");
                cursor = buffer_len;
                eof = true;
            }
        }
    }

    #[inline]
    fn save_item(aggregator: &mut AggrInfo, key: &[u8], key_a: u64, key_b: u64, value: i16) {
        let hash = {
            let p0 = key_a;
            let p3 = key_b;
            let p1 = key_a >> 20;
            let p4 = key_b >> 20;
            let p2 = key_a >> 40;
            let p5 = key_b >> 40;
            (p0 ^ p1) ^ (p2 ^ p3) ^ (p4 ^ p5)
        };

        let hash: usize = (hash % (1024*1024)) as usize;
        // let item = &mut self.hashes[hash];
        let item = unsafe { aggregator.hashes.get_unchecked_mut(hash) };
        if likely(item.key_a == key_a && item.key_b == key_b) {
            debug_assert_eq!(item.key, key);
            item.count += 1;
            item.sum += value as i32;
            item.min = item.min.min(value as i32);
            item.max = item.max.max(value as i32);
            return;
        }
        else {
            Self::slow_save(aggregator, key, key_a, key_b, value, hash);
        }
    }

    #[inline(never)]
    fn slow_save(aggragator: &mut AggrInfo, key: &[u8], key_a: u64, key_b: u64, value: i16, from: usize) {
        for i in from..from + 1024 {    // search at most 1024 entry
            let item: &mut AggrItem = &mut aggragator.hashes[i];
            if unlikely(item.key_a == key_a && item.key_b == key_b) {
                debug_assert_eq!(item.key, key);
                item.count += 1;
                item.sum += value as i32;
                item.min = item.min.min(value as i32);
                item.max = item.max.max(value as i32);
                return;
            } else if likely(item.key.is_empty()) {
                item.key_a = key_a;
                item.key_b = key_b;
                item.key = key.to_vec();
                item.count = 1;
                item.sum = value as i32;
                item.min = value as i32;
                item.max = value as i32;
                return;
            }
        }
        panic!("can't find a entry, the hash is very bad");
    }

}

const MASKS: [u64;9] = [
    0,
    0x0000_0000_0000_00FF,
    0x0000_0000_0000_FFFF,
    0x0000_0000_00FF_FFFF,
    0x0000_0000_FFFF_FFFF,
    0x0000_00FF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
    0x00FF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
];

// TODO can u128 be faster?
#[inline]
fn str_to_hash_normal(name: &[u8] ) -> (u64, u64) {
    // normal version
    let len = name.len();
    let ptr1 = name.as_ptr();

    let key_a: u64 = u64::from_le_bytes(unsafe { *(ptr1 as *const [u8; 8]) });
    let key_b: u64 = u64::from_le_bytes(unsafe { *(ptr1.add(8) as *const [u8; 8]) });

    let len_a = len.min(8);
    let len_b = (len - len_a).min(8);

    let key_a = key_a & MASKS[len_a];        // test base: 1.46s
    let key_b = key_b & MASKS[len_b];

    (key_a, key_b)
}

#[test]
fn test_str_to_hash3(){

    let name = "12345678ABCDEFGH+-=;";
    let name1 = &name[0..5];
    let name2 = &name[0..10];
    let name3 = &name[0..20];

    let (name1_a, name1_b, name2_a, name2_b, name3_a, name3_b) = str_to_hash_simd_3(name1.as_bytes(), name2.as_bytes(), name3.as_bytes());

    println!("name1_a: {:x}, name1_b: {:x}, name2_a: {:x}, name2_b: {:x}, name3_a: {:x}, name3_b: {:x}", name1_a, name1_b, name2_a, name2_b, name3_a, name3_b);

}


#[derive(Clone)]
#[repr(C, align(64))]
struct AggrItem {
    key_a:  u64,     // 0
    key_b:  u64,     // 8
    min:    i32,     // 16  most write from min to sum
    max:    i32,     // 20
    count:  u32,     // 24
    sum:    i32,     // 28
    key: Vec<u8>,    // 32
}

struct AggrInfo {
    hashes: Vec<AggrItem>
}

impl AggrInfo {

    fn new_item() -> AggrItem {
        AggrItem {
            key_a: 0,
            key_b: 0,
            key: Vec::new(),
            min: 0,
            max: 0,
            count: 0,
            sum: 0
        }
    }

    fn new() -> AggrInfo {
        let hashes = vec![Self::new_item(); 1024*1024 + 1024];

        AggrInfo {
            hashes
        }
    }


}


#[inline(never)]
// based on ver12
pub fn ver18() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {     // 8.96s

    let file = std::fs::File::open(MEASUREMENT_FILE)?;

    // let mmap = unsafe { Mmap::map(&file)? };
    let mmap = unsafe {
        MmapOptions::new().huge(Some(21)).populate()
            .map(&file)?
    };

    let reader = FileReader::new(mmap);

    // let (pos1, pos2, pos1_count, pos2_count) = unsafe { reader.load_current_128(0) };

    // println!("pos1: {:x}, pos2: {:x}, pos1_count: {}, pos2_count: {}", pos1, pos2, pos1_count, pos2_count);
    let mut aggr = AggrInfo::new();
    reader.scan_loop(&mut aggr);

    check_result(&aggr);
    // check_result(&aggr);

    Ok( HashMap::new() )
}

fn check_result(aggr: &AggrInfo) {
    let mut count = 0;
    for i in 0.. aggr.hashes.len() {
        let item = & aggr.hashes[i];
        if !item.key.is_empty() {
            count += 1;
            let is_dupicated = if i> 0 {
                aggr.hashes[i-1].key_a != 0
            }
            else {
                false
            };
            let key = unsafe { std::str::from_utf8_unchecked( item.key.as_slice() ) };
            if is_dupicated {
                println!("{};\t{}\t{}", key, i, is_dupicated);
            }
        }
    }
    assert_eq!(count, 413);
    println!("total entries: {}", count);
}
