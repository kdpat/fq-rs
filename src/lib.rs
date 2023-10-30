#![allow(unused)]

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Error, Pool, Sqlite, SqlitePool};

pub mod auth;
pub mod game;
pub mod routes;
pub mod theory;
pub mod user;
pub mod ws;

pub async fn create_db_pool(filename: &str) -> Result<Pool<Sqlite>, Error> {
    let opts = SqliteConnectOptions::new().filename(filename);
    // .create_if_missing(true);

    SqlitePool::connect_with(opts).await
}

#[cfg(test)]
mod tests {
    use crate::{create_db_pool, game, theory, user};

    const TEST_DB_FILE: &str = "fq.db";

    // #[tokio::test]
    // async fn create_user_and_game() {
    //     let pool = create_db_pool(TEST_DB_FILE).await.unwrap();
    //
    //     let session_id = "session_0";
    //
    //     let user_id = match user::fetch_user_by_session_id(&pool, session_id).await {
    //         Ok(user) => user.id,
    //         _ => user::create_user(&pool, session_id)
    //             .await
    //             .map(|res| res.last_insert_rowid())
    //             .unwrap(),
    //     };
    //
    //     println!("user id: {}", user_id);
    //
    //     let user = user::fetch_user(&pool, user_id).await.unwrap();
    //     println!("user: {:?}", user);
    //
    //     let game = game::Game::new(user_id);
    //     println!("game: {:?}", game);
    //
    //     let game_id = game::db::insert_game(&pool, game).await.unwrap();
    //     println!("game id: {}", game_id);
    //
    //     let found_game = game::db::fetch_game(&pool, game_id).await.unwrap();
    //     println!("found game: {:?}", found_game);
    // }
}
