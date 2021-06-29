// --- std ---
use std::{fmt, fs::File};
// --- crates.io ---
use blake2_rfc::blake2b;
use serde_json::Value;
// --- github.com ---
use mmr::{
	util::{MemMMR, MemStore},
	MMRStore, Merge,
};

pub struct Hasher;
impl Merge for Hasher {
	type Item = Hash;

	fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
		pub fn hash(data: &[u8]) -> [u8; 32] {
			array_bytes::dyn2array!(blake2b::blake2b(32, &[], data).as_bytes(), 32)
		}

		let mut data = vec![];

		data.extend_from_slice(&lhs.0);
		data.extend_from_slice(&rhs.0);

		Hash(hash(&data))
	}
}

#[derive(Clone, PartialEq)]
pub struct Hash([u8; 32]);
impl From<[u8; 32]> for Hash {
	fn from(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}
}
impl Hash {
	fn from_hex_unchecked(hex: impl AsRef<str>) -> Self {
		array_bytes::hex_into_unchecked(hex)
	}
}
impl fmt::Display for Hash {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", array_bytes::bytes2hex("0x", self.0))
	}
}
impl fmt::Debug for Hash {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		<Self as fmt::Display>::fmt(&self, f)
	}
}

fn main() {
	let store = MemStore::default();

	for (position, hash) in
		serde_json::from_reader::<_, Value>(File::open("pangolin-mmr-50000.json").unwrap())
			.unwrap()
			.as_array()
			.unwrap()
			.iter()
			.enumerate()
	{
		(&store)
			.append(
				position as _,
				vec![array_bytes::hex_into_unchecked(hash.as_str().unwrap())],
			)
			.unwrap();
	}

	let mem_mmr = <MemMMR<Hash, Hasher>>::new(mmr::leaf_index_to_mmr_size(5678), store);
	let proof = mem_mmr
		.gen_proof(vec![mmr::leaf_index_to_pos(234)])
		.unwrap();

	// subalfred send-rpc --method headerMMR_genProof --params '[234, 5678]' --uri https://pangolin-rpc.darwinia.network
	// {
	//    "id" : 1,
	//    "jsonrpc" : "2.0",
	//    "result" : {
	//       "mmrSize" : "11350",
	//       "proof" : "[0xd27fae7d7c766df9f9752e7cb3797457fb5ab3ef175283c539242aece58660aa, 0xc1fe79d3b6a5530a47a4b58320130414ca4eba748d881bbcf1ae6babe2bba814, 0x97564beea338d965d581b480bbcaa6207bd7c507e2940ce9534a9481eab167c3, 0x1107da282e1d4b27eb4653867f13a447ae0ff1111448a83d34c974110ad64b1e, 0xb01068cbedd81812d808083f2492fb7b7cdb2e341b8348f7700f5d177c5b1c12, 0xe210dae3f7f8732b05d8abbe0d3bbfb5c7f325b840baab088b0eecd91dc9bee4, 0x936c7d52d95626c4994a21d5ddc04285331aadd5129a0c4c88b5015b26409e28, 0x48f0c3bbd24913c18fa09f1f7c6c845fbf2fa685ae5d3e201494529bc2e9085f, 0xfd366cbe5ebbd1556adbd77ab450c3f395ab6d283984c0bab6bcc79962d85128, 0xe04c06c61ea07ec20f5dac46a183931c591682f3b4215ca8adc6caa798766fd5, 0xe6e1d1c399e6162678b0de63e1628d85e6dd414f8e70434f17789b8586ea6ee5, 0xffe1e4dd65a3f0055a2309ab3cb332efcfc6559569d7a5a7269a9ec59c3da1c6, 0x32cb5499c4a10a3d847c155dd4a8e316f285d1db158d296367f1e5c1d2b1719f]"
	//    }
	// }
	assert_eq!(proof.mmr_size(), 11350);
	assert_eq!(
		proof.proof_items().to_vec(),
		[
			"0xd27fae7d7c766df9f9752e7cb3797457fb5ab3ef175283c539242aece58660aa",
			"0xc1fe79d3b6a5530a47a4b58320130414ca4eba748d881bbcf1ae6babe2bba814",
			"0x97564beea338d965d581b480bbcaa6207bd7c507e2940ce9534a9481eab167c3",
			"0x1107da282e1d4b27eb4653867f13a447ae0ff1111448a83d34c974110ad64b1e",
			"0xb01068cbedd81812d808083f2492fb7b7cdb2e341b8348f7700f5d177c5b1c12",
			"0xe210dae3f7f8732b05d8abbe0d3bbfb5c7f325b840baab088b0eecd91dc9bee4",
			"0x936c7d52d95626c4994a21d5ddc04285331aadd5129a0c4c88b5015b26409e28",
			"0x48f0c3bbd24913c18fa09f1f7c6c845fbf2fa685ae5d3e201494529bc2e9085f",
			"0xfd366cbe5ebbd1556adbd77ab450c3f395ab6d283984c0bab6bcc79962d85128",
			"0xe04c06c61ea07ec20f5dac46a183931c591682f3b4215ca8adc6caa798766fd5",
			"0xe6e1d1c399e6162678b0de63e1628d85e6dd414f8e70434f17789b8586ea6ee5",
			"0xffe1e4dd65a3f0055a2309ab3cb332efcfc6559569d7a5a7269a9ec59c3da1c6",
			"0x32cb5499c4a10a3d847c155dd4a8e316f285d1db158d296367f1e5c1d2b1719f"
		]
		.iter()
		.map(Hash::from_hex_unchecked)
		.collect::<Vec<_>>()
	);
}
