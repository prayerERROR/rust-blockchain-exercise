use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use crate::account::state::AccountState;
use crate::config::BlockchainConfig;
use crate::core::block::Block;
use crate::core::transaction::Transaction;
use crate::error::BlockchainError;

// Blockchain
#[derive(Serialize, Deserialize, Debug)]
pub struct BlockChain {
    blocks: Vec<Block>,
    account_state: AccountState,
    config: BlockchainConfig,
    current_difficulty: usize,
    block_index: HashMap<String, usize>,
}

impl BlockChain {
    pub fn new(config: BlockchainConfig) -> Result<Self, BlockchainError> {
        let difficulty = config.initial_difficulty;

        // Initialize new blocks with a genesis block
        let genesis_block = Block::new(
            Vec::new(),
            "0".repeat(64),
            0,
            difficulty,
        )?;

        // Initialize block index
        let mut block_index: HashMap<String, usize> = HashMap::new();
        block_index.insert(genesis_block.hash.clone(), 0);

        Ok(BlockChain {
            blocks: vec![genesis_block],
            account_state: AccountState::new(),
            config: config,
            current_difficulty: difficulty,
            block_index: block_index,
        })
    }

    pub fn add_block(&mut self) {
        unimplemented!()
    }

    pub fn validate_transaction(&mut self, tx: &Transaction) -> Result<(), BlockchainError> {
        tx.validate_basic()?;
        if !tx.is_coinbase() {
            let sender_balance = self.account_state.get_balance(&tx.sender);
            let sender_nonce = self.account_state.get_nonce(&tx.sender);
            
            if sender_balance < tx.calculate_cost() {
                return Err(BlockchainError::InsufficientBalance(
                    format!("{} does not have enough balance.", &tx.sender)
                ))
            }

            if tx.nonce != sender_nonce + 1 {
                return Err(BlockchainError::InvalidNonce(
                    format!("Invalid nonce for {}", &tx.sender)
                ))
            }
        }

        Ok(())
    }

    pub fn process_transaction(&mut self, tx: &Transaction) -> Result<(), BlockchainError> {
        if !tx.is_coinbase() {
            let sender_balance = self.account_state.get_balance(&tx.sender);
            let receiver_balance = self.account_state.get_balance(&tx.receiver);
            let cost = tx.calculate_cost();

            // Update sender and receiver balance
            let new_sender_balance = sender_balance - cost;
            let new_receiver_balance = receiver_balance + tx.amount;
            self.account_state.update_balance(&tx.sender, new_sender_balance)?;
            self.account_state.update_balance(&tx.receiver, new_receiver_balance)?;
            self.account_state.increase_nonce(&tx.sender)?;
        } else {
            // If sender is system, update receiver balance directly
            let new_receiver_balance = self.account_state.get_balance(&tx.receiver) + tx.amount;
            self.account_state.update_balance(&tx.receiver, new_receiver_balance)?;
        }

        Ok(())
    }

    pub fn adjust_difficulty(&mut self) {
        let current_height = self.blocks.len() as u32;
        if current_height % self.config.diffuclty_interval == 0 {
            self.current_difficulty += 1;
        }
    }

    pub fn get_height(&self) -> u64 {
        match self.blocks.last() {
            Some(block) => block.get_height(),
            None => 0,
        }
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        match self.block_index.get(hash) {
            Some(&index) => Some(&self.blocks[index]),
            None => None,
        }
    }
    
}
