use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::{fmt, str};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    fn semitone_offset(&self) -> i32 {
        match self {
            Self::DoubleFlat => -2,
            Self::Flat => -1,
            Self::Natural => 0,
            Self::Sharp => 1,
            Self::DoubleSharp => 2,
        }
    }

    fn maybe_rand() -> Option<Accidental> {
        let rand: f64 = rand::random();

        if rand < 0.5 {
            None
        } else {
            Some(rand::random())
        }
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DoubleFlat => "bb",
                Self::Flat => "b",
                Self::Natural => "n",
                Self::Sharp => "#",
                Self::DoubleSharp => "##",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseAccidentalError;

impl str::FromStr for Accidental {
    type Err = ParseAccidentalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bb" => Ok(Self::DoubleFlat),
            "b" => Ok(Self::Flat),
            "n" => Ok(Self::Natural),
            "#" => Ok(Self::Sharp),
            "##" => Ok(Self::DoubleSharp),
            _ => Err(ParseAccidentalError),
        }
    }
}

impl Distribution<Accidental> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Accidental {
        match rng.gen_range(0..5) {
            0 => Accidental::DoubleFlat,
            1 => Accidental::Flat,
            2 => Accidental::Natural,
            3 => Accidental::Sharp,
            _ => Accidental::DoubleSharp,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
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
    fn semitones_from_c(&self) -> i32 {
        match &self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
        }
    }
}

impl fmt::Display for WhiteKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseWhiteKeyError;

impl str::FromStr for WhiteKey {
    type Err = ParseWhiteKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(ParseWhiteKeyError),
        }
    }
}

impl Distribution<WhiteKey> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WhiteKey {
        match rng.gen_range(0..7) {
            0 => WhiteKey::C,
            1 => WhiteKey::D,
            2 => WhiteKey::E,
            3 => WhiteKey::F,
            4 => WhiteKey::G,
            5 => WhiteKey::A,
            _ => WhiteKey::B,
        }
    }
}

type Octave = i32;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Note {
    pub white_key: WhiteKey,
    pub octave: Octave,
    pub accidental: Option<Accidental>,
}

// ("([A-Z])(#{1,2}|b{1,2})?(\\d)");

impl Note {
    fn pitch_class(&self) -> i32 {
        let acc_offset = &self.accidental.as_ref().map_or(0, |a| a.semitone_offset());
        self.white_key.semitones_from_c() + acc_offset
    }

    fn midi_num(&self) -> i32 {
        self.pitch_class() + 12 * (self.octave + 1)
    }

    fn is_enharmonic(&self, other: Note) -> bool {
        self.midi_num() == other.midi_num()
    }

    // TODO
    pub fn rand_in_range(low_midi: i32, high_midi: i32) -> Note {
        let mut note: Note = rand::random();
        let mut midi = note.midi_num();

        while midi < low_midi || midi > high_midi {
            note = rand::random();
            midi = note.midi_num();
        }

        note
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let accidental = self
            .accidental
            .as_ref()
            .map_or(String::from(""), |a| a.to_string());

        write!(f, "{}{}/{}", self.white_key, accidental, self.octave)
    }
}

impl Distribution<Note> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Note {
        Note {
            white_key: rand::random(),
            accidental: Accidental::maybe_rand(),
            octave: rng.gen_range(3..=7),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FretCoord {
    pub string: i32,
    pub fret: i32,
}

pub type Tuning = Vec<Note>;

#[derive(Debug)]
pub struct Fretboard {
    tuning: Tuning,
    start_fret: i32,
    end_fret: i32,
}

#[cfg(test)]
mod test {
    use crate::theory::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_parse_note() {
        let c_double_sharp = Note {
            white_key: WhiteKey::C,
            accidental: Some(Accidental::DoubleSharp),
            octave: 4,
        };

        let e_double_flat = Note {
            white_key: WhiteKey::E,
            accidental: Some(Accidental::DoubleFlat),
            octave: 4,
        };

        assert!(c_double_sharp.is_enharmonic(e_double_flat));
    }
}
