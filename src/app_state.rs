use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::broadcast;

pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub rooms: Mutex<HashMap<String, Room>>,
}

pub struct Room {
    pub tx: broadcast::Sender<String>,
}

impl Room {
    fn new() -> Self {
        Self {
            tx: broadcast::channel(64).0,
        }
    }
}
