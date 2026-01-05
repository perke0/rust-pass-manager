#![allow(dead_code, unused)]

mod hasher;
mod scanner;

fn main() {
    let a: String = scanner::get_input();
    let b: bool = hasher::hashing();
    println!("Print:{a} {b}");
}
