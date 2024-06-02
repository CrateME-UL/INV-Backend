mod excel_reader;

use dotenv::dotenv;
use excel_reader::read_excel;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Data {
    place: String,
    obj: String,
    qte: String,
    emp: String,
}

#[derive(Debug, Clone)]
struct ParsedData {
    place: String,
    obj: String,
    qte: i32,
    emp: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    // get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let parsed_data = parse_records_from_excel("map_inventaire.xlsx");
    match parsed_data {
        Ok(data) => {
            if let Err(e) = add_places_db(&pool, data.clone()).await {
                eprintln!("Error adding places to the database: {}", e);
            }
            if let Err(e) = add_items_db(&pool, data).await {
                eprintln!("Error adding items to the database: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error parsing data: {}", e);
        }
    }
}

fn parse_records_from_excel(file_path: &str) -> Result<Vec<ParsedData>, Box<dyn Error>> {
    let records = read_excel::<Data>(&file_path)?;
    let max_name_len: usize = env::var("NAME_MAX_LEN")
        .expect("NAME_MAX_LEN must be set")
        .parse()
        .expect("NAME_MAX_LEN must be a valid integer");
    let mut parsed_data: Vec<ParsedData> = Vec::new();
    let parse_error_status = -1;
    let mut i = 0;
    for record in records {
        let int_parse_value = match record.qte.parse::<i32>() {
            Ok(value) => value,
            Err(_) => parse_error_status,
        };

        if int_parse_value == parse_error_status
            || record.obj.len() > max_name_len
            || record.place.len() > max_name_len
            || record.emp.len() > 3
        {
            i += 1;
            println!("parsing error: adjust errors and restart after drop tables and recreate table. alternative: add manually {:#?}", record);
        } else {
            parsed_data.push(ParsedData {
                place: record.place,
                obj: record.obj,
                qte: int_parse_value,
                emp: record.emp,
            });
        }
    }
    println!("nb of parsing errors: {} ", i);

    Ok(parsed_data)
}

async fn add_places_db(pool: &PgPool, parsed_data: Vec<ParsedData>) -> Result<(), Box<dyn Error>> {
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

async fn add_items_db(pool: &PgPool, parsed_data: Vec<ParsedData>) -> Result<(), Box<dyn Error>> {
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
