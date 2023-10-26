use crate::game::Game;
use crate::user::{User, USER_COOKIE};
use crate::{game, theory, user};
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
    note: String,
}

pub async fn game_page(
    State(pool): State<Pool<Sqlite>>,
    Path(game_id): Path<i64>,
) -> Result<GameTemplate, StatusCode> {
    match game::fetch_game(&pool, game_id).await {
        Ok(Game {
            id: Some(id_val),
            status,
            ..
        }) => Ok(GameTemplate {
            id: id_val,
            status: status.to_string(),
            note: theory::Note::rand_in_range(52, 80).string_repr(),
        }),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

fn get_user_cookie(cookies: &Cookies) -> Option<i64> {
    cookies
        .get(USER_COOKIE)
        .map(|c| c.value().parse::<_>().ok())
        .flatten()
}

pub async fn accept_game_create(
    State(pool): State<Pool<Sqlite>>,
    cookies: Cookies,
) -> Result<impl IntoResponse, StatusCode> {
    match get_user_cookie(&cookies) {
        Some(user_id) => {
            let game = Game::new(user_id);

            if let Ok(game_id) = game::insert_game(&pool, game).await {
                let game_url = format!("/games/{}", game_id);
                Ok(Redirect::to(game_url.as_str()))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        _ => Err(StatusCode::EXPECTATION_FAILED),
    }
}
