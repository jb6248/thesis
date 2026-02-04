use std::ops::{Add, Mul, Sub};
use serde::{Deserialize, Serialize};
use num::rational::Ratio;
use num::ToPrimitive;
use serde::ser::SerializeStruct;
// Assuming a standard 12-tone equal temperament system

/// Includes 0.
pub type MusicNat = u32;
pub type Measures = MusicNat;

pub type RestValue = NoteValue;

/// Can only be positive. 1/8 represents an eighth note, for example.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct NoteValue(pub Ratio<MusicNat>);

pub type Beats = Duration;


/// The number of beats (duration) depends on the note value and the time signature.
/// This is also used abstractly for representing offsets in time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Duration(pub NoteValue, pub TimeSignature);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TimeSignature(pub MusicNat, pub MusicNat);
impl TimeSignature {
    pub fn common() -> TimeSignature {
        TimeSignature(4, 4)
    }
    pub fn waltz() -> TimeSignature {
        TimeSignature(3, 4)
    }
}
impl NoteValue {
    pub fn new(numerator: MusicNat, denominator: MusicNat) -> NoteValue {
        NoteValue(Ratio::new(numerator, denominator))
    }
    pub fn with(&self, time_signature: TimeSignature) -> Duration {
        Duration(*self, time_signature)
    }
}
impl Duration {
    /// Get the total number of beats in this duration
    pub fn get_beats(&self) -> Ratio<MusicNat> {
        // 4 on the bottom of the time signature represents quarter notes (NoteValue = 1/4)
        // it would be NoteValue / (1 / time_signature.1) = NoteValue * time_signature.1
        let TimeSignature(_, denom) = self.1;
        let Duration(note_value, _) = self;
        note_value.0 * denom
    }
    /// Get the total number of measures in this duration (exact)
    pub fn get_measures(&self) -> Ratio<MusicNat> {
        // number of beats divided by number of beats per measure
        let TimeSignature(num, _) = self.1;
        self.get_beats() / num
    }
    
    /// Get the whole number of measures in this duration
    pub fn get_whole_measures(&self) -> MusicNat {
        self.get_measures().to_integer()
    }
    
    /// Get the remaining beats that do not make up a whole measure
    pub fn get_rem_beats(&self) -> Ratio<MusicNat> {
        let TimeSignature(num, _) = self.1;
        self.get_beats() % num
    }
    
    /// Get the total number of seconds in this duration at the given BPM
    pub fn get_seconds(&self, bpm: f32) -> f32 {
        let beats = self.get_beats().to_f32().unwrap();
        // units: beats * (minutes / beats) * (seconds / minute) = seconds
        beats / bpm * 60.0
    }
}

impl Serialize for NoteValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("NoteValue", 2)?;
        state.serialize_field("numerator", &self.0.numer())?;
        state.serialize_field("denominator", &self.0.denom())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for NoteValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NoteValueHelper {
            numerator: MusicNat,
            denominator: MusicNat,
        }

        let helper = NoteValueHelper::deserialize(deserializer)?;
        Ok(NoteValue(Ratio::new(helper.numerator, helper.denominator)))
    }
}

impl Add<NoteValue> for NoteValue {
    type Output = NoteValue;

    fn add(self, rhs: NoteValue) -> Self::Output {
        NoteValue(self.0 + rhs.0)
    }
}

impl Add<NoteValue> for Duration {
    type Output = Duration;

    fn add(self, rhs: NoteValue) -> Self::Output {
        Duration(self.0 + rhs, self.1)
    }
}


impl Mul<Ratio<MusicNat>> for NoteValue {
    type Output = NoteValue;

    fn mul(self, rhs: Ratio<MusicNat>) -> Self::Output {
        NoteValue(self.0 * rhs)
    }
}

impl Sub<NoteValue> for NoteValue {
    type Output = NoteValue;

    fn sub(self, rhs: NoteValue) -> Self::Output {
        if self.0 >= rhs.0 {
            NoteValue(self.0 - rhs.0)
        } else {
            panic!("Cannot subtract a larger NoteValue from a smaller one");
        }
    }
}

/// These are the unique pitch classes: [0, 12)
pub type PitchClassNum = u8;
/// This is to restrict conversions to u8 using well-formed methods here.
struct _PitchClassNum(PitchClassNum);

/// Anything beyond this is crazy. [-128, 128)
pub type Octave = i8;

pub type Frequency = f32;

pub enum PitchClass {
    A,
    ASharp,
    Bb,
    B,
    C,
    CSharp,
    Db,
    D,
    DSharp,
    Eb,
    E,
    F,
    FSharp,
    Gb,
    G,
    GSharp,
    Ab,
}

impl PitchClass {
    /// Convert to a number in [0, 12)
    /// Note that this class does not provide a public reverse mapping.
    pub fn to_note_num(&self) -> PitchClassNum {
        // C is 0
        match self {
            PitchClass::C => 0,
            PitchClass::CSharp | PitchClass::Db => 1,
            PitchClass::D => 2,
            PitchClass::DSharp | PitchClass::Eb => 3,
            PitchClass::E => 4,
            PitchClass::F => 5,
            PitchClass::FSharp | PitchClass::Gb => 6,
            PitchClass::G => 7,
            PitchClass::GSharp | PitchClass::Ab => 8,
            PitchClass::A => 9,
            PitchClass::ASharp | PitchClass::Bb => 10,
            PitchClass::B => 11,
        }
    }

    pub fn sharp(&self) -> PitchClass {
        PitchClass::from(_PitchClassNum((self.to_note_num() + 1) % 12))
    }

    pub fn flat(&self) -> PitchClass {
        PitchClass::from(_PitchClassNum((self.to_note_num() + 11) % 12))
    }

    pub fn at(self, octave: Octave) -> Pitch {
        Pitch::new(octave, self)
    }
}

impl From<_PitchClassNum> for PitchClass {
    fn from(p: _PitchClassNum) -> Self {
        match p.0 {
            0 => PitchClass::C,
            1 => PitchClass::CSharp,
            2 => PitchClass::D,
            3 => PitchClass::Eb,
            4 => PitchClass::E,
            5 => PitchClass::F,
            6 => PitchClass::FSharp,
            7 => PitchClass::G,
            8 => PitchClass::Ab,
            9 => PitchClass::A,
            10 => PitchClass::Bb,
            11 => PitchClass::B,
            _ => unreachable!(), // this will never be reached because this type is private
        }
    }
}

pub trait IntoPitchClassNum {
    fn into(self) -> _PitchClassNum;
}

impl<P: Into<PitchClass>> IntoPitchClassNum for P {
    fn into(self) -> _PitchClassNum {
        _PitchClassNum(self.into().to_note_num())
    }
}
macro_rules! impl_into_pitch_class_num_for_num {
    ($($t:ty),+ $(,)?) => {
        $(
            impl IntoPitchClassNum for $t {
                fn into(self) -> _PitchClassNum {
                    _PitchClassNum((self % 12) as PitchClassNum)
                }
            }
        )+
    };
}
macro_rules! impl_into_pitch_class_num_for_signed_num {
    ($($t:ty),+ $(,)?) => {
        $(
            impl IntoPitchClassNum for $t {
                fn into(self) -> _PitchClassNum {
                    _PitchClassNum(self.rem_euclid(12) as PitchClassNum)
                }
            }
        )+
    };
}

impl_into_pitch_class_num_for_num!(u8, u16, u32, u64, u128, usize);
impl_into_pitch_class_num_for_signed_num!(i8, i16, i32, i64, i128, isize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Pitch(Octave, PitchClassNum);

impl Pitch {
    /// Create a pitch. In order to ensure consistency, all pitch class numbers
    /// have to go through the IntoPitchClassNum trait to be properly converted.
    pub fn new<N: IntoPitchClassNum>(octave: Octave, note_num: N) -> Pitch {
        Pitch(octave, note_num.into().0)
    }

    pub fn sharp(self) -> Pitch {
        self.transposed(1)
    }

    pub fn flat(self) -> Pitch {
        self.transposed(-1)
    }

    pub fn to_frequency(&self) -> Frequency {
        let Pitch(octave, note_num) = *self;
        let note_num = note_num as f32;
        let octave = octave as f32;
        let frequency = 440.0 * 2f32.powf(octave - 4. + (note_num - 9.0) / 12.0);
        frequency
    }
    pub fn to_midi_note(&self) -> u8 {
        let Pitch(octave, note_num) = *self;
        let note_num = note_num as u8;
        let octave = octave as u8;
        octave * 12 + note_num + 9
    }

    pub fn letter_name(&self) -> String {
        let Pitch(_, note_num) = *self;
        let note_num = note_num as u8;
        match note_num {
            0 => "A",
            1 => "Bb",
            2 => "B",
            3 => "C",
            4 => "C#",
            5 => "D",
            6 => "Eb",
            7 => "E",
            8 => "F",
            9 => "F#",
            10 => "G",
            11 => "Ab",
            _ => panic!("Invalid note number")
        }.to_string()
    }

    pub fn transpose(&mut self, semitones: i8) {
        let Pitch(octave, note_num) = *self;
        let new_note_num = (note_num as i8 + semitones).rem_euclid(12) as u8;
        let new_octave = octave + ((note_num as i8 + semitones) as f32 / 12.).floor() as i8;
        *self = Pitch(new_octave, new_note_num);
    }

    pub fn transposed(&self, semitones: i8) -> Pitch {
        let mut new_pitch = self.clone();
        new_pitch.transpose(semitones);
        new_pitch
    }
}

#[cfg(test)]
mod composition_element_tests {
    use std::ops::Rem;
    use super::*;

    fn assert_epsilon_close(a: f32, b: f32) {
        if (a - b).abs() < 0.01 {
            ()
        } else {
            panic!("left={} is not close to right={}", a, b);
        }
    }

    #[test]
    fn test_pitch_to_frequency_1() {
        let pitch = Pitch(4, 0); // C4
        let frequency = pitch.to_frequency();
        assert_epsilon_close(frequency, 261.63);
    }

    #[test]
    fn test_pitch_to_frequency_2() {
        let pitch = Pitch(3, 0); // C3
        let frequency = pitch.to_frequency();
        assert_epsilon_close(frequency, 261.63 / 2.);
    }

    #[test]
    fn test_transpose_1() {
        let mut pitch = Pitch(4, 0); // C4
        pitch.transpose(2);
        assert_eq!(pitch, Pitch(4, 2)); // D4
    }

    #[test]
    fn test_transpose_2() {
        let mut pitch = Pitch(4, 0); // C4
        pitch.transpose(-1);
        assert_eq!(pitch, Pitch(3, 11)); // B3
    }

    #[test]
    fn test_transpose_3() {
        let mut pitch = Pitch(4, 2); // D4
        pitch.transpose(-7);
        assert_eq!(pitch, Pitch(3, 7)); // G3
    }

    #[test]
    fn test_transpose_4() {
        let mut pitch = Pitch(4, 0); // C4
        pitch.transpose(12);
        assert_eq!(pitch, Pitch(5, 0)); // C5
    }

    #[test]
    fn test() {
        assert_eq!((-1i8).rem_euclid(12), 11);
    }
}
