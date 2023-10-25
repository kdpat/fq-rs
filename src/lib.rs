#![allow(dead_code)]

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Error, Pool, Sqlite, SqlitePool};
use tower_http::services::ServeDir;

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

pub async fn create_assets_service() -> ServeDir {
    let cwd = std::env::current_dir().unwrap();
    let assets_path = format!("{}/assets", cwd.to_str().unwrap());
    ServeDir::new(assets_path)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn hmm() {
//         assert!(true)
//     }
// }
