use crate::game::{Game, GameId};
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Pool, Sqlite};

pub async fn fetch_game(pool: &Pool<Sqlite>, game_id: GameId) -> Result<Game, sqlx::Error> {
    sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id = ?")
        .bind(game_id)
        .fetch_one(pool)
        .await
}

pub async fn insert_game(pool: &Pool<Sqlite>, game: Game) -> Result<GameId, sqlx::Error> {
    let game_id = sqlx::query(
        "INSERT INTO games (host_id, status, player_ids, opts, rounds) VALUES (?, ?, ?, ?, ?);",
    )
    .bind(game.host_id)
    .bind(game.status)
    .bind(serde_json::to_string(&game.player_ids).unwrap())
    .bind(serde_json::to_string(&game.opts).unwrap())
    .bind(serde_json::to_string(&game.rounds).unwrap())
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(game_id)
}

pub async fn update_game(
    pool: &Pool<Sqlite>,
    game: Game,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("UPDATE games SET host_id = ?, status = ?, player_ids = ?, opts = ?, rounds = ?);")
        .bind(game.host_id)
        .bind(game.status)
        .bind(serde_json::to_string(&game.player_ids).unwrap())
        .bind(serde_json::to_string(&game.opts).unwrap())
        .bind(serde_json::to_string(&game.rounds).unwrap())
        .execute(pool)
        .await
}

// let mut tx = pool.begin().await?;
// .execute(&mut *tx)
// tx.commit().await?;
