use near_primitives::{hash::CryptoHash, merkle::combine_hash, types::MerkleHash};
use near_sdk::borsh::BorshSerialize;

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
