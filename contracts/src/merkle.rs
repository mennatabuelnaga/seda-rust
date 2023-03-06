use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};
use sha2::Digest;

#[cfg_attr(feature = "deepsize_feature", derive(deepsize::DeepSizeOf))]
#[derive(
    Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Ord, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
pub struct CryptoHash(pub [u8; 32]);

impl Default for CryptoHash {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoHash {
    pub const fn new() -> Self {
        Self([0; 32])
    }

    /// Calculates hash of borsh-serialised representation of an object.
    pub fn hash_borsh<T: BorshSerialize>(value: &T) -> CryptoHash {
        let mut hasher = sha2::Sha256::default();
        BorshSerialize::serialize(value, &mut hasher).unwrap();
        CryptoHash(hasher.finalize().into())
    }
}

pub type MerkleHash = CryptoHash;

pub fn combine_hash(hash1: &MerkleHash, hash2: &MerkleHash) -> MerkleHash {
    CryptoHash::hash_borsh(&(hash1, hash2))
}

/// Modified from nearcore/core/primitives/src/merkle.rs
pub fn merklize<T: BorshSerialize>(arr: &[T]) -> MerkleHash {
    if arr.is_empty() {
        return MerkleHash::default();
    }
    let mut len = arr.len().next_power_of_two();
    let mut hashes = arr.iter().map(CryptoHash::hash_borsh).collect::<Vec<_>>();

    hashes.sort();

    // degenerate case
    if len == 1 {
        return hashes[0];
    }
    let mut arr_len = arr.len();

    while len > 1 {
        len /= 2;
        for i in 0..len {
            let hash = if 2 * i >= arr_len {
                continue;
            } else if 2 * i + 1 >= arr_len {
                hashes[2 * i]
            } else {
                combine_hash(&hashes[2 * i], &hashes[2 * i + 1])
            };
            hashes[i] = hash;
        }
        arr_len = (arr_len + 1) / 2;
    }
    hashes[0]
}
