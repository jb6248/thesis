use crate::RenderError;
use crate::cfg::{Grammar, MusicPrimitive, MusicString, Performer};
use crate::composition::{Composition, Event, Track, Volume};
use crate::scan::ScanError;
use music_primitives::{Duration, Pitch, TimeSignature};
use num::ToPrimitive;
use std::fmt::Write as FmtWrite;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;

pub fn call_lilypond_cli(lilypond_file: &str, output_folder: &str, print_output: bool) -> std::io::Result<()> {
    use std::process::Command;

    let mut cmd = Command::new("lilypond");
    cmd
        .arg("-o")
        .arg(output_folder)
        .arg(lilypond_file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    println!("> {:?}", cmd);
    io::stdout().flush()?;
    io::stderr().flush()?;
    let output = cmd.output()?;

    if print_output {
        // io::stdout().write_all(&output.stdout)?;
        // io::stdout().write_all(&output.stderr)?;
        let lp_stdout = String::from_utf8_lossy(&output.stdout)
            .split("\n")
            .filter(|line| line.trim().len() > 0)
            .map(|line| format!("lilypond stdout: {}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let lp_stderr = String::from_utf8_lossy(&output.stderr)
            .split("\n")
            .filter(|line| line.trim().len() > 0)
            .map(|line| format!("lilypond stderr: {}", line))
            .collect::<Vec<String>>()
            .join("\n");
        if lp_stdout.trim().len() > 0 {
            println!("{}", lp_stdout);
        }
        if lp_stderr.trim().len() > 0 {
            eprintln!("{}", lp_stderr);
        }
    }

    if !output.status.success() {
        eprintln!("LilyPond command failed with status: {}", output.status);
    }

    Ok(())
}

/// Configuration for LilyPond rendering
#[derive(Debug, Clone)]
pub struct LilyPondConfig {
    /// LilyPond version string
    pub version: String,
    /// Default tempo in BPM
    pub tempo: Option<u32>,
    /// Title of the piece
    pub title: Option<String>,
    /// Composer name
    pub composer: Option<String>,
    /// Whether to write volumes
    pub write_dynamics: bool,
    /// Maximum 2^(-n) for note duration denominators (this should be <= 0)
    pub min_neg_power: Option<i32>,
}

impl Default for LilyPondConfig {
    fn default() -> Self {
        Self {
            version: "2.24.0".to_string(),
            tempo: Some(120),
            title: None,
            composer: None,
            write_dynamics: true,
            min_neg_power: None
        }
    }
}

/// Renders musical compositions to LilyPond format
pub struct LilyPondRenderer {
    config: LilyPondConfig,
}

impl LilyPondRenderer {
    pub fn new(config: LilyPondConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(LilyPondConfig::default())
    }

    /// Render a composition to a LilyPond file string
    pub fn render(&self, composition: &Composition) -> String {
        let mut output = String::new();

        // Version header
        writeln!(output, "\\version \"{}\"", self.config.version).unwrap();
        writeln!(output).unwrap();

        // Header block with metadata
        if self.config.title.is_some() || self.config.composer.is_some() {
            writeln!(output, "\\header {{").unwrap();
            if let Some(ref title) = self.config.title {
                writeln!(output, "  title = \"{}\"", title).unwrap();
            }
            if let Some(ref composer) = self.config.composer {
                writeln!(output, "  composer = \"{}\"", composer).unwrap();
            }
            writeln!(output, "}}").unwrap();
            writeln!(output).unwrap();
        }

        // Generate score
        writeln!(output, "\\score {{").unwrap();
        writeln!(output, "  <<").unwrap();

        // Render each track as a separate staff
        for track in &composition.tracks {
            self.render_track(&mut output, track, composition.time_signature);
        }

        writeln!(output, "  >>").unwrap();

        // Layout block
        writeln!(output, "  \\layout {{ }}").unwrap();

        // MIDI block
        writeln!(output, "  \\midi {{").unwrap();
        if let Some(tempo) = self.config.tempo {
            writeln!(output, "    \\tempo 4 = {}", tempo).unwrap();
        }
        writeln!(output, "  }}").unwrap();

        writeln!(output, "}}").unwrap();

        output
    }

    fn render_track(&self, output: &mut String, track: &Track, time_signature: TimeSignature) {
        writeln!(output, "    \\new Staff {{").unwrap();
        writeln!(
            output,
            "      \\set Staff.instrumentName = \"{}\"",
            format!("{:?}", track.instrument)
        )
        .unwrap();

        // Time signature
        let TimeSignature(num, denom) = time_signature;
        writeln!(output, "      \\time {}/{}", num, denom).unwrap();

        // Start absolute mode
        writeln!(output, "      \\absolute {{").unwrap();

        // Render events
        let music = self.render_track_music(track, time_signature);
        writeln!(output, "        {}", music).unwrap();

        writeln!(output, "      }}").unwrap();
        writeln!(output, "      \\bar \"|.\"").unwrap();
        writeln!(output, "    }}").unwrap();
    }

    fn render_track_music(&self, track: &Track, time_signature: TimeSignature) -> String {
        let mut output = String::new();

        if !track.validate_contiguous() {
            panic!("Track must be contiguous to be written! Found gaps or overlaps.");
        }

        // Combine events and rests, sort by start time
        let mut all_events: Vec<(Duration, bool, Event)> = Vec::new();

        for event in &track.events {
            all_events.push((event.start, false, *event));
        }
        for rest in &track.rests {
            all_events.push((rest.start, true, *rest));
        }

        all_events.sort_by_key(|(start, _, _)| *start);

        // Track current position to insert rests for gaps
        let mut current_pos = Duration::zero(time_signature);

        for (start, is_rest, event) in all_events {
            // Insert rest for any gap
            if start > current_pos {
                let gap_duration = start - current_pos;
                write!(output, "{} ", self.render_rest_with_measures(gap_duration, current_pos, time_signature)).unwrap();
                current_pos = start;
            }

            // Render the event
            if is_rest {
                write!(output, "{} ", self.render_rest_with_measures(event.duration, current_pos, time_signature)).unwrap();
            } else {
                write!(output, "{} ", self.render_event_with_measures(&event, current_pos, time_signature)).unwrap();
            }

            current_pos = start + event.duration;
        }

        output.trim().to_string()
    }

    fn render_event(&self, event: &Event) -> String {
        if !event.duration.binary_expandable() {
            panic!("Event duration {:?} is not binary expandable for LilyPond rendering!", event.duration);
        }
        // do this for each expanded duration and tie them together with ~
        let mut result = String::new();
        let expanded_durations = event.duration.binary_expand(self.config.min_neg_power);
        // tie them all together
        for (i, dur) in expanded_durations.into_iter().enumerate() {
            if i > 0 {
                result.push_str("~");
            }
            write!(
                result,
                "{}{}{}",
                self.pitch_to_lilypond(event.pitch),
                self.duration_to_lilypond(dur),
                if self.config.write_dynamics && i == 0 {
                    self.volume_to_lilypond_dynamics(event.volume)
                } else {
                    "".to_string()
                }
            )
            .unwrap();
        }
        result.trim().to_string()
    }

    fn render_rest(&self, duration: Duration) -> String {
        // render multiple rests (no need to tie them together) based on binary expansion
        if !duration.binary_expandable() {
            panic!("Rest duration {:?} is not binary expandable for LilyPond rendering!", duration);
        }
        let expanded_durations = duration.binary_expand(self.config.min_neg_power);
        let mut result = String::new();
        for dur in expanded_durations {
            write!(result, "r{} ", self.duration_to_lilypond(dur)).unwrap();
        }
        result.trim().to_string()
    }

    /// Render an event, splitting it at measure boundaries if necessary
    fn render_event_with_measures(&self, event: &Event, start_pos: Duration, time_signature: TimeSignature) -> String {
        use music_primitives::{Beats, MusicNat, NoteValue};
        use num::rational::Ratio;

        // Calculate measure length in beats
        let measure_length_beats = Ratio::<MusicNat>::from_integer(time_signature.0 as MusicNat);

        // Convert start_pos note value to beats for this time signature
        // In time signature n/d, 1 beat = 1/d note value
        // So note_value / (1/d) = note_value * d = beats
        let start_beats = start_pos.value.0 * Ratio::from_integer(time_signature.1 as MusicNat);
        let event_beats = event.duration.value.0 * Ratio::from_integer(time_signature.1 as MusicNat);

        // Find position within current measure (in beats)
        let beats_into_measure = start_beats % measure_length_beats;
        let beats_until_next_measure = measure_length_beats - beats_into_measure;

        // If this event fits entirely in the current measure, render normally
        if event_beats <= beats_until_next_measure {
            return self.render_event(event);
        }

        // Otherwise, split at measure boundary
        let duration_before_boundary = Duration::from_beats_with_ts(beats_until_next_measure, time_signature);
        let duration_after_boundary = event.duration - duration_before_boundary;

        // Create first part
        let first_event = Event {
            start: event.start,
            duration: duration_before_boundary,
            volume: event.volume,
            pitch: event.pitch,
        };

        // Create second part (recursively handle if it also crosses boundaries)
        let second_event = Event {
            start: event.start + duration_before_boundary,
            duration: duration_after_boundary,
            volume: event.volume,
            pitch: event.pitch,
        };

        // Render both parts with tie
        let first_part = self.render_event(&first_event);
        let second_part = self.render_event_with_measures(&second_event, start_pos + duration_before_boundary, time_signature);

        format!("{}~{}", first_part, second_part)
    }

    /// Render a rest, splitting it at measure boundaries if necessary (no ties)
    fn render_rest_with_measures(&self, duration: Duration, start_pos: Duration, time_signature: TimeSignature) -> String {
        use music_primitives::MusicNat;
        use num::rational::Ratio;

        // Calculate measure length in beats
        let measure_length_beats = Ratio::<MusicNat>::from_integer(time_signature.0 as MusicNat);

        // Convert durations from note values to beats for this time signature
        // In time signature n/d, 1 beat = 1/d note value
        // So note_value / (1/d) = note_value * d = beats
        let start_beats = start_pos.value.0 * Ratio::from_integer(time_signature.1 as MusicNat);
        let duration_beats = duration.value.0 * Ratio::from_integer(time_signature.1 as MusicNat);

        // Find position within current measure (in beats)
        let beats_into_measure = start_beats % measure_length_beats;
        let beats_until_next_measure = measure_length_beats - beats_into_measure;

        // If this rest fits entirely in the current measure, render normally
        if duration_beats <= beats_until_next_measure {
            return self.render_rest(duration);
        }

        // Otherwise, split at measure boundary
        let duration_before_boundary = Duration::from_beats_with_ts(beats_until_next_measure, time_signature);
        let duration_after_boundary = duration - duration_before_boundary;

        // Render both parts WITHOUT tie (rests don't tie)
        let first_part = self.render_rest(duration_before_boundary);
        let second_part = self.render_rest_with_measures(duration_after_boundary, start_pos + duration_before_boundary, time_signature);

        format!("{} {}", first_part, second_part)
    }

    /// Convert a Pitch to LilyPond notation
    /// Note: C3 in our system maps to c in LilyPond (no octave marks)
    /// C4 (middle C) maps to c'
    fn pitch_to_lilypond(&self, pitch: Pitch) -> String {
        let (octave, note_num) = pitch.data();

        // Convert note number to letter name
        let (letter, accidental) = match note_num % 12 {
            0 => ("c", ""),
            1 => ("c", "is"), // C# (cis in LilyPond)
            2 => ("d", ""),
            3 => ("d", "is"), // D# but we'll use es for Eb
            4 => ("e", ""),
            5 => ("f", ""),
            6 => ("f", "is"), // F#
            7 => ("g", ""),
            8 => ("g", "is"), // G# but we'll use as for Ab
            9 => ("a", ""),
            10 => ("b", "es"), // Bb (bes in LilyPond)
            11 => ("b", ""),
            _ => unreachable!(),
        };

        // For flats, use a more natural representation
        let (letter, accidental) = match note_num % 12 {
            1 => ("c", "is"),  // C#
            3 => ("e", "es"),  // Eb (more common than D#)
            6 => ("f", "is"),  // F#
            8 => ("a", "es"),  // Ab (more common than G#)
            10 => ("b", "es"), // Bb
            _ => (letter, accidental),
        };

        // Calculate octave marks
        // In LilyPond: c (no mark) = C3, c' = C4, c'' = C5, c, = C2, c,, = C1
        // Our octave 3 = c, octave 4 = c', octave 5 = c''
        let octave_mark = if octave >= 3 {
            "'".repeat((octave - 3) as usize)
        } else {
            ",".repeat((3 - octave) as usize)
        };

        format!("{}{}{}", letter, accidental, octave_mark)
    }

    /// Convert a Duration to LilyPond duration notation
    /// LilyPond uses: 1 (whole), 2 (half), 4 (quarter), 8 (eighth), 16 (sixteenth), etc.
    fn duration_to_lilypond(&self, duration: Duration) -> String {
        // Get the note value as a ratio
        let note_value = duration.value.0;

        // For LilyPond, we need the reciprocal (if note_value is 1/4, LilyPond uses "4")
        // Handle whole notes (value = 1) and fractional notes

        if *note_value.numer() == 0 {
            // Zero duration - use a very short note
            return "64".to_string();
        }

        // If numerator is 1, it's a simple note (1/4 -> 4, 1/8 -> 8)
        if *note_value.numer() == 1 {
            return format!("{}", note_value.denom());
        }

        // If denominator is 1, it's a whole note or multiple whole notes
        if *note_value.denom() == 1 {
            let num = *note_value.numer();
            if num == 1 {
                return "1".to_string();
            }
            // Multiple whole notes - use tied notes
            return format!("1~{}", "1~".repeat(num as usize - 1).trim_end_matches('~'));
        }

        // For dotted notes (like 3/8 = 1/4.), check if it's (2^n - 1) / 2^n pattern
        let numer = *note_value.numer();
        let denom = *note_value.denom();

        // Check for dotted note pattern: value * 1.5 = base_note
        // 3/8 = (1/4) * 1.5, so 3/8 = dotted quarter in 4/4 time
        // Actually, let me reconsider: in terms of note values, 3/8 beats is not standard

        // For now, approximate with the closest standard duration
        // Calculate the closest power-of-2 denominator
        let total_float = note_value.to_f32().unwrap_or(0.25);
        let duration_num = (1.0f32 / total_float).round() as u32;

        // Clamp to valid LilyPond durations
        let duration_num = duration_num.max(1).min(128);

        format!("{}", duration_num)
    }

    /// Convert volume to LilyPond dynamics markings
    fn volume_to_lilypond_dynamics(&self, volume: Volume) -> String {
        let vol = volume.0;

        // Map volume to standard dynamics markings
        let dynamic = if vol >= 90 {
            "\\ff" // fortissimo
        } else if vol >= 75 {
            "\\f" // forte
        } else if vol >= 60 {
            "\\mf" // mezzo-forte
        } else if vol >= 45 {
            "\\mp" // mezzo-piano
        } else if vol >= 30 {
            "\\p" // piano
        } else {
            "\\pp" // pianissimo
        };

        format!("{}", dynamic)
    }
}

pub fn render_to_lilypond(
    composition: Composition,
    output_path: &str,
    config: Option<LilyPondConfig>,
) -> Result<(), RenderError> {
    let config = config.unwrap_or(LilyPondConfig::default());

    let renderer = LilyPondRenderer::new(config);

    let lilypond_output = renderer.render(&composition);
    let (output_folder, output_filename) = if Path::new(output_path).is_dir() {
        (output_path.to_string(), "output.ly".to_string())
    } else {
        let path = Path::new(output_path);
        let folder = path
            .parent()
            .unwrap_or(Path::new("."))
            .to_str()
            .unwrap()
            .to_string();
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        (folder, filename)
    };
    std::fs::create_dir_all(output_folder.as_str())?;
    std::fs::write(
        Path::new(&output_folder).join(output_filename),
        lilypond_output,
    )
    .expect("Failed to write LilyPond output");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfg::{Grammar, MusicPrimitive, MusicString, Symbol};
    use crate::composition::{Instrument, Track, TrackId};
    use music_primitives::Beats;
    use std::fs;

    #[test]
    fn test_pitch_to_lilypond() {
        let renderer = LilyPondRenderer::with_defaults();

        // Middle C (C4)
        assert_eq!(renderer.pitch_to_lilypond(Pitch::new(4, 0)), "c'");

        // C3
        assert_eq!(renderer.pitch_to_lilypond(Pitch::new(3, 0)), "c");

        // C5
        assert_eq!(renderer.pitch_to_lilypond(Pitch::new(5, 0)), "c''");

        // C2
        assert_eq!(renderer.pitch_to_lilypond(Pitch::new(2, 0)), "c,");

        // A4 (A above middle C)
        assert_eq!(renderer.pitch_to_lilypond(Pitch::new(4, 9)), "a'");

        // F#4
        assert_eq!(renderer.pitch_to_lilypond(Pitch::new(4, 6)), "fis'");

        // Bb3
        assert_eq!(renderer.pitch_to_lilypond(Pitch::new(3, 10)), "bes");
    }

    #[test]
    fn test_duration_to_lilypond() {
        let renderer = LilyPondRenderer::with_defaults();
        let ts = TimeSignature::common();

        // Quarter note
        let quarter = Duration::from_beats_with_ts(Beats::from_integer(1), ts);
        assert_eq!(renderer.duration_to_lilypond(quarter), "4");

        // Half note
        let half = Duration::from_beats_with_ts(Beats::from_integer(2), ts);
        assert_eq!(renderer.duration_to_lilypond(half), "2");

        // Whole note
        let whole = Duration::from_beats_with_ts(Beats::from_integer(4), ts);
        assert_eq!(renderer.duration_to_lilypond(whole), "1");
    }

    #[test]
    fn test_render_simple_composition() {
        let ts = TimeSignature::common();
        let renderer = LilyPondRenderer::with_defaults();

        let composition = Composition {
            time_signature: ts,
            tracks: vec![Track {
                identifier: TrackId::Instrument(Instrument::Piano),
                instrument: Instrument::Piano,
                events: vec![
                    Event {
                        start: Duration::from_beats_with_ts(Beats::from_integer(0), ts),
                        duration: Duration::from_beats_with_ts(Beats::from_integer(1), ts),
                        volume: Volume(80),
                        pitch: Pitch::new(4, 0), // C4
                    },
                    Event {
                        start: Duration::from_beats_with_ts(Beats::from_integer(1), ts),
                        duration: Duration::from_beats_with_ts(Beats::from_integer(1), ts),
                        volume: Volume(80),
                        pitch: Pitch::new(4, 2), // D4
                    },
                ],
                rests: vec![],
            }],
        };

        let output = renderer.render(&composition);

        // Check that output contains expected elements
        assert!(output.contains("\\version"));
        assert!(output.contains("\\score"));
        assert!(output.contains("\\time 4/4"));
        assert!(output.contains("c'"));
        assert!(output.contains("d'"));
    }

    #[test]
    fn test_parse_all_grammar_examples() {
        let examples_dir = "data/grammar/examples";

        // Read all files in the examples directory
        let entries = fs::read_dir(examples_dir).expect("Failed to read examples directory");

        let mut file_count = 0;
        let mut rng = rand::rng();
        for entry in entries {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();

            // Only process .mt files
            if path.extension().and_then(|s| s.to_str()) == Some("mt") {
                file_count += 1;
                let file_name = path.file_name().unwrap().to_str().unwrap();

                // Read the file contents
                let contents = fs::read_to_string(&path)
                    .expect(&format!("Failed to read file: {}", file_name));

                // Parse the grammar
                let result = contents.parse::<Grammar>();

                // Assert that parsing succeeds
                assert!(
                    result.is_ok(),
                    "Failed to parse {}: {:?}",
                    file_name,
                    result.err()
                );

                let grammar = result.unwrap();
                // now try to do at least 1 production
                let axiom = MusicString(vec![MusicPrimitive::Simple(Symbol::NT(
                    grammar.start.clone(),
                ))]);

                axiom.parallel_rewrite(&grammar, &mut rng, true);
            }
        }

        // Ensure we actually tested some files
        assert!(file_count > 0, "No .mt files found in examples directory");
    }
}

#[cfg(test)]
mod render_fun {
    use crate::cfg::{Grammar, MusicPrimitive, MusicString, GrammarDerivationConfig, Performer};
    use crate::compose_from_grammar;
    use crate::lilypond::{LilyPondConfig, call_lilypond_cli, render_to_lilypond};
    use music_primitives::TimeSignature;

    #[test]
    fn render_test_1() {
        let mut rng = rand::rng();
        let filename = "26_repeat_split"; // "02_repeat_pattern";
        let composition = compose_from_grammar(
            format!("data/grammar/examples/{}.mt", filename).as_str(),
            GrammarDerivationConfig {
                iterations: 3,
                panic_on_bad_production: true,
                rounded: true,
                max_depth: 10,
            },
            &mut rng,
        )
        .unwrap();
        let lilypond_filename = format!("data/lilypond/examples/{}.ly", filename);
        render_to_lilypond(
            composition,
            lilypond_filename.as_str(),
            Some(LilyPondConfig {
                write_dynamics: false,
                ..LilyPondConfig::default()
            }),
        )
        .unwrap();

        call_lilypond_cli(lilypond_filename.as_str(), "data/lilypond/output", true).unwrap();
    }
}
