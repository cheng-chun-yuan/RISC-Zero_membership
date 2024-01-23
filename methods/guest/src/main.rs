// #![no_main]
// // If you want to try std support, also update the guest Cargo.toml file

// use risc0_zkvm::guest::env;
// use ethabi::ethereum_types::Address;
// risc0_zkvm::guest::entry!(main);

// pub fn main() {
//     // Read the address to check
//     let address_bytes = env::read::<[u8; 20]>();
//     let address = Address::from_slice(&address_bytes);

//     // Read the length of the address array
//     let array_length: usize = env::read::<usize>();

//     // Initialize a flag to false
//     let mut is_member = false;

//     // Read each address in the array and check for membership
//     for _ in 0..array_length {
//         let address_in_array_bytes = env::read::<[u8; 20]>();
//         let address_in_array = Address::from_slice(&address_in_array_bytes);

//         if address == address_in_array {
//             is_member = true;
//             break;
//         }
//     }

//     // Write public output to the journal
//     env::commit(&is_member);
// }
#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental

use risc0_zkvm::guest::env;
use ethabi::ethereum_types::Address;
risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Read the first input using env::read
    let target_address: Address = env::read();

    let address_in_array: [Address; 5] = [
        Address::from_slice(&[0x00; 20]), // Fake address 1
        Address::from_slice(&[0x01; 20]), // Fake address 2
        Address::from_slice(&[0x02; 20]), // Fake address 3
        Address::from_slice(&[0x03; 20]), // Fake address 4
        Address::from_slice(&[0x04; 20]), // Fake address 5
    ];
    
    let mut output = false;

    // if target_address in address_in_array:  return true
    for address in address_in_array {
        if target_address == address {
            output = true;
            break;
        }
    }

    // Write public output to the journal
    env::commit(&output);
}