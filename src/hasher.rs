use sha2::{Digest, Sha512};
pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}
