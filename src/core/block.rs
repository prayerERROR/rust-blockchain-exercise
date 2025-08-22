use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::crypto::hash::get_hash;
use crate::crypto::merkle::MerkleTree;
use crate::error::BlockchainError;
use super::transaction::Transaction;
use crate::utils::serialization::serialize;

// Block header
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub height: u64,
    pub timestamp: i64,
    pub nonce: u64,
    pub merkle_root: String,
    pub pre_hash: String,
    pub difficulty: usize,
}

impl BlockHeader {
    pub fn calculate_hash(&self) -> Result<String, BlockchainError> {
        let serialized_header = serialize(self)?;
        Ok(get_hash(&serialized_header))
    }
}

// Block
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        transactions: Vec<Transaction>,
        pre_hash: String,
        height: u64,
        difficulty: usize
    ) -> Result<Block, BlockchainError> {
        // Validate all transactions
        for tx in &transactions {
            tx.validate_basic()?;
        }

        // Calculate merkle root
        let tx_data: Vec<Vec<u8>> = transactions
            .iter()
            .map(|x| serialize(x).unwrap())
            .collect();
        let merkle_tree = MerkleTree::new(tx_data);
        let merkle_root = merkle_tree.get_root().unwrap_or("0".repeat(64));

        // Construct block header
        let block_header = BlockHeader {
            height: height,
            timestamp: Utc::now().timestamp(),
            nonce: 0,
            merkle_root: merkle_root,
            pre_hash: pre_hash,
            difficulty: difficulty,
        };

        // Construct block
        let hash = block_header.calculate_hash()?;
        let block = Block {
            header: block_header,
            hash: hash,
            transactions: transactions,
        };

        Ok(block)
    }

    pub fn mine(&mut self) {
        let target = "0".repeat(self.header.difficulty);

        // Assume we can find the valid hash in u32 range.
        for _ in 0..=u32::MAX {
            if self.hash.starts_with(target.as_str()) {
                break;
            } else {
                self.header.nonce += 1;
                self.hash = self.calculate_hash();
            }
        }

        // Print info
        println!("Block mined! Nonce: {}, Hash: {}", self.header.nonce, self.hash);
    }

    pub fn calculate_hash(&self) -> String {
        self.header.calculate_hash().unwrap()
    }

    pub fn verify(&self) -> Result<(), BlockchainError> {
        // Verify hash difficulty requirement
        let target = "0".repeat(self.header.difficulty);
        if !self.hash.starts_with(&target) {
            return Err(BlockchainError::InvalidBlock(
                "Block hash doesn't match requirement.".to_string()
            ));
        }

        // Verify hash correctness
        let correct_hash = self.calculate_hash();
        if self.hash != correct_hash {
            return Err(BlockchainError::InvalidBlock(
                "Block hash is wrong.".to_string()
            ));
        }

        // Verify tx again.
        for tx in &self.transactions {
            tx.validate_basic();
        }

        Ok(())
    }

}