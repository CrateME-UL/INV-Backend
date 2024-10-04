use std::sync::Arc;

use crate::{ports::item_ports::ItemRepository, Item, ItemId};

use super::domain_error::DomainError;

pub struct Inventory {
    inventory_repository: Arc<dyn ItemRepository>,
}

impl Inventory {
    pub fn new(inventory_repository: Arc<dyn ItemRepository>) -> Self {
        Self {
            inventory_repository,
        }
    }

    pub async fn get_item_by_id(&self, item_id: ItemId) -> Result<Item, DomainError> {
        match self
            .inventory_repository
            .fetch_item_by_id(item_id.m_id)
            .await
        {
            Ok(item) => Ok(item.unwrap()),
            _ => Err(DomainError::ItemError("Item not found".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{models::item::Item, ItemId};

    const VALID_ID_NUMBER: i32 = 42;

    trait MockItemId {
        fn mock(id: i32) -> ItemId;
    }

    impl MockItemId for ItemId {
        fn mock(id: i32) -> ItemId {
            ItemId { m_id: id }
        }
    }

    trait MockItem {
        fn mock(id: ItemId, name: &str) -> Self;
    }

    impl MockItem for Item {
        fn mock(id: ItemId, name: &str) -> Self {
            Self {
                m_id: id,
                m_name: name.trim().to_string(),
            }
        }
    }

    pub struct MockItemRepository {
        pub mock_data: Option<Item>,
    }

    impl MockItemRepository {
        fn new(mock_data: &Option<Item>) -> Self {
            Self {
                mock_data: mock_data.clone(),
            }
        }
    }

    impl ItemRepository for MockItemRepository {
        fn fetch_item_by_id(
            &self,
            _id: i32,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<Option<Item>, Box<dyn std::error::Error>>>
                    + Send,
            >,
        > {
            let result = self.mock_data.clone();

            Box::pin(async move { Ok(result) })
        }
    }

    #[tokio::test]
    async fn given_id_when_fetching_item_by_id_should_return_item() {
        const TAKEN_NAME: &str = "Bob";
        let valid_id: ItemId = ItemId::mock(VALID_ID_NUMBER);
        let expected_id: ItemId = ItemId::mock(VALID_ID_NUMBER);
        let valid_item: Item = Item::mock(valid_id, TAKEN_NAME);
        let inventory: Inventory =
            Inventory::new(Arc::new(MockItemRepository::new(&Option::Some(valid_item))));

        let actual_id = inventory
            .get_item_by_id(expected_id.to_owned())
            .await
            .unwrap()
            .get_id();

        assert_eq!(expected_id, actual_id);
    }
}
