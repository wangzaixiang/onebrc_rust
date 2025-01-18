use std::arch::asm;
use std::collections::HashMap;
use std::error::Error;
use std::intrinsics::{likely, offset, unlikely};
use std::mem::{transmute};
use std::ops::{BitAnd, Mul, Shl, Shr, Sub};
use crate::MEASUREMENT_FILE;

use std::simd::{i16x16, i16x4, i16x8, i32x4, i32x8, i64x1, i64x2, i64x4, i8x16, i8x32, i8x4, simd_swizzle, u16x16, u16x4, u16x8, u32x1, u32x4, u32x8, u64x1, u64x2, u64x4, u8x1, u8x16, u8x32, u8x64, u8x8, Mask};
use std::simd::cmp::{SimdOrd, SimdPartialEq, SimdPartialOrd};
use std::simd::num::{SimdInt, SimdUint};
use std::slice;
use std::slice::from_raw_parts;
use memmap2::{Mmap, MmapOptions};
use log::debug;

#[cfg(target_arch = "aarch64")]
unsafe fn preload(ptr: *const u8) {
    asm! {
    "prfm pldl1keep, [{x}]",
    "prfm pldl1keep, [{x},128]",
    "prfm pldl1keep, [{x},256]",
    "prfm pldl1keep, [{x},384]",
    "prfm pldl1keep, [{x},512]",
    "prfm pldl1keep, [{x},640]",
    "prfm pldl1keep, [{x},768]",
    "prfm pldl1keep, [{x},896]",
    "prfm pldl1keep, [{x},1024]",
    "prfm pldl1keep, [{x},1152]",
    "prfm pldl1keep, [{x},1280]",
    "prfm pldl1keep, [{x},1408]",
    "prfm pldl1keep, [{x},1536]",
    "prfm pldl1keep, [{x},1664]",
    "prfm pldl1keep, [{x},1792]",
    "prfm pldl1keep, [{x},1920]",
    "prfm pldl1keep, [{x},2048]",
    "prfm pldl1keep, [{x},2176]",
    "prfm pldl1keep, [{x},2304]",
    "prfm pldl1keep, [{x},2432]",
    "prfm pldl1keep, [{x},2560]",
    "prfm pldl1keep, [{x},2688]",
    "prfm pldl1keep, [{x},2816]",
    "prfm pldl1keep, [{x},2944]",
    "prfm pldl1keep, [{x},3072]",
    "prfm pldl1keep, [{x},3200]",
    "prfm pldl1keep, [{x},3328]",
    "prfm pldl1keep, [{x},3456]",
    "prfm pldl1keep, [{x},3584]",
    "prfm pldl1keep, [{x},3712]",
    "prfm pldl1keep, [{x},3840]",
    "prfm pldl1keep, [{x},3968]",
    x = in(reg) ptr,
    }
}

/// low [ x x x ; - 3 1 .2  ] high
fn parse_value_u8x8(val: u8x8) -> i16 {
    let val = val.cast::<u16>();

    let signed = val.simd_eq(u16x8::splat(b'-' as u16))
        .to_int().rotate_elements_right::<1>();   // [ x x x x ? ? ? ? ]  if ? = 1
    let signed: i64x2 = unsafe { transmute(signed) };
    let signed = signed.simd_eq(i64x2::splat(0))
        .select(i64x2::splat(1), i64x2::splat(-1)).cast::<i16>() ;

    let val = val.cast::<i16>();
    let scale: i16x8 = i16x8::from_array([ 0, 0, 0, 0, 100, 10, 0, 1 ]);
    let mask = val.simd_ge(i16x8::splat(b'0' as i16)) & val.simd_le(i16x8::splat(b'9' as i16));
    let val = mask.select(val, i16x8::splat(b'0' as i16));
    let sub_0 = val - i16x8::splat(b'0' as i16);      // (c - '0')
    let mul_scale = sub_0 * scale;                                // (c - '0') * scale

    let sum_1 = mul_scale + mul_scale.rotate_elements_right::<2>();       // 100 + 0 + 10 + 1
    let sum_2 = sum_1 + sum_1.rotate_elements_right::<1>();       // 100 + 0 + 10 + 1
    signed[1] * sum_2[7]
}

fn parse_val_u8x8x4(val1: u8x8, val2: u8x8, val3: u8x8, val4: u8x8) -> (i16, i16, i16, i16) {
    let vals: u64x4  =unsafe {  u64x4::from_array([ transmute(val1), transmute(val2), transmute(val3), transmute(val4) ]) };
    let vals: u8x32 = unsafe { transmute(vals) };

    let signed = vals.simd_eq(u8x32::splat(b'-'))
        .to_int().rotate_elements_right::<1>();   // [ x x x x ? ? ? ? ]  if ? = 1
    let signed: i32x8 = unsafe { transmute(signed) };
    let signed = signed.simd_eq(i32x8::splat(0))
        .select(i32x8::splat(1), i32x8::splat(-1)).cast::<i16>() ;

    let vals: i32x8 = unsafe { transmute(vals) };
    let vals: i32x4 = simd_swizzle!(vals, [1,3,5,7 ]);
    let vals: i8x16 = unsafe { transmute(vals) };

    let vals = vals.cast::<i16>();
    let scale: i16x16 = i16x16::from_array([ 100, 10, 0, 1, 100, 10, 0, 1, 100, 10, 0, 1, 100, 10, 0, 1]);
    let mask = vals.simd_ge(i16x16::splat(b'0' as i16)) & vals.simd_le(i16x16::splat(b'9' as i16));
    let vals = mask.select(vals, i16x16::splat(b'0' as i16));
    let sub_0 = vals - i16x16::splat(b'0' as i16);      // (c - '0')
    let mul_scale = sub_0 * scale;                                // (c - '0') * scale

    let sum_1 = mul_scale + mul_scale.rotate_elements_right::<2>();       // 100 + 0 + 10 + 1
    let sum_2 = sum_1 + sum_1.rotate_elements_right::<1>();       // 100 + 0 + 10 + 1
    (signed[1] * sum_2[3], signed[3] * sum_2[7], signed[5] * sum_2[11], signed[7] * sum_2[15])
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
        for i in 0..8 {
            println!("loop{}: {}", i, self.counts[i]);
        }
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
    fn preload_key_u64x2(buffer: *const u8, offset: usize) -> u64x2 {
        unsafe { transmute( u8x16::from_slice( from_raw_parts(buffer.add(offset), 16) ) ) }
    }

    #[inline]
    fn preload_val_u64(buffer: *const u8, offset: usize) -> u64 {
        unsafe { transmute::<u8x8,u64>( u8x8::from_slice( from_raw_parts(buffer.add(offset), 8) ) ) }
    }

    #[inline(never)]
    fn scan_loop(&self, _aggr: &mut AggrInfo) {
        // let mut last_pos1: u64 = 0;

        let mut cursor: usize = 0;                  // force register, it may add a -x offset
        let buffer: *const u8 = self._mmap.as_ptr();      //
        let length = self._mmap.len();             //
        let mut last_pos1 = 0u64;       // force register
        let mut pos1 = 0u64;            // force register
        let mut pos2 = 0u64;            // force register
        let mut line_start = 0usize;    // the next line's start position
        let hash_table = _aggr.linar_hash_table.as_mut_ptr();        // force register

        let mut block0 : u8x64 = u8x64::from_slice(unsafe { from_raw_parts(buffer, 64) });

        while cursor < length {
            if (cursor + 64) <= length {
                pos1 = block0.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
                pos2 = block0.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;
                // preload next block   TODO cursor + 64 maybe out of bounds, but it works now.
                block0 = u8x64::from_slice(unsafe { from_raw_parts(buffer.add(cursor+64), 64) });

                let mut lines = pos2.count_ones();
                while likely(lines >= 4) {  // 4..=8
                    // process line 1 - delimiter ';'
                    let l1_preload_key: u64x2 = Self::preload_key_u64x2(buffer, line_start );

                    let l1_pos1 =
                        if last_pos1 != 0 { cursor - 64 + Self::get_and_clear(&mut last_pos1) }
                        else { cursor + Self::get_and_clear(&mut pos1) };

                    let l1_key_hash_code = truncate_key_simd(l1_preload_key, l1_pos1 - line_start);
                    let l1_preload_hash_entry: (*mut HashEntry, u64x2, AggrItem) = Self::preload_hash_item(hash_table, l1_key_hash_code);

                    // process line 1 - delimiter '\n'
                    let l1_pos2 = cursor + Self::get_and_clear(&mut pos2);
                    let l1_preload_val: u64 = Self::preload_val_u64(buffer, l1_pos2 - 8);

                    // process line 2 - delimiter ';'
                    let l2_preload_key: u64x2 = Self::preload_key_u64x2(buffer, l1_pos2 + 1);
                    let l2_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    let l2_key_hash_code =         truncate_key_simd(l2_preload_key, l2_pos1 - l1_pos2 - 1);
                    let l2_preload_hash_entry: (*mut HashEntry, u64x2, AggrItem) = Self::preload_hash_item(hash_table, l2_key_hash_code);

                    // process line 2 - delimiter '\n'
                    let l2_pos2 = cursor + Self::get_and_clear(&mut pos2);
                    // let l1_val: i16 = parse_value_u8x8( unsafe { transmute::<u64, u8x8>( 0xFFFF_FFFF_FF00_0000 & l1_preload_val) } );
                    let l2_preload_val: u64 = Self::preload_val_u64(buffer, l2_pos2 - 8);

                    // process line 3 - delimiter ';'
                    let l3_preload_key = Self::preload_key_u64x2(buffer, l2_pos2 + 1);
                    let l3_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    let l3_key_hash_code =         truncate_key_simd(l3_preload_key, l3_pos1 - l2_pos2 - 1);
                    let l3_preload_hash_entry: (*mut HashEntry, u64x2, AggrItem) = Self::preload_hash_item(hash_table, l3_key_hash_code);

                    // process line 3 - delimiter '\n'
                    let l3_pos2 = cursor + Self::get_and_clear(&mut pos2);
                    // let l2_val: i16 = parse_value_u8x8( unsafe { transmute::<u64,u8x8>( 0xFFFF_FFFF_FF00_0000 & l2_preload_val) } );
                    let l3_preload_val: u64 = Self::preload_val_u64(buffer, l3_pos2 - 8);

                    // process line 4 - delimiter ';'
                    let l4_preload_key = Self::preload_key_u64x2(buffer, l3_pos2 + 1);
                    let l4_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    let l4_key_hash_code = truncate_key_simd(l4_preload_key, l4_pos1 - l3_pos2 - 1);
                    let l4_preload_hash_entry = Self::preload_hash_item(hash_table, l4_key_hash_code);

                    // process line 4 - delimiter '\n'
                    let l4_pos2 = cursor + Self::get_and_clear(&mut pos2);
                    let l4_preload_val = Self::preload_val_u64(buffer, l4_pos2 - 8);
                    // let l3_val: i16 = parse_value_u8x8( unsafe { transmute::<u64,u8x8>( 0xFFFF_FFFF_FF00_0000 & l3_preload_val) });
                    // let l4_val: i16 = parse_value_u8x8( unsafe { transmute::<u64,u8x8>( 0xFFFF_FFFF_FF00_0000 & l4_preload_val) });

                    let (l1_val, l2_val, l3_val, l4_val) = parse_val_u8x8x4(
                        unsafe { transmute::<u64, u8x8>( 0xFFFF_FFFF_FF00_0000 & l1_preload_val) },
                        unsafe { transmute::<u64, u8x8>( 0xFFFF_FFFF_FF00_0000 & l2_preload_val) },
                        unsafe { transmute::<u64, u8x8>( 0xFFFF_FFFF_FF00_0000 & l3_preload_val) },
                        unsafe { transmute::<u64, u8x8>( 0xFFFF_FFFF_FF00_0000 & l4_preload_val) },
                    );

                    if likely(l1_key_hash_code == l1_preload_hash_entry.1 && l2_key_hash_code == l2_preload_hash_entry.1 &&
                        l3_key_hash_code == l3_preload_hash_entry.1 && l4_key_hash_code == l4_preload_hash_entry.1 ) {

                        AggrInfo::fast_save(l1_preload_hash_entry.0, l1_preload_hash_entry.2, l1_val);
                        AggrInfo::fast_save(l2_preload_hash_entry.0, l2_preload_hash_entry.2, l2_val);
                        AggrInfo::fast_save(l3_preload_hash_entry.0, l3_preload_hash_entry.2, l3_val);
                        AggrInfo::fast_save(l4_preload_hash_entry.0, l4_preload_hash_entry.2, l4_val);

                    }
                    else {
                        _aggr.save_item_u64x2(unsafe { from_raw_parts(buffer.add(line_start), l1_pos1 - line_start) }, l1_key_hash_code, l1_val);
                        _aggr.save_item_u64x2(unsafe { from_raw_parts(buffer.add(l1_pos2 + 1), l2_pos1 - l1_pos2 - 1) }, l2_key_hash_code, l2_val);
                        _aggr.save_item_u64x2(unsafe { from_raw_parts(buffer.add(l2_pos2 + 1), l3_pos1 - l2_pos2 - 1) }, l3_key_hash_code, l3_val);
                        _aggr.save_item_u64x2(unsafe { from_raw_parts(buffer.add(l3_pos2 + 1), l4_pos1 - l3_pos2 - 1) }, l4_key_hash_code, l4_val);
                    }

                    lines -= 4;
                    line_start = l4_pos2 + 1;
                }

                while likely(lines > 0) {
                    let key_preload_1: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(buffer.add(line_start), 16) ) ) };
                    let l1_pos1 = if last_pos1 != 0 { cursor - 64 + Self::get_and_clear(&mut last_pos1) } else { cursor + Self::get_and_clear(&mut pos1) };
                    let l1_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    let val1 = {
                        let val_preload_1: u8x8 = u8x8::from_slice(unsafe { from_raw_parts(buffer.add(l1_pos2 - 8 ), 8) });
                        parse_value_u8x8(val_preload_1)
                    };

                    let key1_hash = truncate_key_simd(key_preload_1, l1_pos1 - line_start);
                    _aggr.save_item_u64x2(unsafe { from_raw_parts(buffer.add(line_start), l1_pos1 - line_start) }, key1_hash, val1);

                    lines -= 1;
                    line_start = l1_pos2 + 1;
                }

                if last_pos1 == 0 {
                    last_pos1 = pos1;
                }
                else {
                    // already save last_pos on the last loop
                }
                cursor += 64;
            }
            else {
                println!("process last block");
                cursor = length;
            }
        }

    }

    fn preload_hash_item(hash_table: *mut HashEntry, hash_code: u64x2) -> (*mut HashEntry, u64x2, AggrItem) {
        let hash_code = AggrInfo::compute_hash_code(hash_code);
        let ptr: *mut HashEntry = unsafe { hash_table.add(hash_code as usize) };
        unsafe { ( ptr, (*ptr).key_hash, (*ptr).data) }
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
fn truncate_key_simd(key: u64x2, len: usize) -> u64x2 {
    let key: u8x16 = unsafe { transmute(key) };
    let index = u8x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let mask = index.simd_lt(u8x16::splat(len as u8));
    let key = mask.select(key, u8x16::splat(0));
    unsafe { transmute(key) }
}

#[derive(Clone)]
#[repr(C, align(64))]
struct HashEntry {
    key_hash:  u64x2,     // 0
    data: AggrItem,
    // min:    i32,     // 16  most write from min to sum
    // max:    i32,     // 20
    // count:  u32,     // 24
    // sum:    i32,     // 28
    key: Vec<u8>,    // 32
}

#[derive(Clone, Copy)]
struct AggrItem {
    min: i32,
    max: i32,
    count: u32,
    sum: i32,
}

impl AggrItem {
    fn new() -> AggrItem {
        AggrItem {
            min: 0,
            max: 0,
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
            key_hash: u64x2::splat(0),
            key: Vec::new(),
            data: AggrItem::new()
        }
    }

    fn new() -> AggrInfo {
        let hashes = vec![Self::new_item(); 1024*1024 + 1024];

        AggrInfo {
            linar_hash_table: hashes
        }
    }

    fn fast_save(entry: *mut HashEntry, mut item: AggrItem, val1: i16) {
        item.count += 1;
        item.sum += val1 as i32;
        item.min = item.min.min(val1 as i32);
        item.max = item.max.max(val1 as i32);
        unsafe { (*entry).data = item };
    }


    #[inline]
    fn save_item_u64x2(&mut self, key: &[u8], hash: u64x2, value: i16) {
        let (l, h) = (hash[0], hash[1]);
        let hash_code =  Self::compute_hash_code(hash) as usize;

        let hash_entry: &mut HashEntry = unsafe { self.linar_hash_table.get_unchecked_mut(hash_code) };
        if likely(hash_entry.key_hash == hash ) {
            debug_assert_eq!(hash_entry.key, key);
            let mut item = hash_entry.data;
            item.count += 1;
            item.sum += value as i32;
            item.min = item.min.min(value as i32);
            item.max = item.max.max(value as i32);
            hash_entry.data = item;
            return;
        }
        else if likely(hash_entry.key.is_empty()) {
            hash_entry.key_hash = hash;
            hash_entry.key = key.to_vec();
            hash_entry.data = AggrItem {
                min: value as i32,
                max: value as i32,
                count: 1,
                sum: value as i32,
            };
            return;
        }
        else {
            self.slow_save(key, hash, value, hash_code);
        }
    }

    fn compute_hash_code(hash: u64x2) -> u32 {
        let (l, h) = (hash[0], hash[1]);
        let hash_code = {
            let p0 = l;
            let p3 = h;
            let p1 = (l >> 20);
            let p4 = (h >> 20);
            let p2 = (l >> 40);
            let p5 = (h >> 40);
            (p0 ^ p1) ^ (p2 ^ p3) ^ (p4 ^ p5)
        };
        (hash_code % (1024*1024)) as u32
    }

    #[inline(never)]
    fn slow_save(&mut self, key: &[u8], key_hash: u64x2, value: i16, from: usize) {
        for i in from..from + 1024 {    // search at most 1024 entry
            let hash_entry: &mut HashEntry = &mut self.linar_hash_table[i];
            if unlikely(hash_entry.key_hash == key_hash) {
                debug_assert_eq!(hash_entry.key, key);
                let mut item = hash_entry.data;
                item.count += 1;
                item.sum += value as i32;
                item.min = item.min.min(value as i32);
                item.max = item.max.max(value as i32);
                hash_entry.data = item;
                return;
            } else if likely(hash_entry.key.is_empty()) {
                hash_entry.key_hash = key_hash;
                hash_entry.key = key.to_vec();
                hash_entry.data = AggrItem {
                    min: value as i32,
                    max: value as i32,
                    count: 1,
                    sum: value as i32,
                };
                return;
            }
        }
        panic!("can't find a entry, the hash is very bad");
    }
}


#[inline(never)]
// based on ver12
pub fn ver21() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {     // 8.96s

    let file = std::fs::File::open(MEASUREMENT_FILE)?;

    // let mmap = unsafe { Mmap::map(&file)? };
    let mmap = unsafe {
        MmapOptions::new().huge(Some(21)).populate()
            .map(&file)?
    };

    let reader = FileReader::new(mmap);

    let mut aggr = AggrInfo::new();
    reader.scan_loop(&mut aggr);

    check_result(&aggr);
    // check_result(&aggr);

    Ok( HashMap::new() )
}

fn check_result(aggr: &AggrInfo) {
    let mut count = 0;
    for i in 0.. aggr.linar_hash_table.len() {
        let item = & aggr.linar_hash_table[i];
        if !item.key.is_empty() {
            count += 1;
            let is_dupicated = if i> 0 {
                item.key_hash == aggr.linar_hash_table[i-1].key_hash
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