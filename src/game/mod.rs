pub mod db;

pub mod guess;
pub mod round;

// use crate::theory;
// use crate::theory::{Accidental, FretCoord, Note, WhiteKey};
// use sqlx::query::Query;
// use sqlx::sqlite::{SqliteQueryResult, SqliteRow};
// use sqlx::{Acquire, Error, Pool, Row, Sqlite};
// use std::fmt;

// #[derive(Debug)]
// pub struct Game {
//     pub id: Option<i64>,
//     pub host_id: Option<i64>,
//     pub status: Status,
//     pub player_ids: Vec<i64>,
//     pub opts: Opts,
//     pub rounds: Vec<Round>,
// }

// impl Game {
//     pub fn new(host_id: i64) -> Game {
//         Game {
//             id: None,
//             host_id: Some(host_id),
//             status: Status::Init,
//             opts: Opts::new(),
//             rounds: vec![],
//             player_ids: vec![host_id],
//         }
//     }

//     pub fn curr_round(&self) -> Option<&Round> {
//         self.rounds.last()
//     }
// }

// #[derive(Debug)]
// pub enum Status {
//     Init,
//     Playing,
//     RoundOver,
//     GameOver,
//     NoPlayers,
// }

// impl Status {
//     fn from(s: &str) -> Option<Status> {
//         match s {
//             "Init" => Some(Status::Init),
//             "Playing" => Some(Status::Playing),
//             "RoundOver" => Some(Status::RoundOver),
//             "GameOver" => Some(Status::GameOver),
//             "NoPlayers" => Some(Status::NoPlayers),
//             _ => None,
//         }
//     }
// }

// impl fmt::Display for Status {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// #[derive(Debug)]
// pub struct Opts {
//     id: Option<i64>,
//     game_id: Option<i64>,
//     num_rounds: i32,
//     start_fret: i32,
//     end_fret: i32,
// }

// impl Opts {
//     fn new() -> Opts {
//         Opts {
//             id: None,
//             game_id: None,
//             num_rounds: 4,
//             start_fret: 0,
//             end_fret: 4,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Guess {
//     id: Option<i64>,
//     user_id: Option<i64>,
//     round_id: i64,
//     clicked_fret_coord: FretCoord,
//     is_correct: bool,
// }

// #[derive(Debug)]
// pub struct Round {
//     pub id: Option<i64>,
//     pub note_to_guess: Note,
//     pub guesses: Vec<Guess>,
// }

// impl Round {
//     fn new() -> Round {
//         Round {
//             id: None,
//             note_to_guess: Note::rand_in_range(40, 68),
//             guesses: vec![],
//         }
//     }
// }
