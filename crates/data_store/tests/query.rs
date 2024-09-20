use chrono::NaiveDateTime;
use data_store::{generate_push_binds, GetFieldNames};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool, Postgres, QueryBuilder};

const BIND_LIMIT: usize = 65535;

#[tokio::test]
async fn test_query() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;

    let user = sqlx::query_as!(User, "SELECT id, username, email, created_at FROM users",)
        .fetch_all(&pool)
        .await?;

    println!("{:?}", user);

    Ok(())
}

// execute() 用于执行插入、更新、删除等不返回数据的 SQL 语句
// fetch_one(): 获取一条记录
// fetch_optional(): 获取零或一条记录
// fetch_all(): 获取多条记录
// fetch() 用于执行查询并返回 Stream，可以逐条获取记录

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

async fn add_user(pool: &PgPool, user: User) -> anyhow::Result<i32> {
    let rec = sqlx::query!(
        r#"
INSERT INTO users (username, email)
VALUES ($1, $2)
RETURNING id
        "#,
        user.username,
        user.email
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

async fn insert_users_build(
    pool: &PgPool,
    users: impl Iterator<Item = User>,
) -> Result<(), sqlx::Error> {
    let field_names: Vec<&str> = User::field_names()
        .iter()
        .filter(|&&field| field != "id")
        .copied()
        .collect();

    let mut query_builder =
        QueryBuilder::new(format!("INSERT INTO users ({}) ", field_names.join(", ")));

    query_builder.push_values(users.take(BIND_LIMIT / 4), |mut b, user| {
        // generate_push_binds!(b, user, [username, email, created_at]);
        generate_push_binds!([b, user, username, email, created_at]);
    });

    let query = query_builder.build();
    let _result = query.execute(pool).await?;

    Ok(())
}

#[tokio::test]
async fn test_add_user() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;
    let user = User {
        username: "Licke".to_string(),
        email: "test2".to_string(),
        ..Default::default()
    };
    add_user(&pool, user).await?;
    Ok(())
}

#[tokio::test]
async fn test_add_users() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?;
    let users = (0..10).map(|i| User {
        username: format!("test_user_{i}"),
        email: format!("test-user-{i}@example.com"),
        created_at: NaiveDateTime::parse_from_str("2024-09-19 12:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap_or_default(),
        ..Default::default()
    });

    insert_users_build(&pool, users).await?;
    Ok(())
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

#[test]
fn test_push_bind() {
    let mut qb: QueryBuilder<'_, Postgres> = QueryBuilder::new("SELECT * FROM users WHERE id = ");

    qb.push_bind(42i32)
        .push(" OR membership_level = ")
        .push_bind(3i32);

    println!("{}", qb.sql());
    assert_eq!(
        qb.sql(),
        "SELECT * FROM users WHERE id = $1 OR membership_level = $2"
    );
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
