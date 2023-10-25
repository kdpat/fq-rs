use axum::routing::{get, post};
use axum::Router;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

const PORT: u16 = 4000;
const DB_FILENAME: &str = "fq.db";

#[tokio::main]
async fn main() {
    let pool = fq::create_db_pool(DB_FILENAME).await.unwrap();

    fq::user::ensure_users_table(&pool).await.unwrap();
    fq::game::ensure_games_table(&pool).await.unwrap();

    let assets_service = fq::create_assets_service().await;

    let router = Router::new()
        .route("/", get(fq::routes::index_page))
        .route("/users/:id", get(fq::routes::user_page))
        .route("/games/:id", get(fq::routes::game_page))
        .route("/games", post(fq::routes::accept_game_create))
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
