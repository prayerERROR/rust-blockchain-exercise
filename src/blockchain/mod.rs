mod block;
mod coder;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let mut blocks = Vec::new();
        let genesis_block = block::Block::new_block(
            String::from("This is the genesis block."),
            String::new()
        );
        blocks.push(genesis_block);
        BlockChain { blocks: blocks }
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.blocks.last().unwrap();
        let pre_hash = last_block.get_hash();
        let new_block = block::Block::new_block(data, pre_hash);
        self.blocks.push(new_block);
    }

}