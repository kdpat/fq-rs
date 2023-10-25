use crate::game::Game;
use crate::user::{User, USER_COOKIE};
use crate::{game, user};
use askama_axum::{IntoResponse, Template};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use sqlx::{Error, Pool, Sqlite};
use tower_cookies::{Cookie, Cookies};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index_page(State(pool): State<Pool<Sqlite>>, cookies: Cookies) -> IndexTemplate {
    if None == cookies.get(USER_COOKIE) {
        let cookie = create_user_and_cookie(&pool).await.unwrap();
        cookies.add(cookie);
    }

    IndexTemplate {}
}

async fn create_user_and_cookie<'a>(pool: &Pool<Sqlite>) -> Result<Cookie<'a>, Error> {
    let res = user::create_user(pool).await?;
    let user_id = res.last_insert_rowid();
    let cookie = Cookie::new(USER_COOKIE, user_id.to_string());
    Ok(cookie)
}

#[derive(Template)]
#[template(path = "user.html")]
pub struct UserTemplate {
    name: String,
}

pub async fn user_page(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<UserTemplate, StatusCode> {
    match user::fetch_user(&pool, id).await {
        Ok(User { name, .. }) => Ok(UserTemplate { name }),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Template)]
#[template(path = "game.html")]
pub struct GameTemplate {
    id: i64,
    status: String,
}

pub async fn game_page(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<GameTemplate, StatusCode> {
    match game::fetch_game(&pool, id).await {
        Ok(Game { id, status, .. }) => Ok(GameTemplate {
            id,
            status: status.to_string(),
        }),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

fn get_user_cookie(cookies: &Cookies) -> Option<i64> {
    cookies
        .get(USER_COOKIE)
        .map(|c| c.value().parse::<i64>().ok())
        .flatten()
}

pub async fn accept_game_create(
    State(pool): State<Pool<Sqlite>>,
    cookies: Cookies,
) -> Result<impl IntoResponse, StatusCode> {
    match get_user_cookie(&cookies) {
        Some(user_id) => {
            if let Ok(game_res) = game::create_game(&pool, user_id).await {
                let game_id = game_res.last_insert_rowid();
                let game_url = format!("/games/{}", game_id);
                Ok(Redirect::to(game_url.as_str()))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        _ => Err(StatusCode::EXPECTATION_FAILED),
    }
}
