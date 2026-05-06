# thesis (main crate)

The top-level experiment runner for the thesis project. It wires together the grammar DSL, a genetic algorithm engine, and music-theory distance metrics to evolve chord progressions that satisfy harmonic objectives.

## Overview

The binary (`cargo run`) currently runs **experiment 8** (`template_7`) — a genetic algorithm that evolves music by evaluating chord progressions against Lerdahl-inspired pitch-space distance metrics, then renders the best results to LilyPond sheet music and (optionally) MIDI/MP3 samples.

## Module Map

```
src/
├── main.rs                        — entry point; selects which template to run
├── util.rs                        — chord-grammar generator helpers, MIDI → MP3 batch renderer
├── template_6.rs                  — experiment template 6 (prolongational-tree fitness)
├── template_7.rs                  — experiment template 7 (current default; adds audio rendering)
├── early_experiments.rs           — archived earlier experiment runners
│
├── genetic_simulation/
│   ├── mod.rs                     — generic Genome trait + Simulation<G> engine
│   ├── grammar_derivation_genome.rs  — Genome v1
│   ├── grammar_derivation_genome2.rs — Genome v2 (smooth chord-distance fitness)
│   ├── grammar_derivation_genome3.rs — Genome v3
│   ├── grammar_derivation_genome4.rs — Genome v4 (hierarchical analysis)
│   ├── grammar_derivation_genome5.rs — Genome v5 (current)
│   └── analysis/
│       ├── mod.rs                 — chord-structure extraction from derivation trees
│       ├── lerdahl.rs             — interchordal distance metrics (Lerdahl pitch-space)
│       └── prolongational_tree.rs — prolongational tree analysis (WIP)
│
└── distance/
    ├── mod.rs
    ├── pitch_class_space.rs       — PitchClassSpace + SpaceLevel hierarchy
    └── chord_enumeration.rs       — generates all diatonic chord labels
```

## Key Concepts

### Genetic Algorithm (`genetic_simulation`)

The core loop is in `Simulation<G>` where `G: Genome`:

```rust
pub trait Genome {
    type Config;
    fn generate(config: &Self::Config, rng: &mut impl Rng) -> Self;
    fn mutate(&mut self, config: &Self::Config, rng: &mut impl Rng);
    fn crossover(&self, other: &Self, config: &Self::Config, rng: &mut impl Rng) -> Self;
    fn fitness(&self, config: &Self::Config) -> f64;
}
```

`Simulation::step()` performs fitness-proportionate selection, optional crossover (subtree swap on `GrammarDerivation`), and mutation (re-expand a random non-terminal).

Each genome is a `GrammarDerivation` — a derivation tree produced by expanding a `.mt` grammar file. Crossover swaps matching-NT subtrees between two derivations.

### Fitness Function

The fitness function penalises or rewards chord progressions based on two objectives:

- **Smooth voice leading** (`get_total_interchordal_distances`) — minimise the sum of adjacent chord distances to create smooth harmonic motion.
- **Harmonic range** (`get_maximum_distance`) — maximise the maximum distance from the opening chord to encourage harmonic journey/tension.

A `distance_bias` parameter (`0.0`–`1.0`) blends between these two goals.

### Pitch-Class Space (`distance::pitch_class_space`)

`PitchClassSpace` encodes a chord as an array of 12 `SpaceLevel` values (one per pitch class). `SpaceLevel` has five levels of harmonic proximity (Lerdahl's tonal pitch space model):

| Level | Name | Description |
|---|---|---|
| A | Octave | Octave equivalence |
| B | Fifth | Perfect-fifth stability |
| C | Triadic | Chord-tone membership |
| D | Diatonic | Scale membership |
| E | Chromatic | Chromatic note (least stable) |

`PitchClassSpace::total_distance(other)` computes the sum of level differences for all 12 pitch classes.

### Chord Grammar Utilities (`util`)

`generate_chord_nts(tone_center, prefix, duration)` — generates grammar rules for all 98 diatonic chords (7 chords × 7 diatonic regions × major + minor) in a given key, for use as non-terminals in `.mt` grammars.

`render_midi_to_samples(experiment_location)` — finds `.midi` files in the LilyPond output directory and batch-renders them to MP3 using `fluidsynth`.

### Experiment Templates

| Template | Function | Description |
|---|---|---|
| `template_6` | `template_6(trials, distance_bias, path)` | Runs GA with `GrammarDerivationGenome4`, logs metrics every 20 generations, saves best genomes as LilyPond files |
| `template_7` | `template_7(trials, distance_bias, path)` | Extends template 6; auto-locates a soundfont and renders audio samples |

Each template reads its grammar from `<experiment_location>/grammar.mt`.

## Running

```sh
cargo run --release
```

Output (LilyPond files, MIDI, optionally MP3) is written under `data/experiments/<N>/`.

## External Tools

| Tool | Purpose |
|---|---|
| `lilypond` | Compiles `.ly` files to PDF and MIDI |
| `fluidsynth` | Synthesises MIDI to MP3 using a SoundFont (`.sf2`) |

Both must be installed separately and available on `$PATH`.

## Dependencies

- [`music_primitives`](../music_primitives) — pitch and duration types
- [`music_turtle_lang`](../lang) — grammar DSL and composition engine
- [`rayon`](https://crates.io/crates/rayon) — parallel fitness evaluation
- [`rand`](https://crates.io/crates/rand) — RNG for genetic operations
- [`num`](https://crates.io/crates/num) — rational arithmetic
- [`lazy_static`](https://crates.io/crates/lazy_static) — static pitch-space tables
- [`enumkit`](https://crates.io/crates/enumkit) — enum mapping utilities
- [`display_tree`](https://crates.io/crates/display_tree) — pretty-printing derivation trees
