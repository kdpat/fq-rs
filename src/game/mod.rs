pub mod db;

use crate::theory;
use crate::user::UserId;

use serde::{Deserialize, Serialize};
use std::fmt;

pub type GameId = i64;

#[derive(Debug, sqlx::FromRow)]
pub struct Game {
    pub id: Option<GameId>,
    pub host_id: Option<UserId>,
    pub status: Status,
    #[sqlx(json)]
    pub player_ids: Vec<UserId>,
    #[sqlx(json)]
    pub opts: Opts,
    #[sqlx(json)]
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn new(host_id: UserId) -> Game {
        Game {
            id: None,
            host_id: Some(host_id),
            status: Status::Init,
            opts: Opts::new(),
            rounds: vec![],
            player_ids: vec![host_id],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Opts {
    pub num_rounds: i32,
    pub start_fret: i32,
    pub end_fret: i32,
}

impl Opts {
    pub fn new() -> Opts {
        Opts {
            num_rounds: 4,
            start_fret: 0,
            end_fret: 4,
        }
    }
}

#[derive(Debug, sqlx::Type)]
pub enum Status {
    Init,
    Playing,
    RoundOver,
    GameOver,
    NoPlayers,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Guess {
    user_id: Option<i64>,
    clicked_fret: theory::FretCoord,
    is_correct: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Round {
    pub note_to_guess: theory::Note,
    pub guesses: Vec<Guess>,
}

impl Round {
    fn new() -> Round {
        Round {
            note_to_guess: theory::Note::rand_in_range(40, 68),
            guesses: vec![],
        }
    }
}
