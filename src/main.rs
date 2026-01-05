#![allow(dead_code, unused)]
mod io {
    fn get_input() -> String {
        use std::io;
        println!("Type password here:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input from user");

        input
    }
}
mod crypt {
    use hex_literal::hex;
    use sha2::{Digest, Sha512};

    pub fn hashing() -> bool {
        let mut hasher = Sha512::new();
        hasher.update(b"hello world");
        let result = hasher.finalize();

        result[..]
            == hex!(
                "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f
    989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
            )[..]
    }
}
fn main() {
    use crypt;
    let s: bool = crypt::hashing();
    println!("{s}");
}
