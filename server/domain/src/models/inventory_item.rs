#[derive(Debug)]
pub struct InventoryItem {
    pub item_id: Option<i32>,
    pub place_type: Option<String>,
    pub place_name: String,
    pub item_name: Option<String>,
    pub nb_of_items: i32,
}
