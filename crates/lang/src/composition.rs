use std::ops::{Add, Div, Mul, Sub};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize, Serializer};
use enumkit::EnumValues;
use num::{Integer, ToPrimitive, Zero};
use num::rational::Ratio;
use serde::ser::SerializeStruct;
use music_primitives::{Duration, Pitch, TimeSignature};

pub type Seconds = f32;

/// Represents either a duration or absolute position in music.
/// The measures must be positive. The beats should also be positive, and constrained
/// within the measure, if it is an absolute position.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct MusicTime(pub Measure, pub Beat);

pub type BPM = f32;

pub type Measure = u32;


pub type BeatUnit = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Beat(Ratio<BeatUnit>);



#[derive(Debug, Clone, Copy)]
pub struct TimeCompression(pub Ratio<isize>);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MusicTimeWithSignature {
    pub time: MusicTime,
    pub time_signature: TimeSignature,
}

impl Beat {
    pub fn new(num: BeatUnit, denom: BeatUnit) -> Self {
        Beat(Ratio::new(num, denom))
    }

    pub fn whole(num: BeatUnit) -> Self {
        Beat(Ratio::new(num, 1))
    }

    pub fn as_float(&self) -> f32 {
        self.0.to_f32().unwrap_or_else(|| {
            println!("WARNING: Beat {self:?} could not be converted to f32. Defaulting to 0.");
            0.
        })
    }

    pub fn as_music_time(&self, time_signature: TimeSignature) -> MusicTime {
        let measures = (self.0 / time_signature.0).floor().to_integer();
        let leftover = self.0 % time_signature.0;
        MusicTime(measures, Beat(leftover))
    }

    pub fn zero() -> Self {
        Beat(Ratio::zero())
    }

    pub fn numerator(&self) -> BeatUnit {
        self.0.numer().to_u32().unwrap_or_else(|| {
            println!("WARNING: Beat {self:?} numerator could not be converted to u32. Defaulting to 0.");
            0
        })
    }

    pub fn denominator(&self) -> BeatUnit {
        self.0.denom().to_u32().unwrap_or_else(|| {
            println!("WARNING: Beat {self:?} denominator could not be converted to u32. Defaulting to 1.");
            1
        })
    }
}

impl MusicTime {
    pub fn with(self, time_signature: TimeSignature) -> MusicTimeWithSignature {
        MusicTimeWithSignature {
            time_signature,
            time: self
        }
    }

    pub fn from_seconds(time_signature: TimeSignature, bpm: BPM, seconds: Seconds) -> Self {
        let bps = bpm / 60.;
        let beats = bps * seconds;
        // instead of using Ratio::from_f32, I'll calculate the fraction myself
        let precision = 1000000.0; // to avoid floating point precision issues
        let numerator = (beats * precision).floor() as BeatUnit;
        let denominator = precision as BeatUnit;
        let beats = Beat(Ratio::new(numerator, denominator));
        beats.as_music_time(time_signature)
    }

    pub fn from_whole_beats(time_signature: TimeSignature, beats: BeatUnit) -> Self {
        let measures = beats / time_signature.0;
        let beats = beats % time_signature.0;
        MusicTime(measures, Beat::whole(beats))
    }

    pub fn to_seconds(&self, time_signature: TimeSignature, bpm: BPM) -> Seconds {
        let MusicTime(measures, beats) = *self;
        let total_beats = (measures * time_signature.0) as f32 + beats.as_float();
        total_beats * 60. / bpm
    }

    pub fn zero() -> Self {
        MusicTime(0, Beat::zero())
    }

    pub fn beats(beats: BeatUnit) -> Self {
        MusicTime(0, Beat::whole(beats))
    }

    pub fn measures(measures: Measure) -> Self {
        MusicTime(measures, Beat::zero())
    }
}

impl Add<Beat> for Beat {
    type Output = Beat;

    fn add(self, rhs: Beat) -> Self::Output {
        Beat(self.0 + rhs.0)
    }
}

impl Sub<Beat> for Beat {
    type Output = Beat;

    fn sub(self, rhs: Beat) -> Self::Output {
        Beat(self.0 - rhs.0)
    }
}

impl Add<MusicTime> for MusicTimeWithSignature {
    type Output = MusicTime;

    fn add(self, rhs: MusicTime) -> Self::Output {
        let MusicTime(measure, beat) = self.time;
        let MusicTime(measure2, beat2) = rhs;
        let new_measure = measure + measure2;
        let MusicTime(beat_measures, beat) = (beat + beat2).as_music_time(self.time_signature);
        MusicTime(new_measure + beat_measures, beat)
    }
}

impl Sub<MusicTime> for MusicTimeWithSignature {
    type Output = MusicTime;

    fn sub(self, rhs: MusicTime) -> Self::Output {
        let MusicTime(measure, beat) = self.time;
        let MusicTime(measure2, beat2) = rhs;
        let mut new_measure = measure - measure2;
        let mut new_beat = beat;
        if beat2 > new_beat {
            new_beat = new_beat + Beat::whole(self.time_signature.0);
            new_measure -= 1;
        }
        new_beat = new_beat - beat2;
        let MusicTime(beat_measures, beats) = new_beat.as_music_time(self.time_signature);
        MusicTime(beat_measures + new_measure, beats)
    }
}

impl Mul<Ratio<BeatUnit>> for MusicTimeWithSignature {
    type Output = MusicTimeWithSignature;

    fn mul(self, rhs: Ratio<BeatUnit>) -> Self::Output {
        let total_beats = self.total_beats();
        let new_total_beats = Beat(total_beats.0 * rhs);
        let music_time = new_total_beats.as_music_time(self.time_signature);
        MusicTimeWithSignature {
            time: music_time,
            time_signature: self.time_signature,
        }
    }
}

impl MusicTimeWithSignature {
    pub fn total_beats(&self) -> Beat {
        Beat::new(self.time.0 * self.time_signature.0 as BeatUnit, 1) + self.time.1
    }
}

impl Serialize for Beat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("Beat", 2)?;
        let num = self.numerator();
        let denom = self.denominator();
        state.serialize_field("numerator", &num)?;
        state.serialize_field("denominator", &denom)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Beat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Beat_ {
            numerator: u32,
            denominator: u32,
        }

        let data = Beat_::deserialize(deserializer)?;
        Ok(Beat::new(data.numerator, data.denominator))
    }
}

impl Serialize for TimeCompression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("TimeCompression", 1)?;
        state.serialize_field("numerator", &self.0.numer())?;
        state.serialize_field("denominator", &self.0.denom())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for TimeCompression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct TimeCompression {
            numerator: isize,
            denominator: isize,
        }

        let data = TimeCompression::deserialize(deserializer)?;
        Ok(TimeCompression(Ratio::new(data.numerator, data.denominator)))
    }
}

impl Display for TimeCompression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0.numer(), self.0.denom())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_music_time() {
        let mt1 = MusicTime(0, Beat::new(0, 1));
        let mt2 = MusicTime(0, Beat::new(1191, 23819));
        assert!(mt1 < mt2);
        assert!(mt2 > mt1);
    }

    #[test]
    fn test_music_time_sub_1() {
        let ts = TimeSignature::common();
        let mt1 = MusicTime(1, Beat::whole(0));
        let mt2 = MusicTime(0, Beat::whole(3));
        assert_eq!(mt1.with(ts) - mt2, MusicTime(0, Beat::whole(1)));
    }

    #[test]
    fn test_music_time_sub_2() {
        let ts = TimeSignature::common();
        let mt1 = MusicTime(1, Beat::whole(3));
        let mt2 = MusicTime(0, Beat::whole(0));
        assert_eq!(mt1.with(ts) - mt2, MusicTime(1, Beat::whole(3)));
    }

    #[test]
    fn test_music_time_sub_3() {
        let ts = TimeSignature::common();
        let mt1 = MusicTime(2, Beat::whole(0));
        let mt2 = MusicTime(0, Beat::whole(3));
        assert_eq!(mt1.with(ts) - mt2, MusicTime(1, Beat::whole(1)));
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize, EnumValues)]
pub enum Instrument {
    SineWave,
    Piano,
    Bass,
    // percussion
    BassDrum,
    HiHatOpen,
    HiHatClosed,
    Snare,
    Snare2,
    BongoHigh,
    BongoLow,
    Shaker1,
    Shaker2,
}

impl Instrument {
    pub fn is_percussion(&self) -> bool {
        // matches!(self, Instrument::Drum | Instrument::Snare | Instrument::Cymbal)
        false
    }
    pub fn str_values() -> impl Iterator<Item=(Instrument, String)> {
        Instrument::values()
            .map(|i| (i, format!("{:?}", i)))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TrackId {
    Instrument(Instrument),
    Custom(usize),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Track {
    pub identifier: TrackId,
    pub instrument: Instrument,
    pub events: Vec<Event>,
    pub rests: Vec<Event>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Event {
    pub start: MusicTime,
    pub duration: Duration,
    pub volume: Volume,
    pub pitch: Pitch,
}

pub const MAX_VOLUME: u32 = 100;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Volume(pub u32);

impl Volume {
    pub fn as_f32(&self) -> f32 {
        self.0 as f32 / MAX_VOLUME as f32
    }
}

impl Event {
    pub fn get_end(&self, time_signature: TimeSignature) -> MusicTime {
        self.start.with(time_signature) + self.duration.as_music_time(time_signature)
    }
}

// weird that option doesn't work like this
fn min_option<T: Ord>(a: Option<T>, b: Option<T>) -> Option<T> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.min(y)),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}
fn max_option<T: Ord>(a: Option<T>, b: Option<T>) -> Option<T> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.max(y)),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

impl Track {
    pub fn visualize(&self, columns: usize, time_signature: TimeSignature, start: MusicTime, end: MusicTime) -> String {
        let mut s = String::new();
        s.push('[');
        let bpm = 1.;
        let start_time = start.to_seconds(time_signature, bpm);
        let end_time = end.to_seconds(time_signature, bpm);
        for i in 0..columns {
            let time = start_time + (end_time - start_time) * i as f32 / columns as f32;
            let mt = MusicTime::from_seconds(time_signature, bpm, time);
            let evts = self.get_events_at(mt, time_signature);
            let rest_evts = self.get_rests_at(mt, time_signature);
            if evts.is_empty() {
                if rest_evts.is_empty() {
                    s.push(' ');
                } else {
                    s.push('-');
                }
            } else {
                if rest_evts.is_empty() {
                    s.push('X');
                } else {
                    s.push('?');
                }
            }
        }
        s.push(']');
        s
    }
    fn get_events_at(&self, time: MusicTime, time_signature: TimeSignature) -> Vec<Event> {
        self.events.iter()
            .filter(|e| time >= e.start && time <= e.get_end(time_signature))
            .map(|e| *e)
            .collect()
    }
    fn get_rests_at(&self, time: MusicTime, time_signature: TimeSignature) -> Vec<Event> {
        self.rests.iter()
            .filter(|e| time >= e.start && time <= e.get_end(time_signature))
            .map(|e| *e)
            .collect()
    }
    pub fn get_start(&self) -> Option<MusicTime> {
        min_option(self.events.iter()
                       .map(|e| e.start)
                       .min(), self.rests.iter()
                       .map(|e| e.start)
                       .min())
    }
    pub fn get_end(&self, time_signature: TimeSignature) -> Option<MusicTime> {
        max_option(self.events.iter()
                       .map(|e| e.get_end(time_signature))
                       .max(), self.rests.iter()
                       .map(|e| e.get_end(time_signature))
                       .max())
    }

    pub fn get_duration(&self, time_signature: TimeSignature) -> MusicTime {
        self.get_start()
            .map(|start| self.get_end(time_signature).map(
                |end|
                    end.with(time_signature) - start
            ))
            .flatten()
            .unwrap_or(MusicTime::zero())
    }

    /// End is always inclusive
    /// Doesn't include rests
    pub fn get_events_starting_between(&self, start: MusicTime, end: MusicTime, start_exclusive: bool) -> Vec<Event> {
        if (start_exclusive && start >= end) || start > end {
            return Vec::new();
        }
        let mut es = self.events.iter()
            .filter(|e| if start_exclusive {
                start < e.start
            } else {
                start <= e.start
            } && e.start <= end)
            .map(|e| *e)
            .collect::<Vec<_>>();
        es.sort();
        es
    }

    pub fn shift_by(&mut self, offset: MusicTime, time_signature: TimeSignature) {
        self.events.iter_mut()
            .chain(self.rests.iter_mut())
            .for_each(|e|
                e.start = e.start.with(time_signature) + offset
            );
    }

    pub fn transpose(&mut self, semitones: i8) {
        for event in &mut self.events {
            event.pitch.transpose(semitones);
        }
    }

    /// Flip entire track, keeping it within its start/end bounds.
    pub fn reverse(&mut self, time_signature: TimeSignature) {
        if let (Some(start), Some(end)) = (self.get_start(), self.get_end(time_signature)) {
            self.events.iter_mut()
                .chain(self.rests.iter_mut())
                .for_each(|e| {
                    let offset = e.start.with(time_signature) - start;
                    let new_start = (end.with(time_signature) - offset).with(time_signature) - e.duration.as_music_time(time_signature);
                    e.start = new_start;
                });
            self.events.reverse();
            self.rests.reverse();
        }
    }

    /// Compress all timings by the compression factor.
    /// Example: if the factor is 0.5, it will compress the track to half its length.
    pub fn compress(&mut self, time_signature: TimeSignature, compression: TimeCompression) {
        let factor = compression.0;
        if factor < Ratio::new(0, 1) {
            self.reverse(time_signature);
        }
        let factor = Ratio::new(factor.numer().abs() as BeatUnit, factor.denom().abs() as BeatUnit);
        if let (Some(start), Some(end)) = (self.get_start(), self.get_end(time_signature)) {
            self.events.iter_mut()
                .chain(self.rests.iter_mut())
                .for_each(|e| {
                    let offset = (e.start.with(time_signature) - start).with(time_signature) * factor;
                    e.start = start.with(time_signature) + offset.time;
                    e.duration = (e.duration.as_music_time(time_signature).with(time_signature) * factor).total_beats();
                });
        }
    }
}

impl Add<Self> for Track {
    type Output = Track;

    fn add(self, rhs: Self) -> Self::Output {
        if self.instrument != rhs.instrument {
            panic!("not the same instruments!");
        }
        let mut events = self.events;
        for event in rhs.events {
            events.push(event);
        }
        events.sort();
        let mut rests = self.rests;
        for rest in rhs.rests {
            rests.push(rest);
        }
        rests.sort();
        Track {
            identifier: self.identifier,
            instrument: self.instrument,
            events,
            rests,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Composition {
    pub tracks: Vec<Track>,
    pub time_signature: TimeSignature,
}

impl Composition {
    pub fn visualize(&self, columns: usize) -> String {
        let mut s = String::new();
        let start = MusicTime::zero();
        let end = if let Some(end) = self.get_end() {
            end
        } else {
            return "[No music in this composition]".to_string();
        };
        for track in &self.tracks {
            // pad name of track to fit in the columns
            s.push_str(&format!("{:>12} ", track.identifier.to_string()));
            // print number of events
            // s.push_str(&format!("({:>5} events) ", track.events.len()));
            s.push_str(&track.visualize(columns, self.time_signature, start, end));
            s.push('\n');
        }
        s
    }
    pub fn get_duration(&self) -> MusicTime {
        let start = self.tracks.iter().filter_map(|t| t.get_start())
            .min();
        let end = self.tracks.iter().filter_map(|t| t.get_end(self.time_signature))
            .max();
        match (start, end) {
            (Some(start), Some(end)) => end.with(self.time_signature) - start,
            _ => MusicTime::zero()
        }
    }

    pub fn get_start(&self) -> Option<MusicTime> {
        self.tracks.iter()
            .filter_map(|t| t.get_start())
            .min()
    }

    pub fn get_end(&self) -> Option<MusicTime> {
        self.tracks.iter()
            .filter_map(|t| t.get_end(self.time_signature))
            .max()
    }

    pub fn shift_by(&mut self, offset: MusicTime) {
        self.tracks.iter_mut()
            .for_each(|tr| tr.shift_by(offset, self.time_signature));
    }

    pub fn transpose(&mut self, semitones: i8) {
        for track in &mut self.tracks {
            track.transpose(semitones);
        }
    }

    /// Compress all timings by the compression factor toward the start of the track.
    /// If the factor is negative, it will reverse the track.
    /// Example, if the factor is 0.5, it will compress the track to half its length.
    pub fn compress(&mut self, compression: TimeCompression) {
        for track in &mut self.tracks {
            track.compress(self.time_signature, compression);
        }
    }
}

impl Add<Self> for Composition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.time_signature != rhs.time_signature {
            panic!("differing time signatures!!");
        }
        let mut map = HashMap::new();
        for track in self.tracks {
            let id = track.identifier;
            if let Some(mtrack) = map.remove(&id) {
                let new_track = mtrack + track;
                map.insert(id, new_track);
            } else {
                map.insert(id, track);
            }
        }
        Composition {
            tracks: map.into_values().collect(),
            time_signature: self.time_signature,
        }
    }
}

impl FromStr for Instrument {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "piano" => Ok(Instrument::Piano),
            s => {
                let instrument_enum: HashMap<_, _> = Instrument::str_values()
                    .map(|(i, i_name)| (i_name.to_ascii_lowercase(), i))
                    .collect();
                instrument_enum.get(s)
                    .map(|i| *i)
                    .ok_or(format!("Unknown instrument: {}", s))
            }
        }
    }
}

mod composition_element_tests {
    use num::rational::Ratio;
    use music_primitives::TimeSignature;
    use crate::composition::{Beat, Composition, Event, Instrument, MusicTime, Pitch, TimeCompression, Track, TrackId, Volume};

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

    fn comp_template(events: Vec<Event>) -> Composition {
        Composition {
            tracks: vec![
                Track {
                    identifier: TrackId::Custom(0),
                    instrument: Instrument::SineWave,
                    events,
                    rests: vec![],
                }
            ],
            time_signature: TimeSignature::common(),
        }
    }

    #[test]
    fn test_compression_1() {
        let compression = TimeCompression(Ratio::new(1, 2)); // 50% compression
        let mut composition1 = comp_template(vec![
            Event {
                start: MusicTime::measures(1),
                duration: Beat::whole(2),
                volume: Volume(100),
                pitch: Pitch(4, 0),
            }
        ]);
        let composition_half = comp_template(vec![
            Event {
                start: MusicTime::measures(1),
                duration: Beat::whole(1),
                volume: Volume(100),
                pitch: Pitch(4, 0),
            }
        ]);
        composition1.compress(compression);
        assert_eq!(composition1, composition_half);
    }

    #[test]
    fn test_compression_2() {
        let compression = TimeCompression(Ratio::new(-1, 1)); // -100% compression (reverse)
        let mut composition1 = comp_template(vec![
            Event {
                start: MusicTime::measures(1),
                duration: Beat::whole(2),
                volume: Volume(100),
                pitch: Pitch(4, 0),
            }
        ]);
        let composition_reversed = comp_template(vec![
            Event {
                start: MusicTime::measures(1),
                duration: Beat::whole(2),
                volume: Volume(100),
                pitch: Pitch(4, 0),
            }
        ]);
        composition1.compress(compression);
        assert_eq!(composition1, composition_reversed);
    }

    #[test]
    fn test_compression_3() {
        let compression = TimeCompression(Ratio::new(-1, 1)); // -100% compression (reverse)
        let mut composition1 = comp_template(vec![
            Event {
                start: MusicTime(1, Beat::whole(0)),
                duration: Beat::whole(1),
                volume: Volume(100),
                pitch: Pitch(4, 0),
            },
            Event {
                start: MusicTime(1, Beat::whole(1)),
                duration: Beat::whole(1),
                volume: Volume(100),
                pitch: Pitch(4, 1),
            }
        ]);
        let composition_reversed = comp_template(vec![
            Event {
                start: MusicTime(1, Beat::whole(0)),
                duration: Beat::whole(1),
                volume: Volume(100),
                pitch: Pitch(4, 1),
            },
            Event {
                start: MusicTime(1, Beat::whole(1)),
                duration: Beat::whole(1),
                volume: Volume(100),
                pitch: Pitch(4, 0),
            }
        ]);
        composition1.compress(compression);
        assert_eq!(composition1, composition_reversed);
    }

    #[test]
    fn test_compression_4() {
        let compression = TimeCompression(Ratio::new(1, 2)); // 50% compression
        let mut composition1 = comp_template(vec![
            Event {
                start: MusicTime(1, Beat::whole(0)),
                duration: Beat::whole(2),
                volume: Volume(100),
                pitch: Pitch(4, 0),
            },
            Event {
                start: MusicTime(1, Beat::whole(2)),
                duration: Beat::whole(2),
                volume: Volume(100),
                pitch: Pitch(4, 1),
            }
        ]);
        let composition_half = comp_template(vec![
            Event {
                start: MusicTime(1, Beat::whole(0)),
                duration: Beat::whole(1),
                volume: Volume(100),
                pitch: Pitch(4, 0),
            },
            Event {
                start: MusicTime(1, Beat::whole(1)),
                duration: Beat::whole(1),
                volume: Volume(100),
                pitch: Pitch(4, 1),
            }
        ]);
        composition1.compress(compression);
        assert_eq!(composition1, composition_half);
    }
}

impl Display for TrackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackId::Instrument(instrument) => write!(f, "{:?}", instrument),
            TrackId::Custom(id) => write!(f, "Custom({})", id),
        }
    }
}