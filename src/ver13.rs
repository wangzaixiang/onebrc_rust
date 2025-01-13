use std::collections::HashMap;
use std::intrinsics::{likely, ptr_offset_from, unlikely};
use memchr::{memchr2};
use memmap2::Mmap;
use crate::MEASUREMENT_FILE;

use std::simd::{u8x64};
use std::simd::cmp::SimdPartialEq;

#[inline]
fn _parse_value(_buf: &[u8]) -> i16 {    // ~0.5s
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

type BlockOffset = i16; // 32K at most

struct FileReader {
    _mmap: Mmap,         // const
    length: usize,      // const
    buf: *const u8,     // const

    block_offset: usize,      // read_more will update
    positions: [BlockOffset; FileReader::POSITIONS_SIZE], // 4k block < 1k lines, maybe negative
    position_count: u16,    // next will update
    position_index: u16,    //
    // mask:   u64,        // read_more will set, next will clear
    line_begin: usize,    // next will update
}


impl FileReader {


    const BLOCK_SIZE: usize = 4096;
    const POSITIONS_SIZE: usize = FileReader::BLOCK_SIZE / 2;
    fn new(mmap: Mmap) -> FileReader {
        let length = mmap.len();
        let buf = mmap.as_ptr();

        let mut reader = FileReader {
            _mmap: mmap,
            length,
            buf,

            block_offset: 0,
            positions: [0; FileReader::POSITIONS_SIZE],
            position_count: 0,
            position_index: 0,
            line_begin: 0
        };
        reader.load_block();
        reader
    }

    #[inline]
    fn add_poistion(&mut self, offset: BlockOffset) {
        self.positions[self.position_count as usize] = offset;
        self.position_count += 1;
    }

    #[inline(never)]
    fn advance_block(&mut self) {
        debug_assert!( self.block_offset + Self::BLOCK_SIZE <= self.length, "dont advance if already that last block" );

        if likely ( self.position_count == self.position_index ) {
            self.position_count = 0;
            self.position_index = 0;
        }
        else {
            debug_assert_eq!(self.position_index, self.position_count - 1);
            self.positions[0] = self.positions[self.position_count as usize - 1] - Self::BLOCK_SIZE as i16;
            self.position_index = 0;
            self.position_count = 1;
        }

        self.block_offset += Self::BLOCK_SIZE;
        self.load_block();
    }

    #[inline(never)]
    fn load_block(&mut self) {
        if likely( self.block_offset + Self::BLOCK_SIZE <= self.length ) { // read BLOCK_SIZE using SIMD
            let mask_v1: u8x64 = u8x64::splat(b';');
            let mask_v2: u8x64 = u8x64::splat(b'\n');

            let mut ptr = unsafe { self.buf.add(self.block_offset as usize) };
            let mut offset_base = 0i16;
            let end = unsafe { ptr.add(Self::BLOCK_SIZE as usize) };

            while ptr < end {
                let v1 = unsafe { u8x64::from_array(*(ptr as *const [u8;64])) };
                let v2 = unsafe { u8x64::from_array(*(ptr.add(64) as *const [u8;64])) };
                let mut mask1 = (v1.simd_eq(mask_v1) | v1.simd_eq(mask_v2)).to_bitmask();
                let mut mask2 = (v2.simd_eq(mask_v1) | v2.simd_eq(mask_v2)).to_bitmask();

                while mask1 != 0 {
                    let idx = mask1.trailing_zeros() as i16;
                    self.add_poistion(idx + offset_base);
                    mask1 &= !(1 << idx);
                }
                offset_base += 64;

                while mask2 != 0 {
                    let idx = mask2.trailing_zeros() as i16;
                    self.add_poistion(idx + offset_base);
                    mask2 &= !(1 << idx);
                }
                offset_base += 64;

                unsafe { ptr = ptr.add(128) };
            }
        }
        else {
            let start =  unsafe { self.buf.add(self.block_offset) };
            let end = unsafe { self.buf.add(self.length) };
            let remaining = self.length - self.block_offset;
            let mut ptr =   start;
            while ptr < end {
                let offset_from_start = unsafe { ptr_offset_from(ptr, start) } as usize;
                let slice = unsafe { std::slice::from_raw_parts(ptr, remaining - offset_from_start) };
                match memchr2(b';', b'\n', slice) {
                    Some(index) => {
                        self.add_poistion((offset_from_start  + index) as i16);
                        ptr = unsafe { ptr.add(index+1) };
                    }
                    _ => {
                        panic!("tail block should always have a match");
                    }
                }
            }
        }
    }

    fn next(&mut self) -> Option<(&[u8], &[u8])> {
        if unlikely(self.position_index + 1 >= self.position_count) { // require advance_block
            if self.block_offset + Self::BLOCK_SIZE < self.length {
                self.advance_block();
            } else {
                return None;
            }
        }
        unsafe {
            let pos1 = ((self.block_offset as isize) + self.positions[self.position_index as usize] as isize) as usize;
            let pos2 = ((self.block_offset as isize) + self.positions[self.position_index as usize + 1] as isize) as usize;
            self.position_index += 2;
            let key = std::slice::from_raw_parts(self.buf.add(self.line_begin), pos1 - self.line_begin);
            let value = std::slice::from_raw_parts(self.buf.add(pos1 + 1), pos2 - pos1 - 1);
            self.line_begin = pos2 + 1;
            Some((key, value))
        }
    }
}

#[inline(never)]
pub fn ver13() -> Result<HashMap<String,(f32, f32, f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;

    let mmap = unsafe { Mmap::map(&file)? };
    let mut reader = FileReader::new(mmap);

    let mut count = 0;
    while let Some((_a,_b)) = reader.next() {
        count += 1;
    }

    println!("count: {}", count);
    Ok( HashMap::new() )
}

