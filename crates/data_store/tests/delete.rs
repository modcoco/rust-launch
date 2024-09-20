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
async fn test_delete_01() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    let user_id = 1;
    let _ = query!(r#"DELETE FROM users WHERE id = $1"#, user_id)
        .execute(&pool)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_delete_02() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    let user_id = 2;
    query("delete from users where id = $1")
        .bind(user_id)
        .execute(&pool)
        .await?;

    Ok(())
}
