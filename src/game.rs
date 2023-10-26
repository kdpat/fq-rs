use crate::theory;
use crate::theory::{Note, WhiteKey};
use sqlx::query::Query;
use sqlx::sqlite::{SqliteQueryResult, SqliteRow};
use sqlx::{Acquire, Error, Pool, Row, Sqlite};
use std::fmt;

#[derive(Debug)]
pub struct Game {
    pub id: Option<i64>,
    pub host_id: Option<i64>,
    pub status: Status,
    pub settings: Settings,
    pub rounds: Vec<Round>,
}

const CREATE_GAMES_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY,
    host_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    FOREIGN KEY(host_id) REFERENCES users(id)
);";

impl Game {
    pub fn new(host_id: i64) -> Game {
        Game {
            id: None,
            host_id: Some(host_id),
            status: Status::Init,
            settings: Settings::default(),
            rounds: vec![],
        }
    }
}

#[derive(Debug)]
pub enum Status {
    Init,
    Playing,
    RoundOver,
    GameOver,
    NoPlayers,
}

impl Status {
    fn from(s: &str) -> Option<Status> {
        match s {
            "Init" => Some(Status::Init),
            "Playing" => Some(Status::Playing),
            "RoundOver" => Some(Status::RoundOver),
            "GameOver" => Some(Status::GameOver),
            "NoPlayers" => Some(Status::NoPlayers),
            _ => None,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Settings {
    id: Option<i64>,
    game_id: Option<i64>,
    num_rounds: i32,
    start_fret: i32,
    end_fret: i32,
}

const CREATE_SETTINGS_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL,
    num_rounds INTEGER NOT NULL,
    start_fret INTEGER NOT NULL,
    end_fret INTEGER NOT NULL,
    FOREIGN KEY(game_id) REFERENCES games(id)
);";

impl Settings {
    fn default() -> Settings {
        Settings {
            id: None,
            game_id: None,
            num_rounds: 4,
            start_fret: 0,
            end_fret: 4,
        }
    }
}

#[derive(Debug)]
pub struct Guess {
    id: Option<i64>,
    user_id: Option<i64>,
    round_id: i64,
    clicked_fret_coord: theory::FretCoord,
    is_correct: bool,
}

const CREATE_GUESSES_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS guesses (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    round_id INTEGER NOT NULL,
    clicked_fret INTEGER NOT NULL,
    clicked_string INTEGER NOT NULL,
    is_correct INTEGER NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(round_id) REFERENCES rounds(id)
);";

#[derive(Debug)]
pub struct Round {
    id: Option<i64>,
    note_to_guess: Note,
    guesses: Vec<Guess>,
}

const CREATE_ROUNDS_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS rounds (
    id INTEGER PRIMARY KEY,
    game_id INTEGER NOT NULL,
    note_white_key TEXT NOT NULL,
    note_accidental TEXT NOT NULL,
    note_octave INTEGER NOT NULL,
    FOREIGN KEY(game_id) REFERENCES games(id)
);";

impl Round {
    fn new() -> Round {
        Round {
            id: None,
            note_to_guess: Note {
                white_key: WhiteKey::C,
                octave: 4,
                accidental: None,
            },
            guesses: vec![],
        }
    }
}

pub async fn ensure_games_tables(pool: &Pool<Sqlite>) -> Result<(), Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(CREATE_GAMES_TABLE_SQL)
        .execute(&mut *tx)
        .await?;
    sqlx::query(CREATE_SETTINGS_TABLE_SQL)
        .execute(&mut *tx)
        .await?;
    sqlx::query(CREATE_ROUNDS_TABLE_SQL)
        .execute(&mut *tx)
        .await?;
    sqlx::query(CREATE_GUESSES_TABLE_SQL)
        .execute(&mut *tx)
        .await?;

    tx.commit().await
}

pub async fn insert_game(pool: &Pool<Sqlite>, game: Game) -> Result<(i64), Error> {
    let mut tx = pool.begin().await?;

    let game_id = sqlx::query("INSERT INTO games (host_id, status) VALUES (?, ?);")
        .bind(game.host_id)
        .bind(game.status.to_string())
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();

    let settings_id = sqlx::query(
        "INSERT INTO SETTINGS (game_id, num_rounds, start_fret, end_fret) VALUES (?, ?, ?, ?);",
    )
    .bind(game_id)
    .bind(game.settings.num_rounds)
    .bind(game.settings.start_fret)
    .bind(game.settings.end_fret)
    .execute(&mut *tx)
    .await?
    .last_insert_rowid();

    tx.commit().await?;
    Ok(game_id)
}

pub async fn fetch_game(pool: &Pool<Sqlite>, game_id: i64) -> Result<Game, Error> {
    let mut conn = pool.acquire().await.unwrap();

    let settings = sqlx::query("SELECT * FROM settings WHERE game_id = ?")
        .bind(game_id)
        .map(|row: SqliteRow| Settings {
            id: Some(row.get::<i64, _>("id")),
            game_id: Some(game_id),
            num_rounds: row.get::<i32, _>("num_rounds"),
            start_fret: row.get::<i32, _>("start_fret"),
            end_fret: row.get::<i32, _>("end_fret"),
        })
        .fetch_one(&mut *conn)
        .await
        .unwrap();

    sqlx::query("SELECT * FROM games where id = ?")
        .bind(game_id)
        .fetch_one(&mut *conn)
        .await
        .map(|row: SqliteRow| Game {
            id: Some(game_id),
            host_id: Some(row.get::<i64, _>("host_id")),
            status: Status::from(row.get::<String, _>("status").as_str()).unwrap(),
            settings,
            rounds: vec![],
        })
}
