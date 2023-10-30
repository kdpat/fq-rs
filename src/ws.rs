use askama_axum::IntoResponse;
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::TypedHeader;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::ControlFlow;

pub async fn upgrade_ws(
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
    ws.on_upgrade(move |socket| ws_callback(socket, addr))
}

/// runs async for each connection
async fn ws_callback(mut socket: WebSocket, who: SocketAddr) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {}...", who);
    } else {
        println!("Could not send ping {}!", who);
        return;
    }

    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            println!("client {who} abruptly disconnected");
            return;
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum WsMessage {
    StartGame { user_id: u64 },
}

fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {} sent str: {:?}", who, t);
            let v: WsMessage = serde_json::from_str(t.as_str()).unwrap();
            println!("ws msg: {:?}", v);
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
