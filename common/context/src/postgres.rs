use common::sqlx;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pg_pool() -> PgPool {
    let database_url = "postgres://user:password@localhost/dbname";
    PgPoolOptions::new()
        .max_connections(15)
        .min_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await
        .expect("Failed to create PostgreSQL pool")
}
