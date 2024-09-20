use chrono::NaiveDateTime;
use data_store::GetFieldNames;
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow, PgPool};

#[derive(Debug, Default, Serialize, Deserialize, FromRow, GetFieldNames)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

async fn insert_and_verify(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    query!(
        r#"INSERT INTO users (username, email)
        VALUES ( $1, $2 )
        "#,
        "test",
        "test todo"
    )
    .execute(&mut **transaction)
    .await?;

    let _ = query!(r#"SELECT FROM users WHERE id = $1"#, id)
        .fetch_one(&mut **transaction)
        .await?;

    Ok(())
}

async fn explicit_rollback_example(
    pool: &sqlx::PgPool,
    test_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool.begin().await?;

    insert_and_verify(&mut transaction, test_id).await?;

    transaction.rollback().await?;

    Ok(())
}

#[tokio::test]
async fn test_transaction() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;
    let _ = explicit_rollback_example(&pool, 1).await;
    Ok(())
}
