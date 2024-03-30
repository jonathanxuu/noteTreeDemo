use rs_merkle::{algorithms::Rescue, MerkleTree};

use crate::helper::{
    convert::{compute_note_tree_leave, convert_u64_to_hash, convert_u8_to_u64},
    inputs::construct_miden_input,
};

pub mod helper;

fn main() {
    // ===================== Bob's work(done in Frontend)===========================================================
    // Bob Generate randomness (this should be done in the Frontend, not in the canister/rust)
    // The random should be u64, to make it easy to hash
    let bob_random: u64 = 10000;

    // Bob compute the hash(random)(this can be done in the Frontend & in the canister/rust)
    let bob_random_hash = convert_u64_to_hash(bob_random);

    // In canister this random_hash should be unique!! Avoid ramdom_hash collision

    // Bob decide the value to request
    let pay_value: u64 = 20000;

    // Compute the nullfier
    let nullfier = convert_u64_to_hash(bob_random + 1);
    println!("nullfier: {:?}", convert_u8_to_u64(nullfier));

    // =================== Bob sends `bob_random_hash` & `pay_value` to Alice(Frontend) ======================
    // send(bob_random_hash, pay_value)
    // =================== Alice's work ===========================================================

    // Deposit pay_value With bob_random_hash
    // deposit(bob_random_hash, value)
    // New NoteTree Leave append to the NoteTree
    let note_leave = compute_note_tree_leave(bob_random_hash, pay_value);
    let note_leave1 = make_test_note_leave(10001, 20001);
    let note_leave2 = make_test_note_leave(10002, 20002);

    let leaves: Vec<[u8; 32]> = [note_leave, note_leave1, note_leave2].to_vec();
    // Construct Merkle Roothash
    let merkle_tree = MerkleTree::<Rescue>::from_leaves(&leaves);
    let roothash = merkle_tree.root().unwrap();
    println!("Root hash: {:?}", convert_u8_to_u64(roothash));

    // Pick the leave to prove
    // In canister the indices should be returned by canister(or just query the merklePath via the leaveHash)
    // In the second way, there should be a mapping, map leaveHash to indice
    let indices_to_prove = vec![0];

    let leaves_to_prove = &[*leaves.get(0).ok_or("can't get leaves to prove").unwrap()];
    // println!("&leaves_to_prove[0] is {:?}", leaves_to_prove[0]);
    println!("leaves_to_prove is : {:?}", convert_u8_to_u64(note_leave));

    // Generate the MerkleProof & zkFlag
    // let (index_list, merkle_proof) = merkle_tree.proof(&indices_to_prove);
    let (index_list, merkle_proof) = merkle_tree.proof(&indices_to_prove);

    // Generate the miden_inputs
    let miden_inputs = construct_miden_input(merkle_proof, index_list);
    println!("miden_inputs is {:?}", miden_inputs);

    // Parse proof back on the client
    // let merkle_root = merkle_tree
    // .root()
    // .ok_or("couldn't get the merkle root")
    // .unwrap();
    // let proof_bytes = merkle_proof.to_bytes();
    // let proof = MerkleProof::<Rescue>::try_from(proof_bytes).unwrap();

    // assert!(proof.verify(
    //     merkle_root,
    //     &indices_to_prove,
    //     leaves_to_prove,
    //     leaves.len()
    // ));

    // A test for purely mekleTree without random_hash & pay_value
    // let leaf_values = [
    //     "3a657e95b8c80edda9252f9b257f2d194151985e174cd81f82dfd637016878c7",
    //     "bcccd471ccd18845f8c1812a65ea7bf5a3568f43c38969a379533c69601b31e2",
    //     "105606c8779bf17f4ba8439bfa442539ff196b7a4b6bd0a51508b0796b3fa919",
    // ];

    // // Construct leaves
    // let leaves: Vec<[u8; 32]> = leaf_values
    //     .iter()
    //     .map(|x| hex::decode(x).unwrap().try_into().unwrap())
    //     .collect();

    // println!("Leaves: {:?}", leaves);

    // // Construct Merkle Roothash
    // let merkle_tree = MerkleTree::<Rescue>::from_leaves(&leaves);
    // let roothash = merkle_tree.root().unwrap();
    // println!("Root hash: {:?}", convert_u8_to_u64(roothash));

    // // Pick the leave to prove
    // let indices_to_prove = vec![0];
    // let leaves_to_prove = &[*leaves.get(0).ok_or("can't get leaves to prove").unwrap()];
    // // println!("&leaves_to_prove[0] is {:?}", leaves_to_prove[0]);
    // // println!(
    // //     "leaves_to_prove is : {:?}",
    // //     convert_u8_to_u64(leaves_to_prove[0])
    // // );

    // // Generate the MerkleProof & zkFlag
    // // let (index_list, merkle_proof) = merkle_tree.proof(&indices_to_prove);
    // let (index_list, merkle_proof) = merkle_tree.proof(&indices_to_prove);

    // // Generate the miden_inputs
    // let miden_inputs = construct_miden_input(leaves_to_prove[0], merkle_proof, index_list);
    // println!("miden_inputs is {:?}", miden_inputs);

    // // Parse proof back on the client
    // // let merkle_root = merkle_tree
    // // .root()
    // // .ok_or("couldn't get the merkle root")
    // // .unwrap();
    // // let proof_bytes = merkle_proof.to_bytes();
    // // let proof = MerkleProof::<Rescue>::try_from(proof_bytes).unwrap();

    // // assert!(proof.verify(
    // //     merkle_root,
    // //     &indices_to_prove,
    // //     leaves_to_prove,
    // //     leaves.len()
    // // ));
}

// Helper: help construct note_leave with random & pay_value
fn make_test_note_leave(random: u64, pay_value: u64) -> [u8; 32] {
    let random_hash = convert_u64_to_hash(random);
    compute_note_tree_leave(random_hash, pay_value)
}

// {
//     '@context': [ 'https://www.w3.org/2018/credentials/v1' ],
//     version: '2',
//     ctype: '0xaab222068195a452210e0171e40e5d11c13ca581b1ac24690831a63ae9e0a906',
//     issuanceDate: 1711630605307,
//     credentialSubject: {
//       kyc_status: 1,
//       chain_code: 'bfc',
//       on_chain_address: 'BFC987a5c1897d743751fcf475ffbf23d76f851f37036f23c21b70262ed38aafc479783'
//     },
//     issuer: [ 'did:zk:0x57E7b664aaa7C895878DdCa5790526B9659350Ec' ],
//     holder: 'did:zk:0x11f8b77F34FCF14B7095BF5228Ac0606324E82D1',
//     hasher: [ 'RescuePrimeOptimized', 'Keccak256' ],
//     digest: '0x3760e2f8f2be0f8d05efed3e76bd8d0643e592f9a7c422f2a3ab7246149e1a06',
//     proof: [
//       {
//         type: 'EcdsaSecp256k1SignatureEip191',
//         created: 1711630605313,
//         verificationMethod: 'did:zk:0x57E7b664aaa7C895878DdCa5790526B9659350Ec#key-0',
//         proofPurpose: 'assertionMethod',
//         proofValue: 'zDRb1yQjvUETTrC76waqPZ97nScMDi83kTS1JYWqCPNsWfVxVPq4o942QFCxECE3dio9G7E7iZHS3Enziz8Uh2qvWj'
//       }
//     ],
//     credentialSubjectHashes: [
//       '0x3a657e95b8c80edda9252f9b257f2d194151985e174cd81f82dfd637016878c7',
//       '0xbcccd471ccd18845f8c1812a65ea7bf5a3568f43c38969a379533c69601b31e2',
//       '0x105606c8779bf17f4ba8439bfa442539ff196b7a4b6bd0a51508b0796b3fa919'
//     ],
//     credentialSubjectNonceMap: {
//       '0xfb20ae03df36661b746c4f247181eebd15850f6af189995ccbf8aa62223fd5ed': '0x233d171a61559865b27c5051c9d2ab84d7fbfb2fe7d35c30a257249adaac0f0e',
//       '0xb69d65be8c91e2910422863fd14483f7201b4c8c5fc1a565c0b5e8ea6654e67c': '0x9bdb53da27b58a1855c164f840a97ed096171f9534fca90277ea06cc6abcd81f',
//       '0xa56b53d4553b56459781b54ca9184e3e1881ee127897bb6918878e0eef100621': '0xbaea59cd40cc375c8bcc2d96573f48bf17e925d760128cf5aa333568c0f7eb79'
//     }
//   }
