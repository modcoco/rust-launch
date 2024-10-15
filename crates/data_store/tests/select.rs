use chrono::NaiveDateTime;
use data_store::GetFieldNames;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool};

// execute() 用于执行插入、更新、删除等不返回数据的 SQL 语句
// fetch_one(): 获取一条记录
// fetch_optional(): 获取零或一条记录
// fetch_all(): 获取多条记录
// fetch() 用于执行查询并返回 Stream，可以逐条获取记录

#[tokio::test]
async fn test_query() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    let user: Vec<User> = sqlx::query_as("SELECT id, username, email, created_at FROM users")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", user);

    Ok(())
}

#[tokio::test]
async fn test_query_02() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    let test = query(
        r#"
SELECT id, username, email, created_at FROM users
"#,
    )
    // .bind(s)
    .execute(&pool)
    .await?;

    println!("{:?}", test);

    Ok(())
}

#[tokio::test]
async fn test_query_03() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    let test: Vec<User> = query_as(
        r#"
SELECT id, username, email, created_at FROM users
"#,
    )
    .fetch_all(&pool)
    .await?;

    println!("{:?}", test);

    Ok(())
}

#[derive(Debug, Default, Serialize, Deserialize, FromRow, GetFieldNames)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}
