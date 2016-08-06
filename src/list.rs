use hashable::*;


#[derive(Debug)]
pub enum MerkleList<'a> {
    Empty,
    Node(String, &'a MerkleList<'a>)
}

impl<'a> MerkleList<'a> {
    pub fn new<T: Hashable>(value: T, child: &'a MerkleList<'a>) -> MerkleList<'a> {
        match *child {
            MerkleList::Empty => MerkleList::Node(hash(value), child),
            MerkleList::Node(ref hv, _) => MerkleList::Node(hash(hash(value) + &hv), child)
        }
    }
}




#[cfg(test)]
mod tests {
    use super::MerkleList;

    #[test]
    fn new_list() {
        let empty = MerkleList::Empty;
        let list = MerkleList::new("", &empty);
        match list {
            MerkleList::Node(hv, &MerkleList::Empty) =>
                assert_eq!(hv, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            _ => assert!(false)
        }
    }

    #[test]
    fn one_child() {
        let empty = MerkleList::Empty;
        let child = MerkleList::new("", &empty);
        let list = MerkleList::new("", &child);
        match list {
            MerkleList::Node(ref hv, &MerkleList::Node(ref chv, &MerkleList::Empty)) => {
                assert_eq!(hv, "3b7546ed79e3e5a7907381b093c5a182cbf364c5dd0443dfa956c8cca271cc33");
                assert_eq!(chv, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
            }
            _ => assert!(false)
        }
    }

}
