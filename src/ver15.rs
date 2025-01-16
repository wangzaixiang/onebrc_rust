use std::collections::HashMap;
use std::intrinsics::{likely, unlikely};
use std::mem::{transmute};
use memchr::{memchr2};
use crate::MEASUREMENT_FILE;

use std::simd::{i16x16, i8x16, u32x4, u8x16, u8x64, Mask};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use memmap2::Mmap;

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
unsafe fn extend_i8x16(v: i8x16) -> i16x16 {
    use std::arch::aarch64::*;

    let v_p = &v as *const i8x16 as *const i8;

    let i16_1: int16x8_t = vmovl_s8( *(v_p as *const int8x8_t) );
    let i16_2: int16x8_t = vmovl_s8( *(v_p.add(8) as *const int8x8_t) );

    let result = int16x8x2_t(i16_1, i16_2);
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

    let v2  = unsafe { extend_i8x16(v2) };
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
    eof: bool,          // has more content
    cursor: usize,      // read_more will update, 当前读取位置，已读取并分析结果保存在 mask 中
    mask:   u128,        // read_more will set, next will clear
    line_begin: usize,    // next will update，下一行的开始位置
}

impl FileReader {

    fn new(mmap: Mmap) -> FileReader {
        let length = mmap.len();
        let buf = mmap.as_ptr();
        let mask_v1: u8x64 = u8x64::splat(b';');
        let mask_v2: u8x64 = u8x64::splat(b'\n');

        let u8x64 = u8x64::from_array( unsafe { *( buf as *const[u8;64]) } );
        let mask1: Mask<i8, 64> = u8x64.simd_eq(mask_v1) | u8x64.simd_eq(mask_v2);
        let mask1 = mask1.to_bitmask();

        let u8x64 = u8x64::from_array( unsafe { *( buf.add(64) as *const[u8;64]) } );
        let mask2: Mask<i8, 64> = u8x64.simd_eq(mask_v1) | u8x64.simd_eq(mask_v2);
        let mask2 = mask2.to_bitmask();

        FileReader {
            _mmap: mmap,
            length,
            buf,
            eof: false,
            cursor: 0,
            mask: mask1 as u128 | ((mask2 as u128)<< 64),
            line_begin: 0
        }
    }

    #[inline]
    fn read_block_at_cursor(&mut self) {
        // change to unlikely fastup from 11.5s ~ 6.65s
        if unlikely(self.mask == 0) {    // need more

            self.cursor += 128;

            if likely(self.cursor + 128 <= self.length) {
                let mask_v1: u8x16 = u8x16::splat(b';');
                let mask_v2: u8x16 = u8x16::splat(b'\n');

                let v1 = u8x16::from_array( unsafe { *( self.buf.add(self.cursor) as *const[u8;16]) } );
                let v2 = u8x16::from_array( unsafe { *( self.buf.add(self.cursor + 16) as *const[u8;16]) } );
                let v3 = u8x16::from_array( unsafe { *( self.buf.add(self.cursor + 32) as *const[u8;16]) } );
                let v4 = u8x16::from_array( unsafe { *( self.buf.add(self.cursor + 48) as *const[u8;16]) } );
                let v5 = u8x16::from_array( unsafe { *( self.buf.add(self.cursor + 64) as *const[u8;16]) } );
                let v6 = u8x16::from_array( unsafe { *( self.buf.add(self.cursor + 80) as *const[u8;16]) } );
                let v7 = u8x16::from_array( unsafe { *( self.buf.add(self.cursor + 96) as *const[u8;16]) } );
                let v8 = u8x16::from_array( unsafe { *( self.buf.add(self.cursor + 112) as *const[u8;16]) } );

                let v1_mask = (v1.simd_eq(mask_v1) | v1.simd_eq(mask_v2)).to_bitmask() as u128;
                let v2_mask = (v2.simd_eq(mask_v1) | v2.simd_eq(mask_v2)).to_bitmask() as u128;
                let v3_mask = (v3.simd_eq(mask_v1) | v3.simd_eq(mask_v2)).to_bitmask() as u128;
                let v4_mask = (v4.simd_eq(mask_v1) | v4.simd_eq(mask_v2)).to_bitmask() as u128;
                let v5_mask = (v5.simd_eq(mask_v1) | v5.simd_eq(mask_v2)).to_bitmask() as u128;
                let v6_mask = (v6.simd_eq(mask_v1) | v6.simd_eq(mask_v2)).to_bitmask() as u128;
                let v7_mask = (v7.simd_eq(mask_v1) | v7.simd_eq(mask_v2)).to_bitmask() as u128;
                let v8_mask = (v8.simd_eq(mask_v1) | v8.simd_eq(mask_v2)).to_bitmask() as u128;
                self.mask = v1_mask | (v2_mask << 16) | (v3_mask << 32) | (v4_mask << 48) |
                    v5_mask << 64 | v6_mask << 80 | v7_mask << 96 | v8_mask << 112;
            }
            else {
                self.read_last_block();      //
            }
        }
    }

    #[inline(never)]
    fn read_last_block(&mut self) {
        let ptr = unsafe { self.buf.add(self.cursor) };
        let count = self.length - self.cursor;  // maybe zero
        let slice = unsafe { std::slice::from_raw_parts(ptr, count) };
        let mut base = 0usize;
        loop {
            if base >= count {
                break;
            }
            match memchr2(b';', b'\n', &slice[base..]) {
                Some(index) => {
                    self.mask |= 1 << (base+index);
                    base += index+1;
                }
                _ => {
                    panic!("tail block should always have a match");
                }
            }
        }
        self.eof = true;
    }

    // TODO wrongs for the last line

    fn next(&mut self) -> Option<(&'static [u8], &'static [u8])> {
        if likely(self.eof == false) {
            self.read_block_at_cursor();
            let first = {
                let index = self.mask.trailing_zeros();
                self.mask &= !(1 << index);
                self.cursor + index as usize
            };

            self.read_block_at_cursor();
            let second = {
                let index = self.mask.trailing_zeros();
                self.mask &= !(1 << index);
                self.cursor + index as usize
            };

            let key: &[u8] = unsafe { std::slice::from_raw_parts(self.buf.add(self.line_begin), first - self.line_begin) };
            let value: &[u8] = unsafe { std::slice::from_raw_parts(self.buf.add(first + 1), second - first - 1) };

            let result = (key, value);
            self.line_begin = second + 1;
            Some(result)
        }
        else {
            None
        }
    }
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
fn process_one(name: &[u8], value: &[u8], aggr: &mut AggrInfo) {

    let ptr = name.as_ptr();
    let key_a: u64 = u64::from_le_bytes( unsafe { *(ptr as *const[u8;8]) });
    let key_b: u64 = u64::from_le_bytes( unsafe { *(ptr.add(8) as *const[u8;8]) });

    let len_a = if name.len() >= 8 { 8 } else { name.len() };
    let len_b = if name.len() >= 16 { 8 } else if name.len() > 8 { name.len() - 8 }  else { 0 };

    let key_a_1 = key_a & MASKS[len_a];
    let key_b_1 = key_b & MASKS[len_b];

    let v1 = parse_value(value);
    aggr.save_item(name, key_a_1, key_b_1, v1);
}

#[inline(never)]
// based on ver12
pub fn ver15() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;

    let mmap = unsafe { Mmap::map(&file)? };

    let mut reader = FileReader::new(mmap);

    let mut aggr = AggrInfo::new();

    loop {
        let r1 = reader.next();
        let r2 = reader.next();
        let r3 = reader.next();
        let r4 = reader.next();

        if likely(r4.is_some()) {
            let Some((r1_name, r1_value)) = r1 else { unreachable!() };
            let Some((r2_name, r2_value)) = r2 else { unreachable!() };
            let Some((r3_name, r3_value)) = r3 else { unreachable!() };
            let Some((r4_name, r4_value)) = r4 else { unreachable!() };

            let ptr1 = r1_name.as_ptr();
            let key_a_1: u64 = u64::from_le_bytes( unsafe { *(ptr1 as *const[u8;8]) });
            let key_b_1: u64 = u64::from_le_bytes( unsafe { *(ptr1.add(8) as *const[u8;8]) });

            let ptr2 = r2_name.as_ptr();
            let key_a_2: u64 = u64::from_le_bytes( unsafe { *(ptr2 as *const[u8;8]) });
            let key_b_2: u64 = u64::from_le_bytes( unsafe { *(ptr2.add(8) as *const[u8;8]) });

            let ptr3 = r3_name.as_ptr();
            let key_a_3: u64 = u64::from_le_bytes( unsafe { *(ptr3 as *const[u8;8]) });
            let key_b_3: u64 = u64::from_le_bytes( unsafe { *(ptr3.add(8) as *const[u8;8]) });

            let ptr4 = r4_name.as_ptr();
            let key_a_4: u64 = u64::from_le_bytes( unsafe { *(ptr4 as *const[u8;8]) });
            let key_b_4: u64 = u64::from_le_bytes( unsafe { *(ptr4.add(8) as *const[u8;8]) });


            let len_a_1 = if r1_name.len() >= 8 { 8 } else { r1_name.len() };
            let len_b_1 = if r1_name.len() >= 16 { 8 } else if r1_name.len() > 8 { r1_name.len() - 8 }  else { 0 };

            let len_a_2 = if r2_name.len() >= 8 { 8 } else { r2_name.len() };
            let len_b_2 = if r2_name.len() >= 16 { 8 } else if r2_name.len() > 8 { r2_name.len() - 8 }  else { 0 };

            let len_a_3 = if r3_name.len() >= 8 { 8 } else { r3_name.len() };
            let len_b_3 = if r3_name.len() >= 16 { 8 } else if r3_name.len() > 8 { r3_name.len() - 8 }  else { 0 };

            let len_a_4 = if r4_name.len() >= 8 { 8 } else { r4_name.len()  };
            let len_b_4 = if r4_name.len() >= 16 { 8 } else if r4_name.len() > 8 { r4_name.len() - 8 }  else { 0 };

            let key_a_1 = key_a_1 & MASKS[len_a_1];
            let key_b_1 = key_b_1 & MASKS[len_b_1];
            let key_a_2 = key_a_2 & MASKS[len_a_2];
            let key_b_2 = key_b_2 & MASKS[len_b_2];
            let key_a_3 = key_a_3 & MASKS[len_a_3];
            let key_b_3 = key_b_3 & MASKS[len_b_3];
            let key_a_4 = key_a_4 & MASKS[len_a_4];
            let key_b_4 = key_b_4 & MASKS[len_b_4];

            let (v1, v2, v3, v4) = unsafe { parse_values(r1_value, r2_value, r3_value, r4_value) };
            aggr.save_item(r1_name, key_a_1, key_b_1, v1);
            aggr.save_item(r2_name, key_a_2, key_b_2, v2);
            aggr.save_item(r3_name, key_a_3, key_b_3, v3);
            aggr.save_item(r4_name, key_a_4, key_b_4, v4);

        }
        else {
            if let Some((name, value)) = r1 {
                process_one(name, value, &mut aggr);
            }
            if let Some((name, value)) = r2 {
                process_one(name, value, &mut aggr);
            }
            if let Some((name, value)) = r3 {
                process_one(name, value, &mut aggr);
            }
            break;
        }
    }

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

