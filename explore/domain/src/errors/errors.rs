use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum DomainError {
    InventoryError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::InventoryError(msg) => write!(f, "Inventory error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}
