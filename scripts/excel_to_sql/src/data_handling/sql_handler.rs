use crate:: ParsedData;
use sqlx::PgPool;
use std::error::Error;

pub async fn add_places_db(pool: &PgPool, parsed_data: Vec<ParsedData>) -> Result<(), Box<dyn Error>> {
    let mut i = 0;
    for parsed_data in parsed_data {
        let exists: Option<bool> = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM Places WHERE placeName = $1)",
            parsed_data.place
        )
        .fetch_one(pool)
        .await?
        .exists;

        if exists.unwrap_or(false) != true {
            sqlx::query!(
                "INSERT INTO Places (placeName, placeType) VALUES ($1, $2)",
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
    let absent = -1;
    for parsed_data in parsed_data {
        let place_id: Option<i32> = sqlx::query!(
            "SELECT placeId FROM Places WHERE placeName = $1",
            parsed_data.place
        )
        .fetch_optional(pool)
        .await?
        .map(|record| record.placeid);

        let item_id: Option<i32> = sqlx::query!(
            "SELECT itemId FROM Items WHERE itemName = $1 AND placeId = $2",
            parsed_data.obj,
            place_id.unwrap_or(absent),
        )
        .fetch_optional(pool)
        .await?
        .map(|record| record.itemid);

        if place_id.unwrap_or(absent) != absent && item_id.unwrap_or(absent) == absent {
            sqlx::query!(
                "INSERT INTO Items (placeId, nbOfItems, itemName) VALUES ($1, $2, $3)",
                place_id,
                parsed_data.qte,
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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
