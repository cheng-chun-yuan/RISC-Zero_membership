#![no_main]
// If you want to try std support, also update the guest Cargo.toml file

use risc0_zkvm::guest::env;
use ethabi::ethereum_types::Address;
risc0_zkvm::guest::entry!(main);

// use std::hash::{Hash, Hasher};
// use std::collections::hash_map::DefaultHasher;

// fn main() {
//     let sum: u32 = 4; // The sum you want to test

//     let mut hasher = DefaultHasher::new();
//     sum.hash(&mut hasher);
//     let hash_value: u64 = hasher.finish();

//     println!("Hash of 4 is: {}", hash_value);
// }
fn main() {
    let address = Address::from_slice(&[0u8; 20]);
    let mut is_member:bool = false;
    env::commit(&is_member);
}

