use std::sync::Arc;

use crate::{models::domain_error::DomainError, ports::item_ports::ItemRepository, Item, ItemNo};

#[derive(Clone)]
pub struct ItemService {
    item_repository: Arc<dyn ItemRepository>,
}

impl ItemService {
    pub fn new(item_repository: Arc<dyn ItemRepository>) -> Self {
        Self { item_repository }
    }

    pub async fn fetch_item_by_name(&self, item_name: String) -> Result<Item, DomainError> {
        match self.item_repository.fetch_item_by_name(item_name).await {
            Ok(item) => match item {
                Some(_) => Ok(item.unwrap()),
                None => Err(DomainError::ItemError("Item not found.".to_string())),
            },
            _ => Err(DomainError::ItemError(
                "Unhandled error while fetching the item with inventory from repository."
                    .to_string(),
            )),
        }
    }

    pub async fn store_item(&self, item: Item) -> Result<ItemNo, DomainError> {
        //TODO: fix TDA here, let the repository perform the storage logic
        match self
            .item_repository
            .fetch_item_by_name(item.clone().name)
            .await
        {
            Ok(item_obtained) => match item_obtained {
                Some(_) => Err(DomainError::ItemError(
                    "Item already exists, cannot store duplicate items.".to_string(),
                )),
                _ => match self.item_repository.store_item(item).await {
                    Ok(item_no) => Ok(item_no),
                    _ => Err(DomainError::ItemError(
                        "Unhandled error while storing the item with inventory from repository."
                            .to_string(),
                    )),
                },
            },
            _ => Err(DomainError::ItemError(
                "Unhandled error while storing the item in the repository.".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::item::Item, ItemNo};

    const ANY_ITEM_NUMBER: i32 = 42;

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
    }

    impl MockItemRepository {
        fn mock_with_item(stub_data: &Option<Item>) -> Self {
            Self {
                stub_item: stub_data.clone(),
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

        fn store_item(
            &self,
            item: Item,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<ItemNo, Box<dyn std::error::Error>>> + Send,
            >,
        > {
            todo!()
        }
    }

    #[tokio::test]
    async fn given_existing_item_name_when_fetching_item_by_name_then_return_corresponding_item() {
        const EXISTING_ITEM_NAME: &str = "Bob";
        let valid_id: ItemNo = ItemNo::stub(ANY_ITEM_NUMBER);
        let expected_id: ItemNo = ItemNo::stub(ANY_ITEM_NUMBER);
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
            Err(DomainError::ItemError(_))
        ));
    }

    #[tokio::test]
    async fn given_duplicate_item_when_storing_duplicate_item_then_reject_it() {
        const ANY_ITEM_NAME: &str = "Bob's hammer";
        let any_item_no: ItemNo = ItemNo::stub(ANY_ITEM_NUMBER);
        let any_item: Item = Item::stub(any_item_no, ANY_ITEM_NAME);
        let item_service: ItemService = ItemService::new(Arc::new(
            MockItemRepository::mock_with_item(&Option::Some(any_item.clone())),
        ));

        assert!(matches!(
            item_service
                .store_item(any_item.clone())
                .await,
            Err(DomainError::ItemError(_))
        ));
    }
}
