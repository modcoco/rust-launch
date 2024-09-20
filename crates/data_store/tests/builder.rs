use chrono::NaiveDateTime;
use data_store::GetFieldNames;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, QueryBuilder};

#[derive(Debug, Default, Serialize, Deserialize, FromRow, GetFieldNames)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
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
