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
            difficulty
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

    pub fn create_block(&mut self, txs: Vec<Transaction>, miner_address: &str) -> Result<Block, BlockchainError> {
        let pre_block = self.get_last_block().unwrap();
        let pre_hash = pre_block.hash.clone();
        let height = 1 + pre_block.get_height();
        
        // A vector contains a transaction which is reward for miner
        let mut transactions = vec![
            Transaction::coinbase(miner_address, self.config.block_reward)
        ];

        transactions.extend(txs);
        Block::new(
            transactions,
            pre_hash,
            height,
            self.current_difficulty,
        )
    }

    pub fn validate_transaction(&mut self, tx: &Transaction) -> Result<(), BlockchainError> {
        if tx.is_coinbase() {
            tx.validate_basic()?;
        } else {
            let sender_balance = self.account_state.get_balance(&tx.sender);
            let expected_nonce = 1 + self.account_state.get_nonce(&tx.sender);
            tx.validate_with_state(sender_balance, expected_nonce+1)?;
        }

        Ok(())
    }

    pub fn process_transaction(&mut self, tx: &Transaction) -> Result<(), BlockchainError> {
        if tx.is_coinbase() {
            self.account_state.credit(&tx.receiver, tx.amount)?;
        } else {
            let cost = tx.calculate_cost();
            self.account_state.credit(&tx.receiver, tx.amount)?;
            self.account_state.debit(&tx.sender, cost)?;
            self.account_state.increase_nonce(&tx.sender)?;
        }

        Ok(())
    }

    pub fn adjust_difficulty(&mut self) {
        let current_height = self.blocks.len() as u32;
        if current_height % self.config.diffuclty_interval == 0 {
            self.current_difficulty += 1;
            println!("Difficulty adjusted to: {}.", self.current_difficulty);
        }
    }

    pub fn get_height(&self) -> u64 {
        match self.blocks.last() {
            Some(block) => block.get_height(),
            None => 0,
        }
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        match self.block_index.get(hash) {
            Some(&index) => Some(&self.blocks[index]),
            None => None,
        }
    }

    pub fn get_block_by_height(&self, height: u64) -> Option<&Block> {
        self.blocks.get(height as usize)
    }

    pub fn get_account_balance(&self, address: &str) -> f64 {
        self.account_state.get_balance(address)
    }

    pub fn get_account_nonce(&self, address: &str) -> u64 {
        self.account_state.get_nonce(&address)
    }

    pub fn create_account(&mut self, address: &str) -> Result<(), BlockchainError> {
        self.account_state.create_account(address)
    }
    
}
