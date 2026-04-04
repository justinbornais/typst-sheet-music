# Scorify

Render professional sheet music directly inside Typst documents using SMuFL-aware glyph placement and CeTZ drawing primitives.

## Features

- **Pure Typst** - no WASM plugin, no external binary dependency (no LilyPond, no MuseScore CLI)
- SMuFL/Bravura-aware glyph placement with precise bounding-box anchors
- Notes, rests, chords, accidentals, key signatures, time signatures, clefs
- Dynamics, articulations, fingerings, and chord symbols - all inline in the music string
- Beams, ties, slurs, repeat barlines, dotted notes
- Grand staff and multi-staff layout with vertical beat alignment
- System/line breaks via measures-per-line, literal `\n`, or automatic width-based breaking
- Header block with title, subtitle, composer, arranger, lyricist
- Produces crisp, resolution-independent vector PDF output

## Quick Start

### Via Typst Package Manager (recommended)

Add the import to your document and start writing music:

```typ
#import "@preview/scorify:0.1.1": score, melody

#melody(
  title: "Scale",
  key: "C",
  time: "4/4",
  music: "c4 d e f | g a b c'",
)
```

Then compile (point `--font-path` at the directory containing `Bravura.otf` - see [Font Setup](#font-setup)):

```
typst compile your-file.typ --font-path /path/to/bravura/
```

### Manual Installation

For local development, copy `lib.typ`, `src/`, and `data/` into your project and import with a relative path:

```typ
#import "lib.typ": score, melody
```

Compile with:

```
typst compile your-file.typ --font-path /path/to/bravura/ --root .
```

## Font Setup

scorify uses the [Bravura](https://github.com/steinbergmedia/bravura) SMuFL font for music glyph rendering. Typst packages cannot embed fonts, so you must install Bravura separately before compiling any document that uses this package.

1. Download the latest Bravura release from the [Bravura GitHub releases page](https://github.com/steinbergmedia/bravura/releases). The `.zip` archive contains `Bravura.otf`.
2. Either:
    - **System install (easiest):** Copy `Bravura.otf` into your system fonts folder. Typst will find it automatically and no extra flag is needed.
    - **Project-local:** Keep `Bravura.otf` in a folder of your choice and pass `--font-path /path/to/that/folder/` when compiling.

## API Reference

### `score()`

The primary entry point. Renders one or more staves with full layout control.

```typ
#score(
  staves: (
    (clef: "treble", music: "c4 d e f | g a b c'"),
    (clef: "bass", music: "c2 g | c1"),
  ),
  key: "C",
  time: "4/4",
  title: "My Piece",
  composer: "Composer Name",
  staff-group: "grand",
)
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `staves` | array | `()` | Array of staff dicts (see below) |
| `key` | string | `none` | Key signature (`"C"`, `"G"`, `"D"`, `"Bb"`, `"f#"`, etc.) |
| `time` | string | `none` | Time signature (`"4/4"`, `"3/4"`, `"6/8"`, `"common"`/`"C"`, `"cut"`/`"C|"`) |
| `title` | string | `none` | Piece title |
| `subtitle` | string | `none` | Subtitle |
| `composer` | string | `none` | Composer name |
| `arranger` | string | `none` | Arranger name |
| `lyricist` | string | `none` | Lyricist name |
| `staff-group` | string | `"none"` | `"none"`, `"grand"` (brace + spanning barlines) |
| `staff-size` | length | `1.75mm` | Staff space distance |
| `system-spacing` | length | `12mm` | Vertical space between systems |
| `staff-spacing` | length | `8mm` | Vertical space between staves within a system |
| `width` | length/auto | `auto` | Explicit width or auto (fills page) |
| `measures-per-line` | int | `none` | Force this many measures per system |

**Staff dict fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `clef` | string | `none` | `"treble"`, `"bass"`, `"alto"`, `"tenor"`, `"treble-8a"`, `"treble-8b"`, `"treble-15a"`, `"treble-15b"`, `"bass-8a"`, `"bass-8b"`, `"bass-15a"`, `"bass-15b"`, `"percussion"`; if `none`, no clef glyph is drawn (treble mapping is used for internal staff-position calculations) |
| `music` | string | `""` | Music string (see syntax above) |
| `fingering-position` | string | `"above"` | Default fingering position: `"above"` or `"below"` |

### `melody()`

Convenience wrapper for a single-staff score.

```typ
#melody(
  music: "c4 d e f | g a b c'",
  key: "C",
  time: "4/4",
  clef: "treble",
  title: "My Melody",
  composer: "Composer",
)
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `music` | string | `""` | Music string |
| `key` | string | `none` | Key signature (if `none`, no key glyph is drawn) |
| `time` | string | `none` | Time signature (if `none`, no time glyph is drawn) |
| `clef` | string | `none` | Clef (if `none`, no clef glyph is drawn; treble mapping used for positions) |
| `title` | string | `none` | Title |
| `composer` | string | `none` | Composer |
| `staff-size` | length | `1.75mm` | Staff space |
| `width` | length/auto | `auto` | Width |
| `measures-per-line` | int | `none` | Measures per system |

### `lead-sheet()`

Melody with lyrics (lyrics not yet implemented).

```typ
#lead-sheet(
  music: "c4 d e f | g a b c'",
  key: "C",
  time: "4/4",
  title: "Song Title",
)
```

## Full Example

Here is the Ode to Joy example demonstrating a grand staff with fingerings, chord symbols, dynamics, and articulations:

```typ
#import "@preview/scorify:0.1.1": score

#set page(margin: 1.5cm)

#score(
  title: "Ode to Joy",
  composer: "L. van Beethoven",
  key: "D",
  time: "4/4",
  staff-group: "grand",
  staves: (
    (
      clef: "treble",
      music: "
        f#4n[3][D] f# g a | a8[D/A] b g4 f#[A] e |
        d[D] d e f# | f#4.[A] e8 e2 |
        f#4[D] f# g a | a[A] g f# e |
        d[D] d e f# | e4.[A] d8 d2[D] |
      ",
    ),
    (
      clef: "bass",
      fingering-position: "below",
      music: "
        d1n[1] | a, | d | a, |
        d | a, | d | a,2 d4 r |
      ",
    ),
  ),
)
```

## Clefs and Time Signatures

### Supported Clefs

The following clefs are supported and will render with proper key signature positioning:

| Clef | Usage | Common Use |
|------|-------|-----------|
| `"treble"` | Default treble clef (G4) | Melody, flute, clarinet, trumpet, violin |
| `"bass"` | Bass clef (F3) | Cello, tuba, left hand on piano |
| `"alto"` | Alto clef (C4 middle line) | Viola, trombone |
| `"tenor"` | Tenor clef (C4 2nd line) | Bassoon, trombone, cello (sometimes) |
| `"treble-8a"` | Treble clef 8va (ottava alta) | Notation with an '8' above the clef (sounds an octave higher) |
| `"treble-8b"` | Treble clef 8vb (ottava bassa) | Notation with an '8' below the clef (sounds an octave lower) |
| `"treble-15a"` | Treble clef 15ma (quindicesima alta) | Two-octave alta clef variant |
| `"treble-15b"` | Treble clef 15mb (quindicesima bassa) | Two-octave bassa clef variant |
| `"bass-8a"` | Bass clef 8va (ottava alta) | Bass clef with 8 above (sounds an octave higher) |
| `"bass-8b"` | Bass clef 8vb (ottava bassa) | Bass clef with 8 below (sounds an octave lower) |
| `"bass-15a"` | Bass clef 15ma | Two-octave alta bass clef |
| `"bass-15b"` | Bass clef 15mb | Two-octave bassa bass clef |
| `"percussion"` | Unpitched percussion clef | Percussion instruments |

When specifying a staff, use the clef name in the staff dictionary:

```typ
#score(
  staves: (
    (clef: "alto", music: "c4 d e f"),
    (clef: "tenor", music: "g,4 a b c"),
  ),
)
```

### Time Signatures

Time signatures can be specified as either traditional notation or by name:

| Input | Result | Meaning |
|-------|--------|---------|
| `"4/4"` | Standard 4/4 | Four quarter notes per measure |
| `"3/4"` | Waltz time | Three quarter notes per measure |
| `"6/8"` | Compound duple | Six eighth notes per measure |
| `"2/2"` | Cut time (alla breve) | Two half notes per measure |
| `"common"` or `"C"` | Common time (₵) | Equivalent to 4/4 |
| `"cut"` or `"C|"` | Cut time (₵) | Equivalent to 2/2 |

Examples:

```typ
#melody(music: "c4 d e f", time: "common")     // Common time symbol
#melody(music: "c4 d e f", time: "C")          // Same as above
#melody(music: "c2 d", time: "cut")            // Cut time symbol
#melody(music: "c2 d", time: "C|")             // Same as above
```

## Music String Syntax

This section documents the inline music-string syntax accepted by `score()`, `melody()`, and related helpers. The parser is intentionally compact and expressive - here are the primary constructs and examples.

- Notes: `name` + optional accidentals + octave markers + duration + optional dots.
  - Examples: `c4`, `d8.`, `f#4`, `g'2`, `a,16` (comma lowers octave, apostrophe raises it).
  - Accidentals: `#` (sharp), `##` (double-sharp), `&` (flat), `&&` (double-flat), `=` (natural).
  - Duration is sticky: if omitted the note uses the last explicit duration (e.g., `c4 d e f` uses quarter notes for all).

- Rests and spacers:
  - Rest: `r4`, `r8.`
  - Spacer (invisible rest): `s4`

- Chords (simultaneous notes):
  - Syntax: `<c e g>4` produces a C major chord as a quarter note.
  - Chord-level articulations, dynamics, ties, slurs, and fingering can be applied to the chord as a whole (see examples below).

- Articulations: appended as single-character markers after the duration
  - `>` = accent, `*` = staccato, `-` = tenuto, `_` = fermata
  - Example: `c4>*` (accent + staccato)

- Ties and slurs:
  - Tie: `~` connects the note to the following note of the same pitch (e.g., `c4~ c4`).
  - Slurs: `(` and `)` mark slur start/end. They may appear immediately after a note, or on their own to attach to the previous note.
    - Example: `c4( d e) f` or `c4 d( e f ) g`

- Dynamics:
  - Inline dynamic markers use `v[...]` where `[...]` is a dynamic text like `mf`, `f`, `pp`.
  - Example: `c4v[pp] d4 v[ff]` - dynamics are rendered below the staff.

- Fingering:
  - Fingering numbers are attached with `n[...]` (above) or `n_[...]` (below).
  - Multiple fingerings are supported by separating numbers with a space inside the brackets: `n[1 3]`.
  - Example: `c4n[3]` or `d4n_[2]` (below the staff).

- Inline chord symbols:
  - Append a bracketed symbol after a note or chord: `[C]`, `[Am7]`, `[D/F#]`.
  - Example: `c4[C]` or `<c e g>2[Dm]`.

- Beams and grouping:
  - Square brackets `[` and `]` can be used to force beam starts/ends when not interpreted as a chord symbol.

- Tuplets:
  - Syntax: `{n,m:notes}` where `n` is the number of beats the entire tuplet should occupy (used for spacing) and `m` is the number printed on the tuplet bracket (the tuplet count).
  - `m` may be omitted; when omitted it defaults to the same value as `n`.
  - Behavior: the contained events are laid out so the whole group occupies `n` beats; the group's width is distributed evenly across the contained events, so individual durations inside the tuplet do not affect its overall spacing.
  - Examples:

```typ
// Triplet that spans two beats (prints a "3" above the bracket)
{2,3:d4 e d}

// Five whole notes that nevertheless take the space of 2 beats
{2,3:c1 d e f g}
```

- Grand staff / multi-staff layout:
  - Use the `staves` array passed to `#score` and set `staff-group: "grand"` to request grand-staff rendering (brace and shared barlines).
  - Each staff can set `clef`, `music`, and `fingering-position` (`"above"` or `"below"`). See the full example above for a grand-staff sample.

Examples (combined):

```typ
#melody(music: "<c e g>4n[1][C] d4>* f#4v[mp] g4~ g4")
// chord with fingering and chord symbol, accent + staccato on next note,
// dynamic marking, tie on the last two notes
```

Refer to the parser logic in `src/parser.typ` for the complete, definitive syntax and edge cases.

## Notes

- Scorify uses the Bravura SMuFL font for accurate music glyph placement. See [Font Setup](#font-setup) above for installation instructions.
- Spacing parameters (note spacing base, duration factors, accidental padding, dot size) are tunable in `src/constants.typ`.
- The library requires Typst 0.14.0+ and CeTZ 0.4.2 (CeTZ is declared as a package dependency and is resolved automatically when using the Typst package manager).

## Contributing

Bug reports, feature requests, and pull requests are welcome in the [official repository](https://github.com/justinbornais/typst-sheet-music).

## License

MIT - see [LICENSE](LICENSE).
