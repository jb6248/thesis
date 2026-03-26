pub mod lerdahl;
pub mod prolongational_tree;

use music_turtle_lang::cfg::{GrammarDerivation, MusicTransform, NonTerminal};
use crate::distance::pitch_class_space::PitchClassSpace;

/// Perform an in-order traversal of a derivation, looking for non-terminals that are chords.
/// Chords are prefixed with a specific string (e.g. "#") to distinguish them from other non-terminals.
/// For example, a non-terminal with the name "#I/V"
pub fn extract_chord_structure(
    derivation: &GrammarDerivation,
    chord_prefix: &str,
) -> Vec<PitchClassSpace> {
    let chords = extract_symbolic_chord_structure(derivation, chord_prefix);
    chords.into_iter()
        .map(|chord_str| chord_str.parse().expect(&format!("Failed to parse chord from NT name: {}", chord_str)))
        .collect()
}

pub fn extract_symbolic_chord_structure(
    derivation: &GrammarDerivation,
    chord_prefix: &str,
) -> Vec<String> {
    let process_derivations = |devs: &[GrammarDerivation]| {
        devs.iter()
            .flat_map(|dev| extract_symbolic_chord_structure(dev, chord_prefix))
            .collect()
    };
    let get_nt_chord = |nt: &NonTerminal| {
        match nt {
            NonTerminal::Custom(name) => {
                name.strip_prefix(chord_prefix)
                    .map(|s| s.to_owned())
            }
        }
    };
    match derivation {
        GrammarDerivation::Branch { nt, content } => {
            // check the content if it isn't a chord NT
            if let Some(chord) = get_nt_chord(nt) {
                vec![chord.to_owned()]
            } else {
                process_derivations(content)
            }
        }
        GrammarDerivation::Wrapped { content, transform } => {
            match transform {
                MusicTransform::Transpose { .. } => {
                    // can't do analysis on transposed content yet
                    vec![]
                }
                MusicTransform::Repeat { num } => {
                    // extract chords from content and repeat them num times
                    let mut chords: Vec<_> = process_derivations(&content);
                    (0..*num).flat_map(|_| chords.iter().cloned()).collect()
                }
                MusicTransform::Compression { .. } => {
                    // the same as regular content because we don't care about timing
                    process_derivations(&content)
                }
            }
        }
        GrammarDerivation::Split { branches } => {
            // We'll probably expect to find chords in only one branch, so we will just keep the
            // first non-empty results, and discard the rest.
            branches.iter()
                .map(|branch| process_derivations(branch))
                .filter(|chords| !chords.is_empty())
                .next()
                .unwrap_or_else(|| vec![])
        }
        GrammarDerivation::NTLeaf(nt) => {
            // this was never rendered, but we'll count it anyway just in case
            if let Some(chord) = get_nt_chord(nt) {
                vec![chord]
            } else {
                vec![]
            }
        }
        GrammarDerivation::TLeaf(_) => vec![]
    }
}

#[cfg(test)]
mod test {
    use music_primitives::Pitch;
    use music_turtle_lang::cfg::{GrammarDerivationConfig, GrammarDerivationGenerator, Performer};
    use music_turtle_lang::composition::{Instrument, Volume};
    use music_turtle_lang::grammar_from_file;
    use music_turtle_lang::lilypond::{call_lilypond_cli, render_to_lilypond, LilyPondConfig};
    use crate::distance::pitch_class_space::PitchClassSpace;
    use crate::genetic_simulation::analysis::extract_chord_structure;
    use crate::genetic_simulation::analysis::lerdahl::{get_maximum_distance, get_total_interchordal_distances};

    #[test]
    fn test_generation_with_analysis() {
        let experiment_name = "initial";

        let config = GrammarDerivationConfig {
            iterations: 5,
            panic_on_bad_production: true,
            rounded: false,
            max_depth: 10,
        };

        let grammar =
            grammar_from_file(format!("data/experiments/{experiment_name}/grammar.mt").as_str())
                .unwrap();
        let generator = GrammarDerivationGenerator::new(config, &grammar);
        let mut rng = rand::rng();
        let derivation = generator.produce(&mut rng);

        let implied_initial = PitchClassSpace::c_maj();
        let chord_progression = extract_chord_structure(&derivation, "#");
        println!("{:?}", chord_progression);
        let max_dist_from_initial = get_maximum_distance(&chord_progression, &implied_initial);
        let total_interchordal_distance = get_total_interchordal_distances(&chord_progression, &implied_initial);
        println!("Max distance from initial: {}", max_dist_from_initial);
        println!("Total interchordal distance: {}", total_interchordal_distance);

        let performer = Performer {
            instrument: Instrument::Piano,
            volume: Volume(50),
            pitch: Pitch::middle_c(),
        };
        let mut composition = derivation.to_music_string().compose_v2(grammar.time_signature, performer)
            .unwrap();
        composition.transpose(24);

        let lilypond_filename = format!("data/experiments/{experiment_name}/render.ly");
        render_to_lilypond(
            composition,
            lilypond_filename.as_str(),
            Some(LilyPondConfig {
                write_dynamics: false,
                ..LilyPondConfig::default()
            }),
        )
        .unwrap();

        call_lilypond_cli(
            lilypond_filename.as_str(),
            &format!("data/experiments/{experiment_name}"),
            true,
        )
        .unwrap();
    }
}