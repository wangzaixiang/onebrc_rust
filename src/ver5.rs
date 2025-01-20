use crate::MEASUREMENT_FILE;
use std::collections::HashMap;
use std::io::Read;


enum State {
    Name,
    Value,
}

struct StateMachine {
    state: State,
    name: [u8; 512],
    name_length: usize,
    value: i64,
    sign: i64,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::Name,
            name: [0; 512],
            name_length: 0,
            value: 0,
            sign: 1,
        }
    }

    #[inline]
    fn next<F>(&mut self, ch: u8, op: &mut F) where F: FnMut(&[u8], i64) {
        match self.state {
            State::Name => {
                debug_assert!(ch != b'\n');
                if ch == b';' {
                    self.state = State::Value;
                }
                else {
                    // avoid index out of bounds check
                    let p = &mut self.name as *mut u8;
                    unsafe { *p.offset(self.name_length as isize) = ch; }; // avoid bounds check
                    self.name_length += 1;
                }
            }
            State::Value => {
                if ch == b'\n' {
                    op(&self.name[0..self.name_length], self.value * self.sign);
                    self.state = State::Name;
                    self.name_length = 0;
                    self.value = 0;
                    self.sign = 1;
                } else if ch == b'.' {
                } else if ch == b'-' {
                    self.sign = -1;
                } else {
                    self.value = self.value * 10 + (ch - b'0') as i64;
                }
            }
        }
    }
}

#[allow(dead_code)]
#[inline(never)]
pub fn ver5() -> Result<HashMap<String, (f32,f32,f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;
    let reader = std::io::BufReader::new(file);

    struct Item {
        min: i64,
        max: i64,
        count: u32,
        sum: i64,
    }
    let mut hash: HashMap<String, Item> = HashMap::new();

    let mut callback = |name: &[u8], value: i64| {
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

    let mut machine = StateMachine::new();
    reader.bytes().for_each(|b| {
        machine.next(b.unwrap(), &mut callback);
    });

    let result = hash.iter().map(|(name, item)| {
        (name.to_string(), (item.min as f32 / 10.0, item.max as f32 / 10.0, item.sum as f32 / (10.0 * item.count as f32)))
    }).collect();

    Ok( result )
}
