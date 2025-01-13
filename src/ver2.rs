use std::collections::HashMap;
use std::io::BufRead;
use crate::MEASUREMENT_FILE;

// 132.82s
#[allow(dead_code)]
#[inline(never)]
pub fn ver2() -> Result<HashMap<String, (f32, f32, f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;
    let mut reader = std::io::BufReader::new(file);

    struct Item {
        min: f32,
        max: f32,
        count: i32,
        sum: f32,
    }
    let mut hash: HashMap<String, Item> = std::collections::HashMap::new();
    let mut line = String::new();

    // name;value
    while reader.read_line(&mut line)? > 0 {
        if line.ends_with('\n')  {
            line.pop();
        }

        let parts = line.split(';').collect::<Vec<&str>>();
        let name = parts.get(0).unwrap();
        let value = parts.get(1).unwrap().parse::<f32>()?;

        match hash.get_mut(*name) {
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
