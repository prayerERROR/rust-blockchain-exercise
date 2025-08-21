use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub fee: f64,
    pub timestamp: i64,
    pub nonce: u64,
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: f64, nonce: u64) -> Transaction {
        Transaction { 
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount: amount,
            fee: 1.0,
            timestamp: Utc::now().timestamp(),
            nonce: nonce,
        }
    }

    pub fn coinbase(receiver: &str, amount: f64) -> Transaction {
        Transaction { 
            sender: "SYSTEM".to_string(),
            receiver: receiver.to_string(),
            amount: amount,
            fee: 0.0,
            timestamp: Utc::now().timestamp(),
            nonce: 0,
        }
    }

    pub fn is_coinbase(&self) -> bool {
        self.sender == "SYSTEM"
    }

    pub fn calculate_cost(&self) -> f64 {
        self.amount + self.fee
    }
}