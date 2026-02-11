use enumkit::EnumValues;
use music_primitives::{Duration, MusicNat, Pitch, TimeSignature};
use num::rational::Ratio;
use num::{Integer, ToPrimitive, Zero};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct TimeCompression(pub Ratio<isize>);

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
    use music_primitives::Beats;

    #[test]
    fn test_music_time() {
        let time_signature = TimeSignature::common();
        let mt1 = Duration::measures_and_beats_with_ts(0, Beats::new(0, 1), time_signature);
        let mt2 = Duration::measures_and_beats_with_ts(0, Beats::new(1191, 23819), time_signature);
        assert!(mt1 < mt2);
        assert!(mt2 > mt1);
    }

    // #[test]
    // fn test_music_time_sub_1() {
    //     let ts = TimeSignature::common();
    //     let mt1 = Duration::measures_and_beats_with_ts(1, Beats::from_integer(0), ts);
    //     let mt2 = Duration::measures_and_beats_with_ts(0, Beats::from_integer(3), ts);
    //     assert_eq!(mt1 - mt2, Duration::measures_and_beats_with_ts(0, Beats::from_integer(1)));
    // }
    //
    // #[test]
    // fn test_music_time_sub_2() {
    //     let ts = TimeSignature::common();
    //     let mt1 = Duration(1, Beats::from_integer(3));
    //     let mt2 = Duration(0, Beats::from_integer(0));
    //     assert_eq!(mt1.with(ts) - mt2, Duration(1, Beats::from_integer(3)));
    // }
    //
    // #[test]
    // fn test_music_time_sub_3() {
    //     let ts = TimeSignature::common();
    //     let mt1 = Duration(2, Beats::from_integer(0));
    //     let mt2 = Duration(0, Beats::from_integer(3));
    //     assert_eq!(mt1.with(ts) - mt2, Duration(1, Beats::from_integer(1)));
    // }
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
    pub start: Duration,
    pub duration: Duration,
    pub volume: Volume,
    pub pitch: Pitch,
}

pub const MAX_VOLUME: u32 = 100;


/// Between 0 and MAX_VOLUME
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Volume(pub u32);

impl Volume {
    pub fn as_f32(&self) -> f32 {
        self.0 as f32 / MAX_VOLUME as f32
    }
}

impl Event {
    pub fn get_end(&self, time_signature: TimeSignature) -> Duration {
        self.start + self.duration
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
    pub fn visualize(&self, columns: usize, time_signature: TimeSignature, start: Duration, end: Duration) -> String {
        let mut s = String::new();
        s.push('[');
        let bpm = 1.;
        let total_beats = self.get_duration(time_signature).get_beats();
        for i in 0..columns {
            let time = total_beats / (columns as MusicNat) * (i as MusicNat);
            let mt = Duration::from_beats_with_ts(time, time_signature);
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
    fn get_events_at(&self, time: Duration, time_signature: TimeSignature) -> Vec<Event> {
        self.events.iter()
            .filter(|e| time >= e.start && time <= e.get_end(time_signature))
            .map(|e| *e)
            .collect()
    }
    fn get_rests_at(&self, time: Duration, time_signature: TimeSignature) -> Vec<Event> {
        self.rests.iter()
            .filter(|e| time >= e.start && time <= e.get_end(time_signature))
            .map(|e| *e)
            .collect()
    }

    /// Get start of track (including rests)
    pub fn get_start(&self) -> Option<Duration> {
        min_option(self.events.iter()
                       .map(|e| e.start)
                       .min(), self.rests.iter()
                       .map(|e| e.start)
                       .min())
    }

    /// Get end of track (including rests)
    pub fn get_end(&self, time_signature: TimeSignature) -> Option<Duration> {
        max_option(self.events.iter()
                       .map(|e| e.get_end(time_signature))
                       .max(), self.rests.iter()
                       .map(|e| e.get_end(time_signature))
                       .max())
    }

    /// Validate the track by making sure that there are no overlaps or gaps between
    /// the start and the end. If there are gaps, they should be filled with rests, but
    /// this method does not do that.
    pub fn validate_contiguous(&self) -> bool {
        let mut all_events = self.events.clone();
        all_events.sort_by(|a, b| a.start.cmp(&b.start));
        if all_events.is_empty() {
            return true;
        }
        let mut current_time = all_events[0].start;
        for event in all_events {
            if event.start > current_time {
                return false; // gap
            }
            if event.start < current_time {
                return false; // overlap
            }
            current_time = event.get_end(current_time.time_signature);
        }
        true
    }

    pub fn get_duration(&self, time_signature: TimeSignature) -> Duration {
        self.get_start()
            .map(|start| self.get_end(time_signature).map(
                |end|
                    end - start
            ))
            .flatten()
            .unwrap_or(Duration::zero(time_signature))
    }

    /// End is always inclusive
    /// Doesn't include rests
    pub fn get_events_starting_between(&self, start: Duration, end: Duration, start_exclusive: bool) -> Vec<Event> {
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

    pub fn shift_by(&mut self, offset: Duration, insert_rests: bool) {
        if let Some(previous_start) = self.get_start() {
            self.events.iter_mut()
                .chain(self.rests.iter_mut())
                .for_each(|e|
                    e.start += offset
                );
            if !offset.is_zero() && insert_rests {
                // insert a rest at the beginning to fill the gap
                let rest_event = Event {
                    start: previous_start,
                    duration: offset,
                    volume: Volume(0),
                    pitch: Pitch::none(), // pitch doesn't matter for rests
                };
                self.rests.insert(0, rest_event);
            }
        }
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
                    let offset = e.start - start;
                    let new_start = (end - offset) - e.duration;
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
        let factor = Ratio::new(factor.numer().abs() as MusicNat, factor.denom().abs() as MusicNat);
        if let (Some(start), Some(end)) = (self.get_start(), self.get_end(time_signature)) {
            self.events.iter_mut()
                .chain(self.rests.iter_mut())
                .for_each(|e| {
                    let offset = (e.start - start) * factor;
                    e.start = start + offset;
                    e.duration = e.duration * factor;
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
        let start = Duration::zero(self.time_signature);
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
    
    pub fn add_rests_to_last_measure(&mut self) -> Option<()> {
        let end = self.get_end()?;
        let last_measure_start = Duration::measures_with_ts(end.get_whole_measures(), self.time_signature);
        let remaining_beats = end - last_measure_start;
        let rounded_end = last_measure_start + if !remaining_beats.is_zero() {
            Duration::measures_with_ts(1, self.time_signature)
        } else {
            Duration::zero(self.time_signature)
        };
        for track in &mut self.tracks {
            if let Some(track_end) = track.get_end(self.time_signature) {
                if track_end < rounded_end {
                    let rest_start = track_end;
                    let rest_duration = rounded_end - track_end;
                    let rest_event = Event {
                        start: rest_start,
                        duration: rest_duration,
                        volume: Volume(0),
                        pitch: Pitch::none(), // pitch doesn't matter for rests
                    };
                    track.rests.push(rest_event);
                }
            }
        }
        Some(())
    }
    pub fn get_duration(&self) -> Duration {
        let start = self.tracks.iter().filter_map(|t| t.get_start())
            .min();
        let end = self.tracks.iter().filter_map(|t| t.get_end(self.time_signature))
            .max();
        match (start, end) {
            (Some(start), Some(end)) => end - start,
            _ => Duration::zero(self.time_signature)
        }
    }

    pub fn get_start(&self) -> Option<Duration> {
        self.tracks.iter()
            .filter_map(|t| t.get_start())
            .min()
    }

    pub fn get_end(&self) -> Option<Duration> {
        self.tracks.iter()
            .filter_map(|t| t.get_end(self.time_signature))
            .max()
    }

    pub fn shift_by(&mut self, offset: Duration, insert_rests: bool) {
        self.tracks.iter_mut()
            .for_each(|tr| tr.shift_by(offset, insert_rests));
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

#[cfg(test)]
mod composition_element_tests {
    use crate::composition::{Composition, Duration, Event, Instrument, Pitch, TimeCompression, Track, TrackId, Volume};
    use music_primitives::{Beats, TimeSignature};
    use num::rational::Ratio;
    use crate::cfg::{MusicPrimitive, MusicString, Performer, Symbol, Terminal};

    fn assert_epsilon_close(a: f32, b: f32) {
        if (a - b).abs() < 0.01 {
            ()
        } else {
            panic!("left={} is not close to right={}", a, b);
        }
    }

    #[test]
    fn test_pitch_to_frequency_1() {
        let pitch = Pitch::new(4, 0); // C4
        let frequency = pitch.to_frequency();
        assert_epsilon_close(frequency, 261.63);
    }

    #[test]
    fn test_pitch_to_frequency_2() {
        let pitch = Pitch::new(3, 0); // C3
        let frequency = pitch.to_frequency();
        assert_epsilon_close(frequency, 261.63 / 2.);
    }

    #[test]
    fn test_transpose_1() {
        let mut pitch = Pitch::new(4, 0); // C4
        pitch.transpose(2);
        assert_eq!(pitch, Pitch::new(4, 2)); // D4
    }

    #[test]
    fn test_transpose_2() {
        let mut pitch = Pitch::new(4, 0); // C4
        pitch.transpose(-1);
        assert_eq!(pitch, Pitch::new(3, 11)); // B3
    }

    #[test]
    fn test_transpose_3() {
        let mut pitch = Pitch::new(4, 2); // D4
        pitch.transpose(-7);
        assert_eq!(pitch, Pitch::new(3, 7)); // G3
    }

    #[test]
    fn test_transpose_4() {
        let mut pitch = Pitch::new(4, 0); // C4
        pitch.transpose(12);
        assert_eq!(pitch, Pitch::new(5, 0)); // C5
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
        let ts = TimeSignature::common();
        let compression = TimeCompression(Ratio::new(1, 2)); // 50% compression
        let mut composition1 = comp_template(vec![
            Event {
                start: Duration::measures_with_ts(1, ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(2), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 0),
            }
        ]);
        let composition_half = comp_template(vec![
            Event {
                start: Duration::measures_with_ts(1, ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(1), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 0),
            }
        ]);
        composition1.compress(compression);
        assert_eq!(composition1, composition_half);
    }

    #[test]
    fn test_compression_2() {
        let ts = TimeSignature::common();
        let compression = TimeCompression(Ratio::new(-1, 1)); // -100% compression (reverse)
        let mut composition1 = comp_template(vec![
            Event {
                start: Duration::measures_with_ts(1, ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(2), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 0),
            }
        ]);
        let composition_reversed = comp_template(vec![
            Event {
                start: Duration::measures_with_ts(1, ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(2), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 0),
            }
        ]);
        composition1.compress(compression);
        assert_eq!(composition1, composition_reversed);
    }

    #[test]
    fn test_compression_3() {
        let ts = TimeSignature::common();
        let compression = TimeCompression(Ratio::new(-1, 1)); // -100% compression (reverse)
        let mut composition1 = comp_template(vec![
            Event {
                start: Duration::measures_and_beats_with_ts(1,  Beats::from_integer(0), ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(1), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 0),
            },
            Event {
                start: Duration::measures_and_beats_with_ts(1,  Beats::from_integer(1), ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(1), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 1),
            }
        ]);
        let composition_reversed = comp_template(vec![
            Event {
                start: Duration::measures_and_beats_with_ts(1,  Beats::from_integer(0), ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(1), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 1),
            },
            Event {
                start: Duration::measures_and_beats_with_ts(1,  Beats::from_integer(1), ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(1), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 0),
            }
        ]);
        composition1.compress(compression);
        assert_eq!(composition1, composition_reversed);
    }

    #[test]
    fn test_compression_4() {
        let ts = TimeSignature::common();
        let compression = TimeCompression(Ratio::new(1, 2)); // 50% compression
        let mut composition1 = comp_template(vec![
            Event {
                start: Duration::measures_and_beats_with_ts(1,  Beats::from_integer(0), ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(2), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 0),
            },
            Event {
                start: Duration::measures_and_beats_with_ts(1,  Beats::from_integer(2), ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(2), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 1),
            }
        ]);
        let composition_half = comp_template(vec![
            Event {
                start: Duration::measures_and_beats_with_ts(1,  Beats::from_integer(0), ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(1), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 0),
            },
            Event {
                start: Duration::measures_and_beats_with_ts(1,  Beats::from_integer(1), ts),
                duration: Duration::from_beats_with_ts(Beats::from_integer(1), ts),
                volume: Volume(100),
                pitch: Pitch::new(4, 1),
            }
        ]);
        composition1.compress(compression);
        assert_eq!(composition1, composition_half);
    }

    #[test]
    fn test_compose_v2_1() {
        let ts = TimeSignature::common();
        let music_string = MusicString(vec![
            MusicPrimitive::Simple(Symbol::T(Terminal::CurrentSound {duration: Duration::from_beats_with_ts(Beats::from_integer(2), ts)}),)
        ]);
        let composition = music_string.compose_v2(ts, Performer {
            instrument: Instrument::Piano,
            volume: Volume(80),
            pitch: Pitch::middle_c()
        }).unwrap();
        assert_eq!(composition.tracks.len(), 1);
        let track = &composition.tracks[0];
        assert_eq!(track.instrument, Instrument::Piano);
        assert_eq!(track.events.len(), 1);
        let event = &track.events[0];
        assert_eq!(event.duration, Duration::from_beats_with_ts(Beats::from_integer(2), ts));

        assert_eq!(composition.get_start(), Some(Duration::from_beats_with_ts(Beats::from_integer(0), ts)));
        assert_eq!(composition.get_end(), Some(Duration::from_beats_with_ts(Beats::from_integer(2), ts)));
    }

    #[test]
    fn test_compose_v2_2() {
        // check that different length branches in a split cause an error
        let ts = TimeSignature::common();
        let music_string = MusicString(vec![
            MusicPrimitive::Split {
                branches: vec![
                    MusicString(vec![
                        MusicPrimitive::Simple(Symbol::T(Terminal::CurrentSound {duration: Duration::from_beats_with_ts(Beats::from_integer(2), ts)}),)
                    ]),
                    MusicString(vec![
                        MusicPrimitive::Simple(Symbol::T(Terminal::CurrentSound {duration: Duration::from_beats_with_ts(Beats::from_integer(3), ts)}),)
                    ]),
                ]
            },
        ]);
        let composition = music_string.compose_v2(ts, Performer {
            instrument: Instrument::Piano,
            volume: Volume(80),
            pitch: Pitch::middle_c()
        });
        assert!(composition.is_err());
    }

    #[test]
    fn test_compose_v2_3() {
        let ts = TimeSignature::common();
        let music_string = MusicString(vec![
            MusicPrimitive::Split {
                branches: vec![
                    MusicString(vec![
                        MusicPrimitive::Simple(Symbol::T(Terminal::CurrentSound {duration: Duration::from_beats_with_ts(Beats::from_integer(2), ts)}),)
                    ]),
                    MusicString(vec![
                        MusicPrimitive::Simple(Symbol::T(Terminal::CurrentSound {duration: Duration::from_beats_with_ts(Beats::from_integer(2), ts)}),)
                    ]),
                ]
            },
        ]);
        let composition = music_string.compose_v2(ts, Performer {
            instrument: Instrument::Piano,
            volume: Volume(80),
            pitch: Pitch::middle_c()
        }).unwrap();

        assert_eq!(composition.tracks.len(), 2);
        for track in &composition.tracks {
            assert_eq!(track.instrument, Instrument::Piano);
            assert_eq!(track.events.len(), 1);
            let event = &track.events[0];
            assert_eq!(event.duration, Duration::from_beats_with_ts(Beats::from_integer(2), ts));
            assert_eq!(event.start, Duration::from_beats_with_ts(Beats::from_integer(0), ts));
        }
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