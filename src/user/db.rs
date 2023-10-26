use crate::user;

use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Error, Pool, Sqlite};

const CREATE_USERS_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS users (
       id INTEGER PRIMARY KEY,
       name TEXT NOT NULL
);";

pub async fn ensure_users_table(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    sqlx::query(CREATE_USERS_TABLE_SQL).execute(pool).await
}

pub async fn create_user(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(user::DEFAULT_USERNAME)
        .execute(pool)
        .await
}

pub async fn fetch_user(pool: &Pool<Sqlite>, id: i64) -> Result<user::User, Error> {
    sqlx::query_as::<_, user::User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}
