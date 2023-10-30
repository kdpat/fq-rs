use crate::game::{Game, GameId};
use sqlx::{Error, Pool, Sqlite};

pub async fn insert_game(pool: &Pool<Sqlite>, game: Game) -> Result<GameId, Error> {
    let mut tx = pool.begin().await?;

    let game_id = sqlx::query(
        "INSERT INTO games (host_id, status, player_ids, opts, rounds) VALUES (?, ?, ?, ?, ?);",
    )
    .bind(game.host_id)
    .bind(game.status)
    .bind(serde_json::to_string(&game.player_ids).unwrap())
    .bind(serde_json::to_string(&game.opts).unwrap())
    .bind(serde_json::to_string(&game.rounds).unwrap())
    .execute(&mut *tx)
    .await?
    .last_insert_rowid();

    tx.commit().await?;
    Ok(game_id)
}

pub async fn fetch_game(pool: &Pool<Sqlite>, game_id: GameId) -> Result<Game, Error> {
    sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id = ?")
        .bind(game_id)
        .fetch_one(pool)
        .await
}
