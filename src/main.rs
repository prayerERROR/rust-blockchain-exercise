mod account;
mod config;
mod core;
mod crypto;
mod error;
mod mempool;
mod network;
mod storage;
mod utils;

use config::BlockchainConfig;
use core::blockchain::BlockChain;
use core::transaction::Transaction;

fn main() {
    println!("hello, blockchain.");
}
