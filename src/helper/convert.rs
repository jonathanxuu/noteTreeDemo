use bytemuck::cast_slice;
use rs_merkle::{algorithms::Rescue, Hasher, MerkleProof};

pub fn convert_proof_to_u64(proof: MerkleProof<Rescue>) -> Vec<Vec<u64>> {
    let mut hash_u64 = Vec::new();

    for hash in proof.proof_hashes() {
        let u64_slice: &[u64] = cast_slice(hash);
        assert!(u64_slice.len() == 4, "Proof result length invalid");
        hash_u64.push(u64_slice.to_owned())
    }
    return hash_u64;
}

pub fn convert_u8_to_u64(origin: [u8; 32]) -> Vec<u64> {
    let u64_slice: &[u64] = cast_slice(&origin);

    assert!(u64_slice.len() == 4, "Proof result length invalid");
    return u64_slice.to_vec();
}

pub fn convert_u64_to_hash(origin: u64) -> [u8; 32] {
    let mut hash_u64 = Vec::<u64>::new();
    hash_u64.push(origin);

    let u8_slice: &[u8] = cast_slice(&hash_u64);
    // assert!(u8_slice.len() == 32, "leave result length invalid");

    Rescue::hash(u8_slice)
}

pub fn compute_note_tree_leave(random_hash: [u8; 32], pay_value: u64) -> [u8; 32] {
    let hash_u64: &[u64] = cast_slice(&random_hash);
    assert!(hash_u64.len() == 4, "Random Hash length invalid");
    let mut hash_u64_mut = hash_u64.to_vec(); 
    hash_u64_mut[0] += pay_value; 

    let u8_slice: &[u8] = cast_slice(&hash_u64_mut);
    assert!(u8_slice.len() == 32, "Leave length invalid");
    Rescue::hash(u8_slice)
}
