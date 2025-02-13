use crate::MEASUREMENT_FILE;
use std::collections::HashMap;
use std::intrinsics::{likely, unlikely};
use std::mem::transmute;

use memmap2::{Mmap, MmapOptions};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::{SimdInt, SimdUint};
use std::simd::{i16x16, i16x4, i32x8, i8x16, i8x32, simd_swizzle, u16x4, u32x4, u32x8, u64x2, u64x4, u8x16, u8x64, u8x8};
use std::slice::from_raw_parts;

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
        // *pos &= unsafe { (!1u64).unbounded_shl(at) };       // unbounded_shl 对应好几条指令，不是最佳方案
        at as usize
    }

    fn buffer(&self, offset: usize) -> *const u8 {
        unsafe { self._mmap.as_ptr().add(offset) }
    }
    fn length(&self) -> usize {
        self._mmap.len()
    }

    #[inline(never)]
    fn scan_loop(&self, _aggr: &mut AggrInfo) {
        let mut cursor: usize = 0;                  // force register, it may add a -x offset
        let aggregator = _aggr;        // force register
        let mut last_pos1 = 0u64;       // force register
        let mut pos1 ;            // force register
        let mut pos2 ;            // force register
        let mut line_start = 0usize;    // the next line's start position

        let mut block0 : u8x64 = u8x64::from_slice(unsafe { from_raw_parts(self.buffer(0), 64) });

        while likely(cursor < self.length() ) {
            if likely( (cursor + 64) <= self.length() ) {
                // let block0 = u8x64::from_slice(unsafe { from_raw_parts(self.buffer(cursor), 64) }); // 因为有写入操作，所以有等待读的操作。
                pos1 = block0.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;  // 有多次 load 操作，从stack中去读 block0，没有寄存器化
                pos2 = block0.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64; //

                // preload next block   move to here than Line 191, -360ms
                block0 = u8x64::from_slice(unsafe { from_raw_parts(self.buffer(cursor+64), 64) }); // 因为有写入操作，所以有等待读的操作。

                let mut lines = pos2.count_ones();

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
                    let key_preload_1: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(self.buffer(line_start), 16) ) ) };
                    let key_preload_2: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(self.buffer(l1_pos2 + 1), 16) ) ) };
                    let key_preload_3: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(self.buffer( l2_pos2 + 1), 16) ) ) };
                    let key_preload_4: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(self.buffer( l3_pos2 + 1), 16) ) ) };


                    let (val1, val2, val3, val4) = {
                        let val_preload_1: u64 = 0xFFFF_FFFF_FF00_0000 &    // keep low 5 bytes
                            unsafe { transmute::<u8x8,u64>( u8x8::from_slice( from_raw_parts(self.buffer(l1_pos2 - 8 ), 8) ) ) };
                        let val_preload_2: u64 = 0xFFFF_FFFF_FF00_0000 &    // keep low 5 bytes
                            unsafe { transmute::<u8x8,u64>( u8x8::from_slice(from_raw_parts(self.buffer(l2_pos2 - 8 ), 8) ) ) };
                        let val_preload_3: u64 = 0xFFFF_FFFF_FF00_0000 &    // keep low 5 bytes
                            unsafe { transmute::<u8x8,u64>( u8x8::from_slice(from_raw_parts(self.buffer(l3_pos2 - 8 ), 8) ) ) };
                        let val_preload_4: u64 = 0xFFFF_FFFF_FF00_0000 &    // keep low 5 bytes
                            unsafe { transmute::<u8x8,u64>( u8x8::from_slice(from_raw_parts(self.buffer(l4_pos2 - 8 ), 8) ) ) };


                        let val_preload: u64x4 = u64x4::from_array([ val_preload_1, val_preload_2, val_preload_3, val_preload_4 ]) ;
                        let val_preload: i8x32 = unsafe { transmute::<u64x4,i8x32>(val_preload) };

                        let signed = val_preload.simd_eq(i8x32::splat(b'-' as i8))
                            .to_int().rotate_elements_left::<1>();
                        let signed: i32x8 = unsafe { transmute::<i8x32, i32x8>(signed) };
                        let signed = signed.simd_eq(i32x8::splat(0))
                            .select(i32x8::splat(1), i32x8::splat(-1)).cast::<i16>() ;
                        let signed:i16x4 = simd_swizzle!(signed, [1,3,5,7]);    // each for a val

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

                    aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(line_start), l1_pos1 - line_start) }, key1_hash, val1);
                    aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(l1_pos2 + 1), l2_pos1 - l1_pos2 - 1) }, key2_hash, val2);
                    aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(l2_pos2 + 1), l3_pos1 - l2_pos2 - 1) }, key3_hash, val3);
                    aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(l3_pos2 + 1), l4_pos1 - l3_pos2 - 1) }, key4_hash, val4);



                    lines -= 4;
                    line_start = l4_pos2 + 1;
                }


                while likely(lines > 0) {
                    // debug.add_count(LoopAt::Loop1);
                    // l1: (line_start, l1_pos1, l1_pos2)
                    let l1_pos1 = if last_pos1 != 0 { cursor - 64 + Self::get_and_clear(&mut last_pos1) } else { cursor + Self::get_and_clear(&mut pos1) };
                    let l1_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    // preload memory
                    let key_preload_1: u64x2 = unsafe {transmute( u8x16::from_slice( from_raw_parts(self.buffer(line_start), 16) ) ) };


                    let val1 = {
                        let val_preload_1: u8x8 = u8x8::from_slice(unsafe { from_raw_parts(self.buffer(l1_pos2 - 8 ), 8) });
                        let val1_sign = val_preload_1.simd_eq(u8x8::splat(b'-')).to_bitmask().trailing_zeros() <= 5;
                        let val1 = if val1_sign { -1 } else { 1 } * parse_value_u8x8(val_preload_1);
                        val1
                    };
                    //
                    let key1_hash = truncate_key_simd(key_preload_1, l1_pos1 - line_start);

                    aggregator.save_item_u64x2(unsafe { from_raw_parts(self.buffer(line_start), l1_pos1 - line_start) }, key1_hash, val1);

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
                cursor = self.length();
            }
        }

    }

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
            let p1 = l >> 20;
            let p4 = h >> 20;
            let p2 = l >> 40;
            let p5 = h >> 40;
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
    let mut dupicated = 0;
    for i in 0.. aggr.linar_hash_table.len() {
        let item = & aggr.linar_hash_table[i];
        if !item.key.is_empty() {
            count += 1;
            let is_dupicated = if i> 0 {
                // item.key_hash == aggr.linar_hash_table[i-1].key_hash
                aggr.linar_hash_table[i-1].key_hash[0] != 0
            }
            else {
                false
            };
            let key = unsafe { std::str::from_utf8_unchecked( item.key.as_slice() ) };
            if is_dupicated {
                dupicated += 1;
                println!("{};\t{}\t{} prev:{}", key, i, is_dupicated, unsafe {std::str::from_utf8_unchecked( aggr.linar_hash_table[i-1].key.as_slice() )});
            }
        }
    }
    assert_eq!(count, 413);
    println!("total entries: {}, duplicated: {}", count, dupicated);
}