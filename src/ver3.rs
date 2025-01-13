use std::collections::HashMap;
use std::io::BufRead;
use crate::MEASUREMENT_FILE;

#[inline]
fn read_line<'a>(reader: &mut std::io::BufReader<std::fs::File>, line: &'a mut Vec<u8>) -> Result<Option<(&'a [u8], &'a [u8])>, Box<dyn std::error::Error>> {
    let pos = line.len();
    let n1 = reader.read_until(b';', line)?;
    if n1 > 0 {
        // let part1 = &line[pos..pos+n1-1];
        let n2 = reader.read_until(b'\n', line)?;
        if n2 > 0 {
            let part1 = &line[pos..pos + n1 - 1];
            let part2 = &line[pos + n1..pos + n1 + n2 - 1];
            Ok(Some((part1, part2)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}


#[allow(dead_code)]
#[inline(never)]
pub fn ver3() -> Result<HashMap<String,(f32,f32,f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;
    let mut reader = std::io::BufReader::new(file);

    struct Item {
        min: f32,
        max: f32,
        count: i32,
        sum: f32,
    }
    let mut hash: HashMap<String, Item> = std::collections::HashMap::new();
    let mut line: Vec<u8> = Vec::with_capacity(256);

    // name;value
    while let Some((p1, p2)) = read_line(&mut reader, &mut line)? {

        // p1 as &str
        let name = core::str::from_utf8(p1)?;
        let value = core::str::from_utf8(p2)?.parse::<f32>()?;

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
        (name.clone(), (item.min, item.max, item.sum / item.count as f32))
    }).collect();

    Ok(result)
}
