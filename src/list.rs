use hashable::*;


#[derive(Debug)]
pub enum MerkleList<'a, T: 'a> {
    Empty,
    Node(String, T, &'a MerkleList<'a, T>)
}

impl<'a, T> MerkleList<'a, T>
    where T: Hashable + Copy {

    pub fn new(value: T, child: &'a MerkleList<'a, T>) -> MerkleList<'a, T> {
        let new_hash = hash(value);
        match child {
            &MerkleList::Empty => MerkleList::Node(new_hash, value, child),
            &MerkleList::Node(ref hv, ref cv, _) => {
                assert_eq!(hash(*cv), *hv);
                MerkleList::Node(hash(new_hash + &hv), value, child)
            }
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
            MerkleList::Node(hv, v, &MerkleList::Empty) => {
                assert_eq!(hv, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
                assert_eq!(v, "")
            }
            _ => assert!(false)
        }
    }

    #[test]
    fn one_child() {
        let empty = MerkleList::Empty;
        let child = MerkleList::new("", &empty);
        let list = MerkleList::new("", &child);
        match list {
            MerkleList::Node(ref hv, v, &MerkleList::Node(ref chv, cv, &MerkleList::Empty)) => {
                assert_eq!(v, "");
                assert_eq!(hv, "3b7546ed79e3e5a7907381b093c5a182cbf364c5dd0443dfa956c8cca271cc33");
                assert_eq!(cv, "");
                assert_eq!(chv, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
            }
            _ => assert!(false)
        }
    }

}
