use dotenv::dotenv;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

static POOL: Lazy<PgPool> = Lazy::new(|| {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database_url)
        .expect("Failed to create pool")
});

pub fn get_db_pool() -> &'static PgPool {
    &POOL
}
