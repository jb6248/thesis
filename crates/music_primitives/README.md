# music_primitives

A foundational library for representing music theory concepts in Rust. This crate defines the core data types shared across the workspace — pitches, note values, durations, and time signatures — and provides the arithmetic needed to work with them precisely.

## Overview

All durations and note values are stored as exact rational numbers (`num::Ratio<u32>`) rather than floating-point, ensuring lossless arithmetic throughout the pipeline.

## Key Types

### `Pitch`
A concrete pitch combining an octave (`i8`) and a `PitchClassNum` (0–11, where C = 0). Helper constructors include `Pitch::middle_c()` and `Pitch::none()` (a sentinel for rests).

```
Pitch::new(4, PitchClass::C)  // C4 (middle C)
PitchClass::G.at(3)           // G3
```

### `PitchClass`
An enum of the 12 pitch classes in 12-TET (C, C♯/D♭, D, …, B). Provides `to_note_num()`, `sharp()`, `flat()`, and `at(octave)`.

### `NoteValue`
A rational number (`Ratio<u32>`) representing the *written* duration of a note as a fraction of a whole note. `1/4` is a quarter note, `1/8` is an eighth note, and so on.

Key methods:
- `binary_expand(min_power)` — decomposes a `NoteValue` into a sum of standard power-of-two note values, useful for rendering.
- `binary_expandable()` — returns `true` if the denominator is a power of two.

### `Duration`
A `NoteValue` paired with a `TimeSignature`, providing time-aware arithmetic. A `Duration` knows how many beats or whole measures it spans.

Key methods:
- `from_beats_with_ts(beats, ts)` — construct from a beat count.
- `measures_and_beats_with_ts(measures, beats, ts)` — construct from measures + remaining beats.
- `get_beats()` / `get_whole_measures()` / `get_rem_beats()` — decompose.
- `to_seconds(bpm)` — convert to wall-clock time.
- `binary_expand(min_power)` — decompose into standard note values.

Durations support `+`, `-`, `*` and implement `Ord`.

### `TimeSignature`
A `(numerator, denominator)` pair (e.g. `TimeSignature(4, 4)` for common time).

Convenience constructors: `TimeSignature::common()` (4/4), `TimeSignature::waltz()` (3/4).

### Type Aliases
| Alias | Underlying type | Meaning |
|---|---|---|
| `MusicNat` | `u32` | Non-negative integer used throughout |
| `Beats` | `Ratio<u32>` | Beat count |
| `Measures` | `u32` | Whole-measure count |
| `Octave` | `i8` | Octave number |
| `PitchClassNum` | `u8` | Pitch-class integer in `[0, 12)` |
| `RestValue` | `NoteValue` | Duration of a rest |

## Serialization
`Pitch`, `Duration`, `NoteValue`, and `TimeSignature` all implement `serde::Serialize` / `Deserialize`. `NoteValue` is serialized as `{ numerator, denominator }` to preserve exactness.

## Dependencies
- [`num`](https://crates.io/crates/num) — rational arithmetic
- [`serde`](https://crates.io/crates/serde) — serialization
