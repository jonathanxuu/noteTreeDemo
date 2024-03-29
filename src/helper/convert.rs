use bytemuck::cast_slice;
use rs_merkle::{algorithms::Rescue, MerkleProof};

pub fn convert_proof_to_u64(proof: MerkleProof<Rescue>) -> Vec<Vec<u64>> {
    let mut hash_u64 = Vec::new();

    for hash in proof.proof_hashes() {
        let u64_slice: &[u64] = cast_slice(hash);
        assert!(u64_slice.len() == 4, "Proof result length invalid");
        hash_u64.push(u64_slice.to_owned())
    }
    return hash_u64;
}

pub fn convert_u8_to_u64(origin: &[u8; 32]) -> Vec<u64> {
    let u64_slice: &[u64] = cast_slice(origin);
    assert!(u64_slice.len() == 4, "Proof result length invalid");
    return u64_slice.to_vec();
}
