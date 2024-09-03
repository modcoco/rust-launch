use common::{sqlx, tracing};
use regex::Regex;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pg_pool() -> PgPool {
    let cfg = config::get_config();
    show_pg_connect(&cfg.database_url_postgres);
    PgPoolOptions::new()
        .max_connections(15)
        .min_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(&cfg.database_url_postgres)
        .await
        .expect("Failed to create PostgreSQL pool")
}

fn show_pg_connect(std_pg_url: &str) {
    let re = Regex::new(r"postgres://([^:]+):[^@]+@([^/]+)/([^?]+)")
        .expect("Invalid format, failed to make pg url regex");
    if let Some(captures) = re.captures(std_pg_url) {
        let username = &captures[1];
        let host = &captures[2];
        let dbname = &captures[3];
        let formatted_string = format!("Connect {} {} {}", host, username, dbname);
        tracing::info!("{}", formatted_string);
    } else {
        tracing::info!("Invalid connection string format");
    }
}
