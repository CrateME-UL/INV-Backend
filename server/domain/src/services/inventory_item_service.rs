// use crate::{InventoryItem, InventoryItemFetchable};
// use std::{error::Error, sync::Arc};

// #[derive(Clone)]
// pub struct InventoryItemService {
//     inventory_item_service_repository: Option<Arc<dyn InventoryItemFetchable + Send + Sync>>,
// }

// impl InventoryItemService {
//     pub fn new(repository: Arc<dyn InventoryItemFetchable + Send + Sync>) -> Self {
//         Self {
//             inventory_item_service_repository: Some(repository),
//         }
//     }
//     pub async fn add_inventory_items(
//         &self,
//         inventory_item: InventoryItem,
//     ) -> Result<InventoryItem, Box<dyn Error>> {
//         self.inventory_item_service_repository
//             .as_ref()
//             .expect("Repository not initialized")
//             .add_inventory_items(inventory_item)
//             .await
//     }
// }

// // pub async fn fetch_inventory_items(
// //     &self,
// //     inventory_item: InventoryItem,
// // ) -> Result<Vec<InventoryItem>, Box<dyn Error>> {
// //     self.inventory_item_service_repository
// //         .as_ref()
// //         .expect("Repository not initialized")
// //         .fetch_inventory_items(inventory_item)
// //         .await
// // }

// // impl fmt::Debug for InventoryItemService {
// //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// //         f.debug_struct("InventoryItemService")
// //             .field("inventory_item_service_repository", &"...") // Hide or customize the field display
// //             .finish()
// //     }
// // }
