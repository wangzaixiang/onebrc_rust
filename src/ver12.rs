use std::collections::HashMap;
use std::intrinsics::{likely, unlikely};
use memchr::{memchr2};
use crate::MEASUREMENT_FILE;

use std::simd::{u8x64, Mask};
use std::simd::cmp::SimdPartialEq;
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

struct FileReader {
    _mmap: Mmap,         // const
    length: usize,      // const
    buf: *const u8,     // const
    eof: bool,          // has more content
    cursor: usize,      // read_more will update, 当前读取位置，已读取并分析结果保存在 mask 中
    mask:   u64,        // read_more will set, next will clear
    line_begin: usize,    // next will update，下一行的开始位置
}

impl FileReader {

    fn new(mmap: Mmap) -> FileReader {
        let length = mmap.len();
        let buf = mmap.as_ptr();
        let u8x64 = u8x64::from_array( unsafe { *( buf as *const[u8;64]) } );
        let mask_v1: u8x64 = u8x64::splat(b';');
        let mask_v2: u8x64 = u8x64::splat(b'\n');
        let mask: Mask<i8, 64> = u8x64.simd_eq(mask_v1) | u8x64.simd_eq(mask_v2);
        let mask = mask.to_bitmask();
        FileReader {
            _mmap: mmap,
            length,
            buf,
            eof: false,
            cursor: 0,
            mask,
            line_begin: 0
        }
    }

    #[inline]
    fn read_block_at_cursor(&mut self) {
        // change to unlikely fastup from 11.5s ~ 6.65s
        if unlikely(self.mask == 0) {    // need more

            self.cursor += 64;

            if likely(self.cursor + 64 <= self.length) {
                let mask_v1: u8x64 = u8x64::splat(b';');
                let mask_v2: u8x64 = u8x64::splat(b'\n');

                let u8x64 = u8x64::from_array( unsafe { *( self.buf.add(self.cursor) as *const[u8;64]) } );
                let mask: Mask<i8, 64> = u8x64.simd_eq(mask_v1) | u8x64.simd_eq(mask_v2);
                self.mask = mask.to_bitmask();
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

    fn next(&mut self) -> Option<(&[u8], &[u8])> {
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

#[test]
fn test_x(){

    let arr = &[ 0x11u8, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x12, 0x23];

}

#[inline(never)]
pub fn ver12() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;

    let mmap = unsafe { Mmap::map(&file)? };

    let mut reader = FileReader::new(mmap);

    let mut aggr = AggrInfo::new();
    let mut max_len = 0usize;
    let mut min_len = usize::MAX;

    // let mut sum = 0;
    let mut callback = |name: &[u8], value: i16| {  // ~13.5s 60%
        let ptr = name.as_ptr() ;

        let key_a = u64::from_le_bytes( unsafe {  *(ptr as *const [u8;8]) }   );
        let key_b = u64::from_le_bytes( unsafe {  *(ptr.offset(8) as *const [u8;8]) }   );

        let len_a = if name.len() >= 8 { 64 } else { name.len() * 8 };
        let len_b = if name.len() >= 16 { 64 } else if name.len() > 8 { name.len() * 8 - 64 }  else { 0 };
        let mask_a = u64::MAX.unbounded_shr((64 - len_a) as u32) ;
        let mask_b = u64::MAX.unbounded_shr((64 - len_b) as u32) ;

        let key_a = key_a & mask_a;
        let key_b = key_b & mask_b;

        aggr.save_item(name, key_a, key_b, value);
    };

    while let Some((a,b)) = reader.next() {
        let len = a.len() + b.len() + 2;
        max_len = max_len.max(len);
        min_len = min_len.min(len);
        let value = parse_value(b);
        callback(a, value);
    }

    println!("max_len: {}, min_len: {}", max_len, min_len);

    // check dupicated
    // let mut count = 0;
    // for i in 0.. aggr.hashes.len() {
    //     let item = & aggr.hashes[i];
    //     if !item.key.is_empty() {
    //         count += 1;
    //         let check = if i> 0 {
    //             item.key_a == aggr.hashes[i-1].key_a && item.key_b == aggr.hashes[i-1].key_b
    //         }
    //         else {
    //             false
    //         };
    //         let key = unsafe { std::str::from_utf8_unchecked( item.key.as_slice() ) };
    //         println!("{};\t{}\t{}", key, i, check);
    //     }
    // }
    // assert_eq!(count, 413);
    // println!("total entries: {}", count);

    Ok( HashMap::new() )
}

#[test]
fn test_ver12() {
    let result = ver12();
    assert!(result.is_ok());
}
