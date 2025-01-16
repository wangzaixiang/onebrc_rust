use std::arch::asm;
use std::collections::HashMap;
use std::error::Error;
use std::intrinsics::{likely};
use std::mem::{transmute};
use std::ops::{Mul, Shl, Shr, Sub};
use crate::MEASUREMENT_FILE;

use std::simd::{i16x16, i16x4, i64x1, i64x2, i64x4, i8x16, i8x4, simd_swizzle, u16x16, u16x8, u32x1, u32x4, u64x1, u64x2, u64x4, u8x16, u8x64, Mask};
use std::simd::cmp::{SimdOrd, SimdPartialEq, SimdPartialOrd};
use std::simd::num::{SimdInt, SimdUint};
use memmap2::{Mmap, MmapOptions};

#[cfg(target_arch = "aarch64")]
unsafe fn v_poncnt(v: u16x16) -> u8x16 {
    use std::arch::aarch64::*;
    let ab: uint8x16x2_t = transmute(v);
    let a = vcntq_u8( transmute(ab.0) );
    let b = vcntq_u8( transmute(ab.1) );

    // let a = vcntq_u8(a);
    // let b = vcntq_u8(b);

    let a: u8x16 = transmute(a);
    let b: u8x16 = transmute(b);

    let v1 = simd_swizzle!(a, b, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]);
    let v2 = simd_swizzle!(a, b, [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]);
    let result = v1 + v2;

    // println!("result: {:?}", result);

    transmute(result)
}

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

unsafe fn parse_value(val1: &[u8]) -> (i16) {

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

unsafe fn parse_values(val1: &[u8], val2: &[u8], val3: &[u8], val4: &[u8]) -> (i16, i16, i16, i16) {

    let pad_1 = 8 - val1.len() as isize;
    let pad_2 = 8 - val2.len() as isize;
    let pad_3 = 8 - val3.len() as isize;
    let pad_4 = 8 - val4.len() as isize;

    let ptr1 = val1.as_ptr().offset( -pad_1);
    let ptr2 = val2.as_ptr().offset(-pad_2);
    let ptr3 = val3.as_ptr().offset(-pad_3);
    let ptr4 = val4.as_ptr().offset(-pad_4);

    let l1 = u64::from_be_bytes( *(ptr1 as *const [u8;8]) );
    let l2 = u64::from_be_bytes( *(ptr2 as *const [u8;8]) );
    let l3 = u64::from_be_bytes( *(ptr3 as *const [u8;8]) );
    let l4 = u64::from_be_bytes( *(ptr4 as *const [u8;8]) );

    // clear top pad_1 * 8 bits of l1
    let l1 = l1 & (u64::MAX >> (pad_1 * 8));
    let l2 = l2 & (u64::MAX >> (pad_2 * 8));
    let l3 = l3 & (u64::MAX >> (pad_3 * 8));
    let l4 = l4 & (u64::MAX >> (pad_4 * 8));

    let sign_1 = if val1[0] == b'-' { -1 } else { 1 };
    let sign_2 = if val2[0] == b'-' { -1 } else { 1 };
    let sign_3 = if val3[0] == b'-' { -1 } else { 1 };
    let sign_4 = if val4[0] == b'-' { -1 } else { 1 };

    let v: u32x4 = u32x4::from_array([l1 as u32, l2 as u32, l3 as u32,  l4 as u32]);
    let v2: i8x16 = transmute(v);

    // let v2  = unsafe { extend_i8x16(v2) };
    let v2: i16x16  = v2.cast();
    // let v2: i16x32 = simd_swizzle!(v2, [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 23, 22, 21, 20, 19, 18, 17, 16, 31, 30, 29, 28, 27, 26, 25, 24]);

    let scale: i16x16 = i16x16::from_array([ 1, 0, 10, 100, 1, 0, 10, 100, 1, 0, 10, 100, 1, 0, 10, 100 ] );
    let mask = v2.simd_ge(i16x16::splat('0' as i16));
    let v2 = mask.select(v2, i16x16::splat(b'0' as i16));
    let sub = v2 - i16x16::splat(b'0' as i16);      // (c - '0')
    let mul = sub * scale;                                // (c - '0') * scale

    let mul_2 = mul.rotate_elements_right::<2>();       // 100 + 0, 10 + 1
    let sum = mul + mul_2;

    let sum_2 = sum.rotate_elements_right::<1>();       // 100 + 0 + 10 + 1
    let sum = sum + sum_2;

    let array: &[i16;16] = & transmute(sum);
    (sign_1 * array[3], sign_2 * array[7], sign_3 * array[11], sign_4 * array[15])
}

unsafe fn parse_values_3(val1: &[u8], val2: &[u8], val3: &[u8]) -> (i16, i16, i16) {

    let pad_1 = 8 - val1.len() as isize;
    let pad_2 = 8 - val2.len() as isize;
    let pad_3 = 8 - val3.len() as isize;

    let ptr1 = val1.as_ptr().offset( -pad_1);
    let ptr2 = val2.as_ptr().offset(-pad_2);
    let ptr3 = val3.as_ptr().offset(-pad_3);

    let l1 = u64::from_be_bytes( *(ptr1 as *const [u8;8]) );
    let l2 = u64::from_be_bytes( *(ptr2 as *const [u8;8]) );
    let l3 = u64::from_be_bytes( *(ptr3 as *const [u8;8]) );

    // clear top pad_1 * 8 bits of l1
    let l1 = l1 & (u64::MAX >> (pad_1 * 8));
    let l2 = l2 & (u64::MAX >> (pad_2 * 8));
    let l3 = l3 & (u64::MAX >> (pad_3 * 8));

    let sign_1 = if val1[0] == b'-' { -1 } else { 1 };
    let sign_2 = if val2[0] == b'-' { -1 } else { 1 };
    let sign_3 = if val3[0] == b'-' { -1 } else { 1 };

    let v: u32x4 = u32x4::from_array([l1 as u32, l2 as u32, l3 as u32, 0]);
    let v2: i8x16 = transmute(v);

    let v2: i16x16  = v2.cast();

    let scale: i16x16 = i16x16::from_array([ 1, 0, 10, 100, 1, 0, 10, 100, 1, 0, 10, 100, 1, 0, 10, 100 ] );
    let mask = v2.simd_ge(i16x16::splat('0' as i16));
    let v2 = mask.select(v2, i16x16::splat(b'0' as i16));
    let sub = v2 - i16x16::splat(b'0' as i16);      // (c - '0')
    let mul = sub * scale;                                // (c - '0') * scale

    let mul_2 = mul.rotate_elements_right::<2>();       // 100 + 0, 10 + 1
    let sum = mul + mul_2;

    let sum_2 = sum.rotate_elements_right::<1>();       // 100 + 0 + 10 + 1
    let sum = sum + sum_2;

    let array: &[i16;16] = & transmute(sum);
    (sign_1 * array[3], sign_2 * array[7], sign_3 * array[11] )
}


#[test]
fn test_parse_values(){

    let input = ["-22.9", "-2.5", "2.5", "22.5"];
    let (v1, v2, v3, v4) = unsafe { parse_values(input[0].as_bytes(), input[1].as_bytes(), input[2].as_bytes(), input[3].as_bytes()) };

    println!("v1: {}, v2: {}, v3: {}, v4: {}", v1, v2, v3, v4);

}

struct FileReader {
    _mmap: Mmap,         // const
    length: usize,      // const
    buf: *const u8,     // const
    // eof: bool,          // has more content
    // cursor: usize,      // read_more will update, 当前读取位置，已读取并分析结果保存在 mask 中
    // pos1: [u64; 64],
    // pos2: [u64; 64],
    // pos1_head: usize,   // the next position to read
    // pos1_tail: usize,    // the next position to write
    // pos2_head: usize,   // the next position to read, show = pos1_head
    // pos2_tail: usize    // the next position to write
}

impl FileReader {

    fn new(mmap: Mmap) -> FileReader {
        let length = mmap.len();
        let buf = mmap.as_ptr();
        let reader = FileReader {
            _mmap: mmap,
            length,
            buf,
        };

        // preload each pages make code fast 1891ms - 677ms = ~ 1.2s TODO why?
        reader.preload_pages();
        reader
    }

    #[inline(never)]
    fn preload_pages(&self) -> i64 {
        let mut sum = 0i64;
        for i in (0..self.length).step_by(2048*1024) {
            sum ^=  unsafe { *(self.buf.add(i) as *const i64) };
        }
        if sum == 0 {   // avoid optimize
            println!("sum: {}", sum);
        }
        sum
    }

    // fn parse_delimeter(v1: u8x16, v2: u8x16, v3: u8x16, v4: u8x16, delimeter: u8) -> u64 {
    //     let v1_mask1 = v1.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;  // popcnt <= 2
    //     let v2_mask1 = v2.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;
    //     let v3_mask1 = v3.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;
    //     let v4_mask1 = v4.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;
    //
    //     v1_mask1 as u64 | (v2_mask1 as u64) << 16 | (v3_mask1 as u64) << 32 | (v4_mask1 as u64) << 48
    // }

    /// load 128 bytes, 3 ~ 16 lines,
    /// (pos1: u128, pos2: u128, pos1_count: usize, pos2_count: usize)
    #[inline]
    fn load_current_64(&self, cursor: usize) -> (u64, u64) {
        let ptr = unsafe { self.buf.add(cursor) };
        let v1 = u8x64::from_slice(unsafe { std::slice::from_raw_parts(ptr, 64) });

        let pos1 = v1.simd_eq(u8x64::splat(b';')).to_bitmask() as u64;
        let pos2 = v1.simd_eq(u8x64::splat(b'\n')).to_bitmask() as u64;
        (pos1, pos2)
    }

    #[inline]
    fn process_line(&self, line_begin: usize, pos1: usize, pos2: usize) -> (&[u8], &[u8]) {

        let start_ptr = unsafe { self.buf.add(line_begin as usize) };
        let pos1_ptr = unsafe { self.buf.add(pos1 as usize) };


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
    fn scan_loop(&mut self, aggr: &mut AggrInfo) {
        let mut line_start = 0usize;    // the next line's start position
        let mut last_pos1: u64 = 0;

        let mut cursor: usize = 0;
        let mut eof = false;
        let psuedo_val = [b'0', b'0', b'0', b'0', b';', b'0', b'.', b'0'];

        while !eof {
            if cursor + 64 <= self.length {
                let (mut pos1, mut pos2) = self.load_current_64(cursor);    //  1 ~ 8 lines

                let mut lines = pos2.count_ones();
                while lines >= 3 {
                    let l1_pos1 = if last_pos1 != 0 { cursor - 64 + Self::get_and_clear(&mut last_pos1)  }  else { cursor + Self::get_and_clear(&mut pos1)};
                    let l1_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    let l2_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    let l2_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    let l3_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    let l3_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    // let l4_pos1 = cursor + Self::get_and_clear(&mut pos1);
                    // let l4_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    let (key1, val1) = self.process_line(line_start, l1_pos1, l1_pos2);
                    let (key2, val2) = self.process_line(l1_pos2+1, l2_pos1, l2_pos2);
                    let (key3, val3) = self.process_line(l2_pos2+1, l3_pos1, l3_pos2);
                    // let (key4, val4) = self.process_line(l3_pos2+1, l4_pos1, l4_pos2);

                    let (key_a_1, key_b_1, key_a_2, key_b_2, key_a_3, key_b_3) = str_to_hash_normal_3(key1, key2, key3);
                    // let (key_a_1, key_b_1) = str_to_hash_normal(key1);
                    // let (key_a_2, key_b_2) = str_to_hash_normal(key2);
                    // let (key_a_3, key_b_3) = str_to_hash_normal(key3);
                    // let (key_a_4, key_b_4) = str_to_hash(key4);

                    let (v1, v2, v3) = unsafe { parse_values_3(val1, val2, val3) };
                    // let v1 = unsafe { parse_value(val1) };
                    // let v2 = unsafe { parse_value(val1) };
                    // let v3 = unsafe { parse_value(val1) };
                    aggr.save_item(key1, key_a_1, key_b_1, v1);
                    aggr.save_item(key2, key_a_2, key_b_2, v2);
                    aggr.save_item(key3, key_a_3, key_b_3, v3);
                    // aggr.save_item(key4, key_a_4, key_b_4, v4);

                    lines -= 3;
                    line_start = l3_pos2 + 1;
                }
                while lines > 0 {
                    let l1_pos1 = if last_pos1 != 0 { cursor - 64 + Self::get_and_clear(&mut last_pos1)  }  else { cursor + Self::get_and_clear(&mut pos1)};
                    let l1_pos2 = cursor + Self::get_and_clear(&mut pos2);

                    let (key1, val1) = self.process_line(line_start, l1_pos1, l1_pos2);

                    let (key_a_1, key_b_1) = str_to_hash_normal(key1);

                    let v1 = unsafe { parse_value(val1) };
                    aggr.save_item(key1, key_a_1, key_b_1, v1);
                    lines -= 1;
                    line_start = l1_pos2 + 1;
                }
                last_pos1 = pos1;
                cursor += 64;
            }
            else {
                println!("process last block");
                cursor = self.length;
                eof = true;
            }
        }
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
fn str_to_hash_simd(name: &[u8]) -> (u64, u64) {
    {
        let lens = i64x2::from_array([name.len() as i64, name.len() as i64]);

        let const8 = i64x2::splat(8);
        let const0 = i64x2::splat(0);
        let const64 = i64x2::splat(64);

        // let lens: u64x2 = lens.simd_min(const8).mul(const8).cast();   // len().min(8).mul(8)
        let lens: u64x2 = (lens - i64x2::from_array([0, 8])).simd_min(const8).simd_max(const0) // (len() - 8).min(8).max(0).mul(8)
            .mul(const8).cast();

        let key1_a = u64::from_le_bytes(unsafe { *(name.as_ptr() as *const [u8; 8]) });
        let key1_b = u64::from_le_bytes(unsafe { *(name.as_ptr().add(8) as *const [u8; 8]) });

        let v1 = u64x2::from_array([key1_a, key1_b]);

        let v1 = lens.simd_eq(const0.cast()).select(
            const0.cast(),
            v1.shl(const64.cast() - lens).shr(const64.cast() - lens)
        );

        (v1[0], v1[1])
    }
}

// TODO can u128 be faster?
#[inline]
fn str_to_hash_normal(name: &[u8]) -> (u64, u64) {
    // normal version
    let len = name.len();
    let ptr1 = name.as_ptr();

    let len_a = len.min(8);
    let len_b = (len as isize - 8).min(8).max(0) as usize;

    // 0 -> 0, 1 -> 0xFF, 2 -> 0xFFFF, 3 -> 0xFFFF_FF, 4 -> 0xFFFF_FFFF, 5 -> 0xFFFF_FFFF_FF, 6 -> 0xFFFF_FFFF_FFFF, 7 -> 0xFFFF_FFFF_FFFF_FF, 8 -> 0xFFFF_FFFF_FFFF_FFFF
    // 2^0-1       8 -> 2^8-1 2^16-1,   2^24-1, 2^32-1, 2^40-1, 2^48-1, 2^56-1, 2^64-1
    // let mask_a = if len_a == 8 { u64::MAX } else { (1u64 << (len_a * 8)) - 1 };
    // let mask_b = if len_b == 8 { u64::MAX } else { (1u64 << (len_b * 8)) - 1 };

    let key_a: u64 = u64::from_le_bytes(unsafe { *(ptr1 as *const [u8; 8]) });
    let key_b: u64 = u64::from_le_bytes(unsafe { *(ptr1.add(8) as *const [u8; 8]) });

    // let key_a = key_a & mask_a;
    // let key_b = key_b & mask_b;
    let key_a = key_a & MASKS[len_a];        // test base: 1.46s
    let key_b = key_b & MASKS[len_b];

    // let key_a = key_a & u64::MAX.unbounded_shr((64- len_a * 8) as u32);  // test based: 2.22s
    // let key_b = key_b & u64::MAX.unbounded_shr((64 -len_b * 8) as u32);
    // let key_a = key_a.unbounded_shl((64 - len_a * 8) as u32).unbounded_shr((64 - len_a * 8) as u32);    // test based: 2.23s
    // let key_b = key_b.unbounded_shl((64 - len_b * 8) as u32).unbounded_shr((64 - len_b * 8) as u32);

    // let key_b = key_b + 0;
    (key_a, key_b)
}

#[inline]
fn str_to_hash_normal_3(name1: &[u8], name2: &[u8], name3: &[u8]) -> (u64, u64, u64, u64, u64, u64) {
    // normal version
    let len1 = name1.len();
    let len2 = name2.len();
    let len3 = name3.len();
    let ptr1 = name1.as_ptr();
    let ptr2 = name2.as_ptr();
    let ptr3 = name3.as_ptr();

    let len_a_1 = len1.min(8);
    let len_a_2 = len2.min(8);
    let len_a_3 = len3.min(8);
    let len_b_1 = (len1 as isize - 8).min(8).max(0) as usize;
    let len_b_2 = (len2 as isize - 8).min(8).max(0) as usize;
    let len_b_3 = (len3 as isize - 8).min(8).max(0) as usize;

    let key_a_1: u64 = u64::from_le_bytes(unsafe { *(ptr1 as *const [u8; 8]) });
    let key_b_1: u64 = u64::from_le_bytes(unsafe { *(ptr1.add(8) as *const [u8; 8]) });
    let key_a_2: u64 = u64::from_le_bytes(unsafe { *(ptr2 as *const [u8; 8]) });
    let key_b_2: u64 = u64::from_le_bytes(unsafe { *(ptr2.add(8) as *const [u8; 8]) });
    let key_a_3: u64 = u64::from_le_bytes(unsafe { *(ptr3 as *const [u8; 8]) });
    let key_b_3: u64 = u64::from_le_bytes(unsafe { *(ptr3.add(8) as *const [u8; 8]) });

    let key_a_1 = key_a_1 & MASKS[len_a_1];        // test base: 1.46s
    let key_b_1 = key_b_1 & MASKS[len_b_1];
    let key_a_2 = key_a_2 & MASKS[len_a_2];        // test base: 1.46s
    let key_b_2 = key_b_2 & MASKS[len_b_2];
    let key_a_3 = key_a_3 & MASKS[len_a_3];        // test base: 1.46s
    let key_b_3 = key_b_3 & MASKS[len_b_3];

    // let key_a = key_a & u64::MAX.unbounded_shr((64- len_a * 8) as u32);  // test based: 2.22s
    // let key_b = key_b & u64::MAX.unbounded_shr((64 -len_b * 8) as u32);
    // let key_a = key_a.unbounded_shl((64 - len_a * 8) as u32).unbounded_shr((64 - len_a * 8) as u32);    // test based: 2.23s
    // let key_b = key_b.unbounded_shl((64 - len_b * 8) as u32).unbounded_shr((64 - len_b * 8) as u32);

    // let key_b = key_b + 0;
    (key_a_1, key_b_1, key_a_2, key_b_2, key_a_3, key_b_3)
}

fn str_to_hash_simd_3(name1: &[u8], name2: &[u8], name3: &[u8]) -> (u64, u64, u64, u64, u64, u64) {
    let lens = i64x4::from_array([name1.len() as i64, name2.len() as i64, name3.len() as i64, 0]);
    let const8 = i64x4::splat(8);
    let const0 = i64x4::splat(0);
    let const64 = i64x4::splat(64);

    let lens_a: u64x4 = lens.simd_min(const8).mul(const8).cast();   // len().min(8).mul(8)
    let lens_b: u64x4 = (lens - const8).simd_min(const8).simd_max(const0)// (len() - 8).min(8).max(0).mul(8)
        .mul(const8).cast();

    let key1_a = u64::from_le_bytes(unsafe { *(name1.as_ptr() as *const [u8; 8]) });
    let key1_b = u64::from_le_bytes(unsafe { *(name1.as_ptr().add(8) as *const [u8; 8]) });
    let key2_a = u64::from_le_bytes(unsafe { *(name2.as_ptr() as *const [u8; 8]) });
    let key2_b = u64::from_le_bytes(unsafe { *(name2.as_ptr().add(8) as *const [u8; 8]) });
    let key3_a = u64::from_le_bytes(unsafe { *(name3.as_ptr() as *const [u8; 8]) });
    let key3_b = u64::from_le_bytes(unsafe { *(name3.as_ptr().add(8) as *const [u8; 8]) });

    let v1 = u64x4::from_array([key1_a, key2_a, key3_a, 0]);
    let v2 = u64x4::from_array([key1_b, key2_b, key3_b, 0]);

    let v1 = lens_a.simd_eq(const0.cast()).select(
        u64x4::splat(0),
        v1.shl(const64.cast() - lens_a).shr(const64.cast() - lens_a)
    );
    let v2 = lens_b.simd_eq(const0.cast()).select(
        u64x4::splat(0),
        v2.shl(const64.cast() - lens_b).shr(const64.cast() - lens_b)
    );

    (v1[0], v2[0], v1[1], v2[1], v1[2], v2[2])
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
struct AggrItem {
    key_a:  u64,     // 32
    key_b:  u64,     // 40
    key: Vec<u8>,    // 24
    min: i16,       // 42
    max: i16,       // 44
    count: u32,     // 48
    sum: i32,       // 52
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

    #[inline]
    fn save_item(&mut self, key: &[u8], key_a: u64, key_b: u64, value: i16) {
        let mask = (1 << 20) - 1;
        let hash = {
            let p0 = key_a;
            let p3 = key_b;
            let p1 = (key_a >> 20);
            let p4 = (key_b >> 20);
            let p2 = (key_a >> 40);
            let p5 = (key_b >> 40);
            (p0 ^ p1) ^ (p2 ^ p3) ^ (p4 ^ p5)
        };

        let hash: usize = (hash % (1024*1024)) as usize;
        let item = &mut self.hashes[hash];
        if likely(item.key_a == key_a && item.key_b == key_b) {
            debug_assert_eq!(item.key, key);
            item.count += 1;
            item.sum += value as i32;
            item.min = item.min.min(value);
            item.max = item.max.max(value);
            return;
        }
        else {
            self.slow_save(key, key_a, key_b, value, hash);
        }
    }

    #[inline(never)]
    fn slow_save(&mut self, key: &[u8], key_a: u64, key_b: u64, value: i16, from: usize) {
        for i in from..from + 1024 {    // search at most 1024 entry
            let item: &mut AggrItem = &mut self.hashes[i];
            if likely(item.key_a == key_a && item.key_b == key_b) {
                debug_assert_eq!(item.key, key);
                item.count += 1;
                item.sum += value as i32;
                item.min = item.min.min(value);
                item.max = item.max.max(value);
                return;
            } else if likely(item.key.is_empty()) {
                item.key_a = key_a;
                item.key_b = key_b;
                item.key = key.to_vec();
                item.count = 1;
                item.sum = value as i32;
                item.min = value;
                item.max = value;
                return;
            }
        }
        panic!("can't find a entry, the hash is very bad");
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

    let mut reader = FileReader::new(mmap);

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
            let check = if i> 0 {
                item.key_a == aggr.hashes[i-1].key_a && item.key_b == aggr.hashes[i-1].key_b
            }
            else {
                false
            };
            let key = unsafe { std::str::from_utf8_unchecked( item.key.as_slice() ) };
            println!("{};\t{}\t{}", key, i, check);
        }
    }
    assert_eq!(count, 413);
    println!("total entries: {}", count);
}

#[test]
pub fn compare_str_to_hash()  {
    // let name = "12345678ABCDEFGH+-=12345678ABCDEFGH+-=12345678ABCDEFGH+-=12345678ABCDEFGH+-=12345678ABCDEFGH+-=12345678ABCDEFGH+-=12345678ABCDEFGH+-=12345678ABCDEFGH+-=12345678ABCDEFGH+-=12345678ABCDEFGH+-=12345678ABCDEFGH+-=12345678ABCDEFGH+-=;;;;;;;;;;;;12345678ABCDEFGH+-=;12345678ABCDEFGH+-=;";

    // generate a random [u8;1024]
    let vec: Vec<u8> = (0..1024).map(|_| rand::random::<u8>()).collect();
    let name = vec.as_slice();
    let message = format!("name = {:p}", name.as_ptr());
    println!("message = {}", message);

    let mut sum1 = 0u64;
    let mut sum2 = 0u64;
    let time0 = std::time::Instant::now();
    for i in 0..10_0000_0000 {
        let len1 = i % 20;
        let start = i & 0xFF;
        let (name1_a, name1_b, name2_a, name2_b, name3_a, name3_b) =
            str_to_hash_normal_3(&name[start..start+len1], &name[start+1..start+len1+1], &name[start+2..start+len1+2]);
        sum1 += name1_a; sum1 += name2_a; sum1 += name3_a;
        sum2 += name1_b; sum2 += name2_b; sum2 += name3_b;
    }
    let time1 = time0.elapsed();
    println!("normal: {:?}, sum1: {}, sum2: {}", time1, sum1, sum2);

    let mut sum1 = 0u64;
    let mut sum2 = 0u64;
    let time0 = std::time::Instant::now();
    for i in 0..10_0000_0000 {
        let len1 = i % 20;
        let start = i & 0xFF;
        let (name1_a, name1_b,  name2_a, name2_b, name3_a, name3_b) =
            str_to_hash_simd_3(&name[start..start+len1], &name[start+1..start+len1+1], &name[start+2..start+len1+2]);
        sum1 += name1_a; sum1 += name2_a; sum1 += name3_a;
        sum2 += name1_b; sum2 += name2_b; sum2 += name3_b;
    }
    let time1 = time0.elapsed();
    println!("simd: {:?}, sum1: {}, sum2: {}", time1, sum1, sum2);

}