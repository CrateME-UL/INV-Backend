#[derive(Serialize)]
struct Item {
    item_id: i32,
    item_name: String,
}

#[derive(Serialize)]
struct Place {
    place_id: i32,
    place_name: String,
    place_type: String,
}

#[derive(Serialize)]
struct InventoryItem {
    item_id: i32,
    item_name: String,
    nb_of_items: i32,
}

#[derive(Serialize, Debug)]
struct InventoryPlace {
    place_id: i32,
    place_name: String,
    place_type: String,
    nb_of_items: i32,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
