mod genetic_simulation;
mod distance;
mod util;
pub mod template_6;
mod early_experiments;

use music_primitives::{Pitch, TimeSignature};
use music_turtle_lang::cfg::{GrammarDerivation, Performer};
use music_turtle_lang::composition::Instrument::Piano;
use music_turtle_lang::composition::Volume;
use music_turtle_lang::lilypond::{call_lilypond_cli, render_to_lilypond, LilyPondConfig};
use crate::template_6::template_6;

fn main() {
    // we are going to run a simulation
    // template_5(3, 0.05, "data/experiments/5.1");
    // template_5(3, 0.1, "data/experiments/5.2");
    // template_5(3, 0.2, "data/experiments/5.3");
    // template_5(3, 0.4, "data/experiments/5.4");
    // template_5(3, 0.4, "data/experiments/6");
    template_6(3, 0.4, "data/experiments/7");
}

/// Renders a GrammarDerivationGenome2 to a LilyPond file and compiles it.
/// `output_path` should be the full path to the .ly file (e.g. "data/experiments/2/trial_1_best.ly").
fn render_grammar_derivation_to_lilypond(derivation: &GrammarDerivation, time_signature: TimeSignature, output_path: &str) {
    let output_dir = std::path::Path::new(output_path)
        .parent()
        .and_then(|p| p.to_str())
        .expect("Invalid output path");
    let performer = Performer {
        instrument: Piano,
        volume: Volume(50),
        pitch: Pitch::middle_c(),
    };
    let music_string = derivation.to_music_string();
    let mut composition = music_string.compose_v2(time_signature, performer).unwrap();
    composition.transpose(24); // to make it easy to read
    let composition = composition;
    render_to_lilypond(
        composition,
        output_path,
        Some(LilyPondConfig {
            write_dynamics: false,
            piano_staff: true,
            ..LilyPondConfig::default()
        })
    ).unwrap();
    call_lilypond_cli(output_path, &format!("{output_dir}/"), false).unwrap();
}

