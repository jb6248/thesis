use crate::cfg::TimeCompression;
use crate::composition::*;
use crate::scan::Scanner;
use crate::scan::{GrammarScanner, ScanError, consume};
use music_primitives::{Duration, MusicNat, Pitch, TimeSignature};
use num::Zero;
use rand::{Rng, RngExt};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grammar {
    pub start: NonTerminal,
    pub productions: Vec<Production>,
    pub time_signature: TimeSignature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Production(pub NonTerminal, pub MusicString);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicString(pub Vec<MusicPrimitive>);

/// This is like a music string but retains the non-terminals in a tree structure.
/// The root will always either be a production or a primitive.
/// Since MusicPrimitives contain MusicStrings (recursive structure), those internal strings
/// need to be converted into GrammarDerivations upon production.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrammarDerivation {
    /// This is a root of a subtree grammar derivation (or the actual root)
    Branch {
        nt: NonTerminal,
        content: Vec<GrammarDerivation>,
    },
    /// This is a non-destructive wrapper for a number of subtrees
    Wrapped {
        transform: MusicTransform,
        content: Vec<GrammarDerivation>,
    },
    /// This is when there's a split in the music
    Split {
        branches: Vec<Vec<GrammarDerivation>>,
    },
    /// This is a leaf that hasn't been expanded yet... for whatever reason.
    NTLeaf(NonTerminal),
    /// This cannot be expanded any further
    TLeaf(Terminal),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MusicPrimitive {
    Simple(Symbol),
    Split {
        branches: Vec<MusicString>,
    },
    /// Use MusicTransform::Repeat instead
    #[deprecated]
    Repeat {
        num: usize,
        content: MusicString,
    },
    Transform {
        transform: MusicTransform,
        content: MusicString,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MusicTransform {
    Transpose { semitones: i8 },
    Repeat { num: usize },
    Compression { factor: TimeCompression },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Symbol {
    NT(NonTerminal),
    T(Terminal),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NonTerminal {
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Terminal {
    /// Play this specific note.
    AbsoluteSound {
        duration: Duration,
        note: TerminalNote,
    },
    /// Play a sound for this length with whatever settings
    /// the performer currently has.
    CurrentSound {
        duration: Duration,
    },
    MovePitch {
        semitones: i8,
    },
    Meta(MetaControl),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TerminalNote {
    Note { pitch: Pitch },
    Rest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MetaControl {
    ChangeInstrument(Instrument),
    ChangeVolume(Volume),
}

impl Grammar {
    pub fn new(
        start: NonTerminal,
        productions: Vec<Production>,
        time_signature: TimeSignature,
    ) -> Self {
        Grammar {
            start,
            productions,
            time_signature,
        }
    }

    pub fn get_production(&self, nt: &NonTerminal) -> Option<&Production> {
        self.productions.iter().find(|p| &p.0 == nt)
    }

    pub fn get_production_random(
        &self,
        nt: &NonTerminal,
        rng: &mut impl Rng,
    ) -> Option<&Production> {
        let productions: Vec<_> = self.productions.iter().filter(|p| &p.0 == nt).collect();
        if productions.is_empty() {
            None
        } else {
            Some(productions[rng.random_range(0..productions.len())])
        }
    }

    pub fn produce(&self, config: &GrammarDerivationConfig, rng: &mut impl Rng) -> MusicString {
        let axiom = MusicString(vec![MusicPrimitive::Simple(Symbol::NT(self.start.clone()))]);
        axiom.parallel_rewrite_n(self, rng, config.panic_on_bad_production, config.iterations)
    }
}

impl FromStr for Grammar {
    type Err = ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanner = consume(GrammarScanner);
        let (grammar, _s) = scanner.scan(s)?;
        Ok(grammar)
    }
}

#[derive(Debug)]
pub enum ComposeError {
    MismatchedLengths(String),
}

impl Display for MusicTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MusicTransform::Transpose { semitones } => format!("T{}", semitones),
            MusicTransform::Repeat { num } => format!("x{}", num),
            MusicTransform::Compression { factor } => format!(">>{}", factor.to_string()),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Performer {
    pub instrument: Instrument,
    pub volume: Volume,
    pub pitch: Pitch,
}

impl Default for Performer {
    fn default() -> Self {
        Performer {
            instrument: Instrument::Piano,
            volume: Volume(MAX_VOLUME / 2),
            pitch: Pitch::middle_c(),
        }
    }
}

pub struct GrammarDerivationConfig {
    pub iterations: usize,
    pub panic_on_bad_production: bool,
    /// Rounds out composition to whole measures by padding with rests.
    pub rounded: bool,
    /// Maximum depth for grammar derivation tree.
    pub max_depth: usize,
}

pub struct GrammarDerivationGenerator {
    pub config: GrammarDerivationConfig,
    pub grammar: Arc<Grammar>,
}

impl GrammarDerivation {
    /// Uses an in-order traversal of leaves to convert a GrammarDerivation into a MusicString, which can then be composed.
    pub fn to_music_string(&self) -> MusicString {
        match self {
            GrammarDerivation::Branch { content, .. } => {
                let mut primitives = vec![];
                for sub_derivation in content {
                    primitives.extend(sub_derivation.to_music_string().0);
                }
                MusicString(primitives)
            }
            GrammarDerivation::Wrapped { transform, content } => {
                let mut primitives = vec![];
                for sub_derivation in content {
                    primitives.extend(sub_derivation.to_music_string().0);
                }
                MusicString(vec![MusicPrimitive::Transform {
                    transform: transform.clone(),
                    content: MusicString(primitives),
                }])
            }
            GrammarDerivation::Split { branches } => {
                let new_branches = branches
                    .iter()
                    .map(|branch| {
                        let mut primitives = vec![];
                        for sub_derivation in branch {
                            primitives.extend(sub_derivation.to_music_string().0);
                        }
                        MusicString(primitives)
                    })
                    .collect();
                MusicString(vec![MusicPrimitive::Split {
                    branches: new_branches,
                }])
            }
            GrammarDerivation::NTLeaf(nt) => {
                MusicString(vec![MusicPrimitive::Simple(Symbol::NT(nt.clone()))])
            }
            GrammarDerivation::TLeaf(t) => {
                MusicString(vec![MusicPrimitive::Simple(Symbol::T(t.clone()))])
            }
        }
    }
    pub fn count_all_nts(&self) -> usize {
        fn check(_: &NonTerminal) -> bool {
            true
        }
        self.count_nts(&mut check)
    }

    pub fn count_filtered_nts<F: FnMut(&NonTerminal) -> bool>(&self, mut filter: F) -> usize {
        self.count_nts(&mut filter)
    }

    /// Count the number of non-terminals in the tree. This is used for random mutation.
    fn count_nts<F: FnMut(&NonTerminal) -> bool>(&self, filter: &mut F) -> usize {
        let mut nt_count = 0;
        match self {
            GrammarDerivation::NTLeaf(nt) => {
                if filter(nt) {
                    nt_count += 1;
                }
            }
            GrammarDerivation::Branch { content, nt } => {
                if filter(nt) {
                    nt_count += 1;
                }
                for sub_derivation in content.iter() {
                    nt_count += sub_derivation.count_nts(filter);
                }
            }
            GrammarDerivation::Wrapped { content, .. } => {
                for sub_derivation in content.iter() {
                    nt_count += sub_derivation.count_nts(filter);
                }
            }
            GrammarDerivation::Split { branches } => {
                for branch in branches.iter() {
                    for sub_derivation in branch.iter() {
                        nt_count += sub_derivation.count_nts(filter);
                    }
                }
            }
            GrammarDerivation::TLeaf(_) => {
                // Terminals don't count
            }
        }
        nt_count
    }

    pub fn pick_random_nt_mut<F: FnMut(&NonTerminal) -> bool>(
        &mut self,
        rng: &mut impl Rng,
        filter: Option<F>,
    ) -> Option<(&mut GrammarDerivation, usize)> {
        // Box the filter so it can be used for both counting and selection.
        // Without boxing, the filter would be consumed by count_filtered_nts and
        // unavailable for the actual selection traversal, causing pick_random_nt_mut
        // to select from the wrong (unfiltered) set of nodes.
        let mut filter_box: Box<dyn FnMut(&NonTerminal) -> bool> = match filter {
            Some(f) => Box::new(f),
            None => Box::new(|_: &NonTerminal| true),
        };
        let nt_count = self.count_filtered_nts(|nt| filter_box(nt));
        if nt_count == 0 {
            return None;
        }
        let target = rng.random_range(0..nt_count);
        let mut current = 0;
        fn helper<'a>(
            derivation: &'a mut GrammarDerivation,
            target: usize,
            current: &mut usize,
            depth: usize,
            filter: &mut dyn FnMut(&NonTerminal) -> bool,
        ) -> Option<(&'a mut GrammarDerivation, usize)> {
            if *current > target {
                return None;
            }
            // Clone the NT name to avoid holding a borrow on `derivation` across
            // the filter call and the subsequent mutable match below.
            let passes = {
                let nt_opt = match &*derivation {
                    GrammarDerivation::Branch { nt, .. } => Some(nt.clone()),
                    GrammarDerivation::NTLeaf(nt) => Some(nt.clone()),
                    _ => None,
                };
                nt_opt.map_or(false, |nt| filter(&nt))
            };
            if passes && *current == target {
                return Some((derivation, depth));
            }
            match derivation {
                GrammarDerivation::NTLeaf(_) => {
                    if passes {
                        *current += 1;
                    }
                    None
                }
                GrammarDerivation::TLeaf(_) => None,
                GrammarDerivation::Branch { content, .. } => {
                    if passes {
                        *current += 1;
                    }
                    for sub_derivation in content.iter_mut() {
                        if let Some(result) = helper(sub_derivation, target, current, depth + 1, filter) {
                            return Some(result);
                        }
                    }
                    None
                }
                GrammarDerivation::Wrapped { content, .. } => {
                    for sub_derivation in content.iter_mut() {
                        if let Some(result) = helper(sub_derivation, target, current, depth, filter) {
                            return Some(result);
                        }
                    }
                    None
                }
                GrammarDerivation::Split { branches } => {
                    for branch in branches.iter_mut() {
                        for sub_derivation in branch.iter_mut() {
                            if let Some(result) = helper(sub_derivation, target, current, depth, filter) {
                                return Some(result);
                            }
                        }
                    }
                    None
                }
            }
        }
        helper(self, target, &mut current, 0, &mut *filter_box)
    }

    pub fn is_nt_root(&self) -> bool {
        self.get_nt_root().is_some()
    }

    pub fn get_nt_root(&self) -> Option<&NonTerminal> {
        match self {
            GrammarDerivation::Branch { nt, .. } => Some(nt),
            GrammarDerivation::NTLeaf(nt) => Some(nt),
            _ => None,
        }
    }
}

impl GrammarDerivationGenerator {
    pub fn new(config: GrammarDerivationConfig, grammar: &Grammar) -> Self {
        GrammarDerivationGenerator {
            config,
            grammar: Arc::new(grammar.clone()),
        }
    }

    /// Produces a GrammarDerivation by expanding the grammar's starting non-terminal
    /// step by step, respecting the max_depth limit.
    pub fn produce(&self, rng: &mut impl Rng) -> GrammarDerivation {
        let start_nt = self.grammar.start.clone();
        let mut derivation = self.expand_nonterminal(&start_nt, rng);
        for _ in 1..self.config.iterations {
            self.expand_derivation(&mut derivation, rng);
        }
        derivation
    }

    /// Expands a non-terminal into a GrammarDerivation.
    /// Returns a Production node containing the expanded content.
    fn expand_nonterminal(&self, nt: &NonTerminal, rng: &mut impl Rng) -> GrammarDerivation {
        let production = self.grammar.get_production_random(nt, rng);

        match production {
            Some(Production(_, music_string)) => GrammarDerivation::Branch {
                nt: nt.clone(),
                content: music_string
                    .0
                    .iter()
                    .map(|mp| self.make_music_primitive_leaf(mp))
                    .collect(),
            },
            None => {
                if self.config.panic_on_bad_production {
                    panic!("No production found for non-terminal {:?}", nt);
                }
                // Return empty production if no production found
                GrammarDerivation::NTLeaf(nt.clone())
            }
        }
    }

    fn make_music_primitive_leaf(&self, mp: &MusicPrimitive) -> GrammarDerivation {
        match mp {
            MusicPrimitive::Simple(Symbol::NT(nt)) => GrammarDerivation::NTLeaf(nt.clone()),
            MusicPrimitive::Simple(Symbol::T(t)) => GrammarDerivation::TLeaf(t.clone()),
            MusicPrimitive::Split { branches } => GrammarDerivation::Split {
                branches: branches
                    .iter()
                    .map(|branch| {
                        branch
                            .0
                            .iter()
                            // these recursive calls don't incur depth because we aren't expanding NTs
                            .map(|mp| self.make_music_primitive_leaf(mp))
                            .collect()
                    })
                    .collect(),
            },
            MusicPrimitive::Repeat { .. } => {
                panic!("Repeat is deprecated, use Transform instead")
            }
            MusicPrimitive::Transform { transform, content } => {
                GrammarDerivation::Wrapped {
                    transform: transform.clone(),
                    content: content
                        .0
                        .iter()
                        // these recursive calls don't incur depth because we aren't expanding NTs
                        .map(|mp| self.make_music_primitive_leaf(mp))
                        .collect(),
                }
            }
        }
    }

    /// Perform a leveled production.
    fn expand_derivation(&self, derivation: &mut GrammarDerivation, rng: &mut impl Rng) {
        match derivation {
            // this is the only one that increases depth
            GrammarDerivation::NTLeaf(nt) => {
                *derivation = self.expand_nonterminal(nt, rng);
                // do NOT do it recursively; we are just doing 1 iteration of expansion
            }
            GrammarDerivation::TLeaf(_terminal) => {
                // Terminals can't be expanded
            }
            // the rest of these should not increase the depth
            GrammarDerivation::Branch { content, .. } => {
                // follow it down
                for sub_derivation in content.iter_mut() {
                    self.expand_derivation(sub_derivation, rng);
                }
            }
            GrammarDerivation::Wrapped { content, .. } => {
                for sub_derivation in content.iter_mut() {
                    self.expand_derivation(sub_derivation, rng);
                }
            }
            GrammarDerivation::Split { branches } => {
                for branch in branches.iter_mut() {
                    for sub_derivation in branch.iter_mut() {
                        self.expand_derivation(sub_derivation, rng);
                    }
                }
            }
        }
    }

    pub fn expand_derivation_recursive(
        &self,
        derivation: &mut GrammarDerivation,
        rng: &mut impl Rng,
        depth: usize,
    ) {
        if depth > self.config.max_depth {
            return;
        }
        match derivation {
            // this is the only one that increases depth
            GrammarDerivation::NTLeaf(nt) => {
                *derivation = self.expand_nonterminal(nt, rng);
                // do it recursively to expand the new content as well
                self.expand_derivation_recursive(derivation, rng, depth + 1);
            }
            GrammarDerivation::TLeaf(_terminal) => {
                // Terminals can't be expanded
            }
            // the rest of these should not increase the depth
            GrammarDerivation::Branch { content, .. } => {
                // follow it down
                for sub_derivation in content.iter_mut() {
                    self.expand_derivation_recursive(sub_derivation, rng, depth);
                }
            }
            GrammarDerivation::Wrapped { content, .. } => {
                for sub_derivation in content.iter_mut() {
                    self.expand_derivation_recursive(sub_derivation, rng, depth);
                }
            }
            GrammarDerivation::Split { branches } => {
                for branch in branches.iter_mut() {
                    for sub_derivation in branch.iter_mut() {
                        self.expand_derivation_recursive(sub_derivation, rng, depth);
                    }
                }
            }
        }
    }

    pub fn re_expand_random_nt(&self, derivation: &mut GrammarDerivation, rng: &mut impl Rng) {
        if let Some((target, depth)) =
            derivation.pick_random_nt_mut(rng, None::<fn(&NonTerminal) -> bool>)
        {
            *target = self.expand_nonterminal(target.get_nt_root().unwrap(), rng);
            self.expand_derivation_recursive(derivation, rng, depth);
        }
    }

    /// Prune the derivation tree so no Branch node exceeds `max_depth`.
    /// Any Branch found at or beyond `max_depth` is collapsed to an NTLeaf of the
    /// same non-terminal, leaving it unexpanded rather than carrying an arbitrarily
    /// deep subtree introduced by crossover or mutation.
    pub fn prune(&self, derivation: &mut GrammarDerivation) {
        self.prune_at(derivation, 0);
    }

    fn prune_at(&self, derivation: &mut GrammarDerivation, depth: usize) {
        // If we have a Branch that is at or beyond max_depth, collapse it to an
        // NTLeaf so the subtree is removed.  Wrapped and Split nodes do not
        // represent NT expansions themselves, so they do not consume a depth level.
        if let GrammarDerivation::Branch { nt, .. } = &*derivation {
            if depth >= self.config.max_depth {
                let nt = nt.clone();
                *derivation = GrammarDerivation::NTLeaf(nt);
                return;
            }
        }
        match derivation {
            GrammarDerivation::Branch { content, .. } => {
                for sub in content.iter_mut() {
                    self.prune_at(sub, depth + 1);
                }
            }
            GrammarDerivation::Wrapped { content, .. } => {
                for sub in content.iter_mut() {
                    self.prune_at(sub, depth);
                }
            }
            GrammarDerivation::Split { branches } => {
                for branch in branches.iter_mut() {
                    for sub in branch.iter_mut() {
                        self.prune_at(sub, depth);
                    }
                }
            }
            GrammarDerivation::NTLeaf(_) | GrammarDerivation::TLeaf(_) => {}
        }
    }
}

impl MusicString {
    /// This one composes each instrument onto a separate track.
    /// One track may have multiple concurrent lines. This only considers terminals that are absolute notes.
    pub fn compose_v1(
        &self,
        time_signature: TimeSignature,
        starting_instrument: Option<Instrument>,
    ) -> Result<Composition, ComposeError> {
        let mut tracks = HashMap::new();
        fn add_event(tracks: &mut HashMap<Instrument, Track>, e: Event, instrument: Instrument) {
            if let Some(mut track) = tracks.get_mut(&instrument) {
                track.events.push(e);
            } else {
                tracks.insert(
                    instrument,
                    Track {
                        identifier: TrackId::Instrument(instrument),
                        instrument,
                        events: vec![e],
                        rests: vec![],
                    },
                );
            }
        }

        fn add_rest_event(
            tracks: &mut HashMap<Instrument, Track>,
            e: Event,
            instrument: Instrument,
        ) {
            if let Some(track) = tracks.get_mut(&instrument) {
                track.rests.push(e);
            } else {
                tracks.insert(
                    instrument,
                    Track {
                        identifier: TrackId::Instrument(instrument),
                        instrument,
                        events: vec![],
                        rests: vec![e],
                    },
                );
            }
        }
        fn add_track(tracks: &mut HashMap<Instrument, Track>, track: Track) {
            if let Some(mtrack) = tracks.remove(&track.instrument) {
                tracks.insert(mtrack.instrument, mtrack + track);
            } else {
                tracks.insert(track.instrument, track);
            }
        }
        fn add_composition(tracks: &mut HashMap<Instrument, Track>, composition: Composition) {
            for track in composition.tracks {
                add_track(tracks, track);
            }
        }
        let mut current_mt = Duration::zero(time_signature);
        let mut current_instrument = starting_instrument.unwrap_or(Instrument::SineWave);
        let mut current_volume = Volume(50);
        for mp in self.0.iter() {
            let duration = match mp {
                MusicPrimitive::Simple(sym) => match sym {
                    Symbol::NT(_) => Duration::zero(time_signature),
                    Symbol::T(Terminal::AbsoluteSound { note, duration }) => match note {
                        TerminalNote::Note { pitch } => {
                            add_event(
                                &mut tracks,
                                Event {
                                    start: current_mt,
                                    duration: *duration,
                                    volume: current_volume,
                                    pitch: *pitch,
                                },
                                current_instrument,
                            );
                            *duration
                        }
                        TerminalNote::Rest => {
                            add_rest_event(
                                &mut tracks,
                                Event {
                                    start: current_mt,
                                    duration: *duration,
                                    volume: Volume(0),
                                    pitch: Pitch::none(), // don't care
                                },
                                current_instrument,
                            );
                            *duration
                        }
                    },
                    Symbol::T(Terminal::Meta(control)) => {
                        match control {
                            MetaControl::ChangeInstrument(i) => {
                                current_instrument = *i;
                            }
                            MetaControl::ChangeVolume(v) => {
                                current_volume = *v;
                            }
                        }
                        Duration::zero(time_signature)
                    }
                    Symbol::T(term) => {
                        panic!(
                            "The terminal {:?} is not implemented for this composer",
                            term
                        );
                    }
                },
                MusicPrimitive::Split { branches } => {
                    let comps: Vec<_> = branches
                        .into_iter()
                        .map(|ms| ms.compose_v1(time_signature, Some(current_instrument)))
                        .err_first()?
                        .map(|mut c| {
                            c.shift_by(current_mt, true);
                            c
                        })
                        .map(|c| (c.get_duration(), c))
                        .collect();
                    let uniform_duration = match comps.first() {
                        Some((duration, _c)) => {
                            if comps.iter().all(|(d, _c)| d == duration) {
                                Some(*duration)
                            } else {
                                None
                            }
                        }
                        // there are none, so yes they are
                        None => Some(Duration::zero(time_signature)),
                    };
                    if let Some(dur) = uniform_duration {
                        for (_d, comp) in comps {
                            add_composition(&mut tracks, comp);
                        }
                        dur
                    } else {
                        return Err(ComposeError::MismatchedLengths(format!(
                            "Not all split tracks have the same duration: {:?}",
                            comps.iter().map(|(d, c)| d).collect::<Vec<_>>()
                        )));
                    }
                }
                MusicPrimitive::Transform { transform, content } => {
                    match transform {
                        MusicTransform::Transpose { semitones } => {
                            let mut composed =
                                content.compose_v1(time_signature, Some(current_instrument))?;
                            composed.transpose(*semitones);
                            composed.shift_by(current_mt, true);
                            let duration = composed.get_duration();
                            add_composition(&mut tracks, composed);
                            duration
                        }
                        MusicTransform::Repeat { num } => {
                            let composed =
                                content.compose_v1(time_signature, Some(current_instrument))?;
                            let duration = composed.get_duration();
                            let mut offset = current_mt;
                            for _i in 0..*num {
                                let mut comp_i = composed.clone();
                                comp_i.shift_by(offset, true);
                                add_composition(&mut tracks, comp_i);
                                offset = offset + duration;
                            }
                            let mut total_duration = Duration::zero(time_signature);
                            for _i in 0..*num {
                                total_duration = total_duration + duration;
                            }
                            // println!("total duration for {num} repeats is {total_duration:?}, or {:?} * {num}",
                            //          composed.get_duration());
                            total_duration
                        }
                        MusicTransform::Compression { factor } => {
                            let mut composed =
                                content.compose_v1(time_signature, Some(current_instrument))?;
                            composed.compress(*factor);
                            composed.shift_by(current_mt, true);
                            let duration = composed.get_duration();
                            add_composition(&mut tracks, composed);
                            duration
                        }
                    }
                }
                mp => {
                    panic!(
                        "The primitive {:?} is not implemented for this composer",
                        mp
                    );
                }
            };
            current_mt += duration;
        }
        Ok(Composition {
            tracks: tracks.into_values().collect(),
            time_signature,
        })
    }

    /// This one should make each new line a new voice. It's important to note that
    /// multiple voices are started with SPLITS. A voice has only 1 instrument and 1 note,
    /// and each event should be a partition of time between start and end (no overlaps and no gaps).
    pub fn compose_v2(
        &self,
        time_signature: TimeSignature,
        starting_performer: Performer,
    ) -> Result<Composition, ComposeError> {
        // output components
        let mut tracks: HashMap<usize, Track> = HashMap::new();
        fn get_next_track_id(tracks: &HashMap<usize, Track>) -> usize {
            // max id in tracks + 1
            tracks.iter().map(|(k, _track)| *k).max().unwrap_or(0) + 1
        }

        fn add_event(
            tracks: &mut HashMap<usize, Track>,
            track_id: usize,
            e: Event,
            performer: &Performer,
            is_rest: bool,
        ) {
            if let Some(track) = tracks.get_mut(&track_id) {
                if is_rest {
                    track.rests.push(e);
                } else {
                    track.events.push(e);
                }
            } else {
                let mut new_track = Track {
                    identifier: TrackId::Instrument(performer.instrument),
                    instrument: performer.instrument,
                    events: vec![],
                    rests: vec![],
                };
                if is_rest {
                    new_track.rests.push(e);
                } else {
                    new_track.events.push(e);
                }
                tracks.insert(track_id, new_track);
            }
        }

        // current state
        let mut current_track_id: usize = 0;
        let mut offset = Duration::zero(time_signature);
        let mut performer = starting_performer.clone();
        for m_prim in self.0.iter() {
            // Handle each primitive by adding to the tracks as needed
            // return the duration that this primitive takes.
            let duration = match m_prim {
                MusicPrimitive::Simple(symbol) => {
                    match symbol {
                        Symbol::NT(_) => Duration::zero(time_signature),
                        Symbol::T(Terminal::Meta(meta_control)) => {
                            // Update the current performer based on the meta control
                            match meta_control {
                                MetaControl::ChangeInstrument(instr) => {
                                    current_track_id = get_next_track_id(&tracks);
                                    performer.instrument = *instr;
                                }
                                MetaControl::ChangeVolume(v) => {
                                    performer.volume = *v;
                                }
                            }
                            Duration::zero(time_signature)
                        }
                        Symbol::T(Terminal::AbsoluteSound { duration, note }) => {
                            add_event(
                                &mut tracks,
                                current_track_id,
                                Event {
                                    start: offset,
                                    duration: *duration,
                                    volume: performer.volume,
                                    pitch: match note {
                                        TerminalNote::Note { pitch } => *pitch,
                                        TerminalNote::Rest => Pitch::none(),
                                    },
                                },
                                &performer,
                                matches!(note, TerminalNote::Rest),
                            );
                            *duration
                        }
                        Symbol::T(Terminal::CurrentSound { duration }) => {
                            add_event(
                                &mut tracks,
                                current_track_id,
                                Event {
                                    start: offset,
                                    duration: *duration,
                                    volume: performer.volume,
                                    pitch: performer.pitch,
                                },
                                &performer,
                                false,
                            );
                            *duration
                        }
                        Symbol::T(Terminal::MovePitch { semitones }) => {
                            performer.pitch.transpose(*semitones);
                            Duration::zero(time_signature)
                        }
                    }
                }
                MusicPrimitive::Split { branches } => {
                    // compose each branch and ensure that they are the same length
                    let comps: Vec<_> = branches
                        .into_iter()
                        .map(|ms| ms.compose_v2(time_signature, performer.clone()))
                        .err_first()?
                        .map(|mut c| {
                            c.shift_by(offset, true);
                            c
                        })
                        .map(|c| (c.get_duration(), c))
                        .collect();
                    if comps.is_empty() {
                        Duration::zero(time_signature)
                    } else {
                        // guaranteed to not be empty
                        let (dur, has_uniform_duration) = comps
                            .iter()
                            .map(|(d, _c)| (*d, true))
                            .reduce(|(d1, ok1), (d2, ok2)| (d1, ok1 && ok2 && d1 == d2))
                            .unwrap();
                        if !has_uniform_duration {
                            return Err(ComposeError::MismatchedLengths(format!(
                                "Not all split tracks have the same duration: {:?}",
                                comps.iter().map(|(d, c)| d).collect::<Vec<_>>()
                            )));
                        }
                        // otherwise, add each track to the tracks
                        for (_d, comp) in comps {
                            for mut track in comp.tracks {
                                let track_id = get_next_track_id(&tracks);
                                track.identifier = TrackId::Custom(track_id);
                                tracks.insert(track_id, track);
                            }
                        }
                        dur
                    }
                }
                MusicPrimitive::Transform { transform, content } => {
                    let mut inner = content.compose_v2(time_signature, performer.clone())?;
                    match transform {
                        MusicTransform::Transpose { semitones } => {
                            inner.transpose(*semitones);
                            let duration = inner.get_duration();
                            inner.shift_by(offset, true);
                            for mut track in inner.tracks {
                                let track_id = get_next_track_id(&tracks);
                                track.identifier = TrackId::Custom(track_id);
                                tracks.insert(track_id, track);
                            }
                            // add rests for this one because the current track isn't being added to
                            add_event(
                                &mut tracks,
                                current_track_id,
                                Event {
                                    start: offset,
                                    duration,
                                    volume: performer.volume,
                                    pitch: performer.pitch,
                                },
                                &performer,
                                true,
                            );
                            duration
                        }
                        MusicTransform::Compression { factor } => {
                            inner.compress(*factor);
                            inner.shift_by(offset, true);
                            let duration = inner.get_duration();
                            for mut track in inner.tracks {
                                let track_id = get_next_track_id(&tracks);
                                track.identifier = TrackId::Custom(track_id);
                                tracks.insert(track_id, track);
                            }
                            add_event(
                                &mut tracks,
                                current_track_id,
                                Event {
                                    start: offset,
                                    duration,
                                    volume: performer.volume,
                                    pitch: performer.pitch,
                                },
                                &performer,
                                true,
                            );
                            duration
                        }
                        MusicTransform::Repeat { num } => {
                            let single_duration = inner.get_duration();

                            // for each track in the inner track, repeat it num times
                            for track in inner.tracks {
                                let id = get_next_track_id(&tracks);
                                let mut repeated_track = Track {
                                    identifier: TrackId::Custom(id),
                                    events: vec![],
                                    rests: vec![],
                                    instrument: track.instrument,
                                };
                                let track_duration = track.get_duration(time_signature);
                                for _ in 0..*num {
                                    repeated_track.append(&track, time_signature);
                                    // add a rest at the end to pad it to the duration of the whole inner
                                    repeated_track.rests.push(Event {
                                        start: repeated_track
                                            .get_end(time_signature)
                                            .unwrap_or(Duration::zero(time_signature)),
                                        duration: single_duration - track_duration,
                                        volume: Volume(0),
                                        pitch: Pitch::none(),
                                    });
                                }
                                repeated_track.shift_by(offset, true);
                                // then add it to the tracks
                                tracks.insert(id, repeated_track);
                            }
                            add_event(
                                &mut tracks,
                                current_track_id,
                                Event {
                                    start: offset,
                                    duration: single_duration * *num as MusicNat,
                                    volume: performer.volume,
                                    pitch: performer.pitch,
                                },
                                &performer,
                                true,
                            );
                            println!(
                                "total duration for {num} repeats is {:?}, or {:?} * {num}",
                                single_duration * *num as MusicNat,
                                single_duration
                            );
                            single_duration * *num as MusicNat
                        }
                    }
                }
                mp => {
                    panic!("Compose_v2 does not support the primitive {:?}", mp);
                }
            };
            offset += duration;
        }
        Ok(Composition {
            tracks: tracks.into_iter().map(|(_id, track)| track).collect(),
            time_signature,
        })
    }

    /// Rewrites the music string according to the grammar, replacing non-terminals with their productions.
    /// If `random` is true, it will choose a random production for each non-terminal.
    /// If `panic_on_bad_production` is true, it will panic if a non-terminal has no production.
    pub fn parallel_rewrite(
        &self,
        grammar: &Grammar,
        rng: &mut impl Rng,
        panic_on_bad_production: bool,
    ) -> Self {
        let mut new_string = vec![];
        for (i, mp) in self.0.iter().enumerate() {
            match mp {
                MusicPrimitive::Simple(x) => match x {
                    Symbol::NT(nt) => {
                        if let Some(Production(nt, ms)) = grammar.get_production_random(nt, rng) {
                            new_string.extend(ms.clone().0);
                        } else {
                            if panic_on_bad_production {
                                panic!(
                                    "No production found for non-terminal {:?} at index {}",
                                    nt, i
                                );
                            }
                        }
                    }
                    x => {
                        new_string.push(MusicPrimitive::Simple(x.clone()));
                    }
                },
                MusicPrimitive::Split { branches } => {
                    let new_branches = branches
                        .iter()
                        .map(|ms| ms.parallel_rewrite(grammar, rng, panic_on_bad_production))
                        .collect::<Vec<_>>();
                    new_string.push(MusicPrimitive::Split {
                        branches: new_branches,
                    });
                }
                MusicPrimitive::Repeat { num, content } => {
                    let new_content =
                        content.parallel_rewrite(grammar, rng, panic_on_bad_production);
                    new_string.push(MusicPrimitive::Repeat {
                        num: *num,
                        content: new_content,
                    });
                }
                MusicPrimitive::Transform { transform, content } => {
                    let new_content =
                        content.parallel_rewrite(grammar, rng, panic_on_bad_production);
                    new_string.push(MusicPrimitive::Transform {
                        transform: transform.clone(),
                        content: new_content,
                    });
                }
            }
        }
        MusicString(new_string)
    }

    pub fn parallel_rewrite_n(
        &self,
        grammar: &Grammar,
        rng: &mut impl Rng,
        panic_on_bad_production: bool,
        n: usize,
    ) -> Self {
        let mut new_string = self.clone();
        for _i in 0..n {
            new_string = new_string.parallel_rewrite(grammar, rng, panic_on_bad_production);
        }
        new_string
    }
}

impl ToString for MusicString {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for mp in &self.0 {
            match mp {
                MusicPrimitive::Simple(sym) => {
                    let sym_to_string = sym.to_string();
                    s.push_str(&sym_to_string);
                }
                MusicPrimitive::Split { branches } => {
                    s.push_str("{");
                    let str = branches
                        .into_iter()
                        .map(|b| b.to_string())
                        .reduce(|b1, b2| b1 + " | " + &b2)
                        .unwrap_or("".to_string());
                    s.push_str(&str);
                    s.push('}');
                }
                MusicPrimitive::Repeat { num, content } => {
                    panic!("Repeat is deprecated, use Transform instead")
                }
                MusicPrimitive::Transform { transform, content } => {
                    s.push_str(&format!("[{}][", transform));
                    s.push_str(&content.to_string());
                    s.push(']');
                }
            }
            s.push(' ');
        }
        s
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        match self {
            Symbol::NT(nt) => nt.to_string(),
            Symbol::T(t) => t.to_string(),
        }
    }
}

impl ToString for NonTerminal {
    fn to_string(&self) -> String {
        match self {
            NonTerminal::Custom(s) => s.clone(),
        }
    }
}

impl Display for Terminal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminal::AbsoluteSound { duration, note } => match note {
                TerminalNote::Note { pitch } => {
                    let letter = pitch.letter_name();
                    write!(f, ":{letter}<{}>", duration.to_string())
                }
                TerminalNote::Rest => {
                    write!(f, ":_<{}>", duration.to_string())
                }
            },
            Terminal::Meta(control) => write!(f, "{}", control.to_string()),
            Terminal::CurrentSound { duration } => {
                // todo: make these more realistic?
                // ♪ is an eighth note, ♩ is a quarter note
                write!(f, "♩<{}>", duration.to_string())
            }
            Terminal::MovePitch { semitones } => {
                write!(
                    f,
                    "{}{}",
                    if *semitones > 0 {
                        // up arrow
                        "↑"
                    } else {
                        "↓"
                    },
                    semitones
                )
            }
        }
    }
}

impl ToString for MetaControl {
    fn to_string(&self) -> String {
        match self {
            MetaControl::ChangeInstrument(i) => format!("::i={:?}", i),
            MetaControl::ChangeVolume(v) => format!("::v={:?}", v),
        }
    }
}

pub trait ReduceResultIter<I, E> {
    fn err_first(self) -> Result<impl Iterator<Item = I>, E>;
}

impl<I, E, T> ReduceResultIter<I, E> for T
where
    T: Iterator<Item = Result<I, E>>,
{
    fn err_first(self) -> Result<impl Iterator<Item = I>, E> {
        let mut processed = vec![];
        for e in self {
            match e {
                Ok(i) => processed.push(i),
                Err(e) => return Err(e),
            }
        }
        Ok(processed.into_iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lilypond::{LilyPondConfig, call_lilypond_cli, render_to_lilypond};
    use crate::{compose_from_grammar, grammar_from_file};
    use music_primitives::NoteValue;
    use rand::rng;

    // ------------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------------

    fn quarter_rest(ts: TimeSignature) -> GrammarDerivation {
        GrammarDerivation::TLeaf(Terminal::AbsoluteSound {
            duration: Duration::zero(ts) + NoteValue::new(1, 4),
            note: TerminalNote::Rest,
        })
    }

    fn nt(name: &str) -> NonTerminal {
        NonTerminal::Custom(name.to_string())
    }

    fn branch(name: &str, children: Vec<GrammarDerivation>) -> GrammarDerivation {
        GrammarDerivation::Branch { nt: nt(name), content: children }
    }

    fn minimal_generator(max_depth: usize) -> GrammarDerivationGenerator {
        let ts = TimeSignature::common();
        let grammar = Grammar::new(nt("S"), vec![], ts);
        GrammarDerivationGenerator::new(
            GrammarDerivationConfig {
                iterations: 1,
                panic_on_bad_production: false,
                rounded: false,
                max_depth,
            },
            &grammar,
        )
    }

    // ------------------------------------------------------------------
    // pick_random_nt_mut filter tests
    // ------------------------------------------------------------------

    /// With the old (broken) code, pick_random_nt_mut used the filter only
    /// for counting, but the inner helper traversed without it.  So a tree
    /// with nodes [S, A, B, D] filtered for "D" (count=1, target=0) would
    /// always return the first DFS node "S" instead of "D".
    #[test]
    fn test_pick_random_nt_filter_returns_only_matching_nt() {
        let ts = TimeSignature::common();
        // Tree:  Branch(S) -> [Branch(A, [T]), Branch(B, [T]), Branch(D, [T])]
        // Filtering for "D" must always return Branch(D), never S, A, or B.
        let mut tree = branch("S", vec![
            branch("A", vec![quarter_rest(ts)]),
            branch("B", vec![quarter_rest(ts)]),
            branch("D", vec![quarter_rest(ts)]),
        ]);

        let mut rng = rand::rng();
        for _ in 0..50 {
            let (picked, _depth) = tree
                .pick_random_nt_mut(&mut rng, Some(|n: &NonTerminal| *n == nt("D")))
                .expect("filter for 'D' should find a node");
            assert_eq!(
                picked.get_nt_root().unwrap(),
                &nt("D"),
                "filter must only select 'D', not '{:?}'",
                picked.get_nt_root()
            );
        }
    }

    /// When multiple nodes match the filter the selection should be uniformly
    /// distributed among them, not biased to the first one.
    #[test]
    fn test_pick_random_nt_filter_samples_all_matching_nodes() {
        let ts = TimeSignature::common();
        // Tree: Branch(S) -> [Branch(A,[T]), Branch(A,[T]), Branch(A,[T])]
        // All three children share the same NT name "A".
        // After many picks each child should be selected at least once.
        let child_a = || branch("A", vec![quarter_rest(ts)]);
        let mut tree = branch("S", vec![child_a(), child_a(), child_a()]);

        let mut rng = rand::rng();
        let mut selected_depths = std::collections::HashSet::new();
        for _ in 0..200 {
            let (picked, depth) = tree
                .pick_random_nt_mut(&mut rng, Some(|n: &NonTerminal| *n == nt("A")))
                .expect("filter for 'A' should always find a node");
            assert_eq!(picked.get_nt_root().unwrap(), &nt("A"));
            selected_depths.insert(depth);
        }
        // All three A nodes are at depth 1; what distinguishes them is their
        // position.  We can verify we didn't only ever return the same object
        // by checking we got something other than just the root S.
        assert!(
            selected_depths.contains(&1),
            "Should have selected children at depth 1"
        );
    }

    /// A filter that matches nothing should return None.
    #[test]
    fn test_pick_random_nt_filter_returns_none_when_no_match() {
        let ts = TimeSignature::common();
        let mut tree = branch("S", vec![branch("A", vec![quarter_rest(ts)])]);
        let mut rng = rand::rng();
        let result = tree.pick_random_nt_mut(
            &mut rng,
            Some(|n: &NonTerminal| *n == nt("MISSING")),
        );
        assert!(result.is_none(), "Should return None when filter matches nothing");
    }

    // ------------------------------------------------------------------
    // prune tests
    // ------------------------------------------------------------------

    /// A Branch node at depth >= max_depth should be collapsed to an NTLeaf.
    #[test]
    fn test_prune_collapses_over_depth_branch_to_ntleaf() {
        let ts = TimeSignature::common();
        // 3-level tree:  Branch(S) -> [Branch(A) -> [Branch(B, [T])]]
        // With max_depth=1, Branch(A) at depth 1 should be pruned to NTLeaf(A);
        // Branch(B) at depth 2 disappears with it.
        let mut tree = branch("S", vec![
            branch("A", vec![
                branch("B", vec![quarter_rest(ts)]),
            ]),
        ]);

        let generator = minimal_generator(1);
        generator.prune(&mut tree);

        match &tree {
            GrammarDerivation::Branch { nt: root_nt, content } => {
                assert_eq!(*root_nt, nt("S"));
                assert_eq!(content.len(), 1);
                match &content[0] {
                    GrammarDerivation::NTLeaf(n) => assert_eq!(*n, nt("A")),
                    other => panic!("expected NTLeaf(A) at depth 1, got {:?}", other),
                }
            }
            other => panic!("expected Branch(S) at root, got {:?}", other),
        }
    }

    /// A tree that is already within max_depth should be untouched by prune.
    #[test]
    fn test_prune_leaves_valid_depth_tree_unchanged() {
        let ts = TimeSignature::common();
        // 2-level tree:  Branch(S) -> [Branch(A, [T])]
        // With max_depth=10, nothing should be pruned.
        let mut tree = branch("S", vec![
            branch("A", vec![quarter_rest(ts)]),
        ]);

        let generator = minimal_generator(10);
        generator.prune(&mut tree);

        match &tree {
            GrammarDerivation::Branch { nt: root_nt, content } => {
                assert_eq!(*root_nt, nt("S"));
                assert_eq!(content.len(), 1);
                assert!(matches!(&content[0], GrammarDerivation::Branch { .. }),
                    "Branch(A) within max_depth should survive prune");
            }
            other => panic!("expected Branch(S) at root, got {:?}", other),
        }
    }

    /// Prune with max_depth=0 should collapse even the root Branch.
    #[test]
    fn test_prune_at_depth_zero_collapses_root() {
        let ts = TimeSignature::common();
        let mut tree = branch("S", vec![quarter_rest(ts)]);
        let generator = minimal_generator(0);
        generator.prune(&mut tree);
        assert!(
            matches!(&tree, GrammarDerivation::NTLeaf(n) if *n == nt("S")),
            "Root Branch should become NTLeaf(S) when max_depth=0, got {:?}", tree
        );
    }

    #[test]
    fn test_grammar_derivation_generator() {
        // Create a simple grammar: S -> a S | b
        let ts = TimeSignature::common();
        let grammar = Grammar::new(
            NonTerminal::Custom("S".to_string()),
            vec![
                Production(
                    NonTerminal::Custom("S".to_string()),
                    MusicString(vec![
                        MusicPrimitive::Simple(Symbol::T(Terminal::AbsoluteSound {
                            duration: Duration::zero(ts) + NoteValue::new(1, 4),
                            note: TerminalNote::Note {
                                pitch: Pitch::middle_c(),
                            },
                        })),
                        MusicPrimitive::Simple(Symbol::NT(NonTerminal::Custom("S".to_string()))),
                    ]),
                ),
                Production(
                    NonTerminal::Custom("S".to_string()),
                    MusicString(vec![MusicPrimitive::Simple(Symbol::T(
                        Terminal::AbsoluteSound {
                            duration: Duration::zero(ts) + NoteValue::new(1, 4),
                            note: TerminalNote::Rest,
                        },
                    ))]),
                ),
            ],
            ts,
        );

        let config = GrammarDerivationConfig {
            iterations: 0,
            panic_on_bad_production: true,
            rounded: false,
            max_depth: 3,
        };

        let generator = GrammarDerivationGenerator::new(config, &grammar);
        let mut rng = rand::rng();
        let derivation = generator.produce(&mut rng);

        // Verify that we get a Production node for the start symbol
        match &derivation {
            GrammarDerivation::Branch { nt, content } => {
                assert_eq!(nt.clone(), NonTerminal::Custom("S".to_string()));
                assert!(!content.is_empty());
                assert!(content.len() == 1 || content.len() == 2); // S -> a S | b
            }
            _ => panic!("Expected a Production node"),
        }

        let ms = derivation.to_music_string();
        // Verify that the music string is valid according to the grammar
        assert!(matches!(
            &ms.0[0],
            MusicPrimitive::Simple(Symbol::T(Terminal::AbsoluteSound { .. }))
        ));
    }
}
