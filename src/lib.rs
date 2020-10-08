use ckb_merkle_mountain_range::{calculate_peaks_hashes, Merge};
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct MMRMerge;
impl Merge for MMRMerge {
    type Item = H256;
    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        let encodable = (lhs, rhs);
        BlakeTwo256::hash_of(&encodable)
    }
}
pub struct ConvertedProof {
    pub mmr_size: u64,
    pub peaks: Vec<H256>,
    pub siblings: Vec<H256>,
}

pub fn convert(
    leaves: Vec<(u64, H256)>,
    mmr_size: u64,
    proof: Vec<H256>,
) -> Result<ConvertedProof> {
    let mut proof_hashes = proof.clone();
    let peaks_hashes =
        calculate_peaks_hashes::<H256, MMRMerge, _>(leaves, mmr_size, proof.iter()).unwrap();
    proof_hashes.retain(|h| !contains(&peaks_hashes, h));
    Ok(ConvertedProof {
        mmr_size: mmr_size,
        peaks: peaks_hashes,
        siblings: proof_hashes,
    })
}

fn contains(hashes: &Vec<H256>, target: &H256) -> bool {
    hashes.iter().find(|&x| x == target).is_some()
}

#[cfg(test)]
mod tests {
    use crate::H256;
    use ckb_merkle_mountain_range::leaf_index_to_pos;

    #[test]
    fn it_works() {
        // crab
        // root from 1037
        // proof from headerMMR_genProof(1033, 1036)
        // root == proof + 1033 hash

        let leaves = vec![(
            leaf_index_to_pos(1033),
            H256::from_slice(
                &hex::decode("ba574b186b85e24d4463fdd798a27e69d2a7c74f20454064af9b761b4dae1477")
                    .unwrap()[..],
            ),
        )];
        let mmr_size = 2070;
        let proof = vec![
            H256::from_slice(
                &hex::decode("f444ad927fa5b4cb116c47b9b98ca50685149ee661d560c9eca816d18be0fb49")
                    .unwrap()[..],
            ),
            H256::from_slice(
                &hex::decode("93101ab9177cd5e690d1193ae0fe0e3670bcebecb570759e25827528804c8cda")
                    .unwrap()[..],
            ),
            H256::from_slice(
                &hex::decode("b4df4e2cbbd4ca595c35ff6a3162056d9812eeae50537cf6d679d74fc581d4bb")
                    .unwrap()[..],
            ),
            H256::from_slice(
                &hex::decode("8179c80c5eb8912721e0cf77f1233531b4e3e5a992ee951dcea21f1029b8d3cd")
                    .unwrap()[..],
            ),
            H256::from_slice(
                &hex::decode("d79f1f4cddc867a8a9f82d272f1d9d8dc1cea213b8cfe574a5d67e190202bbd9")
                    .unwrap()[..],
            ),
        ];
        let converted_proof = crate::convert(leaves, mmr_size, proof).unwrap();

        assert_eq!(
            converted_proof.peaks,
            vec![
                H256::from_slice(
                    &hex::decode(
                        "f444ad927fa5b4cb116c47b9b98ca50685149ee661d560c9eca816d18be0fb49"
                    )
                        .unwrap()[..]
                ),
                H256::from_slice(
                    &hex::decode(
                        "93101ab9177cd5e690d1193ae0fe0e3670bcebecb570759e25827528804c8cda"
                    )
                        .unwrap()[..]
                ),
                H256::from_slice(
                    &hex::decode(
                        "9de7a78805e4185b60d3201f4dab68ae1b28011e090591d093e1b5739f7c13f5"
                    )
                        .unwrap()[..]
                ),
                H256::from_slice(
                    &hex::decode(
                        "d79f1f4cddc867a8a9f82d272f1d9d8dc1cea213b8cfe574a5d67e190202bbd9"
                    )
                        .unwrap()[..]
                ),
            ]
        );

        assert_eq!(
            converted_proof.siblings,
            vec![
                H256::from_slice(
                    &hex::decode(
                        "b4df4e2cbbd4ca595c35ff6a3162056d9812eeae50537cf6d679d74fc581d4bb"
                    )
                        .unwrap()[..]
                ),
                H256::from_slice(
                    &hex::decode(
                        "8179c80c5eb8912721e0cf77f1233531b4e3e5a992ee951dcea21f1029b8d3cd"
                    )
                        .unwrap()[..]
                ),
            ]
        );

    }
}
