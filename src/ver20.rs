use std::arch::asm;
use std::collections::HashMap;
use std::error::Error;
use std::intrinsics::{likely, offset, unlikely};
use std::mem::{transmute};
use std::ops::{BitAnd, Mul, Shl, Shr, Sub};
use crate::MEASUREMENT_FILE;

use std::simd::{i16x16, i16x4, i16x8, i64x1, i64x2, i64x4, i8x16, i8x32, i8x4, simd_swizzle, u16x16, u16x4, u16x8, u32x1, u32x4, u32x8, u64x1, u64x2, u64x4, u8x1, u8x16, u8x32, u8x64, u8x8, Mask};
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

fn parse_value_u8x8(val: u8x8) -> i16 {
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

    #[inline(never)]
    fn scan_loop(&self, _aggr: &mut AggrInfo) {
        // let mut last_pos1: u64 = 0;

        let mut cursor: usize = 0;                  // force register, it may add a -x offset
        let buffer: *const u8 = self._mmap.as_ptr();      //
        let length = self._mmap.len();             //
        let aggregator = _aggr;        // force register
        let mut last_pos1 = 0u64;       // force register
        let mut pos1 = 0u64;            // force register
        let mut pos2 = 0u64;            // force register
        let mut line_start = 0usize;    // the next line's start position

        // let mut block11 = u8x16::splat(0);
        // let mut block12 = u8x16::splat(0);
        // let mut block13 = u8x16::splat(0);
        // let mut block14 = u8x16::splat(0);
        //
        // unsafe {
        //     asm!(
        //     "mov {tmp}, {buffer}",
        //     "mov {tmp}, {length}",
        //     "mov {tmp}, {cursor}",
        //     "mov {tmp}, {aggregator}",      // 4
        //     "mov {tmp}, {last_pos1}",
        //     "mov {tmp}, {pos1}",
        //     "mov {tmp}, {pos2}",
        //     "mov {tmp}, {line_start}",      // 8
        //
        //     "movi.16b {block01}, #0",
        //     "movi.16b {block02}, #0",
        //     "movi.16b {block03}, #0",
        //     "movi.16b {block04}, #0",
        //     "movi.16b {block11}, #0",
        //     "movi.16b {block12}, #0",
        //     "movi.16b {block13}, #0",
        //     "movi.16b {block14}, #0",
        //     buffer = in(reg) buffer,
        //     length = in(reg) length,
        //     cursor = inout(reg) cursor,
        //     aggregator = in(reg) aggregator,
        //     last_pos1 = inout(reg) last_pos1,
        //     pos1 = inout(reg) pos1,
        //     pos2 = inout(reg) pos2,
        //     line_start = inout(reg) line_start,
        //
        //     block01 = inout(vreg) block01,
        //     block02 = inout(vreg) block02,
        //     block03 = inout(vreg) block03,
        //     block04 = inout(vreg) block04,
        //     block11 = inout(vreg) block11,
        //     block12 = inout(vreg) block12,
        //     block13 = inout(vreg) block13,
        //     block14 = inout(vreg) block14,
        //     tmp = out(reg) _,
        //     );
        // }

        let mut debug = Debug::new();
        let mut block0 : u8x64 = u8x64::from_slice(unsafe { from_raw_parts(buffer, 64) });

        while cursor < length {
            if (cursor + 64) <= length {
                pos1 = block0.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
                pos2 = block0.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;
                // preload next block   TODO cursor + 64 maybe out of bounds, but it works now.
                block0 = u8x64::from_slice(unsafe { from_raw_parts(buffer.add(cursor+64), 64) });

                let mut lines = pos2.count_ones();
                debug.add_count(lines as usize);

                while likely(lines >= 4) {  // 4..=8
                    // debug.add_count(LoopAt::Loop4);

                    // l1: (line_start, l1_pos1, l1_pos2)
                    let l1_pos1 = if last_pos1 != 0 { cursor - 64 + Self::get_and_clear(&mut last_pos1) } else { cursor + Self::get_and_clear(&mut pos1) };
                    let l1_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    // l2: (l1_pos2+1, l2_pos1, l2_pos2)
                    let l2_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    let l2_pos2 = cursor + Self::get_and_clear(&mut pos2);
                    debug_assert!(l1_pos2 > l1_pos1);
                    debug_assert!(l1_pos2 < l2_pos1);

                    // l3: (l2_pos2+1, l3_pos1, l3_pos2)
                    let l3_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    let l3_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    let l4_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    let l4_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    // preload memory
                    let key_preload_1: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(buffer.add(line_start), 16) ) ) };
                    let key_preload_2: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(buffer.add(l1_pos2 + 1), 16) ) ) };
                    let key_preload_3: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(buffer.add( l2_pos2 + 1), 16) ) ) };
                    let key_preload_4: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(buffer.add( l3_pos2 + 1), 16) ) ) };


                    let (val1, val2, val3, val4) = {
                        let val_preload_1: u64 = 0xFFFF_FFFF_FF00_0000 &    // keep low 5 bytes
                            unsafe { transmute::<u8x8,u64>( u8x8::from_slice(unsafe { from_raw_parts(buffer.add(l1_pos2 - 8 ), 8) }) ) };
                        let val_preload_2: u64 = 0xFFFF_FFFF_FF00_0000 &    // keep low 5 bytes
                            unsafe { transmute::<u8x8,u64>( u8x8::from_slice(unsafe { from_raw_parts(buffer.add(l2_pos2 - 8 ), 8) }) ) };
                        let val_preload_3: u64 = 0xFFFF_FFFF_FF00_0000 &    // keep low 5 bytes
                            unsafe { transmute::<u8x8,u64>( u8x8::from_slice(unsafe { from_raw_parts(buffer.add(l3_pos2 - 8 ), 8) }) ) };
                        let val_preload_4: u64 = 0xFFFF_FFFF_FF00_0000 &    // keep low 5 bytes
                            unsafe { transmute::<u8x8,u64>( u8x8::from_slice(unsafe { from_raw_parts(buffer.add(l4_pos2 - 8 ), 8) }) ) };


                        let val_preload: u64x4 = unsafe { u64x4::from_array([ val_preload_1, val_preload_2, val_preload_3, val_preload_4 ]) };
                        let val_preload: i8x32 = unsafe { transmute::<u64x4,i8x32>(val_preload) };

                        let signed = val_preload.simd_eq(i8x32::splat(b'-' as i8))
                            .to_int().rotate_elements_left::<1>();
                        let signed: i64x4 = unsafe { transmute::<i8x32, i64x4>(signed) };
                        let signed = signed.simd_eq(i64x4::splat(0))
                            .select(i64x4::splat(1), i64x4::splat(-1)).cast::<i16>() ;

                        let val_preload: u32x8 = unsafe { transmute(val_preload) };
                        let val_preload: u32x4 = u32x4::from_array([val_preload[1], val_preload[3], val_preload[5], val_preload[7]]);
                        let val_preload: i8x16 = unsafe { transmute(val_preload) };

                        let is_digit = val_preload.simd_ge(i8x16::splat(b'0' as i8)) & val_preload.simd_le(i8x16::splat(b'9' as i8));

                        let preload: i16x16 = is_digit.select(val_preload, i8x16::splat(b'0' as i8)).cast::<i16>()
                            - i16x16::splat(b'0' as i16);
                        let mul_scale = preload * i16x16::from_array([100, 10, 0, 1, 100, 10, 0, 1, 100, 10, 0, 1,  100, 10, 0, 1]);
                        let sum = mul_scale + mul_scale.rotate_elements_right::<2>();
                        let sum: i16x16 = sum + sum.rotate_elements_right::<1>();
                        (signed[0] * sum[3], signed[1] * sum[7], signed[2] * sum[11], signed[3] * sum[15])

                    };
                    //
                    let (key1_hash, key2_hash, key3_hash, key4_hash) = (
                            truncate_key_simd(key_preload_1, l1_pos1 - line_start),
                            truncate_key_simd(key_preload_2, l2_pos1 - l1_pos2 - 1),
                            truncate_key_simd(key_preload_3, l3_pos1 - l2_pos2 - 1),
                            truncate_key_simd(key_preload_4, l4_pos1 - l3_pos2 - 1)
                        );

                    // execute pipeline optimize
                    // A: get line1 poses, get line2 poses, get line3 poses, get line4 poses
                    //    load key1, val1, key2, val2, key3, val3, key4, val4
                    //    calc key1_hash, key2_hash, key3_hash, key4_hash, calc val1, val2, val3, val4
                    //    save1, save2, save3, save4
                    // B: get line1 poses, load key1, val1
                    //    get line2 poses, load key2, val2, calc key1_hash, num1, load hash_item1
                    //    get line3 poses, load key3, val3, calc key2 hash, num2, load hash_item2, save1
                    //    get line4 poses, load key4, val4, calc key3 hash, num3, load hash_item3, save2
                    //    calc key4 hash, num4, load hash_item4, save3, save4

                    // aggregator.batch_save_item(
                    //     unsafe { from_raw_parts(buffer.add(line_start), l1_pos1 - line_start) }, key1_hash, val1,
                    //     unsafe { from_raw_parts(buffer.add(l1_pos2 + 1), l2_pos1 - l1_pos2 - 1)}, key2_hash, val2,
                    //     unsafe { from_raw_parts(buffer.add(l2_pos2 + 1), l3_pos1 - l2_pos2 - 1) }, key3_hash, val3,
                    //     unsafe { from_raw_parts(buffer.add(l3_pos2 + 1), l4_pos1 - l3_pos2 - 1) }, key4_hash, val4,
                    // );

                    aggregator.save_item_u64x2(unsafe { from_raw_parts(buffer.add(line_start), l1_pos1 - line_start) }, key1_hash, val1);
                    aggregator.save_item_u64x2(unsafe { from_raw_parts(buffer.add(l1_pos2 + 1), l2_pos1 - l1_pos2 - 1) }, key2_hash, val2);
                    aggregator.save_item_u64x2(unsafe { from_raw_parts(buffer.add(l2_pos2 + 1), l3_pos1 - l2_pos2 - 1) }, key3_hash, val3);
                    aggregator.save_item_u64x2(unsafe { from_raw_parts(buffer.add(l3_pos2 + 1), l4_pos1 - l3_pos2 - 1) }, key4_hash, val4);



                    lines -= 4;
                    line_start = l4_pos2 + 1;
                }


                while likely(lines > 0) {
                    // debug.add_count(LoopAt::Loop1);
                    // l1: (line_start, l1_pos1, l1_pos2)
                    let l1_pos1 = if last_pos1 != 0 { cursor - 64 + Self::get_and_clear(&mut last_pos1) } else { cursor + Self::get_and_clear(&mut pos1) };
                    let l1_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    // preload memory
                    let key_preload_1: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(buffer.add(line_start), 16) ) ) };


                    let val1 = {
                        let val_preload_1: u8x8 = u8x8::from_slice(unsafe { from_raw_parts(buffer.add(l1_pos2 - 8 ), 8) });
                        let val1_sign = val_preload_1.simd_eq(u8x8::splat(b'-')).to_bitmask().trailing_zeros() <= 5;
                        let val1 = if val1_sign { -1 } else { 1 } * parse_value_u8x8(val_preload_1);
                        val1
                    };
                    //
                    let key1_hash = truncate_key_simd(key_preload_1, l1_pos1 - line_start);

                    aggregator.save_item_u64x2(unsafe { from_raw_parts(buffer.add(line_start), l1_pos1 - line_start) }, key1_hash, val1);

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
fn truncate_key_simd(key: u64x2, len: usize) -> u64x2 {
    let key: u8x16 = unsafe { transmute(key) };
    let index = u8x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let mask = index.simd_lt(u8x16::splat(len as u8));
    let key = mask.select(key, u8x16::splat(0));
    unsafe { transmute(key) }
}

#[derive(Clone)]
#[repr(C, align(64))]
struct AggrItem {
    key_hash:  u64x2,     // 0
    data:      AggrItemValues,
    // min:    i32,     // 16  most write from min to sum
    // max:    i32,     // 20
    // count:  u32,     // 24
    // sum:    i32,     // 28
    key: Vec<u8>,    // 32
}

#[derive(Clone, Copy)]
union AggrItemValues {
    raw: [u8; 16],
    expanded: (i32, i32, u32, i32), // min, max, count, sum
}

impl AggrItemValues {
    fn new() -> AggrItemValues {
        AggrItemValues {
            raw: [0; 16]
        }
    }

    fn explot(&self) -> (i32, i32, u32, i32) {
        unsafe { self.expanded }
    }
}

struct AggrInfo {
    linar_hash_table: Vec<AggrItem>
}

impl AggrInfo {

    fn new_item() -> AggrItem {
        AggrItem {
            key_hash: u64x2::splat(0),
            key: Vec::new(),
            data: AggrItemValues::new()
        }
    }

    fn new() -> AggrInfo {
        let hashes = vec![Self::new_item(); 1024*1024 + 1024];

        AggrInfo {
            linar_hash_table: hashes
        }
    }


    #[inline]
    fn save_item_u64x2(&mut self, key: &[u8], hash: u64x2, value: i16) {
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

        let hash_code: usize = (hash_code % (1024*1024)) as usize;

        let item = unsafe { self.linar_hash_table.get_unchecked_mut(hash_code) };
        if likely(item.key_hash == hash ) {
            debug_assert_eq!(item.key, key);
            let mut item_values = item.data.explot();
            item_values.2 += 1;
            item_values.3 += value as i32;
            item_values.0 = item_values.0.min(value as i32);
            item_values.1 = item_values.1.max(value as i32);
            item.data = AggrItemValues { expanded: item_values };   // 1 store access
            return;
        }
        else if likely(item.key.is_empty()) {
            item.key_hash = hash;
            item.key = key.to_vec();
            item.data = AggrItemValues { expanded: (value as i32, value as i32, 1, value as i32) };
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

    #[inline]
    fn batch_save_item(&mut self, key1: &[u8], hash1: u64x2, value1: i16, key2: &[u8], hash2: u64x2, value2: i16,
                       key3: &[u8], hash3: u64x2, value3: i16, key4: &[u8], hash4: u64x2, value4: i16) {
        let hash_code_1 = (Self::compute_hash_code(hash1) % (1024*1024)) as usize;
        let hash_code_2 = (Self::compute_hash_code(hash2) % (1024*1024)) as usize;
        let hash_code_3 = (Self::compute_hash_code(hash3) % (1024*1024)) as usize;
        let hash_code_4 = (Self::compute_hash_code(hash4) % (1024*1024)) as usize;

        let linar_hash_table = &mut self.linar_hash_table;
        let item1 = unsafe { &mut *(linar_hash_table.get_unchecked_mut(hash_code_1) as *mut AggrItem) };
        let item2 = unsafe { &mut * (linar_hash_table.get_unchecked_mut(hash_code_2) as *mut AggrItem) };
        let item3 = unsafe { &mut * (linar_hash_table.get_unchecked_mut(hash_code_3) as *mut AggrItem) };
        let item4 = unsafe { &mut * (linar_hash_table.get_unchecked_mut(hash_code_4) as *mut AggrItem) };

        // preload data
        let all_matched = {
            let key_hash1 = item1.key_hash;
            let key_hash2 = item2.key_hash;
            let key_hash3 = item3.key_hash;
            let key_hash4 = item4.key_hash;
            key_hash1 == hash1 && key_hash2 == hash2 && key_hash3 == hash3 && key_hash4 == hash4
        };

        if likely( all_matched ) {
            let mut key_data_1 = unsafe { item1.data.expanded };
            let mut key_data_2 = unsafe { item2.data.expanded };
            let mut key_data_3 = unsafe { item3.data.expanded };
            let mut key_data_4 = unsafe { item4.data.expanded };

            key_data_1.2 += 1;
            key_data_1.3 += value1 as i32;
            key_data_1.0 = key_data_1.0.min(value1 as i32);
            key_data_1.1 = key_data_1.1.max(value1 as i32);
            item1.data.expanded = key_data_1;   // 1 store access

            key_data_2.2 += 1;
            key_data_2.3 += value2 as i32;
            key_data_2.0 = key_data_2.0.min(value2 as i32);
            key_data_2.1 = key_data_2.1.max(value2 as i32);
            item2.data.expanded =  key_data_2 ;   // 1 store access

            key_data_3.2 += 1;
            key_data_3.3 += value3 as i32;
            key_data_3.0 = key_data_3.0.min(value3 as i32);
            key_data_3.1 = key_data_3.1.max(value3 as i32);
            item3.data.expanded = key_data_3;   // 1 store access

            key_data_4.2 += 1;
            key_data_4.3 += value4 as i32;
            key_data_4.0 = key_data_4.0.min(value4 as i32);
            key_data_4.1 = key_data_4.1.max(value4 as i32);
            item4.data.expanded = key_data_4;   // 1 store access
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
    fn slow_save(&mut self, key: &[u8], key_hash: u64x2, value: i16, from: usize) {
        for i in from..from + 1024 {    // search at most 1024 entry
            let item: &mut AggrItem = &mut self.linar_hash_table[i];
            if unlikely(item.key_hash == key_hash) {
                debug_assert_eq!(item.key, key);
                let mut item_values = item.data.explot();
                item_values.2 += 1;
                item_values.3 += value as i32;
                item_values.0 = item_values.0.min(value as i32);
                item_values.1 = item_values.1.max(value as i32);
                item.data = AggrItemValues { expanded: item_values };
                // item.count += 1;
                // item.sum += value as i32;
                // item.min = item.min.min(value as i32);
                // item.max = item.max.max(value as i32);
                return;
            } else if likely(item.key.is_empty()) {
                item.key_hash = key_hash;
                item.key = key.to_vec();
                item.data = AggrItemValues { expanded: (value as i32, value as i32, 1, value as i32) };

                // item.count = 1;
                // item.sum = value as i32;
                // item.min = value as i32;
                // item.max = value as i32;
                return;
            }
        }
        panic!("can't find a entry, the hash is very bad");
    }
}


#[inline(never)]
// based on ver12
pub fn ver20() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {     // 8.96s

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