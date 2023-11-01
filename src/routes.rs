use crate::{
    auth::{self, Claims},
    game::{self, Game, GameId},
    theory,
    user::{self, User, UserId},
};
use askama_axum::{IntoResponse, Template};
use axum::extract::{ConnectInfo, Path, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::{headers, Form, Json, TypedHeader};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Sqlite};
use std::borrow::Cow;
use std::future::Future;
use std::net::SocketAddr;
use std::ops::ControlFlow;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};
use tower_sessions::Session;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index_page() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "user.html")]
pub struct UserTemplate {
    id: UserId,
    name: String,
}

pub async fn user_page(cookies: Cookies) -> Result<UserTemplate, Redirect> {
    match auth::decode_user_cookie(&cookies) {
        Ok(claims) => Ok(UserTemplate {
            id: claims.sub,
            name: claims.name,
        }),
        _ => Err(Redirect::to("/")),
    }
}

#[derive(Deserialize)]
pub struct UsernamePayload {
    pub name: String,
}

pub async fn update_username(
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
    Form(payload): Form<UsernamePayload>,
) -> Redirect {
    if let Ok(claims) = auth::decode_user_cookie(&cookies) {
        if user::update_username(&pool, claims.sub, &payload.name)
            .await
            .is_ok()
        {
            let user = User {
                id: claims.sub,
                name: payload.name,
            };

            let token = auth::make_user_token(&user).unwrap();
            let cookie = auth::make_user_cookie(token);
            cookies.add(cookie);
        }
    }
    Redirect::to("/user")
}

#[derive(Template)]
#[template(path = "game.html")]
pub struct GameTemplate {
    id: GameId,
    status: String,
    note: String,
    player_ids: String,
}

impl From<Game> for GameTemplate {
    fn from(game: Game) -> Self {
        GameTemplate {
            id: game.id.unwrap(),
            status: game.status.to_string(),
            note: game
                .curr_note_to_guess()
                .map(|n| n.to_string())
                .unwrap_or_default(),
            player_ids: game.player_ids.iter().map(|id| id.to_string()).collect(),
        }
    }
}

pub async fn game_page(
    State(pool): State<Pool<Sqlite>>,
    Path(game_id): Path<GameId>,
) -> Result<GameTemplate, StatusCode> {
    match game::db::fetch_game(&pool, game_id).await {
        Ok(game) => Ok(GameTemplate::from(game)),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn handle_game_create(
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Redirect, StatusCode> {
    match auth::decode_user_cookie(&cookies) {
        Ok(claims) => {
            let user_id = claims.sub;
            let game = Game::new(user_id);

            if let Ok(game_id) = game::db::insert_game(&pool, game).await {
                let game_url = format!("/games/{}", game_id);
                return Ok(Redirect::to(game_url.as_str()));
            }

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn handle_game_start(
    cookies: Cookies,
    Path(game_id): Path<GameId>,
    State(pool): State<Pool<Sqlite>>,
) {
    if let Ok(claims) = auth::decode_user_cookie(&cookies) {
        if let Ok(mut game) = game::db::fetch_game(&pool, game_id).await {
            if let Some(host_id) = game.host_id {
                if claims.sub == host_id {
                    game.start();
                    if let Ok(_) = game::db::update_game(&pool, game).await {
                        tracing::debug!("game started: {}", game_id);
                    }
                }
            }
        }
    }
}

// pub async fn handle_game_start(
//     Path(game_id): Path<i64>,
//     State(pool): State<Pool<Sqlite>>,
//     cookies: Cookies,
// ) -> Result<impl IntoResponse, StatusCode> {
//     match get_user_cookie(&cookies) {
//         Some(user_id) => {
//             let mut game = fetch_game(&pool, game_id).await.unwrap();
//
//             if user_id == game.host_id.unwrap() {
//                 game.status = Status::Playing;
//                 Ok(Redirect::to("/"))
//             } else {
//                 Err(StatusCode::UNAUTHORIZED)
//             }
//         }
//         _ => Err(StatusCode::EXPECTATION_FAILED),
//     }
// }
