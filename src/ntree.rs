use hashable::*;


#[derive(Debug)]
pub struct MerkleTree<T>(String, T, Vec<MerkleTree<T>>);

impl<T> MerkleTree<T>
    where T: Hashable + Copy {

    pub fn new(value: T) -> MerkleTree<T> {
        MerkleTree::grow(value, vec![])
    }

    pub fn grow(value: T, children: Vec<MerkleTree<T>>) -> MerkleTree<T> {
        let mut children_hash = String::new();
        for child in &children {
            let &MerkleTree(ref child_hash, child_value, _) = child;
            assert_eq!(hash(child_value), *child_hash);
            children_hash = children_hash + &child_hash
        }

        let hash_value = if children_hash.len() > 0 { hash(hash(value) + &children_hash) } else { hash(value) };
        MerkleTree(hash_value, value, children)
    }

}



#[cfg(test)]
mod tests {
    use super::MerkleTree;

    #[test]
    fn new_tree() {
        let root = MerkleTree::new("");

        let MerkleTree(hv, v, c) = root;
        assert_eq!(hv, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(v, "");
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn one_child() {
        let child = vec![MerkleTree::new("")];
        let root = MerkleTree::grow("", child);

        let MerkleTree(hv, v, c) = root;
        assert_eq!(hv, "3b7546ed79e3e5a7907381b093c5a182cbf364c5dd0443dfa956c8cca271cc33");
        assert_eq!(v, "");
        assert_eq!(c.len(), 1);
        let MerkleTree(ref chv, cv, ref cc) = c[0];
        assert_eq!(chv, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(cv, "");
        assert_eq!(cc.len(), 0);
    }

}
