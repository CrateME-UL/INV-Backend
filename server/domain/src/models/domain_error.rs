#[derive(Debug, PartialEq)]
pub enum DomainError {
    ItemError(String),
    ItemIdError(String),
}
