use sha2::{Sha256, Digest};
fn main(somestring:&str) {
    let mut hasher = somestring;
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    println!("SHA-256 hash: {:x}", result);}