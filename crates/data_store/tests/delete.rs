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

#[tokio::test]
async fn test_delete_batch() -> anyhow::Result<()> {
    use sqlx::{PgPool, Postgres, QueryBuilder};
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;
    let user_id_list = vec![2, 3];

    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("DELETE FROM users WHERE id IN (");

    let mut separated = query_builder.separated(", ");
    for user_id in &user_id_list {
        separated.push_bind(user_id);
    }
    separated.push_unseparated(")");

    let query = query_builder.build();
    query.execute(&pool).await?;

    Ok(())
}
