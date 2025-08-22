use std::fs::File;
use std::io::{Read, Write, Error};

use crate::core::blockchain::BlockChain;
use crate::utils::serialization::{serialize, deserialize};

pub fn save_chain(file_name: &str, chain: &BlockChain) -> Result<(), Error> {
    let mut file = File::create(file_name)?;
    let serialized_chain = serialize(chain).unwrap();
    file.write_all(&serialized_chain.as_slice())?;
    Ok(())
}

pub fn load_chain(file_name: &str) -> Result<BlockChain, Error> {
    let mut file = File::open(file_name)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let blockchain: BlockChain = deserialize(&buffer).unwrap();
    Ok(blockchain)
}
