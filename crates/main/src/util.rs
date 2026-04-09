use std::process::Command;
use rayon::prelude::*;
use crate::distance::pitch_class_space::PitchClassSpace;
use crate::distance::pitch_class_space::SpaceLevel::Chromatic;
use music_primitives::{Pitch, PitchClass};

pub fn render_midi_to_samples(experiment_location: &str) {
    let lily_dir    = format!("{experiment_location}/lilypond_output");
    let samples_dir = format!("{experiment_location}/samples");
    let soundfont   = format!("{experiment_location}/soundfont.sf2");

    std::fs::create_dir_all(&samples_dir).expect("Failed to create samples/");

    let midi_files: Vec<_> = std::fs::read_dir(&lily_dir)
        .unwrap_or_else(|e| panic!("Failed to read {lily_dir}: {e}"))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "midi" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    println!("Rendering {} MIDI files to MP3...", midi_files.len());

    midi_files.par_iter().for_each(|midi_path| {
        let stem = midi_path.file_stem()
            .expect("midi file has no stem")
            .to_string_lossy();
        let mp3_path = format!("{samples_dir}/{stem}.mp3");

        let status = Command::new("fluidsynth")
            .args(["-ni", "-r", "44100", "-F", &mp3_path, &soundfont, midi_path.to_str().unwrap()])
            .status()
            .unwrap_or_else(|e| panic!("Failed to launch fluidsynth for {stem}: {e}"));

        if !status.success() {
            eprintln!("fluidsynth exited with non-zero status for {stem}");
        }
    });

    println!("Done. MP3 samples -> {samples_dir}/");
}

fn generate_all_chords() -> Vec<String> {
    // It's either a major or minor region
    // The possible regions are:
    // I II III IV V VI VII and their minor counterparts i ii iii iv v vi vii

    // The chords belonging to a major region can be:
    // I ii iii IV V vi vii_o
    // The chords belonging to a minor region can be:
    // i ii_o III iv v VI VII

    let major_regions = vec!["I", "II", "III", "IV", "V", "VI", "VII"];
    let major_region_chords = vec!["I", "ii", "iii", "IV", "V", "vi", "vii_o"];

    let minor_regions = vec!["i", "ii", "iii", "iv", "v", "vi", "vii"];
    let minor_region_chords = vec!["i", "ii_o", "III", "iv", "v", "VI", "VII"];

    let mut chords = vec![];
    for region in &major_regions {
        for chord in &major_region_chords {
            chords.push(format!("{}/{}", chord, region));
        }
    }
    for region in &minor_regions {
        for chord in &minor_region_chords {
            chords.push(format!("{}/{}", chord, region));
        }
    }
    chords
}

/// Generate the non-terminals for all chords in the key of the given tone center.
/// prefix should be a string that, itself, will identify as a non-terminal
/// duration is the duration that should be assigned to each note in the chord (e.g. "1/2" for half notes).
/// It should be written as "<1/2>" as in the language, or "" to default to quarter notes.
fn generate_chord_nts(tone_center: PitchClass, prefix: &str, duration: &str) -> Vec<String> {
    let note_name = |pitch: Pitch| format!("{}{}", pitch.octave(), pitch.letter_name());
    generate_all_chords()
        .into_iter()
        .map(|chord| {
            let mut pcs: PitchClassSpace = chord.parse().unwrap();
            pcs.rotate_on_level(Chromatic, tone_center.to_note_num() as isize);
            // choose notes from the chord to be played
            // root should be in octave 2
            // the others should be in octave 3
            let root = Pitch::new(2, pcs.get_root());
            let others = pcs
                .get_non_root_chord_pcs()
                .into_iter()
                .map(|pc| Pitch::new(3, pc))
                .collect::<Vec<_>>();
            // should look something like this:
            // #I/V = { :g2<1/2> | :d3<1/2> | :b3<1/2> }
            let start = format!("{prefix}{chord} = {{ :{}{} ", note_name(root), duration);
            let middle = others
                .into_iter()
                .map(|pitch| format!("| :{}{} ", note_name(pitch), duration))
                .collect::<String>();
            format!("{}{}}}", start, middle)
        })
        .collect()
}

fn generate_harmony_chooser(name: &str, prefix: &str) -> String {
    let chords = generate_all_chords();
    chords.iter()
        .map(|chord| format!("{name} = {prefix}{chord}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::*;
    use music_primitives::PitchClass;
    use music_turtle_lang::cfg::{GrammarDerivationConfig, GrammarDerivationGenerator};
    use music_turtle_lang::grammar_from_file;
    use music_turtle_lang::lilypond::{LilyPondConfig, call_lilypond_cli, render_to_lilypond};

    #[test]
    fn test_generate_chord_nts() {
        let prefix = "#";
        let preamble = generate_harmony_chooser("harmony_chooser", prefix);
        let nts = generate_chord_nts(PitchClass::C, prefix, "<1/2>");

        let full_grammar = format!("start harmony_chooser\n{}\n{}", preamble, nts.join("\n"));
        println!("{}", full_grammar);
        // test to make sure it can be parsed
        let grammar = full_grammar.parse::<music_turtle_lang::cfg::Grammar>().unwrap();
    }
}
