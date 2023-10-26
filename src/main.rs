use axum::routing::{get, post};
use axum::Router;
use fq::routes;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const PORT: u16 = 4000;
const DB_FILENAME: &str = "fq.db";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fq=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = fq::create_db_pool(DB_FILENAME).await.unwrap();

    fq::user::db::ensure_users_table(&pool).await.unwrap();
    fq::game::db::ensure_games_tables(&pool).await.unwrap();

    let cwd = std::env::current_dir().unwrap();
    let assets_path = format!("{}/assets", cwd.to_str().unwrap());
    let assets_service = ServeDir::new(assets_path);

    let trace_layer =
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true));

    let router = Router::new()
        .route("/", get(routes::index_page))
        .route("/users/:id", get(routes::user_page))
        .route("/games", post(routes::handle_game_create))
        .route("/games/:id", get(routes::game_page))
        // .route("/games/:id", post(routes::handle_game_start))
        .nest_service("/assets", assets_service)
        .layer(CookieManagerLayer::new())
        .layer(trace_layer)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
