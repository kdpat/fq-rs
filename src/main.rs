use askama_axum::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use sqlx::sqlite::{SqliteConnectOptions, SqliteQueryResult};
use sqlx::{Error, Pool, Sqlite, SqlitePool};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

type AppState = State<Pool<Sqlite>>;

const PORT: u16 = 3030;
const DB_FILENAME: &str = "fq.db";
const DEFAULT_USERNAME: &str = "user";

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("starting server on port {}...", PORT);

    let sql_opts = SqliteConnectOptions::new()
        .filename(DB_FILENAME)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(sql_opts).await?;
    create_users_table(&pool).await?;

    let cwd = std::env::current_dir().unwrap();
    let assets_path = format!("{}/assets", cwd.to_str().unwrap());
    let assets_service = ServeDir::new(assets_path);

    let router = Router::new()
        .route("/", get(index_page))
        .route("/users/:id", get(user_page))
        .nest_service("/assets", assets_service)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index_page() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate {
    name: String,
}

async fn user_page(State(pool): AppState, Path(id): Path<i64>) -> Result<UserTemplate, StatusCode> {
    match fetch_user(&pool, id).await {
        Ok(user) => Ok(UserTemplate { name: user.name }),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

const CREATE_USERS_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS users (
       id INTEGER PRIMARY KEY NOT NULL,
       name TEXT NOT NULL
     )";

async fn create_users_table(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    sqlx::query(CREATE_USERS_TABLE_SQL).execute(pool).await
}

async fn create_user(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, Error> {
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(DEFAULT_USERNAME)
        .execute(pool)
        .await
}

async fn fetch_user(pool: &Pool<Sqlite>, id: i64) -> Result<User, Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}
