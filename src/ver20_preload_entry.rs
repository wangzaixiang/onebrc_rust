use crate::MEASUREMENT_FILE;
use std::collections::HashMap;
use std::intrinsics::{likely, unlikely};
use std::mem::transmute;

use memmap2::{Mmap, MmapOptions};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::{SimdInt, SimdUint};
use std::simd::{i16x1, i16x16, i16x2, i16x4, i16x8, i32x2, i32x4, i32x8, i64x4, i8x16, i8x2, i8x32, i8x4, i8x8, simd_swizzle, u16x4, u32x4, u32x8, u64x2, u64x4, u8x16, u8x2, u8x32, u8x4, u8x64, u8x8};
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


// parse_value_u8x8_old dont process sign and require caller to handle sign
fn parse_value_u8x8_old(val: u8x8) -> i16 {
    let val = val.cast::<u16>();
    let val = u16x4::from_array([val[4], val[5], val[6], val[7]]);
    let scale: u16x4 = u16x4::from_array([ 100, 10, 0, 1 ]);
    let mask = val.simd_ge(u16x4::splat(b'0' as u16)) ;
    let val = mask.select(val, u16x4::splat(b'0' as u16));
    let sub = val - u16x4::splat(b'0' as u16);      // (c - '0')
    let mul = sub * scale;                                // (c - '0') * scale

    let mul_2 = mul.rotate_elements_right::<2>();       // 100 + 0, 10 + 1
    let sum = mul + mul_2;

    let sum_2 = sum.rotate_elements_right::<1>();       // 100 + 0 + 10 + 1
    let sum = sum + sum_2;
    sum[3] as i16
}

struct FileReader {
    _mmap: Mmap,         // const
}

struct Debug {
    counts: [u64; 8]
}
enum LoopAt {
    Loop1,
    Loop4,
}

impl Debug {
    fn new() -> Debug {
        Debug {
            counts: [0; 8]
        }
    }

    #[cfg(feature = "debug")]
    fn add_count(&mut self, at: LoopAt) {
        match at {
            LoopAt::Loop1 => self.loop1 += 1,
            LoopAt::Loop4 => self.loop4 += 1,
        }
    }

    #[cfg(not(feature = "debug"))]
    fn add_count(&mut self, lines: usize) {
        // self.counts[lines-1] += 1;
    }

    #[cfg(feature = "debug")]
    fn print(&self) {
        println!("loop1: {}, loop4: {}", self.loop1, self.loop4);
    }

    #[cfg(not(feature = "debug"))]
    fn print(&self) {
        // for i in 0..8 {
        //     println!("loop{}: {}", i, self.counts[i]);
        // }
    }
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

    /// load 128 bytes, 3 ~ 16 lines,
    /// (pos1: u128, pos2: u128, pos1_count: usize, pos2_count: usize)
    #[inline]
    fn load_current_64(buffer: *const u8, cursor: usize) -> (u64, u64) {
        let ptr = unsafe { buffer.add(cursor) };
        let v1 = u8x64::from_slice(unsafe { std::slice::from_raw_parts(ptr, 64) });

        let pos1 = v1.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
        let pos2 = v1.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;
        (pos1, pos2)
    }

    #[inline]
    fn parse_block(block: u8x64) -> (u64, u64) {
        let pos1 = block.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
        let pos2 = block.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;
        (pos1, pos2)
    }

    #[inline]
    fn get_and_clear(pos: &mut u64) -> usize {
        let at = pos.trailing_zeros();
        *pos &= !1 << at;
        // *pos &= unsafe { (!1u64).unbounded_shl(at) };       // unbounded_shl 对应好几条指令，不是最佳方案
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

    #[inline]
    unsafe fn entry_ptr(base: *const HashEntry, offset: usize) -> *const u64 {
        & (* base.add(offset)).key_hash as *const (u64, u64) as *const u64
    }

    #[inline(never)]
    fn scan_loop(&self, aggregator: &mut AggrInfo) {
        let mut cursor: usize = 0;                  // force register, it may add a -x offset
        let mut last_delimiter1_pos = 0u64;       // force register
        let mut outer_pos1 = 0u64;            // force register
        let mut outer_pos2 = 0u64;            // force register
        let mut line_start = 0usize;    // the next line's start position

        let debug = Debug::new();
        // let mut total_lines = 0;

        let block : u8x64 = u8x64::from_slice(unsafe { from_raw_parts(self.buffer(0), 64) });
        outer_pos1 = block.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
        outer_pos2 = block.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;

        while likely(cursor < self.length() ) {
            if likely( (cursor + 64) <= self.length() ) {

                let mut inner_pos1 = outer_pos1;
                let mut inner_pos2 = outer_pos2;

                let mut lines = inner_pos2.count_ones();

                // TODO the following code is not optimized in rustc, it saved block into stack and then load it again,
                // it shpuld be optimized by using register only
                let block = u8x64::from_slice(unsafe { from_raw_parts(self.buffer(cursor + 64), 64) });
                outer_pos1 = block.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
                outer_pos2 = block.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;

                while likely(lines >= 4) {  // 4..=8
                    // l1: (line_start, l1_pos1, l1_pos2)
                    let l1_pos1 = if last_delimiter1_pos != 0 { cursor - 64 + Self::get_and_clear(&mut last_delimiter1_pos) } else { cursor + Self::get_and_clear(&mut inner_pos1) };
                    let l1_pos2 = cursor + Self::get_and_clear(&mut inner_pos2);

                    // l2: (l1_pos2+1, l2_pos1, l2_pos2)
                    let l2_pos1 = cursor + Self::get_and_clear(&mut inner_pos1);
                    let l2_pos2 = cursor + Self::get_and_clear(&mut inner_pos2);
                    debug_assert!(l1_pos2 > l1_pos1);
                    debug_assert!(l1_pos2 < l2_pos1);

                    // l3: (l2_pos2+1, l3_pos1, l3_pos2)
                    let l3_pos1 = cursor + Self::get_and_clear(&mut inner_pos1);
                    let l3_pos2 = cursor + Self::get_and_clear(&mut inner_pos2);

                    let l4_pos1 = cursor + Self::get_and_clear(&mut inner_pos1);
                    let l4_pos2 = cursor + Self::get_and_clear(&mut inner_pos2);

                    // preload memory
                    let l1_preload_key: u64x2 = Self::preload_key_u64x2(self.buffer(line_start));
                    let l2_preload_key: u64x2 = Self::preload_key_u64x2(self.buffer(l1_pos2+1));
                    let l3_preload_key: u64x2 = Self::preload_key_u64x2(self.buffer(l2_pos2+1));
                    let l4_preload_key: u64x2 = Self::preload_key_u64x2(self.buffer(l3_pos2+1));

                    let (val1, val2, val3, val4) = {
                        let l1_preload_val = Self::preload_val_u8x8(self.buffer(l1_pos2-8)) ;
                        let l2_preload_val = Self::preload_val_u8x8(self.buffer(l2_pos2-8)) ;
                        let l3_preload_val = Self::preload_val_u8x8(self.buffer(l3_pos2-8)) ;
                        let l4_preload_val = Self::preload_val_u8x8(self.buffer(l4_pos2-8)) ;

                        parse_numbers_batch4(l1_preload_val, l2_preload_val, l3_preload_val, l4_preload_val)
                    };

                    let l1_long_hash =    truncate_key_simd(l1_preload_key, l1_pos1 - line_start);
                    let l2_long_hash =    truncate_key_simd(l2_preload_key, l2_pos1 - l1_pos2 - 1);
                    let l3_long_hash =    truncate_key_simd(l3_preload_key, l3_pos1 - l2_pos2 - 1);
                    let l4_long_hash =    truncate_key_simd(l4_preload_key, l4_pos1 - l3_pos2 - 1);

                    // preload has no effect
                    // let p = aggregator.linar_hash_table.as_ptr();
                    // unsafe {
                    //     asm! {
                    //         "prfm pldl1keep, [{x1}]",
                    //         "prfm pldl1keep, [{x2}]",
                    //         "prfm pldl1keep, [{x3}]",
                    //         "prfm pldl1keep, [{x4}]",
                    //         x1 = in(reg) Self::entry_ptr(p, AggrInfo::compute_hash_code(l1_long_hash) ),
                    //         x2 = in(reg) Self::entry_ptr(p, AggrInfo::compute_hash_code(l2_long_hash) ),
                    //         x3 = in(reg) Self::entry_ptr(p, AggrInfo::compute_hash_code(l3_long_hash) ),
                    //         x4 = in(reg) Self::entry_ptr(p, AggrInfo::compute_hash_code(l4_long_hash) ),
                    //     }
                    // }

                    aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(line_start), l1_pos1 - line_start) }, l1_long_hash, val1);
                    aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(l1_pos2 + 1), l2_pos1 - l1_pos2 - 1) }, l2_long_hash, val2);
                    aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(l2_pos2 + 1), l3_pos1 - l2_pos2 - 1) }, l3_long_hash, val3);
                    aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(l3_pos2 + 1), l4_pos1 - l3_pos2 - 1) }, l4_long_hash, val4);

                    lines -= 4;
                    line_start = l4_pos2 + 1;
                }


                while likely(lines > 0) {
                    let l1_pos1 = if last_delimiter1_pos != 0 { cursor - 64 + Self::get_and_clear(&mut last_delimiter1_pos) } else { cursor + Self::get_and_clear(&mut inner_pos1) };
                    let l1_pos2 = cursor + Self::get_and_clear(&mut inner_pos2);

                    let l1_preload_key: u64x2 = Self::preload_key_u64x2(self.buffer(line_start));
                    let val1 = {
                        let l1_preload_val =  Self::preload_val_u8x8(self.buffer(l1_pos2-8));
                        parse_value_u8x8(l1_preload_val)
                    };

                    let key1_hash = truncate_key_simd(l1_preload_key, l1_pos1 - line_start);

                    aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(line_start), l1_pos1 - line_start) }, key1_hash, val1);

                    lines -= 1;
                    line_start = l1_pos2 + 1;
                }

                if last_delimiter1_pos == 0 {
                    last_delimiter1_pos = inner_pos1;
                }
                else {
                    // already save last_pos on the last loop
                }
                cursor += 64;
            }
            else {
                println!("process last block");
                cursor = self.length();
            }
        }
        // println!("total lines: {}", total_lines);
        debug.print();
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

#[inline]
fn truncate_key_normal(key: u64x2, len: usize) -> u64x2 {
    let len_l = len.min(8);     // 1..=8
    let len_h = (len - len_l).min(8);   // 0..=8
    let key_l = key[0] & MASKS[len_l];
    let key_h = key[1] & MASKS[len_h];
    // let key_l = key[0] & (u64::MAX >> (64 - 8 * len_l));
    // let key_h = key[1] & (u64::MAX >> (64 - 8 * len_h));

    // let key_l = key[0] & (u64::MAX >> (64 - 8 * len_l));
    // let key_h = key[1] & (if len_h == 0 { 0 } else { u64::MAX >> (64 - 8 * len_h) });

    u64x2::from_array([key_l, key_h])

}

#[inline]
fn truncate_key_simd(key: u64x2, len: usize) -> (u64,u64) {
    let key: u8x16 = unsafe { transmute(key) };
    let index = u8x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let mask = index.simd_lt(u8x16::splat(len as u8));
    let key = mask.select(key, u8x16::splat(0));
    unsafe { transmute(key) }
}

#[derive(Clone)]
#[repr(C, align(64))]
struct HashEntry {
    key_hash:  (u64, u64),     // 0
    data:      Aggregation,
    // min:    i32,     // 16  most write from min to sum
    // max:    i32,     // 20
    // count:  u32,     // 24
    // sum:    i32,     // 28
    key: Vec<u8>,    // 32
}

#[derive(Clone, Copy)]
struct Aggregation {
    min: i32,
    max: i32,
    count: u32,
    sum: i32,
}

// #[derive(Clone, Copy)]
// union AggrItemValues {
//     raw: [u8; 16],
//     expanded: (i32, i32, u32, i32), // min, max, count, sum
// }

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

struct AggrInfo {
    linar_hash_table: Vec<HashEntry>
}

impl AggrInfo {

    fn new_item() -> HashEntry {
        HashEntry {
            key_hash: (0, 0),
            key: Vec::new(),
            data: Aggregation::new()
        }
    }

    fn new() -> AggrInfo {
        let hashes = vec![Self::new_item(); 1024*1024 + 1024];

        AggrInfo {
            linar_hash_table: hashes
        }
    }


    #[inline]
    fn save_item_u64x2(&mut self, key: &[u8], long_hash: (u64, u64), value: i16) {
        let hash_code: usize = Self::compute_hash_code(long_hash) as usize;

        let item = unsafe { self.linar_hash_table.get_unchecked_mut(hash_code) };
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

    #[inline]
    fn batch_save_item(&mut self, key1: &[u8], hash1: (u64, u64), value1: i16, key2: &[u8], hash2: (u64, u64), value2: i16,
                       key3: &[u8], hash3: (u64, u64), value3: i16, key4: &[u8], hash4: (u64, u64), value4: i16) {
        let hash_code_1 = (Self::compute_hash_code(hash1) % (1024*1024)) as usize;
        let hash_code_2 = (Self::compute_hash_code(hash2) % (1024*1024)) as usize;
        let hash_code_3 = (Self::compute_hash_code(hash3) % (1024*1024)) as usize;
        let hash_code_4 = (Self::compute_hash_code(hash4) % (1024*1024)) as usize;

        let linar_hash_table = &mut self.linar_hash_table;
        let item1 = unsafe { &mut *(linar_hash_table.get_unchecked_mut(hash_code_1) as *mut HashEntry) };
        let item2 = unsafe { &mut * (linar_hash_table.get_unchecked_mut(hash_code_2) as *mut HashEntry) };
        let item3 = unsafe { &mut * (linar_hash_table.get_unchecked_mut(hash_code_3) as *mut HashEntry) };
        let item4 = unsafe { &mut * (linar_hash_table.get_unchecked_mut(hash_code_4) as *mut HashEntry) };

        // preload data
        let all_matched = {
            let key_hash1 = item1.key_hash;
            let key_hash2 = item2.key_hash;
            let key_hash3 = item3.key_hash;
            let key_hash4 = item4.key_hash;
            key_hash1 == hash1 && key_hash2 == hash2 && key_hash3 == hash3 && key_hash4 == hash4
        };

        if likely( all_matched ) {
            let mut key_data_1 = item1.data;
            let mut key_data_2 = item2.data;
            let mut key_data_3 = item3.data;
            let mut key_data_4 = item4.data;

            key_data_1.count += 1;
            key_data_1.sum += value1 as i32;
            key_data_1.min = key_data_1.min.min(value1 as i32);
            key_data_1.max = key_data_1.max.max(value1 as i32);
            item1.data = key_data_1;

            key_data_2.count += 1;
            key_data_2.sum += value2 as i32;
            key_data_2.min = key_data_2.min.min(value2 as i32);
            key_data_2.max = key_data_2.max.max(value2 as i32);
            item2.data =  key_data_2 ;   // 1 store access

            key_data_3.count += 1;
            key_data_3.sum += value3 as i32;
            key_data_3.min = key_data_3.min.min(value3 as i32);
            key_data_3.max = key_data_3.max.max(value3 as i32);
            item3.data = key_data_3;   // 1 store access

            key_data_4.count += 1;
            key_data_4.sum += value4 as i32;
            key_data_4.min = key_data_4.min.min(value4 as i32);
            key_data_4.max = key_data_4.max.max(value4 as i32);
            item4.data = key_data_4;   // 1 store access
            return;
        }
        else {
            self.save_item_u64x2(key1, hash1, value1);
            self.save_item_u64x2(key2, hash2, value2);
            self.save_item_u64x2(key3, hash3, value3);
            self.save_item_u64x2(key4, hash4, value4);
        }
    }


    #[inline(never)]
    fn slow_save(&mut self, key: &[u8], key_hash: (u64, u64), value: i16, from: usize) {
        for i in from..from + 1024 {    // search at most 1024 entry
            let item: &mut HashEntry = &mut self.linar_hash_table[i];
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
pub fn ver20_preload_entry() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {     // 8.96s

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
    let mut dupicated = 0;
    for i in 0.. aggr.linar_hash_table.len() {
        let item = & aggr.linar_hash_table[i];
        if !item.key.is_empty() {
            count += 1;
            let is_dupicated = if i> 0 {
                aggr.linar_hash_table[i-1].key_hash.0 != 0
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