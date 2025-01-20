#![feature(portable_simd)]

use std::arch::asm;
use std::simd::cmp::SimdPartialEq;
use std::simd::{u8x16, u8x64};

fn main() {

    // generate 256 random u8
    let buffer: Vec<u8> = (0..256).map(|_| rand::random::<u8>()).collect();
    let x = test1(&buffer);
    println!("{:?}", x);

}

#[inline(never)]
fn test1(buffer: &[u8]) -> u64{
    let block: u8x16 = u8x16::from_slice(&buffer[0..64]);

    unsafe {
        asm! {
            "mov {x}, {block}",
            block = in(vreg) block,
            x = out(vreg) _,
        }
    }


    block.simd_eq(u8x16::splat(b';')).to_bitmask()
}