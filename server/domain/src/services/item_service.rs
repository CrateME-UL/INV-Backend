use std::sync::Arc;

use crate::{models::domain_error::DomainError, ports::item_ports::ItemRepository, Item, Place};

#[derive(Clone)]
pub struct ItemService {
    inventory_repository: Arc<dyn ItemRepository>,
}

impl ItemService {
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
            _ => Err(DomainError::InventoryError(
                "Unhandled error while fetching the item with inventory from repository."
                    .to_string(),
            )),
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
            _ => Err(DomainError::InventoryError(
                "Unhandled error while fetching the place with inventory from repository."
                    .to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{item::Item, place::Place},
        ItemNo,
    };

    const VALID_ID_NUMBER: i32 = 42;

    trait StubItemNo {
        fn stub(number: i32) -> ItemNo;
    }

    impl StubItemNo for ItemNo {
        fn stub(number: i32) -> ItemNo {
            ItemNo { number }
        }
    }

    trait StubItem {
        fn stub(number: ItemNo, name: &str) -> Self;
    }

    impl StubItem for Item {
        fn stub(number: ItemNo, name: &str) -> Self {
            Self {
                number,
                name: name.trim().to_string(),
            }
        }
    }

    pub struct MockItemRepository {
        pub stub_item: Option<Item>,
        pub stub_place: Option<Place>,
    }

    impl MockItemRepository {
        fn mock_with_item(stub_data: &Option<Item>) -> Self {
            Self {
                stub_item: stub_data.clone(),
                stub_place: None,
            }
        }
        fn mock_with_place(stub_data: &Option<Place>) -> Self {
            Self {
                stub_item: None,
                stub_place: stub_data.clone(),
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
            let result = self.stub_item.clone();
            Box::pin(async move { Ok(result) })
        }

        fn fetch_place_by_name(
            &self,
            _place_name: String,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<Option<Place>, Box<dyn std::error::Error>>>
                    + Send,
            >,
        > {
            let result = self.stub_place.clone();
            Box::pin(async move { Ok(result) })
        }
        
        fn store_item(
            &self,
            item: Item,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ItemNo, Box<dyn std::error::Error>>> + Send>> {
            todo!()
        }

    
    }

    #[tokio::test]
    async fn given_existing_item_name_when_fetching_item_by_name_then_return_corresponding_item() {
        const EXISTING_ITEM_NAME: &str = "Bob";
        let valid_id: ItemNo = ItemNo::stub(VALID_ID_NUMBER);
        let expected_id: ItemNo = ItemNo::stub(VALID_ID_NUMBER);
        let valid_item: Item = Item::stub(valid_id, EXISTING_ITEM_NAME);
        let inventory: ItemService = ItemService::new(Arc::new(
            MockItemRepository::mock_with_item(&Option::Some(valid_item)),
        ));

        let actual_id = inventory
            .fetch_item_by_name(EXISTING_ITEM_NAME.to_string())
            .await
            .unwrap()
            .get_number();

        assert_eq!(expected_id, actual_id);
    }

    #[tokio::test]
    async fn given_not_existing_item_name_when_fetching_item_by_name_then_reject_it() {
        const NOT_EXISTING_ITEM_NAME: &str = "Bob";
        let not_existing_item = None;
        let inventory: ItemService = ItemService::new(Arc::new(
            MockItemRepository::mock_with_item(&not_existing_item),
        ));

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
        let inventory: ItemService = ItemService::new(Arc::new(
            MockItemRepository::mock_with_place(&not_existing_place),
        ));

        assert!(matches!(
            inventory
                .fetch_place_by_name(NOT_EXISTING_PLACE_NAME.to_string())
                .await,
            Err(DomainError::InventoryError(_))
        ));
    }
}
