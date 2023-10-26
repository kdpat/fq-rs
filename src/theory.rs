use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    fn semitone_offset(&self) -> i32 {
        match &self {
            Accidental::DoubleFlat => -2,
            Accidental::Flat => -1,
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
            Accidental::DoubleSharp => 2,
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

#[derive(Debug)]
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
    pub fn semitones_from_c(&self) -> i32 {
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

#[derive(Debug)]
pub struct Note {
    pub white_key: WhiteKey,
    pub octave: Octave,
    pub accidental: Option<Accidental>,
}

// ("([A-Z])(#{1,2}|b{1,2})?(\\d)");

impl Note {
    fn pitch_class(&self) -> i32 {
        let offset = &self.accidental.as_ref().map_or(0, |a| a.semitone_offset());
        self.white_key.semitones_from_c() + offset
    }

    fn midi_num(&self) -> i32 {
        self.pitch_class() + 12 * (self.octave + 1)
    }

    fn is_enharmonic_with(&self, other: Note) -> bool {
        self.midi_num() == other.midi_num()
    }
}

impl Distribution<Note> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Note {
        Note {
            white_key: rand::random(),
            accidental: rand::random(),
            octave: rng.gen_range(0..=9),
        }
    }
}

#[derive(Debug)]
pub struct FretCoord {
    string: i32,
    fret: i32,
}

pub type Tuning = Vec<Note>;

#[derive(Debug)]
pub struct Fretboard {
    tuning: Tuning,
    start_fret: i32,
    end_fret: i32,
}
