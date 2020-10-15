#![feature(array_methods)]

use ckb_merkle_mountain_range::{MMR, MMRStore, util::MemStore};
use darwinia_misc_toolset::MMRMerge;
use sp_runtime::traits::{BlakeTwo256, Hash};
use sp_core::H256;

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
            &hex::decode(
                "9197278f146f85de21a738c806c24e0b18b266d45fc33cbb922e9534ab26dacd"
            ).unwrap()[..]
        ),
        H256::from_slice(
            &hex::decode(
                "488e9565547fec8bd36911dc805a7ed9f3d8d1eacabe429c67c6456933c8e0a6"
            ).unwrap()[..]
        ),

    ];
    let result = darwinia_misc_toolset::merge(hashes);
    println!("{:?}", result);
}