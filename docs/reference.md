# Scorify Reference

Detailed syntax and API documentation for Scorify lives here. For installation, import, font setup, and quick examples, see the [README](../README.md).

## `score()`

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
| `color` | string | `none` | Default SVG color for the whole score, for example `"#b91c1c"` or `"red"` |
| `staff-size` | length | `1.75mm` | Staff space distance |
| `system-spacing` | length | `12mm` | Vertical space between systems |
| `staff-spacing` | length | `8mm` | Vertical space between staves in a system |
| `lyric-line-spacing` | length | `none` | Override stacked lyric line spacing |
| `music-font` | string | `"Leland"` | SMuFL font family |
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

## `melody()`

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
| `music-font` | string | `"Leland"` | SMuFL font family |
| `music-font-metadata` | dictionary/none | `none` | Optional metadata dictionary |
| `width` | length/auto | `auto` | Width |
| `measures-per-line` | int | `none` | Force a fixed number of measures per system |

## Staff Grouping

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

## Supported Clefs

Scorify supports:

- `"treble"`, `"bass"`, `"alto"`, `"tenor"`
- `"treble-8a"`, `"treble-8b"`, `"treble-15a"`, `"treble-15b"`
- `"bass-8a"`, `"bass-8b"`, `"bass-15a"`, `"bass-15b"`
- `"percussion"`

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
  - Beats align inside the voice group, so shorter notes line up with longer notes.

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

### Built-in Color Presets

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
  - Inline clef changes: `... bass ... treble ...`
  - Inline time-signature changes: `... 3/4 ... 5/4 ... common ... cut ...`
  - Literal newlines force a system break

- **Lyrics**
  - Attach with `l[...]`: `c4l[text]`
  - Hyphen continuation: `l[text-]`
  - Melisma/extender: `l[text_]`
  - Carry the previous lyric state with plain `l`
  - Stack multiple lyric lines by attaching multiple lyric entries to one event

## Multiple Voices Example

Use `v{upper;lower}` when two independent rhythms share one staff.

```typ
#melody(
  clef: "treble",
  time: "4/4",
  music: "v{c'2 g';c4 e g c'} | v{<e' g'>2 <d' f'>;c4 d e f}",
)
```

## More Examples

Useful starting points in `examples/`:

- `ode-to-joy.typ`: grand staff with chord symbols, fingerings, and dynamics
- `techniques.typ`: dense mixed-notation showcase
- `inline-clef-changes.typ`: mid-system clef changes
- `grace-notes.typ`: grace notes and acciaccaturas
- `lyrics-demo.typ`: lyrics and multi-line lyric layout
- `clef-variants.typ` and `alto-tenor-demo.typ`: alternate clefs
- `three-endings.typ`: repeat endings / voltas

See `tests/test-render-basic.typ` and `tests/test-colors.typ` for broader syntax coverage.
