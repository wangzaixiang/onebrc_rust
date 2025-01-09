#![feature(core_intrinsics)]
#![feature(portable_simd)]

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
#[derive(Parser)]
#[command(version, author, about)]
struct Args {

    /// run all versions
    #[arg(long)]
    all: bool,

    /// run specific versions v1,v2,..,v8,v9
    #[arg(long)]
    runs: String,

    /// check the result
    #[arg(short, long)]
    check: bool,

    // /// baseline version
    // #[arg(long)]
    // v1: bool,
    //
    // /// baseline + reader.read_line() than reader.lines(), reuse line String
    // #[arg(long)]
    // v2: bool,
    //
    // /// v2 + read_line(): (&u8, &u8), no temp String
    // #[arg(long)]
    // v3: bool,
    //
    // /// v3 + read_line(): (&u8, u64), parse number inline
    // #[arg(long)]
    // v4: bool,
    //
    // /// v4 + using a StateMachine to parse the input
    // #[arg(long)]
    // v5: bool,
    //
    // /// v6: using mmap to read the file
    // #[arg(long)]
    // v6: bool,
    //
    // /// v7: v6 + using FxHashMap than HashMap
    // #[arg(long)]
    // v7: bool,
    //
    // /// v8: v7 + using simd to parse the input
    // #[arg(long, default_value = "true")]
    // v8: bool
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
        runs = vec!["v1".to_string(), "v2".to_string(), "v3".to_string(), "v4".to_string(), "v5".to_string(), "v6".to_string(), "v7".to_string(), "v8".to_string()];
    }

    args.runs.split(',').for_each(|v| {
        let v = &v.trim().to_string();
        if !runs.contains(v) {
            runs.push(v.clone());
        }
    });
    runs.sort();

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

const MEASUREMENT_FILE: &str = "/tmp/0108/1brc/measurements.txt";
