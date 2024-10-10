#[derive(Debug, PartialEq, Clone)]
pub enum DomainError {
    ItemError(String),
    ItemNumberError(String),
    PlaceIdError(String),
    PlaceError(String),
}
