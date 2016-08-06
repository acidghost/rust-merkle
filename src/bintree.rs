use hashable::*;


#[derive(Debug)]
pub enum BinMerkleTree<'a, T: 'a> {
    Empty,
    Node(String, T, &'a BinMerkleTree<'a, T>, &'a BinMerkleTree<'a, T>)
}

impl<'a, T> BinMerkleTree<'a, T>
    where T: Hashable + Copy {

    pub fn new(value: T, left: &'a BinMerkleTree<'a, T>, right: &'a BinMerkleTree<'a, T>) -> BinMerkleTree<'a, T> {
        let mut children_hash = String::new();
        match left {
            &BinMerkleTree::Node(ref hv, v, _, _) => {
                assert_eq!(hash(v), *hv);
                children_hash = children_hash + &hv;
            }
            _ => {}
        }
        match right {
            &BinMerkleTree::Node(ref hv, v, _, _) => {
                assert_eq!(hash(v), *hv);
                children_hash = children_hash + &hv;
            }
            _ => {}
        }

        let hash_value = if children_hash.len() > 0 { hash(hash(value) + &children_hash) } else { hash(value) };
        BinMerkleTree::Node(
            hash_value, value, left, right
        )
    }

}




#[cfg(test)]
mod tests {
    use super::BinMerkleTree;

    #[test]
    fn new_merkle_root() {
        let empty = BinMerkleTree::Empty;
        let root = BinMerkleTree::new("", &empty, &empty);
        match root {
            BinMerkleTree::Node(hv, _, &BinMerkleTree::Empty, &BinMerkleTree::Empty) =>
                assert_eq!(hv, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            _ => assert!(false),
        }
    }

    #[test]
    fn one_child() {
        let empty = BinMerkleTree::Empty;
        let child = BinMerkleTree::new("", &empty, &empty);
        let root = BinMerkleTree::new("", &child, &empty);
        match root {
            BinMerkleTree::Node(ref hv, _, &BinMerkleTree::Node(ref chv, _, _, _), &BinMerkleTree::Empty) => {
                assert_eq!(hv, "3b7546ed79e3e5a7907381b093c5a182cbf364c5dd0443dfa956c8cca271cc33");
                assert_eq!(chv, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
            }
            _ => assert!(false)
        }
    }

    #[test]
    fn two_children() {
        let empty = BinMerkleTree::Empty;
        let child = BinMerkleTree::new("", &empty, &empty);
        let root = BinMerkleTree::new("", &child, &child);
        match root {
            BinMerkleTree::Node(hv, _, _, _) =>
                assert_eq!(hv, "74313561d1897af3dc03f4fae174960d28968f92b49230523faca462b848db60"),
            _ => assert!(false)
        }
    }

}
