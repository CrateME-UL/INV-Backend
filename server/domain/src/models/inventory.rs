// use std::sync::Arc;

// use crate::ports::item_ports::ItemRepository;

// use crate::ItemId;

// use mockall::*;
// use mockall::predicate::*;

use std::sync::Arc;

use crate::ports::item_ports::ItemRepository;


pub struct Inventory {
    inventory_repository: Arc<dyn ItemRepository>
}

impl Inventory {
    pub fn new(inventory_repository: Arc<dyn ItemRepository>) -> Self {
        Self { inventory_repository }
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::{domain_error::DomainError, item::{self, Item}}, ItemId};

    use super::*;

    const VALID_ID_NUMBER: i32 = 42;
    const THE_BUILDER: &str = "Bob";

    trait MockItemId {
        fn new_valid(id: i32) -> ItemId;
    }
    
    impl MockItemId for ItemId {
        fn new_valid(id: i32) -> ItemId {
            ItemId { m_id: id }
        }
    }

    trait MockItem<'a> {
        fn mock(id: &'a ItemId, name: &String, is_found: MockFound) -> Result<Self, DomainError> where Self: Sized;
    }
    
    impl<'a> MockItem<'a> for Item<'a> {
        fn mock(id: &'a item::ItemId, name: &String, is_found: MockFound) -> Result<Self, DomainError> {
            let item = match is_found {
            MockFound::Found => Ok(Self {
                    m_id: &id,
                    m_name: name.trim().to_string(),
                }),
                _ => Err(DomainError::ItemError("Item not found".to_string()))
            };
            item

        }
    }

    pub enum MockFound{
        NotFound,
        Found
    }

    trait MockItemRepository {
        fn fetch_item_by_name(&self, name: &str, is_found: MockFound) -> Option<Item>;
    } 

    impl MockItemRepository for dyn ItemRepository {
        fn fetch_item_by_name(&self, name: &str, is_found: MockFound) -> Option<Item> {
            match Item::mock(&ItemId::new_valid(VALID_ID_NUMBER), &name.to_string(), MockFound::Found) {
                Ok(_) => todo!(),
                Err(_) => todo!(),
                // MockFound::Found => {
                //     // Create a new item to return when found
                //     match Item::mock(&ItemId::new_valid(VALID_ID_NUMBER), &name.to_string(), MockFound::Found) {
                //         Ok(item) => Some(item),
                //         Err(_) => None, 
                //     }
                // },
                // MockFound::NotFound => None,  // Return None if not found
            }
        }
    }
    

    // struct {
    //     expected_name: String,
    //     expected_item: Result<Item, DomainError>,
    // }

    // impl ItemRepository for MockItemRepository {
    //     fn fetch_item_by_name(&self, name: &str) -> Result<Item, DomainError> {
    //         if name == self.expected_name {
    //             self.expected_item.clone()
    //         } else {
    //             Err(DomainError::ItemError("Item not found".to_string()))
    //         }
    //     }
    // }
    
    #[test]
    fn given_name_when_fetching_item_by_name_should_return_item() {
        const TAKEN_NAME: &str = "Bob";
        let valid_id: ItemId = ItemId::new_valid(VALID_ID_NUMBER);

        assert!(matches!(
            Item::new(&valid_id, &TAKEN_NAME.to_string()),
            Err(DomainError::ItemError(_))
        ));


    }}






