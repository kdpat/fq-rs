use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Error, Pool, Sqlite};

pub type UserId = i64;

pub const DEFAULT_USERNAME: &str = "user";

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

pub async fn create_user(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(DEFAULT_USERNAME)
        .execute(pool)
        .await
}

pub async fn fetch_user(pool: &Pool<Sqlite>, id: UserId) -> Result<User, Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}
