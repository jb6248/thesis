use crate::distance::pitch_class_space::SpaceLevel::*;
use enumkit::EnumMapping;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Sub;
use std::str::FromStr;

/// This defines Ord in terms of reverse alphabetical according to the levels' labels: a, b, c, d, e.
/// Therefore, Octave > Fifth.
/// This corresponds with the visual representation in the Display impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumMapping)]
pub enum SpaceLevel {
    Chromatic,
    Diatonic,
    Triadic,
    Fifth,
    Octave,
}

lazy_static! {
    pub static ref SPACE_LEVEL_INT: SpaceLevelMapping<isize> =
        SpaceLevelMapping::new(|sl| match sl {
            Chromatic => 1,
            Diatonic => 2,
            Triadic => 3,
            Fifth => 4,
            Octave => 5,
        });
}

pub const A: SpaceLevel = SpaceLevel::Octave;
pub const B: SpaceLevel = SpaceLevel::Fifth;
pub const C: SpaceLevel = SpaceLevel::Triadic;
pub const D: SpaceLevel = SpaceLevel::Diatonic;
pub const E: SpaceLevel = SpaceLevel::Chromatic;

pub const NUM_LEVELS: usize = 5;
pub const LEVELS: [SpaceLevel; NUM_LEVELS] = [A, B, C, D, E];

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Copy)]
pub struct PitchClassSpace {
    /// Highest level starting with pc0, etc.
    pub highest_levels: [SpaceLevel; 12],
}

impl Sub for SpaceLevel {
    type Output = isize;

    fn sub(self, rhs: Self) -> Self::Output {
        SPACE_LEVEL_INT.get(self) - SPACE_LEVEL_INT.get(rhs)
    }
}

impl Sub for &SpaceLevel {
    type Output = isize;

    fn sub(self, rhs: Self) -> Self::Output {
        *self - *rhs
    }
}

impl PitchClassSpace {
    pub fn c_maj() -> Self {
        // Should initialize a pitch class space that looks like this:
        /*
        I/C

        a: 0
        b: 0             7
        c: 0       4     7
        d: 0   2   4 5   7   9   b
        e: 0 1 2 3 4 5 6 7 8 9 a b
        */
        Self {
            highest_levels: [A, E, D, E, C, D, E, B, E, D, E, D],
        }
    }
    pub fn c_nat_min() -> Self {
        // Should initialize a pitch class space that looks like this:
        /*
        i/C

        a: 0
        b: 0             7
        c: 0     3       7
        d: 0   2 3   5   7 8   a
        e: 0 1 2 3 4 5 6 7 8 9 a b
        */
        Self {
            highest_levels: [A, E, D, C, E, D, E, B, D, E, D, E],
        }
    }

    pub fn c_harmonic_min() -> Self {
        // Should initialize a pitch class space that looks like this:
        /*
        i/C
        Note: this doesn't really make sense because this is only for dominant chords
        like V or vii_o

        a: 0
        b: 0             7
        c: 0     3       7
        d: 0   2 3   5   7 8     b
        e: 0 1 2 3 4 5 6 7 8 9 a b
        */
        Self {
            highest_levels: [A, E, D, C, E, D, E, B, D, E, D, E],
        }
    }

    fn get_steps_on_level(&self, level: SpaceLevel) -> Vec<usize> {
        // This should return the steps that are on the given level.
        // For example, if we are looking for the steps on the fifth level, then we should return
        // the steps that have B as their highest level (generally just 0 and 7).
        self.highest_levels
            .iter()
            .enumerate()
            .filter_map(|(i, &l)| if level <= l { Some(i) } else { None })
            .collect()
    }

    pub fn rotate_on_level(&mut self, level: SpaceLevel, mut units: isize) {
        // This should rotate all the pitch classes "on top" of the level by that amount,
        // on the steps of the level.
        // For example, if we are rotating on fifths, then we should move all the levels above the
        // fifths (octave), stepping by pitches that are on the fifth level (generally just 0 and 7).
        // Everything below that level should stay the same.
        let mut next_highest_levels = self.highest_levels.clone();
        let steps_on_level = self.get_steps_on_level(level);
        while units < 0 {
            units += steps_on_level.len() as isize;
        }
        units = units % (steps_on_level.len() as isize);
        let units = units as usize;
        for (pc, &l) in self.highest_levels.iter().enumerate() {
            if let Some(step_index) = steps_on_level.iter().position(|&s| s == pc) {
                let new_step_index = (step_index + units) % steps_on_level.len();
                let new_pc = steps_on_level[new_step_index];
                next_highest_levels[new_pc] = l;
            } else {
                next_highest_levels[pc] = l;
            }
        }

        self.highest_levels = next_highest_levels;
    }

    pub fn regional_distance_with_result(&self, other: &Self) -> (usize, PitchClassSpace) {
        // Calculate minimum number of rotations along the "region circle" to get somewhere else.
        // we only care about the equality of the diatonic level; i.e., get_steps_on_level(D) should be the same for both spaces.
        // Therefore, we can just check how many rotations it takes to get the diatonic steps to match.
        let mut distance = 0;
        let mut current_space = self.clone();
        while current_space.get_steps_on_level(Diatonic) != other.get_steps_on_level(Diatonic) {
            current_space.rotate_on_level(Chromatic, 7);
            distance += 1;
            if distance > 12 {
                panic!("Target pitch class space is unreachable.");
            }
        }
        // going the other direction is just the inverse of this
        (distance.min(12 - distance), current_space)
    }

    /// This calculates $i$.
    pub fn regional_distance(&self, other: &Self) -> usize {
        self.regional_distance_with_result(other).0
    }

    pub fn chord_distance_with_result(&self, other: &Self) -> (usize, PitchClassSpace) {
        // note that going to the PARALLEL minor should end up being 0 total.

        let mut current_pcs = self.clone();

        let other_region = other.get_steps_on_level(Diatonic);
        let regional_dist = if !other_region.contains(&self.get_root())
            || !other_region.contains(&self.get_fifth())
        {
            // if the other space is not in the same region, then we need to get to the same region first.
            let (regional_distance, pcs) = self.regional_distance_with_result(other);
            current_pcs = pcs;
            regional_distance
        } else {
            0
        };
        // now they're the same-ish region, so we just do chord movement until the root is the same
        let mut chord_dist = 0;
        let mut current_space = current_pcs;
        while current_space.get_root() != other.get_root() {
            current_space.rotate_on_level(Diatonic, 4);
            chord_dist += 1;
            if chord_dist > 7 {
                panic!("Target pitch class space is unreachable.");
            }
        }
        chord_dist = chord_dist.min(7 - chord_dist);

        // the total value for $j$ is the number of circle movements to get to the same diatonic + chord circle movements
        (regional_dist + chord_dist, current_space)
    }

    /// This calculates $j$.
    pub fn chord_distance(&self, other: &Self) -> usize {
        self.chord_distance_with_result(other).0
    }

    /// This calculates $k$.
    pub fn count_new_pcs(&self, other: &Self) -> usize {
        self.highest_levels
            .iter()
            .zip(other.highest_levels.iter())
            .map(|(me, other)| (me - other).max(0))
            .sum::<isize>() as usize
    }

    /// This should be the chord distance function, d(X->Y) = i + j + k.
    pub fn total_distance(&self, other: &Self) -> usize {
        let i = self.regional_distance(other);
        let j = self.chord_distance(other);
        let k = self.count_new_pcs(&other);
        i + j + k
    }

    pub fn get_root(&self) -> usize {
        // The root is the only pitch class that has the octave as its highest level.
        self.highest_levels.iter().position(|&l| l == A).unwrap()
    }

    pub fn get_non_root_chord_pcs(&self) -> Vec<usize> {
        // The non-root chord pcs are the ones that have the triadic level as their highest level.
        self.highest_levels.iter().enumerate().filter_map(|(i, &l)| if l >= Triadic && l < Octave { Some(i) } else { None }).collect()
    }

    pub fn get_fifth(&self) -> usize {
        // The fifth is the only pitch class that has the fifth as its highest level.
        // And it should not be the root.
        self.highest_levels.iter().position(|&l| l == B).unwrap()
    }
}

fn get_roman_numeral_chromatic_offset(roman_numeral: &str) -> Option<usize> {
    if roman_numeral.ends_with("_o") {
        return get_roman_numeral_chromatic_offset(&roman_numeral[..roman_numeral.len() - 2]);
    }
    match roman_numeral {
        "I" | "i" => Some(0),
        "II" | "ii" => Some(2),
        "III" | "iii" => Some(4),
        "IV" | "iv" => Some(5),
        "V" | "v" => Some(7),
        "VI" | "vi" => Some(9),
        "VII" | "vii" => Some(11),
        _ => None,
    }
}

fn get_roman_numeral_diatonic_offset(roman_numeral: &str) -> Option<usize> {
    if roman_numeral.ends_with("_o") {
        return get_roman_numeral_diatonic_offset(&roman_numeral[..roman_numeral.len() - 2]);
    }
    match roman_numeral {
        "I" | "i" => Some(0),
        "II" | "ii" => Some(1),
        "III" | "iii" => Some(2),
        "IV" | "iv" => Some(3),
        "V" | "v" => Some(4),
        "VI" | "vi" => Some(5),
        "VII" | "vii" => Some(6),
        _ => None,
    }
}

/// Returns the chromatic offset of a region given its Roman numeral. For example, "I" should return 0, "II" should return 2, "bIII" should return 3, "#IV" should return 6, etc.
/// A region name is one of I, i, II, ii, ... VII, vii, and can optionally be prefixed with "b" or "#" to indicate a flat or sharp region, respectively.
pub fn get_regional_offset_from_roman_numeral(roman_numeral: &str) -> Option<usize> {
    if let Some(offset) = get_roman_numeral_chromatic_offset(roman_numeral) {
        Some(offset)
    } else {
        if roman_numeral.starts_with("b") {
            // this is a "flat" region
            get_regional_offset_from_roman_numeral(&roman_numeral[1..]).map(|offset| (offset + 11) % 12)
        } else if roman_numeral.starts_with("#") {
            // this is a "sharp" region
            get_regional_offset_from_roman_numeral(&roman_numeral[1..]).map(|offset| (offset + 1) % 12)
        } else {
            None
        }
    }
}

fn is_roman_numeral(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'i' | 'v')
}

fn is_minor_chord(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_lowercase() && is_roman_numeral(c))
}

/// Returns the pitch class space corresponding to a chord assuming a root of pc0.
/// This can be moved to the correct region using get_regional_offset_from_roman_numeral()
/// In addition, it can be suffixed with "_o" to indicate diminished. It can also be suffixed with "_7" to indicate a seventh chord.
pub fn get_pitch_class_space_from_roman_numeral(chord_diatonic_offset: usize, minor_region: bool) -> Option<PitchClassSpace> {
    // we have to determine the diatonic scale of the chord
    // minor region + dominant chord => harmonic minor scale
    // minor region otherwise => natural minor scale
    let is_dominant_chord = chord_diatonic_offset == 4 || chord_diatonic_offset == 6; // V or vii_o
    let mut space = if minor_region {
        if is_dominant_chord {
            PitchClassSpace::c_harmonic_min()
        } else {
            PitchClassSpace::c_nat_min()
        }
    } else {
        PitchClassSpace::c_maj()
    };
    space.rotate_on_level(Diatonic, chord_diatonic_offset as isize);
    Some(space)
}



impl FromStr for PitchClassSpace {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if there's a '/', then split on it
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            // region is explicitly specified: the first part is the chord, second part is the region
            let minor_region = is_minor_chord(parts[1]);
            if let Some(chord_diatonic_offset) = get_roman_numeral_diatonic_offset(parts[0]) {
                if let Some(regional_chromatic_offset) = get_regional_offset_from_roman_numeral(parts[1]) {
                    let mut space = get_pitch_class_space_from_roman_numeral(chord_diatonic_offset, minor_region).ok_or_else(|| format!("Invalid Roman numeral: {}", s))?;
                    space.rotate_on_level(Chromatic, regional_chromatic_offset as isize);
                    Ok(space)
                } else {
                    Err(format!("Invalid region Roman numeral: {}", s))
                }
            } else {
                Err(format!("Invalid chord Roman numeral: {}", s))
            }
        } else {
            Err(format!("Invalid input: {}", s))
        }
    }
}

impl Display for PitchClassSpace {
    /*
    This should display something like this for I/C:
    ```
    a: 0
    b: 0             7
    c: 0       4     7
    d: 0   2   4 5   7   9   b
    e: 0 1 2 3 4 5 6 7 8 9 a b
    ```
    Although this is very redundant, it's consistent with the research writing and easier
    to view at a glance.
     */
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for level in LEVELS {
            let mut line = String::new();
            for (i, highest_level) in self.highest_levels.iter().enumerate() {
                if *highest_level as usize >= level as usize {
                    line.push_str(&format!("{:x} ", i));
                } else {
                    line.push_str("  ");
                }
            }
            writeln!(
                f,
                "{}: {}",
                match level {
                    A => "a",
                    B => "b",
                    C => "c",
                    D => "d",
                    E => "e",
                },
                line
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::distance::pitch_class_space::SpaceLevel::{Chromatic, Diatonic};
    use crate::distance::pitch_class_space::*;


    #[test]
    fn test_chord_equality() {
        let pcs1: PitchClassSpace = "ii_o/vi".parse().unwrap();
        let pcs2: PitchClassSpace = "vii_o/I".parse().unwrap();
        assert_eq!(pcs1, pcs2);

        let pcs1: PitchClassSpace = "I/V".parse().unwrap();
        let pcs2: PitchClassSpace = "V/I".parse().unwrap();
        assert_ne!(pcs1, pcs2);

        let pcs1: PitchClassSpace = "III/vi".parse().unwrap();
        let pcs2: PitchClassSpace = "I/I".parse().unwrap();
        assert_eq!(pcs1, pcs2);

        let pcs1: PitchClassSpace = "iv/vi".parse().unwrap();
        let pcs2: PitchClassSpace = "ii".parse().unwrap();
        assert_eq!(pcs1.to_string(), pcs2.to_string());
        assert_eq!(pcs1, pcs2);

        let pcs1: PitchClassSpace = "VI/vi".parse().unwrap();
        let pcs2: PitchClassSpace = "IV".parse().unwrap();
        assert_eq!(pcs1, pcs2);

        let pcs1: PitchClassSpace = "VII/vi".parse().unwrap();
        let pcs2: PitchClassSpace = "V".parse().unwrap();
        assert_eq!(pcs1, pcs2);
    }

    #[test]
    fn test_parse_pitch_class_space() {
        let pcs: PitchClassSpace = "I/I".parse().unwrap();
        assert_eq!(pcs, PitchClassSpace::c_maj());

        let pcs: PitchClassSpace = "i/i".parse().unwrap();
        assert_eq!(pcs, PitchClassSpace::c_nat_min());

        let pcs: PitchClassSpace = "V/IV".parse().unwrap();
        let mut expected = PitchClassSpace::c_maj();
        expected.rotate_on_level(Chromatic, 5);
        expected.rotate_on_level(Diatonic, 4);
        assert_eq!(pcs, expected);

        let pcs: PitchClassSpace = "ii_o/vi".parse().unwrap();
        let mut expected = PitchClassSpace::c_maj();
        expected.rotate_on_level(Diatonic, 5);
        expected.rotate_on_level(Diatonic, 1);
        assert_eq!(pcs, expected);

        let pcs: PitchClassSpace = "ii/I".parse().unwrap();
        let mut expected = PitchClassSpace::c_nat_min();
        expected.rotate_on_level(Chromatic, 2);

        let pcs: PitchClassSpace = "v/v".parse().unwrap();
        let mut expected = PitchClassSpace::c_harmonic_min();
        expected.rotate_on_level(Chromatic, 7);
        expected.rotate_on_level(Diatonic, 4);
        assert_eq!(pcs, expected);
    }
    #[test]
    fn test_total_distances() {
        let origin = PitchClassSpace::c_maj();

        let mut V_I = origin.clone();
        V_I.rotate_on_level(Diatonic, 4);
        assert_eq!(origin.total_distance(&V_I), 5);

        let mut I_V = origin.clone();
        I_V.rotate_on_level(Chromatic, 7);
        assert_eq!(origin.total_distance(&I_V), 7);

        let mut I_VI = origin.clone();
        I_VI.rotate_on_level(Chromatic, 9);
        assert_eq!(origin.total_distance(&I_VI), 14);

        let mut i_ii = origin.clone();
        i_ii.rotate_on_level(Chromatic, 5);
        i_ii.rotate_on_level(Diatonic, -2);
        assert_eq!(origin.total_distance(&i_ii), 10);

        let mut I_IV = origin.clone();
        I_IV.rotate_on_level(Chromatic, 5);
        assert_eq!(origin.total_distance(&I_IV), 7);

        let mut i_vi = origin.clone();
        i_vi.rotate_on_level(Diatonic, 5);
        assert_eq!(origin.total_distance(&i_vi), 7);

        let mut i_i = origin.clone();
        i_i.rotate_on_level(Diatonic, 5);
        i_i.rotate_on_level(Chromatic, 3);
        assert_eq!(origin.total_distance(&i_i), 7);
    }

    #[test]
    fn test_inter_regional_distance_j() {
        let origin = PitchClassSpace::c_maj();

        // parallel minor: j = 0 for chord distance
        let mut i_i = origin.clone();
        i_i.rotate_on_level(Diatonic, 5); // rotate to A
        i_i.rotate_on_level(Chromatic, 3); // rotate root to C
        println!("i_i:\n{}", i_i);
        let j_i = origin.chord_distance(&i_i);
        assert_eq!(j_i, 0);

        // relative minor: j = 3 for chord distance
        let mut i_vi = origin.clone();
        i_vi.rotate_on_level(Diatonic, 5);
        let j_i_vi = origin.chord_distance(&i_vi);
        println!("i_vi:\n{}", i_vi);
        assert_eq!(j_i_vi, 3);

        // E minor
        let mut i_iii = origin.clone();
        i_iii.rotate_on_level(Chromatic, 7);
        i_iii.rotate_on_level(Diatonic, -2);
        let j_iii = origin.chord_distance(&i_iii);
        println!("i_iii:\n{}", i_iii);
        assert_eq!(j_iii, 3);
    }

    #[test]
    fn test_regional_distance_i() {
        let origin = PitchClassSpace::c_maj();

        let mut IV = origin.clone();
        IV.rotate_on_level(Chromatic, 5);
        let i_IV = origin.regional_distance(&IV);
        assert_eq!(i_IV, 1);

        let mut vii_o = origin.clone();
        vii_o.rotate_on_level(Chromatic, -1);
        let i_vii_o = origin.regional_distance(&vii_o);
        assert_eq!(i_vii_o, 5);

        let mut V = origin.clone();
        V.rotate_on_level(Chromatic, 7);
        let i_V = origin.regional_distance(&V);
        assert_eq!(i_V, 1);

        let mut ii = origin.clone();
        ii.rotate_on_level(Chromatic, 2);
        let i_ii = origin.regional_distance(&ii);
        assert_eq!(i_ii, 2);

        let mut IV_IV = origin.clone();
        IV_IV.rotate_on_level(Chromatic, -2);
        let i_IV_IV = origin.regional_distance(&IV_IV);
        assert_eq!(i_IV_IV, 2);
    }

    #[test]
    fn test_chordal_distance_j_within_region() {
        let origin = PitchClassSpace::c_maj();

        let mut IV = origin.clone();
        IV.rotate_on_level(Diatonic, 4);
        let j_IV = origin.chord_distance(&IV);
        assert_eq!(j_IV, 1);

        let mut vii_o = origin.clone();
        vii_o.rotate_on_level(Diatonic, -1);
        let j_vii_o = origin.chord_distance(&vii_o);
        assert_eq!(j_vii_o, 2);

        let mut iii = origin.clone();
        iii.rotate_on_level(Diatonic, 2);
        let j_iii = origin.chord_distance(&iii);
        assert_eq!(j_iii, 3);

        let mut ii = origin.clone();
        ii.rotate_on_level(Diatonic, 1);
        let j_ii = origin.chord_distance(&ii);
        assert_eq!(j_ii, 2);
    }

    #[test]
    fn test_distinctive_pcs_k() {
        // testing k values
        let origin = PitchClassSpace::c_maj();

        let mut IV = origin.clone();
        IV.rotate_on_level(Diatonic, 4);
        let k_IV = origin.count_new_pcs(&IV);
        assert_eq!(k_IV, 4);

        let mut vii_o = origin.clone();
        vii_o.rotate_on_level(Diatonic, -4);
        vii_o.rotate_on_level(Diatonic, -4);
        let k_vii_o = origin.count_new_pcs(&vii_o);
        assert_eq!(k_vii_o, 6);

        let mut iii = origin.clone();
        iii.rotate_on_level(Diatonic, 2);
        let k_iii = origin.count_new_pcs(&iii);
        assert_eq!(k_iii, 4);
    }

    #[test]
    fn rotate_pcs() {
        /*
        if this is I/C:
        a: 0
        b: 0             7
        c: 0       4     7
        d: 0   2   4 5   7   9   b
        e: 0 1 2 3 4 5 6 7 8 9 a b

        then rotating by 7 on the chromatic level should give us:
        I/V
        a:               7
        b:     2         7
        c:     2         7       b
        d: 0   2   4   6 7   9   b
        e: 0 1 2 3 4 5 6 7 8 9 a b

        and rotating by 4 on the diatonic level should give us:
        V/I
        a:               7
        b:     2         7
        c:     2         7       b
        d: 0   2   4 5   7   9   b
        e: 0 1 2 3 4 5 6 7 8 9 a b
         */

        let space = PitchClassSpace::c_maj();
        let I_V = {
            let mut space = space.clone();
            space.rotate_on_level(Chromatic, 7);
            space
        };
        let I_V2 = {
            let mut space = space.clone();
            space.rotate_on_level(Chromatic, -5);
            space
        };
        assert_eq!(I_V, I_V2);
        let expected_I_V_string = "a:               7         \n\
                                        b:     2         7         \n\
                                        c:     2         7       b \n\
                                        d: 0   2   4   6 7   9   b \n\
                                        e: 0 1 2 3 4 5 6 7 8 9 a b \n";
        assert_eq!(I_V.to_string(), expected_I_V_string);

        let V_I = {
            let mut space = space.clone();
            space.rotate_on_level(Diatonic, 4);
            space
        };
        let V_I2 = {
            let mut space = space.clone();
            space.rotate_on_level(Diatonic, -3);
            space
        };
        assert_eq!(V_I, V_I2);
        let expected_V_I_string = "a:               7         \n\
                                        b:     2         7         \n\
                                        c:     2         7       b \n\
                                        d: 0   2   4 5   7   9   b \n\
                                        e: 0 1 2 3 4 5 6 7 8 9 a b \n";
        assert_eq!(V_I.to_string(), expected_V_I_string);
    }

    #[test]
    fn test_get_steps_on_level() {
        let space = PitchClassSpace::c_maj();
        assert_eq!(space.get_steps_on_level(A), vec![0]);
        assert_eq!(space.get_steps_on_level(B), vec![0, 7]);
        assert_eq!(space.get_steps_on_level(C), vec![0, 4, 7]);
        assert_eq!(space.get_steps_on_level(D), vec![0, 2, 4, 5, 7, 9, 11]);
        assert_eq!(space.get_steps_on_level(E), (0..12).collect::<Vec<_>>());
    }
    #[test]
    fn test_display_pcs() {
        let space = PitchClassSpace::c_maj();
        let expected_string = "a: 0                       \n\
                               b: 0             7         \n\
                               c: 0       4     7         \n\
                               d: 0   2   4 5   7   9   b \n\
                               e: 0 1 2 3 4 5 6 7 8 9 a b \n";
        assert_eq!(space.to_string(), expected_string);

        let space = PitchClassSpace::c_nat_min();
        let expected_string = "a: 0                       \n\
                               b: 0             7         \n\
                               c: 0     3       7         \n\
                               d: 0   2 3   5   7 8   a   \n\
                               e: 0 1 2 3 4 5 6 7 8 9 a b \n";
        assert_eq!(space.to_string(), expected_string);
    }
}
