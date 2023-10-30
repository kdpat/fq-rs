use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Error, Pool, Sqlite};

pub type UserId = i64;

const DEFAULT_USERNAME: &str = "user";

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: UserId,
    pub session_id: String,
    pub name: String,
}

pub async fn create_user(
    pool: &Pool<Sqlite>,
    session_id: &str,
) -> Result<SqliteQueryResult, Error> {
    sqlx::query("INSERT INTO users (session_id, name) VALUES (?, ?)")
        .bind(session_id)
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

pub async fn fetch_user_by_session_id(
    pool: &Pool<Sqlite>,
    session_id: &str,
) -> Result<User, Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE session_id = ?")
        .bind(session_id)
        .fetch_one(pool)
        .await
}
