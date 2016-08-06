extern crate crypto;

use crypto::sha2::Sha256;
use crypto::digest::Digest;


pub trait Hashable {
    fn bytes<'a>(&'a self) -> &'a [u8];
}


impl Hashable for String {
    fn bytes<'a>(&'a self) -> &'a [u8] {
        self.as_bytes()
    }
}

impl<'t> Hashable for &'t str {
    fn bytes<'a>(&'a self) -> &'a [u8] {
        self.as_bytes()
    }
}


fn hash<T: Hashable>(input: T) -> String {
    let mut hash = Sha256::new();
    hash.input(input.bytes());
    hash.result_str()
}


pub enum MerkleTree {
    BinMerkleTree,
}

pub enum BinMerkleTree<'a> {
    EmptyTree,
    BinMerkleNode(String, &'a BinMerkleTree<'a>, &'a BinMerkleTree<'a>)
}

impl<'a> BinMerkleTree<'a> {
    pub fn new<T: Hashable>(value: T, left: &'a BinMerkleTree<'a>, right: &'a BinMerkleTree<'a>) -> BinMerkleTree<'a> {
        let mut children_hash = String::new();
        match left {
            &BinMerkleTree::BinMerkleNode(ref hv, _, _) => children_hash = children_hash + &hv,
            _ => {}
        }
        match right {
            &BinMerkleTree::BinMerkleNode(ref hv, _, _) => children_hash = children_hash + &hv,
            _ => {}
        }

        let hash_value = if children_hash.len() > 0 { hash(hash(value) + &children_hash) } else { hash(value) };
        BinMerkleTree::BinMerkleNode(
            hash_value, left, right
        )
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_merkle_root() {
        let empty = BinMerkleTree::EmptyTree;
        let root = BinMerkleTree::new("", &empty, &empty);
        match root {
            BinMerkleTree::BinMerkleNode(hv, &BinMerkleTree::EmptyTree, &BinMerkleTree::EmptyTree) =>
                assert_eq!(hv, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            _ => assert!(false),
        }
    }

    #[test]
    fn one_child() {
        let empty = BinMerkleTree::EmptyTree;
        let child = BinMerkleTree::new("", &empty, &empty);
        let root = BinMerkleTree::new("", &child, &empty);
        match root {
            BinMerkleTree::BinMerkleNode(ref hv, &BinMerkleTree::BinMerkleNode(ref chv, _, _), &BinMerkleTree::EmptyTree) => {
                assert_eq!(hv, "3b7546ed79e3e5a7907381b093c5a182cbf364c5dd0443dfa956c8cca271cc33");
                assert_eq!(chv, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
            }
            _ => assert!(false)
        }
    }

    #[test]
    fn two_children() {
        let empty = BinMerkleTree::EmptyTree;
        let child = BinMerkleTree::new("", &empty, &empty);
        let root = BinMerkleTree::new("", &child, &child);
        match root {
            BinMerkleTree::BinMerkleNode(hv, _, _) =>
                assert_eq!(hv, "74313561d1897af3dc03f4fae174960d28968f92b49230523faca462b848db60"),
            _ => assert!(false)
        }
    }

}
