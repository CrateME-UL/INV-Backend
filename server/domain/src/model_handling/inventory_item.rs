// use crate::{InventoryItem, InventoryItemRequest};

// impl AddInventoryItem for InventoryItem {
//     async fn add_inventory_item<I>(
//         inventory_item: InventoryItemRequest,
//         repo: &I,
//     ) -> Result<InventoryItem, Box<dyn std::error::Error>>
//     where
//         I: InventoryRepository,
//     {
//         let absent = -1;

//         let place_id = repo.find_place_id(&inventory_item.place_name).await?;
//         let item_id = repo.find_item_id(&inventory_item.item_name).await?;

//         if let (Some(place_id), Some(item_id)) = (place_id, item_id) {
//             let inventory_exists = repo.inventory_exists(place_id, item_id).await?;

//             if !inventory_exists {
//                 repo.add_inventory(place_id, item_id, inventory_item.nb_of_items)
//                     .await?;
//             } else {
//                 repo.update_inventory(place_id, item_id, inventory_item.nb_of_items)
//                     .await?;
//             }

//             Ok(InventoryItem {
//                 item_id,
//                 item_name: inventory_item.item_name,
//                 nb_of_items: inventory_item.nb_of_items,
//             })
//         } else {
//             Err("Place or Item not found".into())
//         }
//     }
// }
