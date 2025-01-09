#![feature(core_intrinsics)]
#![feature(portable_simd)]

use std::collections::HashMap;
use std::time::Duration;
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

    /// baseline version
    #[arg(long)]
    v1: bool,

    /// baseline + reader.read_line() than reader.lines(), reuse line String
    #[arg(long)]
    v2: bool,

    /// v2 + read_line(): (&u8, &u8), no temp String
    #[arg(long)]
    v3: bool,

    /// v3 + read_line(): (&u8, u64), parse number inline
    #[arg(long)]
    v4: bool,

    /// v4 + using a StateMachine to parse the input
    #[arg(long)]
    v5: bool,

    /// v6: using mmap to read the file
    #[arg(long)]
    v6: bool,

    /// v7: v6 + using FxHashMap than HashMap
    #[arg(long)]
    v7: bool,

    /// v8: v7 + using simd to parse the input
    #[arg(long, default_value = "true")]
    v8: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let baseline = ver7::ver7()?;
    let args = Args::parse();
    if args.all | args.v1 {     // 112.90s
        let result = timeit("ver1", || ver1::ver1() )?;
        verify("ver1", &result, &baseline);
    }
    if args.all | args.v2 {     // 91.64s
        let result = timeit("ver2", || ver2::ver2() )?;
        verify("ver2", &result, &baseline);
    }
    if args.all | args.v3 {     // 67.32s
        let result = timeit("ver3", || ver3::ver3() )?;
        verify("ver3", &result, &baseline);
    }
    if args.all | args.v4 {     // 53.36s
        let result = timeit("ver4", || ver4::ver4() )?;
        verify("ver4", &result, &baseline);
    }
    if args.all | args.v5 {     // 47.88s
        let result = timeit("ver5", || ver5::ver5() )?;
        verify("ver5", &result, &baseline);
    }
    if args.all | args.v6 {     // 31.00s
        let result = timeit("ver6", || ver6::ver6() )?;
        verify("ver6", &result, &baseline);
    }
    if args.all | args.v7 {     // 25.34s
        let result = timeit("ver7", || ver7::ver7() )?;
        verify("ver7", &result, &baseline);
    }
    if args.all | args.v8 {     // 38.00s
        let result = timeit("ver8", || ver8::ver8() )?;
        verify("ver8", &result, &baseline);
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
        assert_eq!(*min, item.0, "name: {}, min:{} expect: {}", name, min, item.0);
        assert_eq!(*max, item.1, "name: {}, max:{} expect: {}", name, max, item.1);
        assert_eq!(*avg, item.2, "name: {}, avg:{} expect: {}", name, avg, item.2);
    }
}

const MEASUREMENT_FILE: &str = "/tmp/0108/1brc/measurements.txt";
