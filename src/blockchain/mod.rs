mod account;
mod block;
mod coder;
mod transaction;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
    pub account_state: account::AccountState,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let mut blocks = Vec::new();
        let genesis_block = block::Block::new_block(
            transaction::Transaction::coinbase("", 0.0),
            String::new(),
        );
        blocks.push(genesis_block);

        let account_state = account::AccountState::new();

        BlockChain {
            blocks: blocks,
            account_state: account_state,
        }
    }

    pub fn add_block(&mut self, transaction: transaction::Transaction) {
        let last_block = self.blocks.last().unwrap();
        let pre_hash = last_block.calculate_hash();
        let new_block = block::Block::new_block(transaction, pre_hash);
        self.blocks.push(new_block);
    }
}
