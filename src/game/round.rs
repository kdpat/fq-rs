use crate::{game, theory};

#[derive(Debug, sqlx::FromRow)]
pub struct Round {
    pub id: Option<i64>,
    pub note_to_guess: theory::Note,
    pub guesses: Vec<game::guess::Guess>,
}

impl Round {
    fn new() -> Round {
        Round {
            id: None,
            note_to_guess: theory::Note::rand_in_range(40, 68),
            guesses: vec![],
        }
    }
}
