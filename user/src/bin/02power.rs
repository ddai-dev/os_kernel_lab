#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::sys_yield;

const WIDTH: usize = 10;
const HEIGHT: usize = 3;

#[no_mangle]
fn main() -> i32 {
    println!("Test write_b Begin!");
    for i in 0..HEIGHT {
        for _ in 0..WIDTH { print!("C"); }
        println!(" [{}/{}]", i + 1, HEIGHT);
        sys_yield();
    }
    println!("Test write_c OK!");
    0
}