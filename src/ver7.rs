use std::collections::HashMap;
use memchr::memchr;
use memmap2::Mmap;
use rustc_hash::FxHashMap;
use crate::MEASUREMENT_FILE;


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

#[allow(dead_code)]
pub fn ver7() -> Result<HashMap<String,(f32,f32,f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;

    let mmap = unsafe { Mmap::map(&file)? };
    let mut buf = mmap.as_ref();

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

    let mut _no_used = 0;
    // let mut callback = |_name: &[u8], value: i32| {
    //     _no_used += value;
    // };

    loop {
        match memchr(b';', &buf[0..]) { // scan ~14s
            Some(pos1) => {
                let name = &buf[0..pos1];
                let remain = &buf[pos1+1..];
                match memchr(b'\n', remain) {
                    Some(pos2) => {
                        let value = parse_value(&remain[0..pos2]);
                        callback(name, value);      // 7.5s
                        buf = &remain[pos2+1..];
                    }
                    None => {
                        break;
                    }
                }
            }
            None => {
                break;
            }
        }
    }

   let result = hash.iter().map(|(name, item)| {
        (name.clone(), (item.min as f32/ 10.0, item.max as f32 / 10.0, item.sum as f32 / item.count as f32 / 10.0))
    }).collect();

    Ok(result)
}
