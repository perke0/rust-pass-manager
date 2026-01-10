#![allow(dead_code, unused)]

use std::fs::OpenOptions;

mod cli;
mod file_mng;
mod hasher;
mod scanner;

fn main() {
    let path = "passwords.txt";
    let _ = file_mng::create_file(path);

    let compare_hash: String = String::from(
        "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f",
    );
    file_mng::write_data(path, &compare_hash);
    let contents = file_mng::read_data(path);

    let a: Box<str> = scanner::password_input();
    match file_mng::read_data(path) {
        Ok(contents) => {
            let b: bool = hasher::check_hashing(a.trim(), contents.trim());
            println!("{b}");
        }
        Err(e) => eprintln!("read error: {}", e),
    }

    //hello world is pass
}
