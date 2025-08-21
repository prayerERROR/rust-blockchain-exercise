use bincode;
use serde::{Serialize, Deserialize};
use crypto::{digest::Digest, sha3::Sha3};

pub fn my_serialize<T: Serialize>(value: &T) -> Vec<u8> {
    bincode::serialize(value).unwrap()
}

pub fn my_deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
    bincode::deserialize(bytes).unwrap()
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}