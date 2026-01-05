#![allow(dead_code, unused)]

mod hasher;
mod scanner;

fn main() {
    let a: String = scanner::get_input();
    //hello world
    let a = a.trim();
    let compare_hash: String = String::from(
        "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f",
    );

    let b: bool = hasher::hashing(a, &compare_hash);
    println!("Print:{b}");
}
