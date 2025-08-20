use serde::{Serialize, Deserialize};

mod account;
mod block;
mod coder;
mod transaction;

use std::io::{self, Read, Write};
use std::fs::File;

use crate::blockchain::coder::my_deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockChain {
    pub blocks: Vec<block::Block>,
    pub account_state: account::AccountState,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        // Initialize new blocks with a genesis block
        let mut blocks = Vec::new();
        let genesis_block = block::Block::new_block(
            transaction::Transaction::coinbase("", 0.0),
            String::new(),
        );
        blocks.push(genesis_block);

        // Initialize account states
        let account_state = account::AccountState::new();

        BlockChain {
            blocks: blocks,
            account_state: account_state,
        }
    }

    pub fn add_block(&mut self, transaction: transaction::Transaction) {
        let last_block = self.blocks.last().unwrap();
        let pre_hash = last_block.calculate_hash();
        let new_block = block::Block::new_block(transaction, pre_hash);
        self.blocks.push(new_block);
    }

    pub fn save_chain(&self, file_name: &str) -> Result<(), io::Error> {
        let mut file = File::create(file_name)?;
        let serialized_chain = coder::my_serialize(self);
        file.write_all(&serialized_chain.as_slice())?;
        Ok(())
    }

    pub fn load_chain(file_name: &str) -> Result<BlockChain, io::Error> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let blockchain = my_deserialize(&buffer);
        Ok(blockchain)
    }
}
