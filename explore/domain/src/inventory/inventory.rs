use std::collections::HashMap;

use crate::errors::errors::DomainError;

use super::item_type::{ItemType, ItemTypeNumber};

fn get_key_from_value<T, U>(map: &HashMap<T, U>, value: &U) -> Option<T>
where
    T: Eq + std::hash::Hash + Clone,
    U: PartialEq,
{
    map.iter()
        .find(|(_, v)| *v == value)
        .map(|(k, _)| k.clone())
}

pub struct Inventory {
    item_types: HashMap<ItemTypeNumber, ItemType>,
}

impl Inventory {
    fn new(item_types: HashMap<ItemTypeNumber, ItemType>) -> Self {
        Self { item_types }
    }
    
    pub fn verify_item_type_is_new(&mut self, item_type: ItemType) -> Result<ItemTypeNumber, DomainError> {
        let item_type_number: ItemTypeNumber = item_type.get_number();
        let item_type_number_found = get_key_from_value(&self.item_types, &item_type);

        match item_type_number_found {
            Some(_) => Err(DomainError::InventoryError("the item type already exists".to_string())),
            None => {
                Ok(item_type_number)
            },
        }
        
    }
}

#[cfg(test)]
mod tests {
    use crate::{inventory::item_type::ItemTypeNumber, Dummy};

    use super::*;

    impl Dummy for ItemType {
        fn dummy() -> Self
        where
            Self: Sized,
        {
            Self {
               number: ItemTypeNumber::dummy()
            }
        }
    }

    impl Dummy for ItemTypeNumber {
        fn dummy() -> Self
        where
            Self: Sized,
        {
            Self::default()
        }
    }

    #[test]
    fn given_item_type_when_verify_item_type_is_new_then_return_item_type_number() {
        let that_item_type: ItemType = ItemType::dummy();
        let expected_item_type_number: ItemTypeNumber = that_item_type.get_number();
        let mut inventory = Inventory::new(HashMap::new());

        let actual_item_type_number: ItemTypeNumber = inventory.verify_item_type_is_new(that_item_type).unwrap();

        assert_eq!(expected_item_type_number, actual_item_type_number);
    }    
    
    #[test]
    fn given_already_existing_item_type_when_verify_item_type_is_new_then_reject_it() {
        let that_item_type: ItemType = ItemType::dummy();
        let mut item_types = HashMap::new();
        item_types.insert(that_item_type.get_number(), that_item_type);
        let mut inventory = Inventory::new(item_types);

        let actual_result = inventory.verify_item_type_is_new(that_item_type);

        assert!(matches!(
            actual_result,
            Err(DomainError::InventoryError(_))
        ));
    }
}