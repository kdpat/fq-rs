use axum::error_handling::HandleErrorLayer;
use axum::routing::{get, post};
use axum::{BoxError, Router};
use fq::{routes, ws};
use hyper::StatusCode;
use std::net::SocketAddr;
use tower::ServiceBuilder;
// use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
// use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tower_sessions::{SessionManagerLayer, SqliteStore};
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

    // let span = DefaultMakeSpan::default().include_headers(true);
    // let trace_layer = TraceLayer::new_for_http().make_span_with(span);

    let pool = fq::create_db_pool(DB_FILENAME).await.unwrap();

    let cwd = std::env::current_dir().unwrap();
    let assets_path = format!("{}/assets", cwd.to_str().unwrap());
    let assets_service = ServeDir::new(assets_path);

    let error_layer = HandleErrorLayer::new(|_: BoxError| async { StatusCode::BAD_REQUEST });

    let session_store = SqliteStore::new(pool.clone());
    session_store.migrate().await.unwrap();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_max_age(time::Duration::days(1));

    let session_service = ServiceBuilder::new()
        .layer(error_layer)
        .layer(session_layer);

    let router = Router::new()
        .route("/", get(routes::index_page))
        .route("/users/:id", get(routes::user_page))
        // .route("/games", post(routes::handle_game_create))
        // .route("/games/:id", get(routes::game_page))
        .route("/ws", get(ws::upgrade_ws))
        .nest_service("/assets", assets_service)
        // .layer(CookieManagerLayer::new())
        .layer(session_service)
        // .layer(trace_layer)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
