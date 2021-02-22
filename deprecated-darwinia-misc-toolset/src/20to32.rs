use sp_core::H256;
use std::env;

/// convert ethereum address to darwinia public key
fn main() {
	let args: Vec<String> = env::args().collect();
	let address = &args[1][2..];

	if address.len() == 40 {
		let address_bytes = &hex::decode(address).unwrap()[..];
		let mut data = [0u8; 32];
		data[0..4].copy_from_slice(b"dvm:");
		data[11..31].copy_from_slice(address_bytes);
		let checksum: u8 = data[1..31].iter().fold(data[0], |sum, &byte| sum ^ byte);
		data[31] = checksum;

		let result = H256::from_slice(&data);
		println!("{:?}", result);
	} else {
		println!("Wrong evm address length");
	}
}
