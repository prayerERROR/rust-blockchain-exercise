use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    address: String,
    balance: f64,
    nonce: u64,
}

impl Account {
    pub fn new(address: String) -> Account {
        Account {
            address: address,
            balance: 0.0,
            nonce: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountState {
    accounts: HashMap<String, Account>,
}

impl AccountState {
    pub fn new() -> AccountState {
        AccountState {
            accounts: HashMap::new(),
        }
    }

    pub fn add_account(&mut self, address: &str) -> Result<(), ()> {
        if !self.accounts.contains_key(address) {
            let new_account = Account::new(address.to_string());
            self.accounts.insert(address.to_string(), new_account);
            Ok(())
        } else {
            Err(())
        }
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

    pub fn update_balance(&mut self, address: &str, new_balance: f64) {
        match self.accounts.get_mut(address) {
            Some(account) => account.balance = new_balance,
            None => {
                let mut account = Account::new(address.to_string());
                account.balance = new_balance;
                self.accounts.insert(address.to_string(), account);
            },
        }
    }

}