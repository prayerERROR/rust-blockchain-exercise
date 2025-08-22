use chrono::Utc;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use crate::error::BlockchainError;

// Account
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub address: String,
    pub balance: f64,
    pub nonce: u64,
    pub created_at: i64,
    pub last_active: i64,
}

impl Account {
    pub fn new(address: String) -> Self {
        let curr_time = Utc::now().timestamp();
        Account {
            address: address,
            balance: 0.0,
            nonce: 0,
            created_at: curr_time,
            last_active: curr_time,
        }
    }

    pub fn update_activity(&mut self) {
        self.last_active = Utc::now().timestamp();
    }
}

// Account State
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountState {
    accounts: HashMap<String, Account>,
    total_supply: f64, // why we need this?
}

impl AccountState {
    pub fn new() -> AccountState {
        AccountState {
            accounts: HashMap::new(),
            total_supply: 0.0,
        }
    }

    pub fn create_account(&mut self, address: &str) -> Result<(), BlockchainError> {
        match self.exist_account(address) {
            true => Err(BlockchainError::DuplicatedAccount(
                "Block hash doesn't match requirement.".to_string()
            )),
            false => {
                let new_account = Account::new(address.to_string());
                self.accounts.insert(address.to_string(), new_account);
                Ok(())
            }
        }
    }

    pub fn exist_account(&self, address: &str) -> bool {
        self.accounts.contains_key(address)
    }

    pub fn get_account(&self, address: &str) -> Option<&Account> {
        self.accounts.get(address)
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        match self.accounts.get(address) {
            Some(account) => account.balance,
            None => 0.0,
        }
    }

    pub fn get_nonce(&self, address: &str) -> u64 {
        match self.accounts.get(address) {
            Some(account) => account.nonce,
            None => 0,
        }
    }

    pub fn get_total_supply(&self) -> f64 {
        self.total_supply
    }

    pub fn increase_nonce(&mut self, address: &str) -> Result<(), BlockchainError> {
        match self.accounts.get_mut(address) {
            Some(account) => {
                account.nonce += 1;
                Ok(())
            },
            None => Err(BlockchainError::InvalidAccount(
                "Account does not exist.".to_string()
            )),
        }
    }

    pub fn update_balance(&mut self, address: &str, new_balance: f64) -> Result<(), BlockchainError> {
        match self.accounts.get_mut(address) {
            Some(account) => {
                account.balance = new_balance;
                Ok(())
            },
            None => Err(BlockchainError::InvalidAccount(
                "Account does not exist.".to_string()
            )),
        }
    }

    pub fn credit() {
        unimplemented!()
    }

    pub fn debit() {
        unimplemented!()
    }

}