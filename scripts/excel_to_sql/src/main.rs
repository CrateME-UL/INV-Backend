use dotenv::dotenv;
use excel_to_sql::{parse_records_from_excel, add_places_db, add_items_db, add_inventory_db};
use sqlx::postgres::PgPoolOptions;
use std::env;


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
            if let Err(e) = add_items_db(&pool, data.clone()).await {
                eprintln!("Error adding items to the database: {}", e);
            }            
            if let Err(e) = add_inventory_db(&pool, data).await {
                eprintln!("Error adding inventory entries to the database: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error parsing data: {}", e);
        }
    }
}

