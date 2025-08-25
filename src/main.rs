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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test start
    println!("Hello, blockchain.");

    // Create blockchain with default config
    let config = BlockchainConfig::default();
    let mut blockchain = BlockChain::new(config)?;
    println!("Genesis block created.");
    println!("Blockchain height: {}", blockchain.get_height());

    // Create some accounts
    blockchain.create_account("Alice")?;
    blockchain.create_account("Bob")?;
    blockchain.create_account("Miner1")?;

    // Create a transaction, a block and add the block to blockchain.
    let mut transactions = Vec::new();
    transactions.push(Transaction::coinbase("Alice", 100.0));
    let block1 = blockchain.create_block(transactions, "Miner1")?;
    blockchain.add_block(block1)?;

    println!("Block 1 added. Alice balance: {}", blockchain.get_account_balance("Alice"));
    println!("Blockchain height: {}", blockchain.get_height());

    // Transfer between accounts
    let tx1 = Transaction::new("Alice", "Bob", 25.0, 1.0,
            blockchain.get_account_nonce("Alice")+1);
    
    let block2 = blockchain.create_block(vec![tx1], "Miner1")?;
    blockchain.add_block(block2)?;

    println!("Block 2 added. Alice balance: {}, Bob balance: {}", 
            blockchain.get_account_balance("Alice"),
            blockchain.get_account_balance("Bob"));

    println!("Blockchain height: {}", blockchain.get_height());

    // Test end
    println!("Test passed.");
    Ok(())
}
