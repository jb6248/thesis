use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub};
use serde::{Deserialize, Serialize};
use num::rational::Ratio;
use num::{ToPrimitive, Zero};
use serde::ser::SerializeStruct;
// Assuming a standard 12-tone equal temperament system

/// Includes 0.
pub type MusicNat = u32;
pub type Measures = MusicNat;

pub type RestValue = NoteValue;

/// Can only be positive. 1/8 represents an eighth note, for example.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct NoteValue(pub Ratio<MusicNat>);

pub type Beats = Ratio<MusicNat>;


/// The number of beats (duration) depends on the note value and the time signature.
/// This is also used for representing offsets in time.
/// This is a durable type with robust mathematical properties.
/// It will not be negative.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Duration {
    pub value: NoteValue,
    pub time_signature: TimeSignature,
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let measures = self.get_whole_measures();
        let beats = self.get_rem_beats();

        if measures > 0 {
            write!(f, "{}m", measures)?;
            if beats > Beats::zero() {
                write!(f, "+")?;
            }
        }
        if beats > Beats::zero() {
            if *beats.denom() == 1 {
                write!(f, "{}", beats.numer())?;
            } else {
                write!(f, "{}/{}", beats.numer(), beats.denom())?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TimeSignature(pub MusicNat, pub MusicNat);
impl TimeSignature {
    pub fn common() -> TimeSignature {
        TimeSignature(4, 4)
    }
    pub fn waltz() -> TimeSignature {
        TimeSignature(3, 4)
    }
    pub fn beat_note_value(&self) -> NoteValue {
        NoteValue(Ratio::new(1, self.1))
    }

    pub fn as_note_value(&self, beats: Beats) -> NoteValue {
        NoteValue(self.beat_note_value().0 * beats)
    }
}
impl NoteValue {
    pub fn new(numerator: MusicNat, denominator: MusicNat) -> NoteValue {
        NoteValue(Ratio::new(numerator, denominator))
    }
    pub fn with(&self, time_signature: TimeSignature) -> Duration {
        Duration {
            value: *self,
            time_signature: time_signature
        }
    }
}
impl Duration {
    pub fn zero(time_signature: TimeSignature) -> Duration {
        Duration {
            value: NoteValue::new(0, 1),
            time_signature: time_signature
        }
    }

    pub fn is_zero(&self) -> bool {
        self.value.0.is_zero()
    }

    pub fn from_beats_with_ts(beats: Ratio<MusicNat>, time_signature: TimeSignature) -> Duration {
        // 1 beat in 3/4 is 1 quarter note (1/4)
        // 3 beats in 3/8 is 3 eighth notes (3/8)
        let note_value = NoteValue(Ratio::new(1, time_signature.1) * beats);
        Duration {
            value: note_value,
            time_signature
        }
    }

    pub fn measures_and_beats_with_ts(measures: Measures, beats: Ratio<MusicNat>, time_signature: TimeSignature) -> Duration {
        let TimeSignature(num, _) = time_signature;
        let total_beats = Ratio::from_integer(measures) * num + beats;
        Duration {
            value: time_signature.as_note_value(total_beats),
            time_signature
        }
    }

    pub fn measures_with_ts(measures: Measures, time_signature: TimeSignature) -> Duration {
        Self::measures_and_beats_with_ts(measures, Ratio::from_integer(0), time_signature)
    }

    /// Get the total number of beats in this duration
    pub fn get_beats(&self) -> Ratio<MusicNat> {
        // 4 on the bottom of the time signature represents quarter notes (NoteValue = 1/4)
        // it would be NoteValue / (1 / time_signature.1) = NoteValue * time_signature.1
        let TimeSignature(_, denom) = self.time_signature;
        let Duration { value: note_value, time_signature: _ } = self;
        note_value.0 * denom
    }

    /// Get the total number of measures in this duration (exact)
    pub fn get_measures(&self) -> Ratio<MusicNat> {
        // number of beats divided by number of beats per measure
        let TimeSignature(num, _) = self.time_signature;
        self.get_beats() / num
    }

    /// Get the whole number of measures in this duration
    pub fn get_whole_measures(&self) -> MusicNat {
        self.get_measures().to_integer()
    }

    /// Get the remaining beats that do not make up a whole measure
    pub fn get_rem_beats(&self) -> Ratio<MusicNat> {
        let TimeSignature(num, _) = self.time_signature;
        self.get_beats() % num
    }

    /// Get the total number of seconds in this duration at the given BPM
    pub fn to_seconds(&self, bpm: f32) -> f32 {
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

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
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
        Duration {
            value: self.value + rhs,
            time_signature: self.time_signature
        }
    }
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Self::Output {
        if self.time_signature != rhs.time_signature {
            panic!("Cannot add Durations with different time signatures");
        }
        Duration {
            value: self.value + rhs.value,
            time_signature: self.time_signature
        }
    }
}

impl AddAssign<Duration> for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        self.value = self.value + rhs.value;
    }
}

impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Self::Output {
        if self.time_signature != rhs.time_signature {
            panic!("Cannot subtract Durations with different time signatures");
        }
        Duration {
            value: self.value - rhs.value,
            time_signature: self.time_signature
        }
    }
}


impl Mul<Ratio<MusicNat>> for NoteValue {
    type Output = NoteValue;

    fn mul(self, rhs: Ratio<MusicNat>) -> Self::Output {
        NoteValue(self.0 * rhs)
    }
}

impl Mul<MusicNat> for NoteValue {
    type Output = NoteValue;

    fn mul(self, rhs: MusicNat) -> Self::Output {
        self * Ratio::from_integer(rhs)
    }
}

impl Mul<Ratio<MusicNat>> for Duration {
    type Output = Duration;

    fn mul(self, rhs: Ratio<MusicNat>) -> Self::Output {
        Duration {
            value: self.value * rhs,
            time_signature: self.time_signature
        }
    }
}

impl Mul<MusicNat> for Duration {
    type Output = Duration;

    fn mul(self, rhs: MusicNat) -> Self::Output {
        self * Ratio::from_integer(rhs)
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
/// See PitchClass for mapping to names.
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
    /// A handy impl is From<YourType> for PitchClass which you can use to extend the types allowed.
    pub fn new<N: IntoPitchClassNum>(octave: Octave, note_num: N) -> Pitch {
        Pitch(octave, note_num.into().0)
    }
    
    pub fn octave(&self) -> Octave {
        self.0
    }
    
    pub fn note_num(&self) -> PitchClassNum {
        self.1
    }
    
    pub fn data(&self) -> (Octave, PitchClassNum) {
        (self.0, self.1)
    }
    
    pub fn middle_c() -> Pitch {
        Pitch::new(4, PitchClass::C) // C4
    }

    pub fn none() -> Pitch {
        Pitch(0, 0) // C0 by default
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
