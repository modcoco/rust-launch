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

#[tokio::test]
async fn test_update_01() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    let id = 4;
    let rows_affected = sqlx::query!(
        r#"
UPDATE users
SET email = 'test@gmail.com'
WHERE id = $1
        "#,
        id
    )
    .execute(&pool)
    .await?
    .rows_affected();
    println!("{}", rows_affected);

    Ok(())
}

#[tokio::test]
async fn test_update_02() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    let id = 4;
    let _ = query(
        r#"
UPDATE users
SET email = 'test@gmail.com'
WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&pool)
    .await?;

    Ok(())
}
