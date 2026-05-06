# music_turtle_lang

A domain-specific language (DSL) and runtime for grammar-based music composition. Grammars describe musical patterns as rewriting rules; the runtime expands them into `Composition` objects, which can then be rendered to LilyPond sheet music or MIDI.

## Overview

The name is inspired by *turtle graphics* — a performer "turtle" follows a sequence of musical instructions (move pitch up/down, play current note, rest, change instrument …) produced by expanding a grammar.

### Pipeline

```
.mt file  ──(scan)──►  Grammar  ──(produce)──►  MusicString  ──(compose)──►  Composition  ──(render)──►  .ly / MIDI
```

## Modules

### `scan` — Grammar Parser

Parses `.mt` text files into `Grammar` values.

**Syntax reference** (abbreviated):

```
start <NonTerminal>
<NonTerminal> = <MusicString>
...
```

A `MusicString` is a sequence of `MusicPrimitive`s:

| Syntax | Meaning |
|---|---|
| `NonTerminal` | Reference to another rule |
| `:4c<1/4>` | Absolute note: C4, quarter note |
| `:<NoteValue>` | Current note for given duration |
| `.<NoteValue>` | Play current pitch for duration |
| `+` / `-` | Move performer pitch up/down 1 semitone |
| `*<NoteValue>` | Rest for duration |
| `{ A \| B \| C }` | Split — play branches simultaneously |
| `[x3][…]` | Repeat content 3 times |
| `[T2][…]` | Transpose up 2 semitones |
| `[>>2][…]` | Time compression ×2 (twice as fast) |
| `::i=piano` | Meta: change instrument |
| `::v=80` | Meta: change volume |

**Example — C major scale:**
```
start Scale
Scale = :4c<1/4> :4d<1/4> :4e<1/4> :4f<1/4> :4g<1/4> :4a<1/4> :4b<1/4> :5c<1/4>
```

**Example — Transpose and repeat:**
```
start TransposedMelody
Pattern    = :4c<1/4> :4d<1/4> :4e<1/4> :4f<1/4>
TransposedMelody = Pattern [T2][Pattern]
```

**Example — Split (harmony):**
```
start Harmony
Harmony = { :4c<1> :4e<1> :4g<1> | :3g<1> :4c<1> :4e<1> }
```

Parsing is exposed via `Grammar`'s `FromStr` impl or the `grammar_from_file` helper.

---

### `cfg` — Grammar & Derivation

Defines the core grammar types and the rewriting engine.

**`Grammar`** — A list of `Production` rules plus a start symbol and time signature.

**`MusicPrimitive`** — One element in a music string: a symbol, a split, a transform wrapper, etc.

**`MusicString`** — A flat sequence of `MusicPrimitive`s. The method `compose_v2(time_signature, performer)` walks the string and produces a `Composition`.

**`GrammarDerivation`** — A *tree* that records how a grammar was expanded (branch, wrapped subtree, split, NT leaf, terminal leaf). Required by the genetic algorithm to enable subtree crossover.

**`Performer`** — The mutable state carried during composition: current instrument, volume, and pitch.

**`GrammarDerivationConfig`** — Parameters for grammar expansion:
| Field | Meaning |
|---|---|
| `iterations` | Number of parallel-rewrite steps |
| `panic_on_bad_production` | Panic vs. silently skip undefined NTs |
| `rounded` | Pad final measure with rests to a whole measure |
| `max_depth` | Cap tree depth (prevents infinite recursion) |

**`GrammarDerivationGenerator`** — Combines a grammar with a config and provides `produce(rng)` / `re_expand_random_nt(…)` / `prune(…)` for use by the genetic algorithm.

---

### `composition` — Composition Model

Holds the in-memory representation of a piece of music after grammar expansion.

**`Composition`** — A set of `Track`s plus a `TimeSignature`. Key methods:
- `transpose(semitones)` — shift all events by a fixed interval.
- `add_rests_to_last_measure()` — pad the last measure to a whole measure.
- `validate_contiguous()` — check that a track has no timing gaps.

**`Track`** — One instrument's events (`Vec<Event>`) plus explicit rests (`Vec<Event>`).

**`Event`** — A single note: `start` duration, `duration`, `pitch`, `volume`.

**`Volume`** — A `u8` MIDI velocity (0–127). Constant `MAX_VOLUME = 127`.

**`Instrument`** — Enum of MIDI-like instruments (Piano, Sine, …).

**`TimeCompression`** — A rational factor used by the `>>` transform to speed up or slow down a passage.

---

### `lilypond` — Sheet Music / MIDI Rendering

Converts a `Composition` to a LilyPond `.ly` file and optionally invokes the `lilypond` CLI to compile it.

**`LilyPondConfig`** — Options for the renderer:
| Field | Default | Meaning |
|---|---|---|
| `version` | `"2.24.0"` | LilyPond version string |
| `tempo` | `Some(120)` | BPM |
| `title` | `None` | Piece title |
| `composer` | `None` | Composer name |
| `write_dynamics` | `true` | Emit `\pp` / `\ff` etc. |
| `min_neg_power` | `None` | Smallest note value to render (e.g. `-3` → 1/8) |
| `piano_staff` | `false` | Render as grand staff (treble + bass) |

**`render_to_lilypond(composition, path, config)`** — Top-level convenience function: writes the `.ly` file to disk.

**`call_lilypond_cli(ly_file, output_dir, print_output)`** — Shells out to the `lilypond` binary, producing PDF and MIDI.

---

### `lib` — Public Entry Points

```rust
// Parse a .mt file and expand it into a Composition in one step
pub fn compose_from_grammar(
    grammar_filename: &str,
    config: GrammarDerivationConfig,
    rng: &mut impl Rng,
) -> Result<Composition, RenderError>;

// Parse a .mt file into a Grammar (for inspection or use with genetic algorithms)
pub fn grammar_from_file(grammar_filename: &str) -> Result<Grammar, RenderError>;
```

`RenderError` wraps `IoError`, `ScanError`, and `ComposeError`.

## Dependencies

- [`music_primitives`](../music_primitives) — pitch, duration, time signature types
- [`num`](https://crates.io/crates/num) — rational arithmetic
- [`rand`](https://crates.io/crates/rand) — random grammar expansion
- [`serde`](https://crates.io/crates/serde) — serialization of grammar/derivation structures
- [`enumkit`](https://crates.io/crates/enumkit) — enum utilities
