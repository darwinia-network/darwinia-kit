use sp_core::H256;
use std::env;

/// convert ethereum address to darwinia public key
fn main() {
    let args: Vec<String> = env::args().collect();
    let evm_address = &args[1][2..];

    if evm_address.len() == 40 {
        let mut data = [0u8; 32];
        let evm_address_bytes = &hex::decode(evm_address).unwrap()[..];
        let hash = sp_core::blake2_256(evm_address_bytes);

        data[0..20].copy_from_slice(evm_address_bytes);
        data[20..32].copy_from_slice(&hash[0..12]);

        let result = H256::from_slice(&data);
        println!("{:?}", result);
    } else {
        println!("Wrong evm address length");
    }
}