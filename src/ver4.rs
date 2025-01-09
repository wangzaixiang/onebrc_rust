use std::collections::HashMap;
use std::io::BufRead;
use crate::MEASUREMENT_FILE;

#[inline]
fn read_line<'a>(reader: &mut std::io::BufReader<std::fs::File>, line: &'a mut Vec<u8>) -> Result<Option<(&'a [u8], i64)>, Box<dyn std::error::Error>> {
    let pos = line.len();
    let n1 = reader.read_until(b';', line)?;
    if n1 > 0 {
        // let part1 = &line[pos..pos+n1-1];
        let n2 = reader.read_until(b'\n', line)?;
        if n2 > 0 {
            let part1 = &line[pos..pos + n1 - 1];
            let part2 = &line[pos + n1..pos + n1 + n2 - 1];
            let mut value = 0i64;
            let mut sign = 1;
            for i in part2.iter() {
                if *i == b'.' {
                    continue
                } else if *i == b'-' {
                    sign = -1;
                } else {
                    value = value * 10 + (i - b'0') as i64;
                }
            }
            Ok(Some((part1, value * sign)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}


#[allow(dead_code)]
pub fn ver4() -> Result<HashMap<String, (f32,f32,f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;
    let mut reader = std::io::BufReader::new(file);

    struct Item {
        min: i64,
        max: i64,
        count: u32,
        sum: i64,
    }
    let mut hash: HashMap<String, Item> = HashMap::new();
    let mut line: Vec<u8> = Vec::with_capacity(256);

    // name;value
    while let Some((p1, value)) = read_line(&mut reader, &mut line)? {

        // p1 as &str
        let name = unsafe { core::str::from_utf8_unchecked(p1) };
        match hash.get_mut(name) {
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
                hash.insert(name.to_string(), item);
            }
        }
        line.clear();
    }

    let result = hash.iter().map(|(name, item)| {
        (name.clone(), (item.min as f32/ 10.0, item.max as f32 / 10.0, item.sum as f32 / item.count as f32 / 10.0))
    }).collect();

    Ok(result)
}
