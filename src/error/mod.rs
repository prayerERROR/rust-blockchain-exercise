#[derive(Debug)]
pub enum BlockchainError {
    InsufficientBalance(String),
    InvalidBlock(String),
    InvalidNonce(String),
    SerializationError(String),
    ValidationError(String),
}

impl std::fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlockchainError::InsufficientBalance(msg)
                => write!(f, "Insufficient balance: {}", msg),
            BlockchainError::InvalidBlock(msg)
                => write!(f, "Invalid block: {}", msg),
            BlockchainError::InvalidNonce(msg)
                => write!(f, "Invalid nonce: {}", msg),
            BlockchainError::SerializationError(msg)
                => write!(f, "Serialization error: {}", msg),
            BlockchainError::ValidationError(msg)
                => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for BlockchainError {}