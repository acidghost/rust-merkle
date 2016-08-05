extern crate crypto;

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::mem;
use std::fmt;


fn hash(input: &str) -> String {
    let mut hash = Sha256::new();
    hash.input_str(input);
    hash.result_str()
}


pub struct MerkleNode {
    hash_value: String,
    children: Vec<MerkleNode>
}

impl MerkleNode {
    pub fn new(value: &str) -> MerkleNode {
        MerkleNode {
            hash_value: hash(value),
            children: Vec::new()
        }
    }
}


#[test]
fn new_merkle_root() {
    let root = MerkleNode::new("");
    assert_eq!(root.hash_value, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
}
