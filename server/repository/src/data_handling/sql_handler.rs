use crate::get_db_pool;
use crate::FetchItems;
use axum::extract::Query;
use domain::{
    InventoryItem, InventoryItemQuery, InventoryPlace, InventoryPlaceQuery, Item, ItemListDb, Place,
};

impl FetchItems for ItemListDb {
    async fn fetch_items() -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let items = sqlx::query!("SELECT item_id, item_name FROM Items ORDER BY item_name;")
            .fetch_all(get_db_pool())
            .await?
            .into_iter()
            .map(|record| Item {
                item_id: record.item_id,
                item_name: record.item_name,
            })
            .collect();
        Ok(items)
    }
}

pub async fn get_places_db() -> Result<Vec<Place>, Box<dyn std::error::Error>> {
    let places =
        sqlx::query!("SELECT place_id, place_name, place_type FROM Places ORDER BY place_name;")
            .fetch_all(get_db_pool())
            .await?
            .into_iter()
            .map(|record| Place {
                place_id: record.place_id,
                place_name: record.place_name,
                place_type: record.place_type,
            })
            .collect();

    Ok(places)
}

pub async fn get_inventory_items_db(
    query: &Query<InventoryItemQuery>,
) -> Result<Vec<InventoryItem>, Box<dyn std::error::Error>> {
    let default = "";
    let is_query_empty = query.place_name.as_deref().unwrap_or(default) == default;
    let items = match is_query_empty {
        true => sqlx::query!(
            "SELECT Items.item_id, Items.item_name, SUM (Inventory.nb_of_items) AS nb_of_items
                    FROM Inventory
                    JOIN Places ON Inventory.place_id = Places.place_id
                    JOIN Items ON Inventory.item_id = Items.item_id
                GROUP BY Items.item_id, Items.item_name 
                ORDER BY nb_of_items DESC, Items.item_name;"
        )
        .fetch_all(get_db_pool())
        .await?
        .into_iter()
        .map(|record| InventoryItem {
            item_id: record.item_id,
            item_name: record.item_name,
            nb_of_items: record.nb_of_items.unwrap_or(0) as i32,
        })
        .collect(),
        _ => sqlx::query!(
            "SELECT Items.item_id, Items.item_name, Inventory.nb_of_items
                    FROM Inventory
                    JOIN Places ON Inventory.place_id = Places.place_id
                    JOIN Items ON Inventory.item_id = Items.item_id
                    WHERE (place_name =  $1 OR $1 = '') 
                    ORDER BY Inventory.nb_of_items DESC;",
            query.place_name.as_deref().unwrap_or(default),
        )
        .fetch_all(get_db_pool())
        .await?
        .into_iter()
        .map(|record| InventoryItem {
            item_id: record.item_id,
            item_name: record.item_name,
            nb_of_items: record.nb_of_items,
        })
        .collect(),
    };

    Ok(items)
}

pub async fn get_inventory_places_db(
    query: &Query<InventoryPlaceQuery>,
) -> Result<Vec<InventoryPlace>, Box<dyn std::error::Error>> {
    let default = "";
    let is_query_empty = query.item_name.as_deref().unwrap_or(default) == default;
    let places = match is_query_empty {
        true => {
            sqlx::query!(
                "SELECT Places.place_id, Places.place_name, Places.place_type, SUM(Inventory.nb_of_items) as nb_of_items
                    FROM Inventory
                    JOIN Places ON Inventory.place_id = Places.place_id
                    JOIN Items ON Inventory.item_id = Items.item_id
                    WHERE (place_type = ANY($1::text[]) OR $1 = '{}')
                GROUP BY Places.place_id, Places.place_name, Places.place_type
                ORDER BY nb_of_items DESC, Places.place_name;",
                &query
                .place_type
                .as_deref()
                .unwrap_or(default)
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
            )
            .fetch_all(get_db_pool())
            .await?
            .into_iter()
            .map(|record| InventoryPlace {
                place_id: record.place_id,
                place_name: record.place_name,
                place_type: record.place_type,
                nb_of_items: record.nb_of_items.unwrap_or(0) as i32,
            })
            .collect()
        },
        _ => {
            sqlx::query!(
                "SELECT Places.place_id as place_id, Places.place_name as place_name, Places.place_type as place_type, Inventory.nb_of_items as nb_of_items
                    FROM Inventory
                    JOIN Places ON Inventory.place_id = Places.place_id
                    JOIN Items ON Inventory.item_id = Items.item_id
                    WHERE (item_name =  $1 OR $1 = '') 
                        AND (place_type = ANY($2::text[]) OR $2 = '{}')
                    ORDER BY Inventory.nb_of_items DESC;",
                query.item_name.as_deref().unwrap_or(default),
                &query
                .place_type
                .as_deref()
                .unwrap_or(default)
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
            )
            .fetch_all(get_db_pool())
            .await?
            .into_iter()
            .map(|record| InventoryPlace {
                place_id: record.place_id,
                place_name: record.place_name,
                place_type: record.place_type,
                nb_of_items: record.nb_of_items,
            })
            .collect()
        }
    };

    Ok(places)
}
