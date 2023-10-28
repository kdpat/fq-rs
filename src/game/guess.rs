use crate::theory;

#[derive(Debug, sqlx::FromRow)]
pub struct Guess {
    id: Option<i64>,
    user_id: Option<i64>,
    round_id: i64,
    clicked_fret_coord: theory::FretCoord,
    is_correct: bool,
}
