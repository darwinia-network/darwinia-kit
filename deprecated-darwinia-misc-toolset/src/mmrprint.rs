#![feature(array_methods)]

use ckb_merkle_mountain_range::{util::MemStore, MMRStore, MMR};
use darwinia_misc_toolset::MMRMerge;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash};

use ckb_merkle_mountain_range::pos_height_in_tree;

// fn build_mmr(count: u32){
//     let store = MemStore::default();
//     let mut mmr = MMR::<_, MMRMerge, _>::new(0, &store);
//     let positions: Vec<u64> = (0u32..count)
//         .map(
//             |i| {
//                 let bytes = i.to_le_bytes();
//                 let bytes = bytes.as_slice();
//                 mmr.push(BlakeTwo256::hash(bytes)).unwrap()
//             }
//         )
//         .collect();
//     println!("{:?}", mmr.mmr_size());
//     mmr.commit().expect("commit changes");
// }
//
// fn pos_2_coord(pos: u64) -> (u64, u64) {
//     let height = pos_height_in_tree(pos);
//     let right_leaf_pos = pos - height;
// }

fn main() {
	// build_mmr(11);
	let hashes = vec![
		H256::from_slice(
			&hex::decode("986958dcc848e6f2305f6932713036c58814640d82f5bef53d09bfe27fa5da54")
				.unwrap()[..],
		),
		H256::from_slice(
			&hex::decode("e60e09a2ecf78dba9848d05971de3d0fbdc4ccf7fac2466b2325260e10231c39")
				.unwrap()[..],
		),
	];
	let result = darwinia_misc_toolset::merge(hashes);
	println!("{:?}", result);
}
