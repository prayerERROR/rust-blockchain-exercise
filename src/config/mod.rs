use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockchainConfig {
    pub block_reward: f64,
    pub block_max_tx: u32,
    pub initial_difficulty: usize,
    pub diffuclty_interval: u32,
    pub max_mine_time: u32,
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        BlockchainConfig {
            block_reward: 10.0,
            block_max_tx: 1000,
            initial_difficulty: 4,
            diffuclty_interval: 2000,
            max_mine_time: 1200,
        }
    }
}