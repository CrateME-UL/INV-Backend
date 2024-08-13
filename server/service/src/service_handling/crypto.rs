// See: https://github.com/Keats/rust-bcrypt
extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_pass(user_password: &str) -> String {
    hash(user_password, DEFAULT_COST).unwrap()
}

pub fn verify_hash(user_password: &str, hash: &str) -> bool {
    verify(user_password, hash).unwrap()
}
