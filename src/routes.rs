use crate::app_state::AppState;
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
use std::sync::Arc;
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

impl From<User> for UserTemplate {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}

pub async fn user_page(cookies: Cookies) -> Result<UserTemplate, Redirect> {
    match auth::decode_user_cookie(&cookies) {
        Some(user) => Ok(UserTemplate::from(user)),
        _ => Err(Redirect::to("/")),
    }
}

#[derive(Deserialize)]
pub struct UsernamePayload {
    pub name: String,
}

pub async fn update_username(
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
    Form(payload): Form<UsernamePayload>,
) -> Redirect {
    if let Some(user) = auth::decode_user_cookie(&cookies) {
        if user::update_username(&state.pool, user.id, &user.name)
            .await
            .is_ok()
        {
            let user = User {
                name: payload.name,
                ..user
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
                .current_note_to_guess()
                .map(|n| n.to_string())
                .unwrap_or_default(),
            player_ids: game.player_ids.iter().map(|id| id.to_string()).collect(),
        }
    }
}

pub async fn game_page(
    Path(game_id): Path<GameId>,
    State(state): State<Arc<AppState>>,
) -> Result<GameTemplate, StatusCode> {
    match game::db::fetch_game(&state.pool, game_id).await {
        Ok(game) => Ok(GameTemplate::from(game)),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn handle_game_create(
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, StatusCode> {
    match auth::decode_user_cookie(&cookies) {
        Some(user) => {
            let game = Game::new(user.id);

            if let Ok(game_id) = game::db::insert_game(&state.pool, game).await {
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
    State(state): State<Arc<AppState>>,
) {
    if let Some(user) = auth::decode_user_cookie(&cookies) {
        if let Ok(mut game) = game::db::fetch_game(&state.pool, game_id).await {
            if let Some(host_id) = game.host_id {
                if user.id == host_id {
                    game.start();
                    if game::db::update_game(&state.pool, game).await.is_ok() {
                        tracing::debug!("game started: {}", game_id);
                    }
                }
            }
        }
    }
}
