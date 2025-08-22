use serde::{Serialize, Deserialize};
use crate::error::BlockchainError;

pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, BlockchainError> {
    bincode::serialize(value)
        .map_err(|msg| BlockchainError::SerializationError(msg.to_string()))
}

pub fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T, BlockchainError> {
    bincode::deserialize(bytes)
        .map_err(|msg| BlockchainError::SerializationError(msg.to_string()))
}