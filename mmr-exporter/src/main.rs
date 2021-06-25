// --  std ---
use std::{
	env, fmt,
	fs::File,
	io::{Seek, SeekFrom, Write},
};
// --- crates.io ---
use isahc::AsyncReadResponseExt;
use serde_json::Value;
use subrpcer::state;
use substorager::StorageHasher;

type NodeIndex = u64;

#[derive(Debug)]
struct Hash([u8; 32]);
impl Hash {
	fn from_hex_unchecked(hex: &str) -> Self {
		Self(array_bytes::hex2array_unchecked(hex))
	}
}
impl fmt::Display for Hash {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", array_bytes::bytes2hex("0x", self.0))
	}
}

#[async_std::main]
async fn main() {
	let mut file = File::create("mmr.json").unwrap();

	write!(file, "[").unwrap();

	for position in 0..env::args()
		.collect::<Vec<_>>()
		.get(1)
		.map(|s| s.parse::<NodeIndex>().unwrap_or(10))
		.unwrap_or(10)
	{
		let storage_key = substorager::hex_storage_map_key_with_prefix(
			"0x",
			b"DarwiniaHeaderMMR",
			b"MMRNodeList",
			(StorageHasher::Identity, position.to_ne_bytes()),
		);
		let response = subrpcer::send_rpc(
			"https://rpc.darwinia.network",
			state::get_storage(storage_key, <Option<()>>::None),
		)
		.await
		.unwrap()
		.text()
		.await
		.unwrap();
		let hash = Hash::from_hex_unchecked(
			serde_json::from_str::<Value>(&response).unwrap()["result"]
				.as_str()
				.unwrap(),
		);

		write!(file, "\"{}\",", hash).unwrap();
		println!("{}", hash);
	}

	file.seek(SeekFrom::Current(-1)).unwrap();

	write!(file, "]").unwrap();

	file.sync_all().unwrap();
}
