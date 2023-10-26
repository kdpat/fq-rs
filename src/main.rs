use axum::routing::{get, post};
use axum::Router;
use fq::routes;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

const PORT: u16 = 4000;
const DB_FILENAME: &str = "fq.db";

#[tokio::main]
async fn main() {
    let pool = fq::create_db_pool(DB_FILENAME).await.unwrap();

    fq::user::ensure_users_table(&pool).await.unwrap();
    fq::game::ensure_games_tables(&pool).await.unwrap();

    let cwd = std::env::current_dir().unwrap();
    let assets_path = format!("{}/assets", cwd.to_str().unwrap());
    let assets_service = ServeDir::new(assets_path);

    let router = Router::new()
        .route("/", get(routes::index_page))
        .route("/users/:id", get(routes::user_page))
        .route("/games/:id", get(routes::game_page))
        .route("/games", post(routes::accept_game_create))
        .nest_service("/assets", assets_service)
        .layer(CookieManagerLayer::new())
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
