use crate::game::{Game, Status};
use crate::user::User;
use crate::{game, theory, user};
use askama_axum::{IntoResponse, Template};
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::{headers, TypedHeader};
use futures::{sink::SinkExt, stream::StreamExt};
use sqlx::{Error, Pool, Sqlite};
use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::ControlFlow;
use tower_cookies::{Cookie, Cookies};

pub const USER_COOKIE: &str = "_fq_user";

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
    let res = user::db::create_user(pool).await?;
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
    match user::db::fetch_user(&pool, id).await {
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
    player_ids: String,
}

pub async fn game_page(
    State(pool): State<Pool<Sqlite>>,
    Path(game_id): Path<i64>,
) -> Result<GameTemplate, StatusCode> {
    match game::db::fetch_game(&pool, game_id).await {
        Ok(game) => Ok(GameTemplate {
            id: game.id.unwrap(),
            status: game.status.to_string(),
            note: game
                .curr_round()
                .map(|r| r.note_to_guess.string_repr())
                .unwrap_or_default(),
            player_ids: game.player_ids.iter().map(|id| id.to_string()).collect(),
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

pub async fn handle_game_create(
    State(pool): State<Pool<Sqlite>>,
    cookies: Cookies,
) -> Result<impl IntoResponse, StatusCode> {
    match get_user_cookie(&cookies) {
        Some(user_id) => {
            let game = Game::new(user_id);

            if let Ok(game_id) = game::db::insert_game(&pool, game).await {
                let game_url = format!("/games/{}", game_id);
                Ok(Redirect::to(game_url.as_str()))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        _ => Err(StatusCode::BAD_REQUEST),
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

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {}...", who);
    } else {
        println!("Could not send ping {}!", who);
        return;
    }

    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            println!("client {who} abruptly disconnected");
            return;
        }
    }

    for i in 1..5 {
        if socket
            .send(Message::Text(format!("Hi {i} times!")))
            .await
            .is_err()
        {
            println!("client {who} abruptly disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        let n_msg = 20;
        for i in 0..n_msg {
            if sender
                .send(Message::Text(format!("Server message {i} ...")))
                .await
                .is_err()
            {
                return i;
            }

            tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
        }

        println!("sending close to {who}...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Goodbye"),
            })))
            .await
        {
            println!("Could not send Close due to {}, probably it is ok?", e);
        }
        n_msg
    });
}

fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {} sent str: {:?}", who, t);
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {} somehow sent close message without CloseFrame", who);
            }
            return ControlFlow::Break(());
        }
        Message::Pong(v) => {
            println!(">>> {} sent pong with {:?}", who, v);
        }
        Message::Ping(v) => {
            println!(">>> {} sent ping with {:?}", who, v);
        }
    }
    ControlFlow::Continue(())
}
