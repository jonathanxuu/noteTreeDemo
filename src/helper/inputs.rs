use super::convert::convert_proof_to_u64;
use rs_merkle::{algorithms::Rescue, MerkleProof};

pub fn construct_miden_input(proof: MerkleProof<Rescue>, flag: Vec<usize>) -> String {
    let merkle_proof: Vec<Vec<u64>> = convert_proof_to_u64(proof);
    assert!(
        merkle_proof.len() == flag.len(),
        "flag length not equal to merkle proof length"
    );
    let mut inputs = Vec::<u64>::new();
    inputs.push(flag.len() as u64);

    for (i, proof_vec) in merkle_proof.iter().enumerate() {
        for value in proof_vec {
            inputs.push(*value);
        }
        inputs.push(flag[i] as u64);
    }
    inputs
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
