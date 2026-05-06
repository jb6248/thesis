# Thesis — Grammar-Based Evolutionary Music Composition

This workspace explores **algorithmic composition** using a custom grammar DSL and a genetic algorithm that evolves musically interesting chord progressions. The theoretical backbone is Lerdahl's tonal pitch-space model, which provides a distance metric for measuring harmonic smoothness and range.

## Repository Structure

```
Cargo.toml            — workspace manifest
crates/
├── music_primitives/ — core music-theory types
├── lang/             — music grammar DSL, composition engine, LilyPond renderer
└── main/             — experiment runner, genetic algorithm, distance metrics
```

## Crates

### [`music_primitives`](crates/music_primitives/README.md)
Foundational data types for music theory: `Pitch`, `PitchClass`, `NoteValue`, `Duration`, and `TimeSignature`. All durations are represented as exact rational numbers to ensure lossless arithmetic throughout the pipeline.

### [`music_turtle_lang`](crates/lang/README.md)
A domain-specific language (DSL) for describing music as rewriting grammars. A "turtle" performer follows instructions produced by grammar expansion — playing notes, moving pitch up or down, resting, changing instrument.

Pipeline:
```
.mt grammar file ──► Grammar ──► MusicString ──► Composition ──► LilyPond (.ly + MIDI)
```

Key features:
- Text-based grammar syntax (`.mt` files) with non-terminals, absolute notes, splits (parallel voices), and transforms (repeat, transpose, time-compression).
- `compose_v2` walks a `MusicString` to build a multi-track `Composition`.
- LilyPond renderer outputs PDF sheet music and MIDI; optionally produces a grand staff.

### [`thesis` (main)](crates/main/README.md)
The experiment runner and genetic algorithm. Genomes are grammar derivation trees; crossover swaps matching-NT subtrees; fitness blends two harmonic objectives:

- **Smooth voice leading** — minimise total adjacent chord distance.
- **Harmonic range** — maximise distance from the opening chord.

Distance is measured using Lerdahl's five-level tonal pitch space (`Octave > Fifth > Triadic > Diatonic > Chromatic`). The current experiment (`template_7`) runs 500 generations of 100 individuals and renders the best results to LilyPond and MP3.

## Quick Start

### Prerequisites
- Rust (edition 2024, stable toolchain)
- [`lilypond`](https://lilypond.org/) — for PDF/MIDI output
- [`fluidsynth`](https://www.fluidsynth.org/) + a `.sf2` SoundFont — for MP3 output (optional)

### Build & Run

```sh
cargo build --release
cargo run --release
```

Output is written to `crates/main/data/experiments/<N>/`.

### Run Tests

```sh
cargo test
```

## Conceptual Overview

The system answers the question: *can a genetic algorithm, guided by a formal harmonic distance metric, evolve chord progressions that are both harmonically coherent and interesting?*

1. **Grammar** — A `.mt` file defines the space of possible chord progressions. Non-terminals prefixed with `#` represent chords (e.g. `#I/V`, `#ii/i`).
2. **Genome** — Each individual in the population is a fully-expanded grammar derivation tree.
3. **Fitness** — The chord sequence is extracted from the tree and scored using Lerdahl pitch-space distances.
4. **Operators** — Crossover swaps grammatically-compatible subtrees; mutation re-expands a random non-terminal.
5. **Output** — The fittest individuals are rendered to LilyPond sheet music and (optionally) synthesised to audio.
