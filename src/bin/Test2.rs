#![feature(portable_simd)]

use std::arch::asm;
use std::hint::black_box;
use std::intrinsics::transmute;
use std::ops::BitXor;
use std::simd::{i8x16, u64x2, u8x16};
use std::simd::cmp::SimdPartialOrd;

fn main() {

    let binding = std::env::args().nth(1).unwrap();
    let what = binding.split(',');

    for x in what {
        match x {
            "test1" => test1(),
            "test2" => test2(),
            "test2_1" => test2_1(),
            "test3" => test3(),
            "test4" => test4(),
            "test5" => test5(),
            "test6" => test6(),
            "test7" => test7(),
            "test8" => test8(),
            "test9" => test9(),
            _ => println!("no test"),
        }
    }

}


/// loop 1B * 24 add r1, r1, r0  cost 24 instr/2.2 ns = 10.9 instr/ns
/**
 *         cycles: 31723802902
 *   instructions: 118668948871
 *       branches: 4563731180
 *   branch-misses: 27
 *   IPC: 3.74
 *   M1 has only 4 ALU,
 */
fn test1(){
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;

    let time0 = std::time::Instant::now();

    for _ in 0..100_0000_0000u64 {

        unsafe { asm! {
        "add {i1}, {i1}, {base0}",
        "add {i2}, {i2}, {base0}",
        "add {i3}, {i3}, {base0}",
        "add {i4}, {i4}, {base0}",
        "add {i5}, {i5}, {base0}",
        "add {i6}, {i6}, {base0}",
        "add {i7}, {i7}, {base0}",
        "add {i8}, {i8}, {base0}",
        "add {i9}, {i9}, {base0}",
        "add {i10}, {i10}, {base0}",
        "add {i11}, {i11}, {base0}",
        "add {i12}, {i12}, {base0}",
        "add {i1}, {i1}, {base1}",
        "add {i2}, {i2}, {base1}",
        "add {i3}, {i3}, {base1}",
        "add {i4}, {i4}, {base1}",
        "add {i5}, {i5}, {base1}",
        "add {i6}, {i6}, {base1}",
        "add {i7}, {i7}, {base1}",
        "add {i8}, {i8}, {base1}",
        "add {i9}, {i9}, {base1}",
        "add {i10}, {i10}, {base1}",
        "add {i11}, {i11}, {base1}",
        "add {i12}, {i12}, {base1}",
        base0 = in(reg) base0,
        base1 = in(reg) base1,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test1 total time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / 100_0000_0000.0);
}

/**
       cycles: 31681558398
 instructions: 58422024472
     branches: 2246445308
branch-misses: 141
IPC: 1.84
按道理这个只能跑到 1，为什么能做到 1.84?
 */
fn test2(){
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;

    let time0 = std::time::Instant::now();

    for _ in 0..100_0000_0000u64 {

        unsafe { asm! {
        "add {i1}, {i1}, {base0}",
        "add {i2}, {i2}, {i1}",
        "add {i3}, {i3}, {i2}",
        "add {i4}, {i4}, {i3}",
        "add {i5}, {i5}, {i4}",
        "add {i6}, {i6}, {i5}",
        "add {i7}, {i7}, {i6}",
        "add {i8}, {i8}, {i7}",
        "add {i9}, {i9}, {i8}",
        "add {i10}, {i10}, {i9}",
        "add {i11}, {i11}, {i10}",
        "add {i12}, {i12}, {i11}",
        "add {i1}, {i1}, {i12}",
        "add {i2}, {i2}, {i1}",
        "add {i3}, {i3}, {i2}",
        "add {i4}, {i4}, {i3}",
        "add {i5}, {i5}, {i4}",
        "add {i6}, {i6}, {i5}",
        "add {i7}, {i7}, {i6}",
        "add {i8}, {i8}, {i7}",
        "add {i9}, {i9}, {i8}",
        "add {i10}, {i10}, {i9}",
        "add {i11}, {i11}, {i10}",
        "add {i12}, {i12}, {i11}",
        base0 = in(reg) base0,
        // base1 = in(reg) base1,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test2 total time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / 100_0000_0000.0);
}

fn test2_1(){
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;

    let time0 = std::time::Instant::now();

    for _ in 0..100_0000_0000u64 {

        unsafe { asm! {
        "add {i1}, {i12}, {base0}",
        "add {i2}, {i2}, {i1}",
        "add {i3}, {i3}, {i2}",
        "add {i4}, {i4}, {i3}",
        "add {i5}, {i5}, {i4}",
        "add {i6}, {i6}, {i5}",
        "add {i7}, {i7}, {i6}",
        "add {i8}, {i8}, {i7}",
        "add {i9}, {i9}, {i8}",
        "add {i10}, {i10}, {i9}",
        "add {i11}, {i11}, {i10}",
        "add {i12}, {i12}, {i11}",
        "add {i1}, {i1}, {i12}",
        "add {i2}, {i2}, {i1}",
        "add {i3}, {i3}, {i2}",
        "add {i4}, {i4}, {i3}",
        "add {i5}, {i5}, {i4}",
        "add {i6}, {i6}, {i5}",
        "add {i7}, {i7}, {i6}",
        "add {i8}, {i8}, {i7}",
        "add {i9}, {i9}, {i8}",
        "add {i10}, {i10}, {i9}",
        "add {i11}, {i11}, {i10}",
        "add {i12}, {i12}, {i11}",
        base0 = in(reg) base0,
        // base1 = in(reg) base1,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test2_1 total time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / 100_0000_0000.0);
}


/**
       cycles: 25329280618
 instructions: 125412636671
     branches: 4823063259
branch-misses: 10
IPC: 4.95
对 test2 进行简单改写，每4条指令是一组数据依赖。IPC 比 test2 高合乎逻辑，
但不止1倍？
 */
fn test9(){
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;

    let time0 = std::time::Instant::now();

    for _ in 0..100_0000_0000u64 {

        unsafe { asm! {
        "add {i1}, {i1}, {base0}",
        "add {i2}, {i2}, {i1}",
        "add {i3}, {i3}, {i2}",
        "add {i4}, {i4}, {i3}",
        "add {i5}, {i5}, {base0}",
        "add {i6}, {i6}, {i5}",
        "add {i7}, {i7}, {i6}",
        "add {i8}, {i8}, {i7}",
        "add {i9}, {i9}, {base0}",
        "add {i10}, {i10}, {i9}",
        "add {i11}, {i11}, {i10}",
        "add {i12}, {i12}, {i11}",
        "add {i1}, {i1}, {base0}",
        "add {i2}, {i2}, {i1}",
        "add {i3}, {i3}, {i2}",
        "add {i4}, {i4}, {i3}",
        "add {i5}, {i5}, {base0}",
        "add {i6}, {i6}, {i5}",
        "add {i7}, {i7}, {i6}",
        "add {i8}, {i8}, {i7}",
        "add {i9}, {i9}, {base0}",
        "add {i10}, {i10}, {i9}",
        "add {i11}, {i11}, {i10}",
        "add {i12}, {i12}, {i11}",
        base0 = in(reg) base0,
        // base1 = in(reg) base1,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test9 total time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / 100_0000_0000.0);
}

/**
        cycles: 31640832593
  instructions: 205405799211
      branches: 7899480667
 branch-misses: 44
IPC: 6.49
*/
fn test3(){
    let array = [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;
    let p = &array[0];

    let time0 = std::time::Instant::now();
    const LOOP: u64 = 200_0000_0000u64;

    for _ in 0..LOOP {

        unsafe { asm! {
        "ldr {tmp}, [{p}]",
        "add {i1}, {i1}, {tmp}",
        "ldr {tmp}, [{p},  8]",
        "add {i2}, {i2}, {tmp}",
        "ldr {tmp}, [{p} , 16]",
        "add {i3}, {i3}, {tmp}",
        "ldr {tmp}, [{p} , 24]",
        "add {i4}, {i4}, {tmp}",
        "ldr {tmp}, [{p} , 32]",
        "add {i5}, {i5}, {tmp}",
        "ldr {tmp}, [{p} , 40]",
        "add {i6}, {i6}, {tmp}",
        "ldr {tmp}, [{p} , 48]",
        "add {i7}, {i7}, {tmp}",
        "ldr {tmp}, [{p} , 56]",
        "add {i8}, {i8}, {tmp}",
        "ldr {tmp}, [{p} , 64]",
        "add {i9}, {i9}, {tmp}",
        "ldr {tmp}, [{p} , 72]",
        "add {i10}, {i10}, {tmp}",
        "ldr {tmp}, [{p} , 80]",
        "add {i11}, {i11}, {tmp}",
        "ldr {tmp}, [{p} , 88]",
        "add {i12}, {i12}, {tmp}",
        p = in(reg) p,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        tmp = out(reg) _,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test3 total time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / LOOP as f64);
}


/**
       cycles: 31685915023
 instructions: 205720965401
     branches: 7911667559
branch-misses: 109
IPC: 6.48
*/
fn test4(){
    let array = [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;
    let p = &array[0];

    let time0 = std::time::Instant::now();
    const LOOP: u64 = 200_0000_0000u64;

    for _ in 0..LOOP {

        unsafe { asm! {
        "ldr {tmp}, [{p}]",
        "add {i1}, {i1}, {tmp}",
        "ldr {tmp}, [{p},  8]",
        "add {i2}, {i1}, {tmp}",
        "ldr {tmp}, [{p} , 16]",
        "add {i3}, {i2}, {tmp}",
        "ldr {tmp}, [{p} , 24]",
        "add {i4}, {i3}, {tmp}",
        "ldr {tmp}, [{p} , 32]",
        "add {i5}, {i4}, {tmp}",
        "ldr {tmp}, [{p} , 40]",
        "add {i6}, {i5}, {tmp}",
        "ldr {tmp}, [{p} , 48]",
        "add {i7}, {i6}, {tmp}",
        "ldr {tmp}, [{p} , 56]",
        "add {i8}, {i7}, {tmp}",
        "ldr {tmp}, [{p} , 64]",
        "add {i9}, {i8}, {tmp}",
        "ldr {tmp}, [{p} , 72]",
        "add {i10}, {i9}, {tmp}",
        "ldr {tmp}, [{p} , 80]",
        "add {i11}, {i10}, {tmp}",
        "ldr {tmp}, [{p} , 88]",
        "add {i12}, {i11}, {tmp}",
        p = in(reg) p,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        tmp = out(reg) _,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test4 time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / LOOP as f64);
}

/**
        cycles: 31620787041
  instructions: 205166219057
      branches: 7889817327
IPC: 6.48
*/
fn test6(){
    let array = [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;
    let p = &array[0] as *const u64 as *const u8;
    let p = unsafe { p.offset(1 ) };

    let time0 = std::time::Instant::now();
    const LOOP: u64 = 200_0000_0000u64;

    for _ in 0..LOOP {

        unsafe { asm! {
        "ldur {tmp}, [{p}]",
        "add {i1}, {i1}, {tmp}",
        "ldur {tmp}, [{p},  8]",
        "add {i2}, {i1}, {tmp}",
        "ldur {tmp}, [{p} , 16]",
        "add {i3}, {i2}, {tmp}",
        "ldur {tmp}, [{p} , 24]",
        "add {i4}, {i3}, {tmp}",
        "ldur {tmp}, [{p} , 32]",
        "add {i5}, {i4}, {tmp}",
        "ldur {tmp}, [{p} , 40]",
        "add {i6}, {i5}, {tmp}",
        "ldur {tmp}, [{p} , 48]",
        "add {i7}, {i6}, {tmp}",
        "ldur {tmp}, [{p} , 56]",
        "add {i8}, {i7}, {tmp}",
        "ldur {tmp}, [{p} , 64]",
        "add {i9}, {i8}, {tmp}",
        "ldur {tmp}, [{p} , 72]",
        "add {i10}, {i9}, {tmp}",
        "ldur {tmp}, [{p} , 80]",
        "add {i11}, {i10}, {tmp}",
        "ldur {tmp}, [{p} , 88]",
        "add {i12}, {i11}, {tmp}",
        p = in(reg) p,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        tmp = out(reg) _,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test6 loadur time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / LOOP as f64);
}



/**
       cycles: 31602371417
 instructions: 252486626786
     branches: 6643814991
branch-misses: 115
IPC: 7.99
*/
fn test5(){
    let array = [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let base0 = 1234i64;
    let base1 = 1234i64;
    let mut i1 = 10i64;
    let mut i2 = 20i64;
    let mut i3 = 30i64;
    let mut i4 = 40i64;
    let mut i5 = 50i64;
    let mut i6 = 60i64;
    let mut i7 = 70i64;
    let mut i8 = 80i64;
    let mut i9 = 90i64;
    let mut i10 = 100i64;
    let mut i11 = 110i64;
    let mut i12 = 120i64;
    let mut v1: i8x16 = i8x16::splat(1);
    let mut v2: i8x16 = i8x16::splat(2);
    let mut v3: i8x16 = i8x16::splat(3);
    let mut v4: i8x16 = i8x16::splat(4);
    let mut v5: i8x16 = i8x16::splat(5);
    let mut v6: i8x16 = i8x16::splat(6);
    let mut v7: i8x16 = i8x16::splat(7);
    let mut v8: i8x16 = i8x16::splat(8);
    let mut v9: i8x16 = i8x16::splat(9);
    let mut v10: i8x16 = i8x16::splat(10);
    let p = &array[0];

    let time0 = std::time::Instant::now();
    const LOOP: u64 = 200_0000_0000u64;

    for _ in 0..LOOP {

        unsafe { asm! {
        "ldr {tmp}, [{p}]",
        "add {i1}, {i1}, {tmp}",
        "add.16b {v1}, {v1}, {v10}",

        "ldr {tmp}, [{p},  8]",
        "add {i2}, {i1}, {tmp}",
        "add.16b {v2}, {v2}, {v10}",

        "ldr {tmp}, [{p} , 16]",
        "add {i3}, {i2}, {tmp}",
        "add.16b {v3}, {v3}, {v10}",

        "ldr {tmp}, [{p} , 24]",
        "add {i4}, {i3}, {tmp}",
        "add.16b {v4}, {v4}, {v10}",

        "ldr {tmp}, [{p} , 32]",
        "add {i5}, {i4}, {tmp}",
        "add.16b {v5}, {v5}, {v10}",

        "ldr {tmp}, [{p} , 40]",
        "add {i6}, {i5}, {tmp}",
        "add.16b {v6}, {v6}, {v10}",

        "ldr {tmp}, [{p} , 48]",
        "add {i7}, {i6}, {tmp}",
        "add.16b {v7}, {v7}, {v10}",

        "ldr {tmp}, [{p} , 56]",
        "add {i8}, {i7}, {tmp}",
        "add.16b {v8}, {v8}, {v10}",

        "ldr {tmp}, [{p} , 64]",
        "add {i9}, {i8}, {tmp}",
        "add.16b {v9}, {v9}, {v10}",

        "ldr {tmp}, [{p} , 72]",
        "add {i10}, {i9}, {tmp}",
        "add.16b {v10}, {v10}, {v10}",

        "ldr {tmp}, [{p} , 80]",
        "add {i11}, {i10}, {tmp}",
        "add.16b {v1}, {v1}, {v10}",

        "ldr {tmp}, [{p} , 88]",
        "add {i12}, {i11}, {tmp}",
        "add.16b {v2}, {v2}, {v10}",
        p = in(reg) p,
        i1 = inout(reg) i1,
        i2 = inout(reg) i2,
        i3 = inout(reg) i3,
        i4 = inout(reg) i4,
        i5 = inout(reg) i5,
        i6 = inout(reg) i6,
        i7 = inout(reg) i7,
        i8 = inout(reg) i8,
        i9 = inout(reg) i9,
        i10 = inout(reg) i10,
        i11 = inout(reg) i11,
        i12 = inout(reg) i12,
        v1 =  inout(vreg) v1,
        v2 =  inout(vreg) v2,
        v3 =  inout(vreg) v3,
        v4 =  inout(vreg) v4,
        v5 =  inout(vreg) v5,
        v6 =  inout(vreg) v6,
        v7 =  inout(vreg) v7,
        v8 =  inout(vreg) v8,
        v9 =  inout(vreg) v9,
        v10 =  inout(vreg) v10,
        tmp = out(reg) _,
        }
        }

    }

    let time1 = time0.elapsed();
    println!("test5 time: {:?}, iteration: {:.2}ns", time1, time1.as_nanos() as f64  / LOOP as f64);
}



/**
        cycles: 31665409195
  instructions: 39470341063
      branches: 1643857197
 branch-misses: 46
IPC: 1.246
 */
fn test7(){

    let mut len: u64 = 10;

    let high: u64 = 0x0123456789ABCDEF;
    let low:  u64 = 0xFEDCBA9876543210;
    let mut key: u64x2 = u64x2::from_array([low as u64, high as u64]);

    let r#loop = 200_0000_0000u64;

    let time0 = std::time::Instant::now();
    for _ in 0..r#loop {
        let len1 = if len <= 8 { len } else { 8 };
        let len2 = if len >= 16 { 8 } else if len > 8 { len - 8 } else { 0 };

        key[0] = if len1 == 0 { 0 } else { key[0] & (0xFF_FF_FF_FF_FF_FF_FF_FF >> (64 - len1 * 8)) };
        key[1] = if len2 == 0 { 0 } else { key[1] & (0xFF_FF_FF_FF_FF_FF_FF_FF >> (64 - len2 * 8)) };

        key = key.bitxor(u64x2::splat(black_box(0x7F7F7F7F7F7)));
        len = black_box(1) + len % 24;
    }
    let time = time0.elapsed();

    println!("low = {low} high = {high}");
    println!("test7 time: {:?}, iteration: {:.2}ns", time, time.as_nanos() as f64  / r#loop as f64);



}


/**
       cycles: 31684456077
 instructions: 26483040609
     branches: 1260251999
branch-misses: 41
IPC: 0.84   比 test7 要慢，单实际在 onebc_rust 项目中反而要快很多
*/
fn test8(){

    let mut len: u64 = 10;

    let high: u64 = 0x0123456789ABCDEF;
    let low:  u64 = 0xFEDCBA9876543210;
    let mut key: u64x2 = u64x2::from_array([low as u64, high as u64]);

    let r#loop = 20_0000_0000u64;

    let time0 = std::time::Instant::now();
    for _ in 0..r#loop {
        let u8x16 = unsafe { transmute(key) };
        let index = u8x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        let mask = index.simd_lt(u8x16::splat(len as u8));
        key = unsafe { transmute(mask.select(u8x16, u8x16::splat(0))) };
        key = key.bitxor(u64x2::splat(black_box(0x7F7F7F7F7F7)));
        len = black_box(1) + len  % 24;
    }
    let time = time0.elapsed();

    println!("low = {} high = {}", key[0], key[1]);
    println!("test8 time: {:?}, iteration: {:.2}ns", time, time.as_nanos() as f64  / r#loop as f64);



}

