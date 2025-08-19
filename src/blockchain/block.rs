use chrono::Utc;
use serde::{Serialize, Deserialize};

use super::coder;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    pub fn new_block(data: String, pre_hash: String) -> Block {
        let serialized_tx = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&serialized_tx.as_slice());
        let time = Utc::now().timestamp();
        let header = BlockHeader {
            time: time,
            tx_hash: tx_hash,
            pre_hash: pre_hash,
        };
        let serialized_header = coder::my_serialize(&header);
        let hash = coder::my_deserialize(&serialized_header.as_slice());
        Block {
            header: header,
            hash: hash,
            data: data,
        }      
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
}