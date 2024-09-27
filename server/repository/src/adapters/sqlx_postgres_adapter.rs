use domain::{InventoryItem, InventoryItemFetchable};
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::error::Error;

static POOL: Lazy<PgPool> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database_url)
        .expect("Failed to create pool")
});

pub fn get_db_pool() -> &'static PgPool {
    &POOL
}

#[derive(Debug)]
pub struct SqlxPostgresRepository;

impl InventoryItemFetchable for SqlxPostgresRepository {
    fn add_inventory_items(
        &self,
        inventory_item: InventoryItem,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<InventoryItem, Box<dyn Error>>> + Send>,
    > {
        Box::pin(async move {
            let absent = -1;
            let place_id: Option<i32> = sqlx::query!(
                "SELECT place_id FROM places WHERE TRIM(place_name) = $1",
                inventory_item.place_name
            )
            .fetch_optional(get_db_pool())
            .await?
            .map(|record| record.place_id);

            let item_id: Option<i32> = sqlx::query!(
                "SELECT item_id FROM Items WHERE TRIM(item_name) = $1",
                inventory_item.item_name,
            )
            .fetch_optional(get_db_pool())
            .await?
            .map(|record| record.item_id);

            let inventory_item_exists: Option<bool> = sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM Inventory WHERE item_id = $1 AND place_id = $2)",
                item_id.unwrap_or(absent),
                place_id.unwrap_or(absent)
            )
            .fetch_one(get_db_pool())
            .await?;

            if inventory_item_exists.unwrap_or(false) {
                sqlx::query!(
                    "UPDATE Inventory SET nb_of_items = $3 WHERE place_id = $1 AND item_id = $2",
                    place_id,
                    item_id,
                    inventory_item.nb_of_items,
                )
                .execute(get_db_pool())
                .await?;
            } else if place_id.is_some() && item_id.is_some() {
                sqlx::query!(
                    "INSERT INTO Inventory (place_id, item_id, nb_of_items) VALUES ($1, $2, $3)",
                    place_id,
                    item_id,
                    inventory_item.nb_of_items,
                )
                .execute(get_db_pool())
                .await?;
            } else {
                return Err(Box::from(
                    "Place ID or Item ID is missing. Unable to add inventory item.",
                ));
            }

            Ok(inventory_item)
        })
    }

    fn fetch_inventory_items(
        &self,
        inventory_item: InventoryItem,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<InventoryItem>, Box<dyn Error>>> + Send>,
    > {
        todo!();
        // Box::pin(async move {
        //     let default = "";
        //     let is_query_empty = inventory_item.place_name == default;
        //     let items = match is_query_empty {
        //             true => sqlx::query!(
        //             "SELECT Items.item_id, Items.item_name, SUM (Inventory.nb_of_items) AS nb_of_items
        //                     FROM Inventory
        //                     JOIN Places ON Inventory.place_id = Places.place_id
        //                     JOIN Items ON Inventory.item_id = Items.item_id
        //                 GROUP BY Items.item_id, Items.item_name
        //                 ORDER BY nb_of_items DESC, Items.item_name;"
        //         )
        //             .fetch_all(get_db_pool())
        //             .await?
        //             .into_iter()
        //             .map(|record| InventoryItem {
        //                 item_id: record.item_id,
        //                 item_name: record.item_name,
        //                 place_name: record.place_name,
        //                 place_type: None,
        //                 nb_of_items: record.nb_of_items.unwrap_or(0) as i32,
        //             })
        //             .collect(),
        //             _ => sqlx::query!(
        //                 "SELECT Items.item_id, Items.item_name, Inventory.nb_of_items
        //                     FROM Inventory
        //                     JOIN Places ON Inventory.place_id = Places.place_id
        //                     JOIN Items ON Inventory.item_id = Items.item_id
        //                     WHERE (place_name =  $1 OR $1 = '')
        //                     ORDER BY Inventory.nb_of_items DESC;",
        //                 query.place_name.as_deref().unwrap_or(default),
        //             )
        //             .fetch_all(get_db_pool())
        //             .await?
        //             .into_iter()
        //             .map(|record| InventoryItem {
        //                 item_id: record.item_id,
        //                 item_name: record.item_name,
        //                 place_name: record.place_name,
        //                 place_type: None,
        //                 nb_of_items: record.nb_of_items,
        //             })
        //             .collect(),
        //         };

        //     Ok(items)
        // })
    }
}
