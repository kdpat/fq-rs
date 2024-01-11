use crate::app_state::{AppState, Room};
use crate::auth::{self, Claims};
use crate::game::GameId;
use crate::user::User;
use askama_axum::{IntoResponse, Response};
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::TypedHeader;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::Value;
use sqlx::{Pool, Sqlite};
use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::ControlFlow;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tower_cookies::Cookies;

pub async fn upgrade_ws(
    ws: WebSocketUpgrade,
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Response, StatusCode> {
    if let Some(user) = auth::decode_user_cookie(&cookies) {
        println!("{:?} at {addr} connected to ws", &user);
        Ok(ws.on_upgrade(move |socket| ws_callback(socket, state, user)))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Debug, Deserialize)]
struct ConnectMessage {
    token: String,
    channel: String,
}

/// runs async for each ws connection
async fn ws_callback(mut socket: WebSocket, state: Arc<AppState>, user: User) {
    let (mut ws_tx, mut ws_rx) = socket.split();

    let mut channel = String::new();
    let mut room_tx = None::<Sender<String>>;

    while let Some(Ok(message)) = ws_rx.next().await {
        if let Message::Text(text) = message {
            let msg = match serde_json::from_str::<ConnectMessage>(&text) {
                Ok(msg) => msg,
                Err(err) => {
                    tracing::error!(%err);
                    let _ = ws_tx
                        .send(Message::Text(String::from(
                            "Failed to parse connect message",
                        )))
                        .await;
                    continue;
                }
            };

            channel = msg.channel.clone();

            // new block to drop our lock on state.rooms
            {
                let mut rooms = state.rooms.lock().unwrap();
                let room = rooms.entry(msg.channel).or_insert_with(Room::new);
                room_tx = Some(room.tx.clone());
            }

            if room_tx.is_some() {
                break;
            } else {
                tracing::error!("somehow, room_tx is None :(");
                return;
            }
        }
    }

    let room_tx = room_tx.expect("room tx not present");
    let mut room_rx = room_tx.subscribe();

    let msg = format!("{} joined channel {}", &user.name, &channel);
    tracing::debug!("{}", msg);
    let _ = room_tx.send(msg);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = room_rx.recv().await {
            if ws_tx.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = {
        let tx = room_tx.clone();
        let name = user.name.clone();

        tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = ws_rx.next().await {
                process_message(&state.pool, &tx, &text).await;
            }
        })
    };

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    let msg = format!("{} left.", user.name);
    tracing::debug!("{}", msg);
    let _ = room_tx.send(msg);
}

#[derive(Debug, Deserialize)]
enum AppMessage {
    // JoinGame { token: String },
    StartGame { token: String, game_id: GameId },
}

async fn process_message(pool: &Pool<Sqlite>, tx: &Sender<String>, text: &str) {
    match serde_json::from_str::<AppMessage>(text) {
        Ok(msg) => {
            tracing::debug!("msg processed: {:?}", &msg);
            tx.send(format!("{:?}", msg));
            match msg {
                // AppMessage::JoinGame { .. } => {}
                AppMessage::StartGame { .. } => {}
            }
        }
        Err(e) => {
            tracing::error!("error processing msg: {:?}", e);
        }
    }
}

async fn handle_start_game(pool: &Pool<Sqlite>, token: String) {
    if let Ok(claims) = auth::decode_user_token(token.as_str()) {
        // if let Ok(mut game) = game::db::fetch_game(pool, )
    }
}

// if let Some(user) = auth::decode_user_cookie(&cookies) {
// if let Ok(mut game) = game::db::fetch_game(&state.pool, game_id).await {
// if let Some(host_id) = game.host_id {
// if user.id == host_id {
// game.start();
// if game::db::update_game(&state.pool, game).await.is_ok() {
// tracing::debug!("game started: {}", game_id);
// }
// }
// }
// }
// }

// fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
//     match msg {
//         Message::Text(t) => {
//             println!(">>> {} sent str: {:?}", who, t);
//             // let v: WsMessage = serde_json::from_str(t.as_str()).unwrap();
//             // println!("ws msg: {:?}", v);
//         }
//         Message::Binary(d) => {
//             println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
//         }
//         Message::Close(c) => {
//             if let Some(cf) = c {
//                 println!(
//                     ">>> {} sent close with code {} and reason `{}`",
//                     who, cf.code, cf.reason
//                 );
//             } else {
//                 println!(">>> {} somehow sent close message without CloseFrame", who);
//             }
//             return ControlFlow::Break(());
//         }
//         Message::Pong(v) => {
//             println!(">>> {} sent pong with {:?}", who, v);
//         }
//         Message::Ping(v) => {
//             println!(">>> {} sent ping with {:?}", who, v);
//         }
//     }
//     ControlFlow::Continue(())
// }

// for i in 1..5 {
//     if socket
//         .send(Message::Text(format!("Hi {i} times!")))
//         .await
//         .is_err()
//     {
//         println!("client {who} abruptly disconnected");
//         return;
//     }
//     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
// }

// let (mut sender, mut receiver) = socket.split();

// let mut send_task = tokio::spawn(async move {
//     let n_msg = 20;
//     for i in 0..n_msg {
//         if sender
//             .send(Message::Text(format!("Server message {i} ...")))
//             .await
//             .is_err()
//         {
//             return i;
//         }
//
//         tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
//     }
//
//     println!("sending close to {who}...");
//     if let Err(e) = sender
//         .send(Message::Close(Some(CloseFrame {
//             code: axum::extract::ws::close_code::NORMAL,
//             reason: Cow::from("Goodbye"),
//         })))
//         .await
//     {
//         println!("Could not send Close due to {}, probably it is ok?", e);
//     }
//     n_msg
// });

// async fn ws_callback(mut socket: WebSocket, state: Arc<AppState>, addr: SocketAddr) {
// if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
//     println!("Pinged {}...", addr);
// } else {
//     println!("Could not send ping {}!", addr);
//     return;
// }
//
// while let Some(msg) = socket.recv().await {
//     if let Ok(msg) = msg {
//         if process_message(msg, addr).is_break() {
//             return;
//         }
//     } else {
//         println!("client {addr} abruptly disconnected");
//         return;
//     }
// }
// }
