use chrono::Utc;
use serde::{Serialize, Deserialize};

use super::coder;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct BlockHeader {
    pub time: i64,
    pub nonce: u32,
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
            nonce: 0,
            tx_hash: tx_hash,
            pre_hash: pre_hash,
        };
        let serialized_header = coder::my_serialize(&header);
        let hash = coder::get_hash(&serialized_header.as_slice());
        Block {
            header: header,
            hash: hash,
            data: data,
        }      
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        // Assume we can find the valid hash in u32 range.
        for _ in 0..=u32::MAX {
            if self.hash.starts_with(target.as_str()) {
                break;
            } else {
                self.header.nonce += 1;
                let serialized_header = coder::my_serialize(&self.header);
                self.hash = coder::get_hash(&serialized_header.as_slice());
            }
        }
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
}