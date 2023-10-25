use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Error, Pool, Sqlite};
use std::fmt;

#[derive(Debug, sqlx::FromRow)]
pub struct Game {
    pub id: i64,
    pub host_id: i64,
    pub status: Status,
}

impl Game {
    pub fn new(&self, host_id: i64) -> Game {
        Game {
            id: 0,
            host_id,
            status: Status::Init,
        }
    }
}

#[derive(Debug, sqlx::Type)]
pub enum Status {
    Init,
    Playing,
    RoundOver,
    GameOver,
    NoPlayers,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Settings {
    start_fret: i32,
    end_fret: i32,
}

const CREATE_GAMES_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY,
    host_id INTEGER,
    status TEXT NOT NULL,
    FOREIGN KEY(host_id) REFERENCES users(id)
);";

pub async fn ensure_games_table(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    sqlx::query(CREATE_GAMES_TABLE_SQL).execute(pool).await
}

pub async fn create_game(pool: &Pool<Sqlite>, host_id: i64) -> Result<SqliteQueryResult, Error> {
    sqlx::query("INSERT INTO games (host_id, status) VALUES (?, ?)")
        .bind(host_id)
        .bind(Status::Init.to_string())
        .execute(pool)
        .await
}

pub async fn fetch_game(pool: &Pool<Sqlite>, id: i64) -> Result<Game, Error> {
    sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}
