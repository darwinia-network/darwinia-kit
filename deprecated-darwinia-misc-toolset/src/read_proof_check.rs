use hex_literal::hex;
use parity_scale_codec::Encode;
use rustc_hex::ToHex;
use sp_core::blake2_256;
use sp_core::bytes::to_hex;
use sp_core::storage::ChildInfo;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash};
use sp_state_machine::{prove_read, read_proof_check, Backend, TrieBackend};
use sp_trie::trie_types::TrieDBMut;
use sp_trie::NodeCodec;
use sp_trie::{KeySpacedDBMut, PrefixedMemoryDB, StorageProof, TrieMut};
use std::collections::HashMap;
use trie_db::NodeCodec as NodeCodecT;

fn main() {
	// state root
	let root = &hex!["2a6b9a056f6336ab2417eedcc455a18abd01517c0a4a9adb958699eed84d8b4b"][..];
	let root = sp_core::H256::from_slice(&root);
	println!("root: {:?}", root);
	// proof
	let proof = vec![
        hex!("80490c80b0eadd9230e584b47e00b7cf8e1f927cee0d50485a3c676a11b384ead2bf36ee80bc95ec1b2225e13c653524cb1ff8714173c780ef580581ff66c9604e7fb536c380fda12eddcd0a2e270526fc0037de9da9dcda5c99049e1f74a520c5cbd85db96b8064d657470e1a79e65884b81f1085d11308fd3182d566ffe2b419413ce0495e1980a60aa390511c25e70c1b029d951ef2fd0255b8a3fc3580bed4c4b2fa31789781")[..].to_vec(),
        hex!("5f00d41e5e16056765bc8461851072c9d74505240000000000000080e36a09000000000200000001000000000000000000000000000200000002000000000000ca9a3b00000000020000000300000000030e017b1e76c223d1fa5972b6e3706100bb8ddffb0aeafaf0200822520118a87e00000300000003000e017b1e76c223d1fa5972b6e3706100bb8ddffb0aeafaf0200822520118a87ef0a4b9550b000000000000000000000000000300000003020d584a4cbbfd9a4878d816512894e65918e54fae13df39a6f520fc90caea2fb00e017b1e76c223d1fa5972b6e3706100bb8ddffb0aeafaf0200822520118a87ef0a4b9550b00000000000000000000000000030000000e060017640700000000000000000000000000000300000003045a9ae1e0730536617c67ca727de00d4d197eb6afa03ac0b4ecaa097eb87813d6c005d9010000000000000000000000000000030000000000c0769f0b00000000000000")[..].to_vec(),
        hex!("80fffb80fbd313c51ce7764956f81ef87ff3ebc489b3232cfed8fef9a8434b7414d6f7c880760034d4c3469cab2f0c3c5417980460d295fd8b49cff262a4afb8290c38a57b80150154959b53b033e56db6cb65aa2fedca9dd0071f25eae7ba262841eaf0dbbd807adb48ce7c7686a6b1f726eca635aecf163fcb1ec47f7cacec194be9f340d7b1805c72f25b1b6304d16667e2766fa1a906cb081788eb4502787df7c3597412b17b802d39230527f49cf88fbdd4bf7e3dbcd564218ea2c20751ee4e4e24ecb44989a5800eb754c27d6302344f80fc4f785eae09c7c6acf58ee0ebddbd2f1755eb37a7de806246fab7082d42447ff6a3e4653cb8c2427408eae98af0c40f9c636b972f91548034260342013b628b1a3409a53683bd72866b974fc4bb1e2db0b50c4abd88df0680468f4c745f210c713c8eee6d4bc90e15ac9e708974088d1bf5e01db7fc0781bb809d5adec17d1f91d73f0a631ffe17af9dae7007f69f11bc4d46ca2b9777a921688090e4fe33f4b3a304329c97d1ee3cb8240585cd8c4a1da47f79423a1d91dd1d7180a7a88069a098bb5725ce52c5cf702bed3b1f6f134a69f585d43ab497995fd35280cbcdf9de3ff34d475ef3dad95c4217e6ee4a1e40897550291620d88e1a77c2bd806440a709fcb73133283c13668a87da24982f6b61060d169deb5a43532b553318")[..].to_vec(),
        hex!("9eaa394eea5630e07c48ae0c9558cef7098d585f0a98fdbe9ce6c55837576c60c7af3850100900000080a4adb17d600ad56fb70d03060fc70c9636b53bac26f3d45a525461b3d9fbd8ea80950043f807c1289b7636f6a759abc843caa0f2da40d133ff2fe8821926fd7d93803520a0cde9eee6081349f75cb2771853207aa1b0136c1303677c394d3b2de74880dc4f83e9b8934c4dcffc1d12f846210d0b469982edff3c19c3e89246d9f9b27a705f09cce9c888469bb1a0dceaa129672ef8284820706f6c6b61646f74")[..].to_vec(),
    ];
	let storage_proof = StorageProof::new(proof.clone());

	// events storage key
	let storage_key = &hex!["26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7"][..];

	// for item in proof {
	//     let hash = blake2_256(item.as_slice());
	//     println!("{:?}", to_hex(&hash, false));
	// }
	let result = read_proof_check::<BlakeTwo256, _>(root, storage_proof, &[storage_key]).is_ok();
	//
	// println!("{:#x?}", result);
}

// fn main() {
//     let bytes = &hex!("80fffb80fbd313c51ce7764956f81ef87ff3ebc489b3232cfed8fef9a8434b7414d6f7c880760034d4c3469cab2f0c3c5417980460d295fd8b49cff262a4afb8290c38a57b80150154959b53b033e56db6cb65aa2fedca9dd0071f25eae7ba262841eaf0dbbd807adb48ce7c7686a6b1f726eca635aecf163fcb1ec47f7cacec194be9f340d7b1805c72f25b1b6304d16667e2766fa1a906cb081788eb4502787df7c3597412b17b802d39230527f49cf88fbdd4bf7e3dbcd564218ea2c20751ee4e4e24ecb44989a5800eb754c27d6302344f80fc4f785eae09c7c6acf58ee0ebddbd2f1755eb37a7de806246fab7082d42447ff6a3e4653cb8c2427408eae98af0c40f9c636b972f91548034260342013b628b1a3409a53683bd72866b974fc4bb1e2db0b50c4abd88df0680468f4c745f210c713c8eee6d4bc90e15ac9e708974088d1bf5e01db7fc0781bb809d5adec17d1f91d73f0a631ffe17af9dae7007f69f11bc4d46ca2b9777a921688090e4fe33f4b3a304329c97d1ee3cb8240585cd8c4a1da47f79423a1d91dd1d7180a7a88069a098bb5725ce52c5cf702bed3b1f6f134a69f585d43ab497995fd35280cbcdf9de3ff34d475ef3dad95c4217e6ee4a1e40897550291620d88e1a77c2bd806440a709fcb73133283c13668a87da24982f6b61060d169deb5a43532b553318");
//
//     let result = NodeCodec::<BlakeTwo256>::decode(bytes).unwrap();
//     println!("{:?}", result);
// }

// FN Custom_read_proof_check(root: &[u8; 32], proof: Vec<&[u8]>, key: &[u8; 32]) {
//     let mut proof_map = HashMap::<[u8;32], &[u8]>::new();
//     for proof_item in proof {
//        let hash = blake2_256(proof_item);
//         proof_map.insert(hash, proof_item);
//     }
//
//     match proof_map.get(root) {
//         Some(&proof_item) => {
//
//         },
//         None => {
//
//         }
//     }
// }

struct Input<'a> {
	data: &'a [u8],
	offset: u32,
}

// impl<'a> Input<'a> {
//     pub fn new(data: &[u8]) -> Input {
//         Input {
//             data: data,
//             offset: 0
//         }
//     }
//
//     pub fn read(&mut self, len: u32) -> &[u8] {
//         let new_offset = self.offset + len;
//         let result = &self.data[self.offset as usize..new_offset as usize];
//         self.offset = new_offset;
//         return result;
//     }
// }
//
// fn decode(mut input: Input) {
//     // read the first byte
//     let first = input.read(1)[0];
//     if first >= 0x80u8 {
//         let nibbles_len = first - 0x80u8;
//         let nibbles_len = if nibbles_len % 2 == 0 { nibbles_len } else { nibbles_len + 1 };
//         let nibbles = input.read(nibbles_len as u32);
//         let branch_mask = input.read(2);
//     } else if first >= 0x40u8 && first < 0x80u8 {
//
//     } else {
//         // wrong
//     }
// }

// fn decode_branch_mask(branch_mask: [u8; 2]) -> Vec<u8> {
//
// }

// const CHILD_KEY_1: &[u8] = b"sub1";
//
// fn test_db() -> (PrefixedMemoryDB<BlakeTwo256>, H256) {
//     let child_info = ChildInfo::new_default(CHILD_KEY_1);
//     let mut root = H256::default();
//     let mut mdb = PrefixedMemoryDB::<BlakeTwo256>::default();
//     {
//         let ks = child_info.keyspace();
//         println!("key spece: {:?}", ks);
//         let mut mdb = KeySpacedDBMut::new(&mut mdb, ks);
//         let mut trie = TrieDBMut::new(&mut mdb, &mut root);
//         trie.insert(b"value3", &[142]).expect("insert failed");
//         trie.insert(b"value4", &[124]).expect("insert failed");
//     };
//
//     {
//         let mut sub_root = Vec::new();
//         root.encode_to(&mut sub_root);
//         let mut trie = TrieDBMut::new(&mut mdb, &mut root);
//         trie.insert(child_info.prefixed_storage_key().as_slice(), &sub_root[..])
//             .expect("insert failed");
//         trie.insert(b"key", b"value").expect("insert failed");
//         trie.insert(b"value1", &[42]).expect("insert failed");
//         trie.insert(b"value2", &[24]).expect("insert failed");
//         trie.insert(b":code", b"return 42").expect("insert failed");
//         for i in 128u8..255u8 {
//             trie.insert(&[i], &[i]).unwrap();
//         }
//     }
//     (mdb, root)
// }

// fn main() {
//     // let (mdb, root) = test_db();
//     // let remote_backend = TrieBackend::new(mdb, root);
//     // println!("{:?}", remote_backend.child_storage(&ChildInfo::new_default(CHILD_KEY_1), b"value3").unwrap());
//     // println!("{:?}", remote_backend.pairs());
//     // let remote_root = remote_backend.storage_root(::std::iter::empty()).0;
//     // println!("remote_root: {:?}", remote_root);
//     // let remote_proof = prove_read(remote_backend, &[b"value2"]).unwrap();
//     // println!("remote_proof: {:?}", remote_proof);
//     let mut root = H256::default();
//     let mut mdb = PrefixedMemoryDB::<BlakeTwo256>::default();
//     {
//         let mut trie = TrieDBMut::new(&mut mdb, &mut root);
//         trie.insert(&hex!("b7113550"), &hex!("1111")).expect("insert failed");
//         trie.insert(&hex!("a7113550"), &hex!("aaaa")).expect("insert failed");
//         trie.insert(&hex!("a7113660"), &hex!("ffff")).expect("insert failed");
//         trie.insert(&hex!("a77d3370"), &hex!("bbbb")).expect("insert failed");
//         trie.insert(&hex!("a77d3371"), &hex!("eeee")).expect("insert failed");
//         trie.insert(&hex!("a7f93650"), &hex!("cccc")).expect("insert failed");
//         trie.insert(&hex!("a77d3970"), &hex!("dddd")).expect("insert failed");
//     }
//
//     let backend = TrieBackend::new(mdb, root);
//     // backend.
//     let proof = prove_read(backend, &[&hex!("a77d3371")]).unwrap();
//     for i in proof.iter_nodes() {
//         println!("proof:");
//         println!("  data: {:?}", to_hex(&i, false));
//         println!("  hash: {:?}", to_hex(&blake2_256(&i), false));
//     }
//
//     println!("root: {:?}", root);
// }

// fn main() {
//     use log::{debug, error, log_enabled, info, trace, Level};
//
//     env_logger::init();
//
//     let mut root = H256::default();
//     let mut mdb = PrefixedMemoryDB::<BlakeTwo256>::default();
//     {
//         let mut trie = TrieDBMut::new(&mut mdb, &mut root);
//         trie.insert(&hex!("23dc111d7c3ad1df9806ce1e8eb4f55f57dba117339c545e7593d1f6c3b02662"),
//                     &hex!("620f5493f137cd126b6eace0bc2f10edd3f3159caf045d753bbc6500215fe5dc")).expect("insert failed");
//         trie.insert(&hex!("332c39dcd398ea34a48b871898d589f55fc4c7bce00562fb670c972e7e1b0720"),
//                     &hex!("a3ce17b27159688a034b81c37c4fc5f81a18272ab894516f4088a2396997cded")).expect("insert failed");
//     }
//
//     let backend = TrieBackend::new(mdb, root);
//     let proof = prove_read(backend, &[&hex!("332c39dcd398ea34a48b871898d589f55fc4c7bce00562fb670c972e7e1b0720")]).unwrap();
//
//     for i in proof.iter_nodes() {
//         println!("data: {:?}", to_hex(&i, false));
//         println!("hash: {:?}", to_hex(&blake2_256(&i), false));
//     }
//
//     println!("root: {:?}", root);
//     // // let a: Vec<u8> = hex!("800c0080081f225d40362a5d3a7d05d9a2bcb101714bbdd62321c0663f09c4a4de62a04780e60e09a2ecf78dba9848d05971de3d0fbdc4ccf7fac2466b2325260e10231c39");
//     //
//     // // let h1 = BlakeTwo256::hash_of(&hex!("7f00032c39dcd398ea34a48b871898d589f55fc4c7bce00562fb670c972e7e1b072080a3ce17b27159688a034b81c37c4fc5f81a18272ab894516f4088a2396997cded"));
//     // let h2 = BlakeTwo256::hash_of(&hex!("800c0080081f225d40362a5d3a7d05d9a2bcb101714bbdd62321c0663f09c4a4de62a04780e60e09a2ecf78dba9848d05971de3d0fbdc4ccf7fac2466b2325260e10231c39"));
//     // // println!("{:?}", h1);
//     // println!("{:?}", h2);
// }
