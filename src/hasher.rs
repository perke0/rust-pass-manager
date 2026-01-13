use sha2::{Digest, Sha512};

#[allow(dead_code)]
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

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}
