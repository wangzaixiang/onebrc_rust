#![feature(unchecked_shifts)]
#![feature(unbounded_shifts)]

fn main() {

    let num: u128 = std::env::args().nth(1).unwrap().parse().unwrap();
    let result = test_u128(num);
    println!("{}", result);

    let num = num as u64;
    let a = test_shift_left(num, 0);
    let b = test_shift_left(num, 1);
    let c = test_shift_left(num, 63);
    let d = test_shift_left(num, 64);    // report overflow

    println!("{} {} {} {}", a, b, c, d);

}

#[inline(never)]
fn test_shift_left(num: u64, num2: u64) -> u64 {
    num << num2
}

fn test_shift_left_unchecked(num: u64, num2: u32) -> u64 {
    unsafe { num.unchecked_shl(num2) }
}

fn test_shift_left_unbound(num: u64, num2: u32) -> u64 {
    unsafe { num.unbounded_shl(num2) }
}

// 结论： arm 没有对 u128 使用 128 位寄存器，而是使用 64 位寄存器，所以 u128 的运算会比较慢
#[inline(never)]
fn test_u128(num: u128) -> u128 {

    let a = num + 1u128;
    let b = num >> 16;

    let c = num.count_ones();
    let d = num.leading_zeros();

    let e = num.rotate_left(8);
    let f = num.rotate_right(8);

    let g = num as u64 as u128;

    a ^ b ^ c as u128 ^ d as u128 ^ e ^ f ^ g
}