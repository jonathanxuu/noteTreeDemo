use crate::helper::convert::convert_u8_to_u64;
use rs_merkle::{algorithms::Rescue, MerkleProof};
use super::convert::convert_proof_to_u64;

pub fn construct_miden_input(
    leave_to_prove: [u8; 32],
    proof: MerkleProof<Rescue>,
    flag: Vec<usize>,
) -> Vec<u64> {
    let merkle_proof: Vec<Vec<u64>> = convert_proof_to_u64(proof);
    assert!(
        merkle_proof.len() == flag.len(),
        "flag length not equal to merkle proof length"
    );
    let mut inputs = Vec::<u64>::new();
    inputs.push(flag.len() as u64);
    let leave = convert_u8_to_u64(leave_to_prove);
    for value in leave {
        inputs.push(value);
    }

    for (i, proof_vec) in merkle_proof.iter().enumerate() {
        for value in proof_vec {
            inputs.push(*value);
        }
        inputs.push(flag[i] as u64);
    }
    inputs
}
