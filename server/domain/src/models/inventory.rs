use std::sync::Arc;

use crate::{ports::item_ports::ItemRepository, Item, ItemId};

use super::{domain_error::DomainError, place::Place};

pub struct Inventory {
    inventory_repository: Arc<dyn ItemRepository>,
}

impl Inventory {
    pub fn new(inventory_repository: Arc<dyn ItemRepository>) -> Self {
        Self {
            inventory_repository,
        }
    }

    pub async fn fetch_item_by_name(&self, item_name: String) -> Result<Item, DomainError> {
        match self
            .inventory_repository
            .fetch_item_by_name(item_name)
            .await
        {
            Ok(item) => match item {
                Some(_) => Ok(item.unwrap()),
                None => Err(DomainError::InventoryError("Item not found.".to_string())),
            },
            _ => Err(DomainError::InventoryError("Unhandled error while fetching the item with inventory from repository.".to_string())),
        }
    }    
    
    pub async fn fetch_place_by_name(&self, item_name: String) -> Result<Place, DomainError> {
        match self
            .inventory_repository
            .fetch_place_by_name(item_name)
            .await
        {
            Ok(place) => match place {
                Some(_) => Ok(place.unwrap()),
                None => Err(DomainError::InventoryError("Place not found.".to_string())),
            },
            _ => Err(DomainError::InventoryError("Unhandled error while fetching the place with inventory from repository.".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::{item::Item, place::Place}, ItemId};

    const VALID_ID_NUMBER: i32 = 42;

    trait MockItemId {
        fn mock(id: i32) -> ItemId;
    }

    impl MockItemId for ItemId {
        fn mock(id: i32) -> ItemId {
            ItemId { id }
        }
    }

    trait MockItem {
        fn mock(id: ItemId, name: &str) -> Self;
    }

    impl MockItem for Item {
        fn mock(id: ItemId, name: &str) -> Self {
            Self {
                id,
                name: name.trim().to_string(),
            }
        }
    }

    pub struct MockItemRepository {
        pub mock_item: Option<Item>,
        pub mock_place: Option<Place>,
    }

    impl MockItemRepository {
        fn mock_with_item(mock_data: &Option<Item>) -> Self {
            Self {
                mock_item: mock_data.clone(),
                mock_place: None
            }
        }        
        fn mock_with_place(mock_data: &Option<Place>) -> Self {
            Self {
                mock_item: None,
                mock_place: mock_data.clone(),
            }
        }
    }

    impl ItemRepository for MockItemRepository {
        fn fetch_item_by_name(
            &self,
            _item_name: String,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<Option<Item>, Box<dyn std::error::Error>>>
                    + Send,
            >,
        > {
            let result = self.mock_item.clone();
            Box::pin(async move { Ok(result) })
        }
        
        fn fetch_place_by_name(
            &self,
            _place_name: String,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<Place>, Box<dyn std::error::Error>>> + Send>> {
            let result = self.mock_place.clone();
            Box::pin(async move { Ok(result) })
        }
    }

    #[tokio::test]
    async fn given_existing_item_name_when_fetching_item_by_name_then_return_corresponding_item() {
        const EXISTING_ITEM_NAME: &str = "Bob";
        let valid_id: ItemId = ItemId::mock(VALID_ID_NUMBER);
        let expected_id: ItemId = ItemId::mock(VALID_ID_NUMBER);
        let valid_item: Item = Item::mock(valid_id, EXISTING_ITEM_NAME);
        let inventory: Inventory =
            Inventory::new(Arc::new(MockItemRepository::mock_with_item(&Option::Some(valid_item))));

        let actual_id = inventory
            .fetch_item_by_name(EXISTING_ITEM_NAME.to_string())
            .await
            .unwrap()
            .get_id();

        assert_eq!(expected_id, actual_id);
    }    
    
    #[tokio::test]
    async fn given_not_existing_item_name_when_fetching_item_by_name_then_reject_it() {
        const NOT_EXISTING_ITEM_NAME: &str = "Bob";
        let not_existing_item = None;
        let inventory: Inventory =
            Inventory::new(Arc::new(MockItemRepository::mock_with_item(&not_existing_item)));

        assert!(matches!(
            inventory
            .fetch_item_by_name(NOT_EXISTING_ITEM_NAME.to_string())
            .await,
            Err(DomainError::InventoryError(_))
        ));
    }    
    
    #[tokio::test]
    async fn given_not_existing_place_name_when_fetching_place_by_name_then_reject_it() {
        const NOT_EXISTING_PLACE_NAME: &str = "Bob's Place";
        let not_existing_place = None;
        let inventory: Inventory =
            Inventory::new(Arc::new(MockItemRepository::mock_with_place(&not_existing_place)));

        assert!(matches!(
            inventory
            .fetch_place_by_name(NOT_EXISTING_PLACE_NAME.to_string())
            .await,
            Err(DomainError::InventoryError(_))
        ));
    }
}
