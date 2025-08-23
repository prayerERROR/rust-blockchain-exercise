use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::error::BlockchainError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub fee: f64,
    pub timestamp: i64,
    pub nonce: u64,
    pub signature: Option<Vec<u8>>,
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: f64, fee: f64, nonce: u64) -> Self {
        Transaction { 
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount: amount,
            fee: fee,
            timestamp: Utc::now().timestamp(),
            nonce: nonce,
            signature: None,
        }
    }

    pub fn coinbase(receiver: &str, amount: f64) -> Self {
        Transaction { 
            sender: "SYSTEM".to_string(),
            receiver: receiver.to_string(),
            amount: amount,
            fee: 0.0,
            timestamp: Utc::now().timestamp(),
            nonce: 0,
            signature: None,
        }
    }

    pub fn is_coinbase(&self) -> bool {
        self.sender == "SYSTEM"
    }

    pub fn calculate_cost(&self) -> f64 {
        self.amount + self.fee
    }

    pub fn validate_basic(&self) -> Result<(), BlockchainError> {
        if self.amount < 0.0 {
            return Err(BlockchainError::ValidationError(
                "Transaction amount should be non-negative.".to_string()
            ));
        }

        if self.fee < 0.0 {
            return Err(BlockchainError::ValidationError(
                "Transaction fee should be non-negative.".to_string()
            ));
        }

        if self.sender.is_empty() {
            return Err(BlockchainError::ValidationError(
                "Sender address should be valid.".to_string()
            ));
        }

        if self.receiver.is_empty() {
            return Err(BlockchainError::ValidationError(
                "Receiver address should be valid.".to_string()
            ));
        }

        Ok(())
    }
}