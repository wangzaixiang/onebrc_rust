use crate::MEASUREMENT_FILE;
use std::collections::HashMap;
use std::intrinsics::{likely, unlikely};
use std::mem::transmute;

use memmap2::{Mmap, MmapOptions};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::{SimdInt, SimdUint};
use std::simd::{i16x16, i16x4, i16x8, i8x16, i8x2, i8x32, i8x8, simd_swizzle, u64x2, u64x4, u8x16, u8x64, u8x8};
use std::slice::from_raw_parts;

fn parse_value_u8x8(val: u8x8) -> i16 {
    // if val1[4] == b'-' || val1[5] == b'-' the number is negative
    let signed: i8x2 = unsafe { transmute( simd_swizzle!(val, [3, 4]) ) };
    let signed = signed.simd_eq(i8x2::splat(b'-' as i8)).to_int();
    let signed: i16 = unsafe { transmute(signed) };
    let signed = if signed == 0 { 1 } else { -1 };

    let val = val.cast::<i16>();

    let is_digit = val.simd_ge(i16x8::splat(b'0' as i16)) & val.simd_le(i16x8::splat(b'9' as i16));

    let nums: i16x8 = is_digit.select(val, i16x8::splat(b'0' as i16)) - i16x8::splat(b'0' as i16);
    let mul_scale = nums * i16x8::from_array([0, 0, 0, 0, 100, 10, 0, 1]);
    let sum = mul_scale + mul_scale.rotate_elements_right::<2>();
    let sum: i16x8 = sum + sum.rotate_elements_right::<1>();
    signed * sum[7]
}

// vals: low -> [ ?, ?, ?,  -, 3, 2, ., 9] <- high
#[inline]
fn parse_numbers_batch4(val1: u8x8, val2: u8x8, val3: u8x8, val4: u8x8) -> (i16, i16, i16, i16){

    let vector: i8x32 = unsafe {
        transmute( u64x4::from_array([ transmute(val1), transmute(val2), transmute(val3), transmute(val4) ]) )};

    // if val1[4] == b'-' || val1[5] == b'-' the number is negative
    let signed: i8x8 = simd_swizzle!(vector, [3, 4, 11, 12, 19, 20, 27, 28]);
    let signed = signed.simd_eq(i8x8::splat(b'-' as i8)).to_int();
    let signed: i16x4 = unsafe { transmute(signed) };
    let signed: i16x4 = signed.simd_eq(i16x4::splat(0))
        .select(i16x4::splat(1), i16x4::splat(-1)).cast::<i16>() ;

    let hight_parts: i8x16 = simd_swizzle!(vector, [ 4, 5, 6, 7, 12, 13, 14, 15, 20, 21, 22, 23, 28, 29, 30, 31]);

    let is_digit = hight_parts.simd_ge(i8x16::splat(b'0' as i8)) & hight_parts.simd_le(i8x16::splat(b'9' as i8));

    let nums: i16x16 = is_digit.select(hight_parts, i8x16::splat(b'0' as i8)).cast::<i16>() - i16x16::splat(b'0' as i16);
    let mul_scale = nums * i16x16::from_array([100, 10, 0, 1, 100, 10, 0, 1, 100, 10, 0, 1,  100, 10, 0, 1]);
    let sum = mul_scale + mul_scale.rotate_elements_right::<2>();
    let sum: i16x16 = sum + sum.rotate_elements_right::<1>();
    (signed[0] * sum[3], signed[1] * sum[7], signed[2] * sum[11], signed[3] * sum[15])
}


// fn process_block64(block0: u8x16, block1: u8x16, block2: u8x16, block3: u8x16) -> (u64, u64) {
//     // let pos1 = block.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
//     // let pos2 = block.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;
//     // (pos1, pos2)
//
//     let pos1_0 = block0.simd_eq(u8x16::splat(b';')).to_bitmask() as u64;
//     let pos2_0 = block0.simd_eq(u8x16::splat(b'\n')).to_bitmask() as u64;
//     let pos1_1 = block1.simd_eq(u8x16::splat(b';')).to_bitmask() as u64;
//     let pos2_1 = block1.simd_eq(u8x16::splat(b'\n')).to_bitmask() as u64;
//     let pos1_2 = block2.simd_eq(u8x16::splat(b';')).to_bitmask() as u64;
//     let pos2_2 = block2.simd_eq(u8x16::splat(b'\n')).to_bitmask() as u64;
//     let pos1_3 = block3.simd_eq(u8x16::splat(b';')).to_bitmask() as u64;
//     let pos2_3 = block3.simd_eq(u8x16::splat(b'\n')).to_bitmask() as u64;
//
//     (
//         pos1_0 | (pos1_1 << 16) | (pos1_2 << 32) | (pos1_3 << 48),
//         pos2_0 | (pos2_1 << 16) | (pos2_2 << 32) | (pos2_3 << 48),
//     )
//
// }

struct FileReader {
    _mmap: Mmap,         // const
}

impl FileReader {

    fn new(mmap: Mmap) -> FileReader {
        let reader = FileReader {
            _mmap: mmap,
        };

        // preload each pages make code fast 1891ms - 677ms = ~ 1.2s TODO why?
        reader.preload_pages();
        reader
    }

    #[inline(never)]
    fn preload_pages(&self) -> i64 {
        let mut sum = 0i64;
        for i in (0..self._mmap.len()).step_by(2048*1024) {
            sum ^=  unsafe { *(self._mmap.as_ptr().add(i) as *const i64) };
        }
        if sum == 0 {   // avoid optimize
            println!("sum: {}", sum);
        }
        sum
    }



    #[inline]
    fn get_and_clear(pos: &mut u64) -> usize {
        let at = pos.trailing_zeros();
        *pos &= !1 << at;
        at as usize
    }

    #[inline]
    fn buffer(&self, offset: usize) -> *const u8 {
        unsafe { self._mmap.as_ptr().add(offset) }
    }

    #[inline]
    fn length(&self) -> usize {
        self._mmap.len()
    }


    #[inline]
    fn preload_key_u64x2(ptr: *const u8) -> u64x2 {
        unsafe { transmute( u8x16::from_slice( from_raw_parts(ptr, 16) ) ) }
    }

    #[inline]
    fn preload_val_u8x8(ptr: *const u8) -> u8x8 {
        unsafe { u8x8::from_slice( from_raw_parts(ptr, 8) ) }
    }

    #[inline(never)]
    fn scan_loop(&self, aggregator: &mut AggrHashTable) {
        let mut cursor: usize = 0;                  // force register, it may add a -x offset
        let mut last_delimiter1_pos = 0u64;       // force register
        let mut outer_pos1 ;            // force register
        let mut outer_pos2 ;            // force register
        let mut line_start = 0usize;    // the next line's start position

        let block : u8x64 = u8x64::from_slice(unsafe { from_raw_parts(self.buffer(0), 64) });
        outer_pos1 = block.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
        outer_pos2 = block.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;

        while likely(cursor < self.length() ) {
            if likely( (cursor + 64) <= self.length() ) {

                let mut inner_pos1 = outer_pos1;
                let mut inner_pos2 = outer_pos2;

                let mut lines = inner_pos2.count_ones();

                let block = u8x64::from_slice(unsafe { from_raw_parts(self.buffer(cursor + 64), 64) });
                outer_pos1 = block.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
                outer_pos2 = block.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;

                while likely(lines >= 4) {  // 4..=8
                    self.process_batch_4(aggregator, cursor, last_delimiter1_pos, &mut line_start, &mut inner_pos1, &mut inner_pos2);
                    last_delimiter1_pos = 0;
                    lines -= 4;
                }


                while likely(lines > 0) {
                    self.process_single(aggregator, cursor, last_delimiter1_pos, &mut line_start, &mut inner_pos1, &mut inner_pos2);
                    last_delimiter1_pos = 0;
                    lines -= 1;
                }

                last_delimiter1_pos = inner_pos1;
                cursor += 64;
            }
            else {
                println!("process last block");
                cursor = self.length();
            }
        }
        // println!("total lines: {}", total_lines);
    }

    // all process_single 5.75s

    fn process_single(&self, aggregator: &mut AggrHashTable, cursor: usize, last_delimiter1_pos: u64, line_start: &mut usize, inner_pos1: &mut u64, inner_pos2: &mut u64) {
        let l1_preload_key: u64x2 = Self::preload_key_u64x2(self.buffer(*line_start));
        let l1_pos1 = if last_delimiter1_pos != 0 { cursor - 64 + last_delimiter1_pos.trailing_zeros() as usize } else { cursor + Self::get_and_clear(inner_pos1) };
        let l1_pos2 = cursor + Self::get_and_clear(inner_pos2);

        let l1_preload_val = Self::preload_val_u8x8(self.buffer(l1_pos2 - 8));
        let val1 = parse_value_u8x8(l1_preload_val);

        let key1_hash = truncate_key_simd(l1_preload_key, l1_pos1 - *line_start);

        aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(*line_start), l1_pos1 - *line_start) }, key1_hash, val1);

        *line_start = l1_pos2 + 1;
    }

    fn process_batch_4(&self, aggregator: &mut AggrHashTable, cursor: usize, last_delimiter1_pos: u64, line_start: &mut usize, inner_pos1: &mut u64, inner_pos2: &mut u64) {
        let l1_pos1 = if last_delimiter1_pos != 0 { cursor - 64 + last_delimiter1_pos.trailing_zeros() as usize } else { cursor + Self::get_and_clear(inner_pos1) };
        let l1_pos2 = cursor + Self::get_and_clear(inner_pos2);

        // l2: (l1_pos2+1, l2_pos1, l2_pos2)
        let l2_pos1 = cursor + Self::get_and_clear(inner_pos1);
        let l2_pos2 = cursor + Self::get_and_clear(inner_pos2);
        debug_assert!(l1_pos2 > l1_pos1);
        debug_assert!(l1_pos2 < l2_pos1);

        // l3: (l2_pos2+1, l3_pos1, l3_pos2)
        let l3_pos1 = cursor + Self::get_and_clear(inner_pos1);
        let l3_pos2 = cursor + Self::get_and_clear(inner_pos2);

        let l4_pos1 = cursor + Self::get_and_clear(inner_pos1);
        let l4_pos2 = cursor + Self::get_and_clear(inner_pos2);

        // preload memory
        let l1_preload_key: u64x2 = Self::preload_key_u64x2(self.buffer(*line_start));
        let l2_preload_key: u64x2 = Self::preload_key_u64x2(self.buffer(l1_pos2 + 1));
        let l3_preload_key: u64x2 = Self::preload_key_u64x2(self.buffer(l2_pos2 + 1));
        let l4_preload_key: u64x2 = Self::preload_key_u64x2(self.buffer(l3_pos2 + 1));

        let (val1, val2, val3, val4) = {
            let l1_preload_val = Self::preload_val_u8x8(self.buffer(l1_pos2 - 8));
            let l2_preload_val = Self::preload_val_u8x8(self.buffer(l2_pos2 - 8));
            let l3_preload_val = Self::preload_val_u8x8(self.buffer(l3_pos2 - 8));
            let l4_preload_val = Self::preload_val_u8x8(self.buffer(l4_pos2 - 8));

            parse_numbers_batch4(l1_preload_val, l2_preload_val, l3_preload_val, l4_preload_val)
        };

        let l1_long_hash = truncate_key_simd(l1_preload_key, l1_pos1 - *line_start);
        let l2_long_hash = truncate_key_simd(l2_preload_key, l2_pos1 - l1_pos2 - 1);
        let l3_long_hash = truncate_key_simd(l3_preload_key, l3_pos1 - l2_pos2 - 1);
        let l4_long_hash = truncate_key_simd(l4_preload_key, l4_pos1 - l3_pos2 - 1);

        aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(*line_start), l1_pos1 - *line_start) }, l1_long_hash, val1);
        aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(l1_pos2 + 1), l2_pos1 - l1_pos2 - 1) }, l2_long_hash, val2);
        aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(l2_pos2 + 1), l3_pos1 - l2_pos2 - 1) }, l3_long_hash, val3);
        aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(l3_pos2 + 1), l4_pos1 - l3_pos2 - 1) }, l4_long_hash, val4);

        *line_start = l4_pos2 + 1;
    }

}
#[inline]
fn truncate_key_simd(key: u64x2, len: usize) -> (u64,u64) {
    let key: u8x16 = unsafe { transmute(key) };
    let index = u8x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let mask = index.simd_lt(u8x16::splat(len as u8));
    let key = mask.select(key, u8x16::splat(0));
    unsafe { transmute(key) }

    // the following code is slower ~700ms than the above code
    // let len1 = if len > 8 { 8 } else { len };
    // let len2 = len - len1;
    // let len2 = if len2 > 8 { 8 } else { len2 };
    //
    // let low = key[0] & ( u64::MAX >> (64 - len1*8) );
    // let high = key[1];
    // let high = if len2 == 0 { 0 } else { high & ( u64::MAX >> (64 - len2*8) ) };
    // (low, high)

    // let len1 = if len <= 8 { len } else { 8 };
    // let len2 = if len >= 16 { 8 } else if len > 8 { len - 8 } else { 0 };
    //
    // let low = if len1 == 0 { 0 } else { key[0] & (0xFF_FF_FF_FF_FF_FF_FF_FF >> (64 - len1 * 8)) };
    // let high= if len2 == 0 { 0 } else { key[1] & (0xFF_FF_FF_FF_FF_FF_FF_FF >> (64 - len2 * 8)) };
    // (low, high)
}

#[derive(Clone)]
#[repr(C, align(64))]
struct HashEntry {
    key_hash:  (u64, u64),     // 0
    data:      Aggregation,
    key: Vec<u8>,    // 32
}

#[derive(Clone, Copy)]
struct Aggregation {
    min: i32,
    max: i32,
    count: u32,
    sum: i32,
}


impl Aggregation {
    fn new() -> Aggregation {
        Aggregation {
            min: i32::MAX,
            max: i32::MIN,
            count: 0,
            sum: 0,
        }
    }
}

struct AggrHashTable {
    linear_hash_table: Vec<HashEntry>
}

impl AggrHashTable {

    fn new_item() -> HashEntry {
        HashEntry {
            key_hash: (0, 0),
            key: Vec::new(),
            data: Aggregation::new()
        }
    }

    fn new() -> AggrHashTable {
        let hashes = vec![Self::new_item(); 1024*1024 + 1024];

        AggrHashTable {
            linear_hash_table: hashes
        }
    }


    #[inline]
    fn save_item_u64x2(&mut self, key: &[u8], long_hash: (u64, u64), value: i16) {
        let hash_code: usize = Self::compute_hash_code(long_hash) as usize;

        let item = unsafe { self.linear_hash_table.get_unchecked_mut(hash_code) };
        if likely( item.key_hash == long_hash) {  // // change from u64x2 makes 100ms fast, but still requires preload data
            debug_assert_eq!(item.key, key);
            let mut item_values = item.data;
            item_values.count += 1;
            item_values.sum += value as i32;
            item_values.min = item_values.min.min(value as i32);
            item_values.max = item_values.max.max(value as i32);
            item.data = item_values;
            return;
        }
        else if likely(item.key.is_empty()) {
            item.key_hash = long_hash;
            item.key = key.to_vec();
            item.data = Aggregation { sum: value as i32,  count:1, min: value as i32, max: value as i32 };
            return;
        }
        else {
            self.slow_save(key, long_hash, value, hash_code);
        }
    }

    fn compute_hash_code(long_hash: (u64, u64)) -> usize {
        let x1 = long_hash.0 ^ long_hash.1;
        let x2 = (x1 ^ (x1 >> 20) ^ (x1 >> 40)) as u32;
        (x2 % (1 << 20)) as usize
    }

    #[inline(never)]
    fn slow_save(&mut self, key: &[u8], key_hash: (u64, u64), value: i16, from: usize) {
        for i in from..from + 1024 {    // search at most 1024 entry
            let item: &mut HashEntry = &mut self.linear_hash_table[i];
            if unlikely(item.key_hash == key_hash) {
                debug_assert_eq!(item.key, key);
                let mut item_values = item.data;
                item_values.count += 1;
                item_values.sum += value as i32;
                item_values.min = item_values.min.min(value as i32);
                item_values.max = item_values.max.max(value as i32);
                item.data = item_values;
                return;
            } else if likely(item.key.is_empty()) {
                item.key_hash = key_hash;
                item.key = key.to_vec();
                item.data = Aggregation { sum: value as i32,  count:1, min: value as i32, max: value as i32 };
                return;
            }
        }
        panic!("can't find a entry, the hash is very bad");
    }
}


#[inline(never)]
// based on ver12
pub fn ver22() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {     // 8.96s

    let file = std::fs::File::open(MEASUREMENT_FILE)?;

    // let mmap = unsafe { Mmap::map(&file)? };
    let mmap = unsafe {
        MmapOptions::new().huge(Some(21)).populate()
            .map(&file)?
    };

    let reader = FileReader::new(mmap);

    let mut aggr = AggrHashTable::new();
    reader.scan_loop(&mut aggr);

    check_result(&aggr);
    Ok( HashMap::new() )
}

fn check_result(aggr: &AggrHashTable) {
    let mut count = 0;
    let mut dupicated = 0;
    for i in 0.. aggr.linear_hash_table.len() {
        let item = & aggr.linear_hash_table[i];
        if !item.key.is_empty() {
            count += 1;
            let is_dupicated = if i> 0 {
                aggr.linear_hash_table[i-1].key_hash.0 != 0
            }
            else {
                false
            };
            let key = unsafe { std::str::from_utf8_unchecked( item.key.as_slice() ) };
            if is_dupicated {
                dupicated += 1;
                println!("{};\t{}\t{}", key, i, is_dupicated);
            }
        }
    }
    assert_eq!(count, 413);
    println!("total entries: {}, duplicated: {}", count, dupicated);
}