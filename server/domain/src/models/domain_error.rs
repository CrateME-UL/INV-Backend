#[derive(Debug, PartialEq, Clone)]
pub enum DomainError {
    ItemError(String),
    ItemIdError(String),
    PlaceIdError(String),
    PlaceError(String),
    InventoryError(String),
}
