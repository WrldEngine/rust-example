use dotenv::dotenv;
use sqlx::{PgPool, PgPoolOptions};

pub struct DataBase;

impl DataBase {
    pub async fn create_pool() -> PgPool {
        dotenv().ok();
        let postgres_url: String = std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");

        PgPoolOptions::new()
            .max_connections(100000)
            .connect(&postgres_url)
            .await
            .expect("Error connecting to the database!")
    }

    pub async fn create_tables(&pool: PgPool) {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mnemonics (
                id SERIAL,
                hash VARCHAR(64) NOT NULL
            )"
        )
        .execute(&pool)
        .await;
    }
}