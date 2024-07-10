use crate:: ParsedData;
use sqlx::PgPool;
use std::error::Error;

pub async fn add_places_db(pool: &PgPool, parsed_data: Vec<ParsedData>) -> Result<(), Box<dyn Error>> {
    let mut i = 0;
    for parsed_data in parsed_data {
        let exists: Option<bool> = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM Places WHERE place_name = $1)",
            parsed_data.place
        )
        .fetch_one(pool)
        .await?
        .exists;

        if !exists.unwrap_or(false) {
            sqlx::query!(
                "INSERT INTO Places (place_name, place_type) VALUES ($1, $2)",
                parsed_data.place,
                parsed_data.emp,
            )
            .execute(pool)
            .await?;
            i += 1;
        }
    }
    println!("nb of places added: {} ", i);
    Ok(())
}

pub async fn add_items_db(pool: &PgPool, parsed_data: Vec<ParsedData>) -> Result<(), Box<dyn Error>> {
    let mut i = 0;
    for parsed_data in parsed_data {
        let exists: Option<bool> = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM Items WHERE item_name = $1)",
            parsed_data.obj
        )
        .fetch_one(pool)
        .await?
        .exists;

        if !exists.unwrap_or(false) {
            sqlx::query!(
                "INSERT INTO items (item_name) VALUES ($1)",
                parsed_data.obj,
            )
            .execute(pool)
            .await?;
            i += 1;
        }
    }
    println!("nb of items added: {} ", i);
    Ok(())
}
pub async fn add_inventory_db(pool: &PgPool, parsed_data: Vec<ParsedData>) -> Result<(), Box<dyn Error>> {
    let mut i = 0;
    let absent = -1;

    for parsed_data in parsed_data {

        if parsed_data.qte == 0 {
            continue;
        }
        let place_id: Option<i32> = sqlx::query!(
            "SELECT place_id FROM Places WHERE place_name = $1",
            parsed_data.place
        )
        .fetch_optional(pool)
        .await?
        .map(|record| record.place_id);

        let item_id: Option<i32> = sqlx::query!(
            "SELECT item_id FROM Items WHERE item_name = $1",
            parsed_data.obj,
        )
        .fetch_optional(pool)
        .await?
        .map(|record| record.item_id);

        let inventory_item_id: Option<bool> = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM Inventory WHERE item_id = $1)",
            item_id,
        )        
        .fetch_one(pool)
        .await?
        .exists;

        let inventory_place_id: Option<bool> = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM Inventory WHERE place_id = $1)",
            place_id.unwrap_or(absent),
        )        
        .fetch_one(pool)
        .await?
        .exists;

        if (place_id.unwrap_or(absent) != absent && item_id.unwrap_or(absent) != absent) && (!inventory_item_id.unwrap_or(false) || !inventory_place_id.unwrap_or(false)) {
            sqlx::query!(
                "INSERT INTO Inventory (place_id, item_id, nb_of_items) VALUES ($1, $2, $3)",
                place_id,
                item_id,
                parsed_data.qte,
            )
            .execute(pool)
            .await?;
            i += 1;
        }
    }
    println!("nb of entries added to inventory: {} ", i);
    Ok(())
}
