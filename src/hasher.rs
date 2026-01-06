use hex_literal::hex;
use sha2::{Digest, Sha512};

pub fn check_hashing(input: &str, expected_hex: &str) -> bool {
    let expected_bytes = match hex::decode(expected_hex) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    result[..] == expected_bytes[..]
}
