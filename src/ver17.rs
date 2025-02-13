use crate::MEASUREMENT_FILE;
use std::collections::HashMap;
use std::intrinsics::likely;
use std::mem::transmute;
use std::ops::{BitAnd, Shl};

use memmap2::{Mmap, MmapOptions};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::{SimdInt, SimdUint};
use std::simd::{i16x16, i8x16, simd_swizzle, u16x16, u16x8, u32x4, u8x16, Mask};

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

    fn parse_delimeter(v1: u8x16, v2: u8x16, v3: u8x16, v4: u8x16, v5: u8x16, v6: u8x16, v7: u8x16, v8: u8x16, delimeter: u8) -> (u128, usize) {
        let v1_mask1 = v1.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;  // popcnt <= 2
        let v2_mask1 = v2.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;
        let v3_mask1 = v3.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;
        let v4_mask1 = v4.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;
        let v5_mask1 = v5.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;
        let v6_mask1 = v6.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;
        let v7_mask1 = v7.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;
        let v8_mask1 = v8.simd_eq(u8x16::splat(delimeter)).to_bitmask() as u16;

        // grp_mask: eg [0x20, 0x80, 0, 0x04, 0x80, 0, 2 , 8] repr 64 bytes , 1 means the position is delimiter
        let bytes_mask = u16x8::from_array([v1_mask1, v2_mask1, v3_mask1, v4_mask1,
            v5_mask1, v6_mask1, v7_mask1, v8_mask1 ]);
        let bytes_mask: u8x16 = unsafe { transmute(bytes_mask) }; // 1 means the position is ';'

        let group_mask: Mask<i8, 16> = bytes_mask.simd_ne(u8x16::splat(0));  // each 8-byte group has delimeter, eg: [1, 1, 0, 1, 1, 0, 1, 1]

        let scatter_index = {    // count the pre 1s for each group eg: [0, 1, 2, 2, 3, 4, 4, 5, ... ]
            let flag1 = group_mask.to_bitmask() as u16;      // 16 bit mask for 16-groups,
            let one_counts_v: u16x16 = u16x16::splat(flag1);      // test each group's pre 1 count which will used for scater

            let popcnt_mask = u16x16::splat(1).shl(
                u16x16::from_array([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15])
            ) - u16x16::splat(1);

            let vx: u16x16 = one_counts_v.bitand(popcnt_mask);
            let cnt_v: u8x16 = unsafe { v_poncnt(vx) };
            cnt_v
        };

        // scatter values
        let group_base = u16x16::from_array([0,8,16,24,32,40,48,56,64,72,80,88,96,104,112,120]); // base for each 8 bytes
        let vals: u16x16 = group_base + bytes_mask.trailing_zeros().cast();
        let mut buffer = [0u16; 16];
        vals.scatter_select(&mut buffer, group_mask.cast(),  scatter_index.cast());

        let poses: u8x16 = u16x16::from_array(buffer).cast();
        (unsafe { transmute(poses) }, group_mask.to_bitmask().count_ones() as usize)
    }

    /// load 128 bytes, 3 ~ 16 lines,
    /// (pos1: u128, pos2: u128, pos1_count: usize, pos2_count: usize)
    #[inline]
    unsafe fn load_current_128(&self, cursor: isize) -> (u128, u128, usize, usize){
        let ptr = unsafe { self.buf.offset(cursor) };
        let v1 = u8x16::from_slice(unsafe { std::slice::from_raw_parts(ptr, 16) });
        let v2 = u8x16::from_slice(unsafe { std::slice::from_raw_parts(ptr.add(16), 16) });
        let v3 = u8x16::from_slice(unsafe { std::slice::from_raw_parts(ptr.add(32), 16) });
        let v4 = u8x16::from_slice(unsafe { std::slice::from_raw_parts(ptr.add(48), 16) });
        let v5 = u8x16::from_slice(unsafe { std::slice::from_raw_parts(ptr.add(64), 16) });
        let v6 = u8x16::from_slice(unsafe { std::slice::from_raw_parts(ptr.add(80), 16) });
        let v7 = u8x16::from_slice(unsafe { std::slice::from_raw_parts(ptr.add(96), 16) });
        let v8 = u8x16::from_slice(unsafe { std::slice::from_raw_parts(ptr.add(112), 16) });

        // pos1 for
        // offset(u8) for every 8 bytes
        let (pos1, pos1_count) = FileReader::parse_delimeter(v1, v2, v3, v4, v5, v6, v7, v8, b';');
        let (pos2, pos2_count) = FileReader::parse_delimeter(v1, v2, v3, v4, v5, v6, v7, v8, b'\n');

        (pos1, pos2, pos1_count, pos2_count)
    }

    fn process_line(&self, line_begin: isize, pos1: isize, pos2: isize) -> (&[u8], &[u8]) {

        let start_ptr = unsafe { self.buf.add(line_begin as usize) };
        let pos1_ptr = unsafe { self.buf.add(pos1 as usize) };
        let pos2_ptr = unsafe { self.buf.add(pos2 as usize) };


        let key: &[u8] = unsafe { std::slice::from_raw_parts( start_ptr, pos1_ptr.sub_ptr(start_ptr)) };
        let value: &[u8] = unsafe { std::slice::from_raw_parts( pos1_ptr.add(1), pos2_ptr.sub_ptr(pos1_ptr)-1) };
        (key, value)
    }

    fn scan_loop(&mut self, aggr: &mut AggrInfo) {
        let mut line_start = 0isize;    // the next line's start position
        let mut last_pos1: Option<i16> = None;   //
        let mut cursor: isize = 0;
        let mut eof = false;
        while !eof {
            if cursor + 128 <= self.length as isize{
                let (mut pos1, mut pos2, mut pos1_count, mut pos2_count) = unsafe { self.load_current_128(cursor) };
                // process 0..pos2_count
                if let Some(mut last_p1) = last_pos1 {
                    last_p1 -= 128;
                    pos1 = ((last_p1 as u128) & 0xFF) | (pos1 << 8);
                    pos1_count += 1;
                }
                assert!(pos1_count >= pos2_count);  // pos2_count: 3..16
                // let pos1_ptr: *const i16 = unsafe { &pos1 as *const u128 as *const i16 };
                // let pos2_ptr: *const i16 = unsafe { &pos2 as *const u128 as *const i16 };
                while pos2_count >= 4 {
                    let pos1_1 = cursor as isize + (pos1 & 0xFF) as i8 as isize;
                    let pos1_2 = cursor as isize + (pos1 >> 8 & 0xFF) as i8 as isize;
                    let pos1_3 = cursor as isize + (pos1 >> 16 & 0xFF) as i8 as isize;
                    let pos1_4 = cursor as isize + (pos1 >> 24 & 0xFF) as i8 as isize;

                    let pos2_1 = cursor as isize + (pos2 & 0xFF) as i8 as isize;
                    let pos2_2 = cursor as isize + (pos2 >> 8 & 0xFF) as i8 as isize;
                    let pos2_3 = cursor as isize + (pos2 >> 16 & 0xFF) as i8 as isize;
                    let pos2_4 = cursor as isize + (pos2 >> 24 & 0xFF) as i8 as isize;

                    let (key1, val1) = self.process_line(line_start as isize, pos1_1, pos2_1);
                    let (key2, val2) = self.process_line(pos2_1+1, pos1_2, pos2_2);
                    let (key3, val3) = self.process_line(pos2_2+1, pos1_3, pos2_3);
                    let (key4, val4) = self.process_line(pos2_3+1, pos1_4, pos2_4);

                    let (key_a_1, key_b_1) = str_to_hash(key1);
                    let (key_a_2, key_b_2) = str_to_hash(key2);
                    let (key_a_3, key_b_3) = str_to_hash(key3);
                    let (key_a_4, key_b_4) = str_to_hash(key4);

                    let (v1, v2, v3, v4) = unsafe { parse_values(val1, val2, val3, val4) };
                    aggr.save_item(key1, key_a_1, key_b_1, v1);
                    aggr.save_item(key2, key_a_2, key_b_2, v2);
                    aggr.save_item(key3, key_a_3, key_b_3, v3);
                    aggr.save_item(key4, key_a_4, key_b_4, v4);

                    line_start = pos2_4 + 1;

                    pos1 = pos1 >> 32;
                    pos2 = pos2 >> 32;
                    pos1_count -= 4;
                    pos2_count -= 4;
                }

                while pos2_count > 0 { // process the last 1..3 lines
                    let pos1_1 = cursor as isize +  (pos1 & 0xFF) as isize ;
                    let pos2_1 = cursor as isize +  (pos2 & 0xFF) as isize ;
                    let (key, val) = self.process_line(line_start, pos1_1, pos2_1);
                    let (key_a, key_b) = str_to_hash(key);
                    let value = parse_value(val);
                    aggr.save_item(key, key_a, key_b, value);
                    pos1 = pos1 >> 8;
                    pos2 = pos2 >> 8;
                    pos1_count -= 1;
                    pos2_count -= 1;
                    line_start = pos2_1 + 1;
                }
                if pos1_count > 0 {
                    last_pos1 = Some(pos1 as i16);
                }
                else {
                    last_pos1 = None;
                }
                cursor += 128;
                eof = cursor >= self.length as isize;
            }
            else {
                println!("process last block");
                cursor = self.length as isize;
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

fn str_to_hash(name: &[u8]) -> (u64, u64) {
    let ptr1 = name.as_ptr();
    let key_a_1: u64 = u64::from_le_bytes( unsafe { *(ptr1 as *const[u8;8]) });
    let key_b_1: u64 = u64::from_le_bytes( unsafe { *(ptr1.add(8) as *const[u8;8]) });


    let len_a_1 = if name.len() >= 8 { 8 } else { name.len() };
    let len_b_1 = if name.len() >= 16 { 8 } else if name.len() > 8 { name.len() - 8 }  else { 0 };

    let key_a_1 = key_a_1 & MASKS[len_a_1];
    let key_b_1 = key_b_1 & MASKS[len_b_1];

    (key_a_1, key_b_1)
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
            let p0 = key_a & mask;
            let p1 = (key_a >> 20) & mask;
            let p2 = (key_a >> 40) & mask;
            let p3 = key_b & mask;
            let p4 = (key_b >> 20) & mask;
            let p5 = (key_b >> 40) & mask;
            p0 ^ p1 ^ p2 ^ p3 ^ p4 ^ p5
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
pub fn ver17() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {     // 8.96s

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
    let mut dupicated = 0;
    for i in 0.. aggr.hashes.len() {
        let item = & aggr.hashes[i];
        if !item.key.is_empty() {
            count += 1;
            let is_dupicated = if i> 0 {
                // item.key_hash == aggr.linar_hash_table[i-1].key_hash
                aggr.hashes[i-1].key_a != 0
            }
            else {
                false
            };
            let key = unsafe { std::str::from_utf8_unchecked( item.key.as_slice() ) };
            if is_dupicated {
                dupicated += 1;
                println!("{};\t{}\t{} prev:{}", key, i, is_dupicated, unsafe {std::str::from_utf8_unchecked( aggr.hashes[i-1].key.as_slice() )});
            }
        }
    }
    assert_eq!(count, 413);
    println!("total entries: {}", count);
}