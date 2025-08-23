// Erros

// A pile of errors
// This will be splitted into specific catgory in the future.
#[derive(Debug)]
pub enum BlockchainError {
    DuplicatedAccount(String),
    InsufficientBalance(String),
    InvalidAccount(String),
    InvalidBlock(String),
    InvalidNonce(String),
    SerializationError(String),
    ValidationError(String),
}

impl std::fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlockchainError::DuplicatedAccount(msg)
                => write!(f, "Duplicated account: {}", msg),
            BlockchainError::InsufficientBalance(msg)
                => write!(f, "Insufficient balance: {}", msg),
            BlockchainError::InvalidAccount(msg)
                => write!(f, "Invalid account: {}", msg),
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


// Memory pool errors
#[derive(Debug)]
pub enum MempoolError {
    MemoryPoolFull,
    TransactionExisted,
    TransactionNotExisted,
}

impl std::fmt::Display for MempoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MempoolError::MemoryPoolFull
                => write!(f, "Memory pool is already full."),
            MempoolError::TransactionExisted
                => write!(f, "Transaction is already in pool."),
            MempoolError::TransactionNotExisted
                => write!(f, "Transaction is not in pool."),
        }
    }
}

impl std::error::Error for MempoolError {}