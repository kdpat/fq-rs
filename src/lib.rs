#![allow(unused)]

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Error, Pool, Sqlite, SqlitePool};

pub mod game;
pub mod routes;
pub mod theory;
pub mod user;

pub async fn create_db_pool(filename: &str) -> Result<Pool<Sqlite>, Error> {
    let opts = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    SqlitePool::connect_with(opts).await
}

#[cfg(test)]
mod tests {
    use crate::{create_db_pool, game, user};

    const TEST_DB_FILE: &str = "fq_test.db";

    #[tokio::test]
    async fn create_user_and_game() {
        let pool = create_db_pool(TEST_DB_FILE).await.unwrap();
        user::ensure_users_table(&pool).await.unwrap();
        game::ensure_games_tables(&pool).await.unwrap();

        let user_res = user::create_user(&pool).await.unwrap();
        let user_id = user_res.last_insert_rowid();

        let user = user::fetch_user(&pool, user_id).await.unwrap();
        // println!("{:?}", user);

        let game = game::Game::new(user_id);
        println!("{:?}", game);

        let (game_id, _) = game::insert_game(&pool, game).await.unwrap();
        let found_game = game::fetch_game(&pool, game_id).await.unwrap();
        println!("found: {:?}", found_game);

        assert!(true)
    }
}
