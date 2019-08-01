pub mod decryption;
use decryption::decrypt;

fn main() {
    decrypt(b"haha".to_vec()).unwrap();
    println!("Hello, world!");
}

