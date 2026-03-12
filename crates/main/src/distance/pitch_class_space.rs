use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// This defines Ord in terms of reverse alphabetical according to the levels' labels: a, b, c, d, e.
/// Therefore, Octave > Fifth.
/// This corresponds with the visual representation in the Display impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpaceLevel {
    Chromatic,
    Diatonic,
    Triadic,
    Fifth,
    Octave
}

pub const A: SpaceLevel = SpaceLevel::Octave;
pub const B: SpaceLevel = SpaceLevel::Fifth;
pub const C: SpaceLevel = SpaceLevel::Triadic;
pub const D: SpaceLevel = SpaceLevel::Diatonic;
pub const E: SpaceLevel = SpaceLevel::Chromatic;

pub const NUM_LEVELS: usize = 5;
pub const LEVELS: [SpaceLevel; NUM_LEVELS] = [A, B, C, D, E];

#[derive(Debug, Clone)]
pub struct PitchClassSpace {
    /// Highest level starting with pc0, etc.
    pub highest_levels: [SpaceLevel; 12],
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
            highest_levels: [
                A,
                E,
                D,
                E,
                C,
                D,
                E,
                B,
                E,
                D,
                E,
                D,
            ],
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
            highest_levels: [
                A,
                E,
                D,
                C,
                E,
                D,
                E,
                B,
                D,
                E,
                D,
                E,
            ],
        }
    }

    fn get_steps_on_level(&self, level: SpaceLevel) -> Vec<usize> {
        // This should return the steps that are on the given level.
        // For example, if we are looking for the steps on the fifth level, then we should return
        // the steps that have B as their highest level (generally just 0 and 7).
        self.highest_levels.iter().enumerate()
            .filter_map(|(i, &l)| if level <= l { Some(i) } else { None })
            .collect()
    }
    pub fn rotate_on_level(&mut self, level: SpaceLevel, units: usize) {
        // This should rotate all the pitch classes "on top" of the level by that amount,
        // on the steps of the level.
        // For example, if we are rotating on fifths, then we should move all the levels above the
        // fifths (octave), stepping by pitches that are on the fifth level (generally just 0 and 7).
        // Everything below that level should stay the same.
        println!("Rotating on level {:?} by {} units", level, units);
        let mut next_highest_levels = self.highest_levels.clone();
        let steps_on_level = self.get_steps_on_level(level);
        for (pc, &l) in self.highest_levels.iter().enumerate() {
            if let Some(step_index) = steps_on_level.iter().position(|&s| s == pc) {
                let new_step_index = (step_index + units) % steps_on_level.len();
                let new_pc = steps_on_level[new_step_index];
                next_highest_levels[new_pc] = l;
            } else {
                next_highest_levels[pc] = l;
            }
        }

        println!("Next highest levels: {:?}", next_highest_levels);

        self.highest_levels = next_highest_levels;
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
            writeln!(f, "{}: {}", match level {
                A => "a",
                B => "b",
                C => "c",
                D => "d",
                E => "e",
            }, line)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::distance::pitch_class_space::*;
    use crate::distance::pitch_class_space::SpaceLevel::{Chromatic, Diatonic};

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
        let space = super::PitchClassSpace::c_maj();
        let expected_string =
                              "a: 0                       \n\
                               b: 0             7         \n\
                               c: 0       4     7         \n\
                               d: 0   2   4 5   7   9   b \n\
                               e: 0 1 2 3 4 5 6 7 8 9 a b \n";
        assert_eq!(space.to_string(), expected_string);

        let space = super::PitchClassSpace::c_nat_min();
        let expected_string =
                              "a: 0                       \n\
                               b: 0             7         \n\
                               c: 0     3       7         \n\
                               d: 0   2 3   5   7 8   a   \n\
                               e: 0 1 2 3 4 5 6 7 8 9 a b \n";
        assert_eq!(space.to_string(), expected_string);
    }

}