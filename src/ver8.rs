use std::collections::HashMap;
use std::intrinsics::unlikely;
use memchr::{memchr2};
use memmap2::Mmap;
use rustc_hash::FxHashMap;
use crate::MEASUREMENT_FILE;

use std::simd::{u8x64, Mask};
use std::simd::cmp::SimdPartialEq;

#[inline]
fn parse_value(_buf: &[u8]) -> i32 {    // ~0.5s
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
    value * sign
}

pub fn ver8() -> Result<HashMap<String,(f32,f32,f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;

    let mmap = unsafe { Mmap::map(&file)? };
    let buf = mmap.as_ref();

    struct Item {
        min: i32,
        max: i32,
        count: u32,
        sum: i32,
    }
    // let mut hash = HashMap::with_capacity_and_hasher(16384, fasthash::spooky::Hash64);
    let mut hash:FxHashMap<String, Item> = FxHashMap::with_capacity_and_hasher(16384, rustc_hash::FxBuildHasher::default());

    let mut callback = |name: &[u8], value: i32| {
        match hash.get_mut(unsafe { core::str::from_utf8_unchecked(name) }) {
            Some(item) => {
                item.count += 1;
                item.sum += value;
                item.min = item.min.min(value);
                item.max = item.max.max(value);
            }
            None => {
                let item = Item {
                    min: value,
                    max: value,
                    count: 1,
                    sum: value
                };
                hash.insert(unsafe { core::str::from_utf8_unchecked(name) }.to_string(), item);
            }
        }
    };

    let v01 = u8x64::splat(b';');
    let v02 = u8x64::splat(b'\n');

    enum State {
        BEGIN(usize),  // only having line_begin
        POS1(usize, usize),   // having line_begin, POS1
    }

    // line_begin, pos1, pos2
    let mut v1: u8x64 = u8x64::from_slice(buf);        // 64 bytes
    let mut mask: Mask<i8,64> = v1.simd_eq(v01) | v1.simd_eq(v02);
    let mut v1_pos: usize = 0;
    let mut raw_pos: usize = 0; // for tail processing
    let mut state = State::BEGIN(0);

    let mut move_forward = || -> usize {
        loop {
            if let Some(index) = mask.first_set() {
                mask.set(index, false);
                return v1_pos + index;
            } else {
                if unlikely(raw_pos > 0) {
                    return if let Some(index) = memchr2(b';', b'\n', &buf[raw_pos..]) {
                        let result = raw_pos + index;
                        raw_pos = raw_pos + index + 1;
                        result
                    } else {
                        panic!("expecting a line end");
                    }
                }
                else {
                    v1_pos += 64;
                    if v1_pos + 64 > buf.len() {
                        raw_pos = v1_pos;
                        continue;
                    }
                    else {
                        v1 = u8x64::from_slice(&buf[v1_pos..v1_pos + 64]);
                        mask = v1.simd_eq(v01) | v1.simd_eq(v02);
                        continue;
                    }
                }
            }
        }
    };

    loop {
        match state {
            State::BEGIN(begin)  =>{
                if begin >= buf.len() {
                    break;
                }
                else {
                    let pos = move_forward();
                    state = State::POS1(begin, pos);
                    continue;
                }
            }
            State::POS1(begin, pos1) => {
                let pos2 = move_forward();
                callback(&buf[begin..pos1], parse_value(&buf[pos1+1..pos2]));
                state = State::BEGIN(pos2+1);
                continue;
            }
        }
    }

    let result = hash.iter().map(|(name, item)| {
        (name.clone(), (item.min as f32/ 10.0, item.max as f32 / 10.0, item.sum as f32 / item.count as f32 / 10.0))
    }).collect();

    Ok(result)
}
