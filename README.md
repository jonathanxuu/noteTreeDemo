# Construct a merkle tree with rescue hash
The example is from a vc example (which is a rescue MerkleTree)

```rust
use rs_merkle::{algorithms::Rescue, Hasher, MerkleProof, MerkleTree};
use bytemuck::cast_slice;
use hex::FromHex;

fn main() {
    let leaf_values = [
        "3a657e95b8c80edda9252f9b257f2d194151985e174cd81f82dfd637016878c7", 
        "bcccd471ccd18845f8c1812a65ea7bf5a3568f43c38969a379533c69601b31e2", 
        "105606c8779bf17f4ba8439bfa442539ff196b7a4b6bd0a51508b0796b3fa919"];
    
    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| hex::decode(x).unwrap().try_into().unwrap())
        .collect();

    println!("Leaves: {:?}", leaves);

    let merkle_tree = MerkleTree::<Rescue>::from_leaves(&leaves);
    let roothash = merkle_tree.root();
    println!("Root hash: {:?}", hex::encode(roothash.unwrap()));
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
```