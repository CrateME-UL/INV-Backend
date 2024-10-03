#[derive(Debug, PartialEq, Clone)]
pub enum DomainError {
    ItemError(String),
    ItemIdError(String),
}
