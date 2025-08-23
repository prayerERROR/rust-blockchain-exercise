use std::collections::{HashSet, VecDeque};

use crate::core::transaction::Transaction;
use crate::error::MempoolError;

// Memory Pool
pub struct MemPool {
    pub pending_tx: VecDeque<Transaction>,
    pub tx_set: HashSet<String>,
    pub max_size: usize,
}

impl MemPool {
    pub fn new(max_size: usize) -> Self {
        MemPool {
            pending_tx: VecDeque::new(),
            tx_set: HashSet::new(),
            max_size: max_size,
        }
    }

    pub fn add_tx(&mut self, tx: &Transaction) -> Result<(), MempoolError> {
        if self.pending_tx.len() >= self.max_size {
            return Err(MempoolError::MemoryPoolFull);
        }

        let tx_id = format!("{}-{}-{}", tx.sender, tx.receiver, tx.nonce);
        match self.tx_set.contains(&tx_id) {
            true => Err(MempoolError::TransactionExisted),
            false => {
                self.pending_tx.push_back(tx.clone());
                self.tx_set.insert(tx_id);
                Ok(())
            },
        }
    }

    pub fn remove_tx(&mut self, tx: &Transaction) -> Result<(), MempoolError> {
        let tx_id = format!("{}-{}-{}", tx.sender, tx.receiver, tx.nonce);
        match self.tx_set.contains(&tx_id) {
            true => {
                let idx = self.pending_tx.iter().position(|x| x == tx).unwrap();
                self.pending_tx.remove(idx);
                self.tx_set.remove(&tx_id);
                Ok(())
            },
            false => Err(MempoolError::TransactionNotExisted),
        }
    }

    pub fn get_k_tx(&mut self, k: usize) -> Vec<Transaction> {
        let mut txs: Vec<Transaction> = Vec::new();
        for _ in 0..k {
            if let Some(tx) = self.pending_tx.pop_front() {
                let tx_id = format!("{}-{}-{}", tx.sender, tx.receiver, tx.nonce);
                self.tx_set.remove(&tx_id);
                txs.push(tx);
            } else {
                break
            }
        }
        txs
    }

    pub fn get_size(&self) -> usize {
        self.pending_tx.len()
    }

    pub fn clear(&mut self) {
        self.pending_tx.clear();
        self.tx_set.clear();
    }
}
