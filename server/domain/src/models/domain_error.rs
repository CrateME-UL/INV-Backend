use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum DomainError {
    ItemError(String),
    ItemNumberError(String),
    PlaceNumberError(String),
    PlaceError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::ItemError(msg) => write!(f, "Item error: {}", msg),
            DomainError::ItemNumberError(msg) => write!(f, "Item number error: {}", msg),
            DomainError::PlaceNumberError(msg) => write!(f, "Place number error: {}", msg),
            DomainError::PlaceError(msg) => write!(f, "Place error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}
