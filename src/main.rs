#![feature(core_intrinsics)]
#![feature(portable_simd)]
#![feature(hasher_prefixfree_extras)]
#![feature(string_remove_matches)]
#![feature(unbounded_shifts)]
#![feature(unchecked_shifts)]
#![feature(ptr_sub_ptr)]

use std::collections::HashMap;
use clap::Parser;

mod ver1;
mod ver2;
mod ver3;
mod ver4;
mod ver5;
mod ver6;
mod ver8;
mod ver7;
mod ver10;
mod ver9;
mod ver11;
mod ver12;
mod ver13;
mod ver14;
mod ver15;
mod ver16;
mod ver17;
mod ver18;
mod ver20;
mod ver21;
mod ver22;

#[derive(Parser)]
#[command(version, author, about)]
struct Args {

    /// run all versions
    #[arg(long)]
    all: bool,

    /// run specific versions v1,v2,..,v8,v9
    #[arg(long)]
    runs: Option<String>,

    /// check the result
    #[arg(short, long)]
    check: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    let baseline = if args.check {
        timeit("run ver7 as baseline", || ver7::ver7())?
    } else {
        HashMap::new()
    };

    let mut runs = vec![];
    if args.all {
        runs = vec!["v1".to_string(), "v2".to_string(), "v3".to_string(), "v4".to_string(), "v5".to_string(), "v6".to_string(),
                    "v7".to_string(), "v8".to_string(), "v9".to_string(), "v10".to_string(), "v11".to_string(), "v12".to_string()];
    }

    if let Some(arg_runs) = args.runs {
        arg_runs.split(',').for_each(|v| {
            let v = &v.trim().to_string();
            if !runs.contains(v) {
                runs.push(v.clone());
            }
        });
    }
    runs.sort_by( |a,b| a.clone().remove_matches('v').cmp(&b.clone().remove_matches('v')));

    for run in &runs {
       let result = timeit(run, || {
           match run.as_str() {
               "v1" => ver1::ver1(),
               "v2" => ver2::ver2(),
               "v3" => ver3::ver3(),
               "v4" => ver4::ver4(),
               "v5" => ver5::ver5(),
               "v6" => ver6::ver6(),
               "v7" => ver7::ver7(),
               "v8" => ver8::ver8(),       // 23.01s
               "v9" => ver9::ver9(),       // 23.01s
               "v10" => ver10::ver10(),
               "v11" => ver11::ver11(),
               "v12" => ver12::ver12(),     // 13.44s, TODO!
               "v13" => ver13::ver13(),     // 8.96s   mask: u64
               "v14" => ver14::ver14(),     // 13.3s   mask: [u64;4]
               "v15" => ver15::ver15(),     // 10.57s   mask: u128
               "v16" => ver16::ver16(),     // based on v13
               "v17" => ver17::ver17(),     // optimize FileReader
               "v18" => ver18::ver18(),     // optimize FileReader
               "v20" => ver20::ver20(),     // 6.10s
               "v21" => ver21::ver21(),     // optimize FileReader
               "v22" => ver22::ver22(),
               // "compare_str_to_hash" => {
               //     ver18::compare_str_to_hash();
               //     Ok(HashMap::new())
               // } ,
               _ => panic!("unknown version")
           }
       })?;
       if args.check {
           verify(run, &result, &baseline);
       }
    }

    Ok(())

}

fn timeit<F, T>(label: &str, mut f: F) -> T where F: FnMut()-> T {
    let start = std::time::Instant::now();
    let t = f();
    let elapsed = start.elapsed();
    println!("{}: {:?}", label, elapsed);
    t
}

fn verify(label: &str, hash: &HashMap<String, (f32,f32,f32)>, baseline: &HashMap<String, (f32,f32,f32)>) {
    for (name, (min, max, avg)) in hash.iter() {
        let item = baseline.get(name).unwrap();
        assert!((min - item.0).abs() < 0.01, "{label} name: {}, min:{} expect: {}", name, min, item.0);
        assert!((max - item.1).abs() < 0.01, "{label} name: {}, max:{} expect: {}", name, max, item.1);
        assert!((avg - item.2).abs() < 0.01, "{label} name: {}, avg:{} expect: {}", name, avg, item.2);
    }
}





const MEASUREMENT_FILE: &str = "measurements.txt";
