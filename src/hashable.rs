extern crate crypto;

use self::crypto::sha2::Sha256;
use self::crypto::digest::Digest;


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


pub fn hash<T: Hashable>(hashable: T) -> String {
    let mut hash = Sha256::new();
    hash.input(hashable.bytes());
    hash.result_str()
}
