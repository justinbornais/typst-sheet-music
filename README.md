# Scorify

Render sheet music directly inside Typst using SMuFL-aware glyph placement and a WASM-backed SVG renderer.

## Features

- WASM-backed Typst API with no LilyPond or MuseScore CLI dependency.
- Notes, rests, spacers, chords, multiple voices, accidentals, key signatures, time signatures, and supported clefs.
- Rhythms from maxima, longa, and breve through standard shorter note values.
- Inline annotations: dynamics, hairpins, articulations, fingerings, chord symbols, expression text, staff text, staff markers, and lyrics.
- Notation features: beams, ties, slurs, tuplets, octave lines, trills, grace notes / acciaccaturas, repeat barlines, endings, and dotted notes.
- Global, per-staff, selection, and element-local color overrides for musical content.
- Inline clef changes, inline time-signature changes, manual spacing via repeated spaces, and explicit system breaks.
- Single-staff, grand-staff, bracketed, and connected multi-staff layout with vertical beat alignment.
- Alternate SMuFL font support via `music-font` and `music-font-metadata`.
- Crisp vector PDF output.

## Quick Start

### Via Typst Package Manager

```typ
#import "@preview/scorify:0.2.0": score, melody

#melody(
  title: "Scale",
  key: "C",
  time: "4/4",
  music: "c4 d e f | g a b c'",
)
```

Compile normally:

```text
typst compile your-file.typ
```

### Manual Installation

Copy `lib.typ` and `scorify_wasm.wasm` into your project, then import locally:

```typ
#import "lib.typ": score, melody
```

Compile with:

```text
typst compile your-file.typ --root .
```

## Font Setup

Scorify defaults to [Bravura](https://github.com/steinbergmedia/bravura) plus bundled Bravura metadata. Bravura is embedded directly in `scorify_wasm.wasm` and rendered as SVG paths by default, so documents do not need a system-installed Bravura font or `--font-path`.

If you select a different SMuFL font with `music-font`, Typst must discover that font from an installed system font or a directory passed with `--font-path`. If Typst cannot find the selected font, compilation continues but the CLI prints an `unknown font family` warning.

### Alternate SMuFL Fonts

You can switch fonts with:

- `music-font`: Typst font family name
- `music-font-metadata`: optional SMuFL metadata dictionary, usually loaded with `json(...)`

```typ
#import "@preview/scorify:0.2.0": melody

#melody(
  music: "c4 d e f | g a b c'",
  music-font: "Your SMuFL Font",
  music-font-metadata: json("your-smufl-metadata.json"),
)
```

## API Reference

### `score()`

Primary entry point for one or more staves.

```typ
#score(
  staves: (
    (clef: "treble", brace-start: true, music: "c4 d e f | g a b c'"),
    (clef: "bass", brace-end: true, music: "c2 g | c1"),
  ),
  key: "C",
  time: "4/4",
  title: "My Piece",
  composer: "Composer Name",
)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `staves` | array | `()` | Array of staff dictionaries |
| `key` | string | `"C"` | Key signature like `"C"`, `"G"`, `"Bb"`, `"f#"` |
| `time` | string | `none` | Time signature like `"4/4"`, `"6/8"`, `"common"`, `"cut"` |
| `title` | string | `none` | Piece title |
| `subtitle` | string | `none` | Subtitle |
| `composer` | string | `none` | Composer name |
| `arranger` | string | `none` | Arranger name |
| `lyricist` | string | `none` | Lyricist name |
| `color` | string | `none` | Default SVG color for the whole score, for example `"#b91c1c"` |
| `staff-size` | length | `1.75mm` | Staff space distance |
| `system-spacing` | length | `12mm` | Vertical space between systems |
| `staff-spacing` | length | `8mm` | Vertical space between staves in a system |
| `lyric-line-spacing` | length | `none` | Override stacked lyric line spacing |
| `music-font` | string | `"Bravura"` | SMuFL font family |
| `music-font-metadata` | dictionary/none | `none` | Optional metadata dictionary |
| `width` | length/auto | `auto` | Explicit width or auto |
| `measures-per-line` | int | `none` | Force a fixed number of measures per system |

Staff dictionaries support:

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `clef` | string | `none` | Any supported clef, including octave-clef variants and `"percussion"` |
| `music` | string | `""` | Music string |
| `instrument-name` | string | `none` | Full name for the first system |
| `instrument-name-cont` | string | `none` | Continued-system name, often abbreviated |
| `instrument-name-shared` | bool | `false` | Share the previous staff's name, centered across both staves |
| `fingering-position` | string | `"above"` | Default fingering position: `"above"` or `"below"` |
| `color` | string | `none` | Default SVG color for everything on this staff |
| `barline-group-start` / `barline-group-end` | bool | `false` | Connect measure lines across adjacent staves without drawing a brace or bracket |
| `bracket-start` / `bracket-end` | bool | `false` | Draw a straight bracket and connected measure lines across adjacent staves |
| `brace-start` / `brace-end` | bool | `false` | Draw a grand-staff brace and connected measure lines across adjacent staves |

Instrument names reserve space before the staff. Use `&`, `#`, and `=` in names for flat, sharp, and natural symbols.

### Staff Grouping

By default, staves are separate: no brace or bracket is drawn, and measure lines do not connect between staves.

Use per-staff start/end fields to group adjacent staves. Mark the top staff with `*-start` and the bottom staff with the matching `*-end`.

```typ
#score(
  staves: (
    (
      clef: "treble",
      brace-start: true,
      music: "c'4 d' e' f'",
    ),
    (
      clef: "bass",
      brace-end: true,
      music: "c,4 e, g, c",
    ),
  ),
)
```

Use `brace-start` / `brace-end` for a grand staff, `bracket-start` / `bracket-end` for a bracketed section, and `barline-group-start` / `barline-group-end` when you only want measure lines connected without a brace or bracket. Groups can overlap when needed, such as a string-section bracket with a two-staff brace inside it.

### `melody()`

Single-staff convenience wrapper around `score()`.

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

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `music` | string | `""` | Music string |
| `key` | string | `"C"` | Key signature |
| `time` | string | `none` | Time signature |
| `clef` | string | `none` | Clef |
| `instrument-name` | string | `none` | Full name for the first system |
| `instrument-name-cont` | string | `none` | Continued-system name, often abbreviated |
| `title` | string | `none` | Title |
| `composer` | string | `none` | Composer |
| `color` | string | `none` | Default SVG color for the melody staff |
| `staff-size` | length | `1.75mm` | Staff space |
| `system-spacing` | length | `12mm` | Vertical space between systems |
| `lyric-line-spacing` | length | `none` | Override stacked lyric line spacing |
| `music-font` | string | `"Bravura"` | SMuFL font family |
| `music-font-metadata` | dictionary/none | `none` | Optional metadata dictionary |
| `width` | length/auto | `auto` | Width |
| `measures-per-line` | int | `none` | Force a fixed number of measures per system |

## Full Example

```typ
#import "@preview/scorify:0.2.0": score

#set page(margin: 1.5cm)

#score(
  title: "Ode to Joy",
  composer: "L. van Beethoven",
  key: "D",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      brace-start: true,
      music: "
        f#4n[3][D] f# g a | a8[D/A] b g4 f#[A] e |
        d[D] d e f# | f#4.[A] e8 e2 |
        f#4[D] f# g a | a[A] g f# e |
        d[D] d e f# | e4.[A] d8 d2[D] |
      ",
    ),
    (
      clef: "bass",
      brace-end: true,
      fingering-position: "below",
      music: "
        d1n[1] | a, | d | a, |
        d | a, | d | a,2 d4 r |
      ",
    ),
  ),
)
```

## Examples

Useful starting points in `examples/`:

- `ode-to-joy.typ`: grand staff with chord symbols, fingerings, and dynamics
- `techniques.typ`: dense mixed-notation showcase
- `inline-clef-changes.typ`: mid-system clef changes
- `grace-notes.typ`: grace notes and acciaccaturas
- `lyrics-demo.typ`: lyrics and multi-line lyric layout
- `clef-variants.typ` and `alto-tenor-demo.typ`: alternate clefs
- `three-endings.typ`: repeat endings / voltas

## Supported Clefs

Scorify supports:

- `"treble"`, `"bass"`, `"alto"`, `"tenor"`
- `"treble-8a"`, `"treble-8b"`, `"treble-15a"`, `"treble-15b"`
- `"bass-8a"`, `"bass-8b"`, `"bass-15a"`, `"bass-15b"`
- `"percussion"`

Example:

```typ
#score(
  staves: (
    (clef: "alto", music: "c4 d e f"),
    (clef: "tenor", music: "g,4 a b c"),
  ),
)
```

## Time Signatures

Examples of accepted inputs:

| Input | Meaning |
|-------|---------|
| `"4/4"` | Four quarter notes per measure |
| `"3/4"` | Three quarter notes per measure |
| `"6/8"` | Compound duple |
| `"2/2"` | Alla breve |
| `"common"` or `"C"` | Common time symbol |
| `"cut"` or `"C\|"` | Cut time symbol |

```typ
#melody(music: "c4 d e f", time: "common")
#melody(music: "c2 d", time: "cut")
```

## Music String Cheat Sheet

- **Notes and rhythm**: `c4`, `d8.`, `f#4`, `g'2`, `a,16`
  - Accidentals: `#`, `##`, `&`, `&&`, `=`
  - Octave markers: `'` raises, `,` lowers
  - Longer notes use names: `cbreve`, `clonga`, `cmaxima`
  - Duration is sticky: `c4 d e f`, `cbreve d`

- **Rests, spacers, and manual spacing**: `r4`, `r8.`, `rbreve`, `rlonga`, `rmaxima`, `s4`, `smaxima`
  - Repeated spaces add extra horizontal gap: `c e   g c`

- **Chords**: `<c e g>4`, `<c e g>breve`, `<c e g>maxima`

- **Multiple voices on one staff**: `v{c2 g,;c4 e g c}`
  - Start a voice group with `v{...;...}`.
  - The first voice, before `;`, is drawn stem-up.
  - The second voice, after `;`, is drawn stem-down.
  - Beats align inside the voice group, so shorter notes line up with longer notes. In `v{c2;c4 e}`, the two quarter notes occupy the same time span as the half note.
  - Use normal note syntax inside each voice, including chords, rests, dotted rhythms, and octave markers: `v{<c e>2.;g4 r g}`.

- **Articulations**: `>` accent, `*` staccato, `-` tenuto, `_` fermata

- **Ties and slurs**: `c4~ c4`, `c4( d e) f`

- **Inline attachments**
  - Dynamics: `v[pp]`, `v[mf]`, `v[ff]`
  - Staff text above: `text[Solo]`
  - Expression text below: `exp[dolce]`
  - Fingerings: `n[3]`, `n_[2]`, `n[1 *3* 5]`
  - Chord symbols: `[C]`, `[Am7]`, `[D/F#]`
  - Staff markers: `bm` (breath mark), `//` (caesura), `ds`, `coda`

- **Color controls**
  - Global score / melody default: `#score(color: "sky blue", ...)`, `#melody(color: "red", ...)`, or raw hex like `#score(color: "#0f766e", ...)`
  - Per-staff default: `(clef: "treble", color: "blue", music: "...")`
  - Selection wrapper: `color{red:d4 e f g | e( d) c2}` or `color{#dc2626:d4 e f g | e( d) c2}`
  - Element-local override: `c4color{red}`, `c4~color{blue} c4`, `c4(color{green} d)`, `<c ecolor{purple} g>4`
  - Selection color affects musical content inside the wrapper but intentionally does not recolor staff lines or measure lines.
  - Built-in color presets:

    | Name | Hex |
    |------|-----|
    | `red` | `#ff0000` |
    | `orange` | `#ffa500` |
    | `yellow` | `#ffcf00` |
    | `green` | `#00ff00` |
    | `blue` | `#0000ff` |
    | `sky blue` / `sky-blue` / `sky_blue` | `#4e9fe5` |
    | `purple` | `#9d0055` |
    | `gold` | `#d4af37` |
    | `white` | `#ffffff` |
    | `black` | `#000000` |
    | `silver` | `#c0c0c0` |
    | `platinum` | `#e5e4e2` |
    | `bronze` | `#cd7f32` |
    | `copper` | `#b87333` |
    | `charcoal` | `#36454f` |
    | `navy` | `#0a2a66` |

- **Spans and ornaments**
  - Hairpins: `cresc{c e g c}`, `decresc{c' b a g}`
  - Trills: `c4tr`, `tr{d'4 e' f' g'}`
  - Grace notes: `grace{c16 d e} f4`
  - Acciaccatura-style slash: `grace{f#16 g a/} b4`
  - Octave lines: `8a{...}`, `8b{...}`, `15a{...}`, `15b{...}`
  - Tuplets: `{2,3:d4 e d}`
  - Endings / voltas: `end{1.: f d e c | g g c c}`

- **Structure**
  - Barlines: `|`, `||`, `|.`, `|:`, `||:`, `:|`, `:||`, `:|:`, `:||:`
  - Forced beaming: `[` and `]` where they are not parsed as chord symbols
  - Inline clef changes: `... bass ... treble ...` (cue-sized mid-system)
  - Inline time-signature changes: `... 3/4 ... 5/4 ... common ... cut ...`
  - Literal newlines force a system break

- **Lyrics**
  - Attach with `l[...]`: `c4l[text]`
  - Hyphen continuation: `l[text-]`
  - Melisma/extender: `l[text_]`
  - Carry the previous lyric state with plain `l`
  - Stack multiple lyric lines by attaching multiple lyric entries to one event

### Multiple Voices Example

Use `v{upper;lower}` when two independent rhythms share one staff. The upper voice always uses upward stems and the lower voice always uses downward stems. Each side of the semicolon is parsed as a normal music string, and events are aligned by beat inside the group.

```typ
#melody(
  clef: "treble",
  time: "4/4",
  music: "v{c'2 g';c4 e g c'} | v{<e' g'>2 <d' f'>;c4 d e f}",
)
```

Use rests when one voice should be silent while the other continues:

```typ
#melody(
  clef: "treble",
  time: "4/4",
  music: "v{a'4 b' c'' d'';a4 r b c'}",
)
```

Short example:

```typ
#score(
  staves: (
    (clef: "treble", brace-start: true, music: "c4[Am]n[1] dtext[Solo] e4tr f | cresc{g a b c'} | end{1.: d'4 e' f' g'}"),
    (clef: "bass", brace-end: true, music: "c,4 e, g, c bass b, a, g, | grace{c16 d e/} f4 | 3/4 c e g"),
  ),
)
```

See `examples/` and `tests/test-render-basic.typ` for more combinations and edge cases.

## Notes

- Scorify defaults to the Bravura SMuFL font and bundled Bravura metadata; default Bravura glyphs are rendered from font data embedded in `scorify_wasm.wasm`.
- Alternate SMuFL fonts may need spacing adjustments depending on their metadata quality.
- Core parsing, layout, glyph metrics, and rendering command generation live in `wasm/src/`.
- The library requires Typst `0.14.0+`.

## Contributing

Bug reports, feature requests, and pull requests are welcome in the [official repository](https://github.com/justinbornais/typst-sheet-music).

## License

MIT - see [LICENSE](LICENSE).
