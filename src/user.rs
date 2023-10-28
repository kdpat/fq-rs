use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Error, Pool, Sqlite};
use tower_sessions::session::SessionId;

const DEFAULT_USERNAME: &str = "user";

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub session_id: String,
    pub name: String,
}

const CREATE_USERS_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS users (
       session_id TEXT PRIMARY KEY,
       name TEXT NOT NULL
);";

pub async fn ensure_users_table(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    sqlx::query(CREATE_USERS_TABLE_SQL).execute(pool).await
}

pub async fn create_user(pool: &Pool<Sqlite>, session_id: String) -> Result<SqliteQueryResult, Error> {
    sqlx::query("INSERT INTO users (session_id, name) VALUES (?, ?)")
        .bind(session_id)
        .bind(DEFAULT_USERNAME)
        .execute(pool)
        .await
}

pub async fn fetch_user(pool: &Pool<Sqlite>, id: i64) -> Result<User, Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}
