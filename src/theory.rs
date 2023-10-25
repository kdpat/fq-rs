pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    pub fn offset(&self) -> i32 {
        match &self {
            Accidental::DoubleFlat => -2,
            Accidental::Flat => -1,
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
            Accidental::DoubleSharp => 2,
        }
    }
}

pub enum WhiteKey {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl WhiteKey {
    pub fn half_steps_from_c(&self) -> i32 {
        match &self {
            WhiteKey::C => 0,
            WhiteKey::D => 2,
            WhiteKey::E => 4,
            WhiteKey::F => 5,
            WhiteKey::G => 7,
            WhiteKey::A => 9,
            WhiteKey::B => 11,
        }
    }
}

pub struct Note {
    white_key: WhiteKey,
    accidental: Accidental,
    octave: i32,
}

// ("([A-Z])(#{1,2}|b{1,2})?(\\d)");

pub struct FretCoord {
    string: i32,
    fret: i32,
}
