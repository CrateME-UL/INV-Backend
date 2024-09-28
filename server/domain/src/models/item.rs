#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemId {
    id: i32,
}
pub struct Item {
    item_id: ItemId,
    item_name: String,
}

impl ItemId {
    pub fn new(id: i32) -> Self {
        Self { id }
    }

    pub fn value(&self) -> &i32 {
        &self.id
    }
}

impl Item {
    pub fn new(item_id: ItemId, item_name: String) -> Self {
        Self { item_id, item_name }
    }
}
