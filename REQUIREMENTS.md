# typst-sheet-music — Requirements Document

> **Version:** 1.0.0-draft  
> **Date:** 2026-03-29  
> **Package Name:** `sheet-music`  
> **Typst Minimum Version:** 0.14.0+

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Goals and Non-Goals](#2-goals-and-non-goals)
3. [Prior Art and Landscape Analysis](#3-prior-art-and-landscape-analysis)
4. [Architecture and Technology Stack](#4-architecture-and-technology-stack)
5. [User-Facing Input Syntax](#5-user-facing-input-syntax)
6. [Feature Requirements](#6-feature-requirements)
   - 6.1 [Staff and System Layout](#61-staff-and-system-layout)
   - 6.2 [Notes, Rests, and Durations](#62-notes-rests-and-durations)
   - 6.3 [Accidentals and Key Signatures](#63-accidentals-and-key-signatures)
   - 6.4 [Time Signatures](#64-time-signatures)
   - 6.5 [Clefs](#65-clefs)
   - 6.6 [Measures and Barlines](#66-measures-and-barlines)
   - 6.7 [Beaming](#67-beaming)
   - 6.8 [Chords (Harmonic)](#68-chords-harmonic)
   - 6.9 [Chord Symbols (Lead Sheet)](#69-chord-symbols-lead-sheet)
   - 6.10 [Lyrics](#610-lyrics)
   - 6.11 [Ties and Slurs](#611-ties-and-slurs)
   - 6.12 [Articulations and Ornaments](#612-articulations-and-ornaments)
   - 6.13 [Dynamics](#613-dynamics)
   - 6.14 [Tempo and Rehearsal Marks](#614-tempo-and-rehearsal-marks)
   - 6.15 [Repeats and Endings](#615-repeats-and-endings)
   - 6.16 [Grace Notes](#616-grace-notes)
   - 6.17 [Tuplets](#617-tuplets)
   - 6.18 [Mid-Staff Changes](#618-mid-staff-changes)
   - 6.19 [Transposition](#619-transposition)
   - 6.20 [Multi-Voice Writing](#620-multi-voice-writing)
   - 6.21 [Title and Header Block](#621-title-and-header-block)
7. [Rendering Engine Design](#7-rendering-engine-design)
8. [Layout Algorithm](#8-layout-algorithm)
9. [Music Font Strategy](#9-music-font-strategy)
10. [Project Structure](#10-project-structure)
11. [API Surface](#11-api-surface)
12. [Error Handling](#12-error-handling)
13. [Testing Strategy](#13-testing-strategy)
14. [Performance Considerations](#14-performance-considerations)
15. [Packaging and Distribution](#15-packaging-and-distribution)
16. [Phased Implementation Roadmap](#16-phased-implementation-roadmap)
17. [Appendix A — Full Syntax Reference](#appendix-a--full-syntax-reference)
18. [Appendix B — SMuFL Glyph Subset](#appendix-b--smufl-glyph-subset)

---

## 1. Project Overview

`sheet-music` is a **pure Typst** library (no WASM plugin, no external binary dependency) that allows users to render professional-quality Western music notation directly inside Typst documents. The output is vector-based and renders natively into PDF, exactly like any other Typst content.

The library draws all notation using **CeTZ** (the de-facto Typst drawing library) combined with glyphs from an embedded **SMuFL-compliant music font** (Bravura). This means:

- No external tool-chain (no LilyPond, no MuseScore CLI).
- No WASM plugin compilation step.
- Works on Typst web app and local CLI identically.
- Produces crisp, resolution-independent vector output.

---

## 2. Goals and Non-Goals

### Goals

| ID | Goal |
|----|------|
| G1 | Render one or more vertically-grouped staves (single, grand staff, SATB, orchestral score) with correct vertical beat alignment across all staves. |
| G2 | Provide a concise, text-based input syntax inspired by LilyPond that is comfortable to type inline in a Typst document. |
| G3 | Support lyrics aligned to notes, with automatic syllable-to-note association and melisma handling. |
| G4 | Support chord symbols (e.g., `Am7`, `Cmaj9`) rendered above the staff. |
| G5 | Support all common Western notation elements: clefs, key signatures, time signatures, notes, rests, accidentals, dots, ties, slurs, beams, dynamics, articulations, ornaments, grace notes, tuplets, repeats, and barlines. |
| G6 | Allow mid-measure changes to clef, key signature, and time signature. |
| G7 | Not enforce beats-per-measure; render whatever the user provides. |
| G8 | Produce beautiful, publication-quality output comparable to LilyPond. |
| G9 | Be distributable as a standard Typst Universe package (`@preview/sheet-music`). |
| G10 | Provide a reasonable default layout while exposing configuration knobs for advanced users. |

### Non-Goals

| ID | Non-Goal |
|----|----------|
| NG1 | Audio/MIDI playback. |
| NG2 | MusicXML import/export (may be considered in future versions). |
| NG3 | Tablature rendering (guitar tabs) in v1 — may be added later. |
| NG4 | Automatic part extraction from a full score. |
| NG5 | Full editorial/critical-edition notation (ossia staves, editorial accidentals, etc.). |

---

## 3. Prior Art and Landscape Analysis

### Existing Typst Packages

| Package | What it does | Limitations for our use case |
|---------|-------------|------------------------------|
| **staves** (0.1.0) | Draws single staves with clefs, key signatures, and sequential notes using CeTZ. Supports treble/bass/alto/tenor clefs, scales, arpeggios. | No measures, barlines, rests, beaming, chords, dynamics, lyrics, multi-staff systems, or rhythmic notation. Not designed for full music notation. |
| **conchord** (0.4.0) | Lyrics + chords + fretboard diagrams. | No actual staff notation; purely text-based lyrics/chords plus guitar diagrams. |
| **chordx** (0.6.1) | Chord diagrams (fretboards) for songs. | No staff notation at all. |

**Conclusion:** No existing Typst package provides full music notation. We are building from scratch, but can study `staves` (CeTZ-based rendering on staff lines) and `conchord` (text-based input for lyrics/chords) for inspiration.

### External Reference Systems

| System | Key Takeaways |
|--------|---------------|
| **LilyPond** | Gold standard for text-input music notation. Its input syntax (`c'4 d'8 e'`) is proven and ergonomic. We will adapt (not copy) this approach for our Typst syntax. Its spacing and engraving algorithms are world-class reference points. |
| **MuseScore** | WYSIWYG editor. Relevant for understanding what users expect in terms of visual output (vertical alignment, beaming rules, spacing). |
| **VexFlow** | JavaScript music notation renderer. Good reference for a programmatic rendering API — demonstrates that quality output can come from placing glyphs + drawing lines/curves. |
| **SMuFL / Bravura** | The W3C standard glyph layout for music fonts. Bravura is the reference SMuFL font (open-source, SIL OFL). Using SMuFL means we get thousands of standard music glyphs with well-defined metrics. |

---

## 4. Architecture and Technology Stack

### Technology Choices

| Component | Choice | Rationale |
|-----------|--------|-----------|
| **Language** | Pure Typst scripting | No external dependencies, works everywhere Typst runs. |
| **Drawing** | CeTZ 0.5.x (`@preview/cetz`) | Mature, well-maintained Typst drawing library with canvas, lines, curves, text placement, and coordinate transforms. Already the foundation of `staves`. |
| **Music Glyphs** | Bravura font (OTF, bundled) | Open-source SMuFL reference font. Contains all required music symbols with standardized codepoints and metrics. Distributed under SIL OFL, compatible with any license. |
| **Glyph Metrics** | Bundled JSON metadata | SMuFL provides `glyphnames.json`, `bravura_metadata.json` with precise bounding boxes, anchors, and staff-position data for each glyph. We parse this at import time for precise placement. |
| **Plugin** | None | All logic is pure Typst. No WASM compilation needed. |

### High-Level Architecture

```
┌─────────────────────────────────────────────────┐
│  User Typst Document                            │
│                                                 │
│  #import "@preview/sheet-music:0.1.0": score    │
│                                                 │
│  #score(                                        │
│    key: "D", time: "4/4",                       │
│    music: "...",                                 │
│    lyrics: "...",                                │
│    chords: "...",                                │
│  )                                              │
└──────────────┬──────────────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────────────┐
│  1. PARSER                                       │
│     Converts string input into internal          │
│     representation (array of music events)       │
└──────────────┬───────────────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────────────┐
│  2. LAYOUT ENGINE                                │
│     Assigns horizontal positions (spacing),      │
│     vertical positions (pitch→staff position),   │
│     beaming groups, line breaks, system breaks   │
└──────────────┬───────────────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────────────┐
│  3. RENDERER (CeTZ canvas)                       │
│     Draws staff lines, barlines, clefs, key      │
│     sigs, time sigs, noteheads, stems, beams,    │
│     flags, accidentals, dots, slurs, ties,       │
│     dynamics, text (lyrics, chords, tempo)        │
│     using CeTZ drawing primitives + Bravura      │
│     font glyphs                                  │
└──────────────┬───────────────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────────────┐
│  Typst Content (block-level)                     │
│  → Flows into page layout like any other content │
└──────────────────────────────────────────────────┘
```

---

## 5. User-Facing Input Syntax

### Design Principles

1. **Minimal boilerplate** — common cases should be short.
2. **One line per voice/staff** — visual correspondence between input and output.
3. **LilyPond-inspired pitch/duration encoding** — familiar to musicians who have used text-based notation.
4. **Typst-native** — uses Typst strings and function arguments, not a custom DSL with a separate parser (though we parse the music strings).

### Pitch Encoding

| Element | Syntax | Example | Meaning |
|---------|--------|---------|---------|
| Note name | `a`–`g` (lowercase) | `c` | Middle octave reference pitch |
| Sharp | `#` after note name | `f#` | F-sharp |
| Flat | `b` after note name (when ambiguous, use `&`) | `e&` | E-flat |
| Double sharp | `##` | `f##` | F double-sharp |
| Double flat | `&&` | `b&&` | B double-flat |
| Natural (explicit) | `=` after note name | `b=` | B natural (override key sig) |
| Octave up | `'` (apostrophe) | `c'` | One octave above reference |
| Octave down | `,` (comma) | `c,` | One octave below reference |
| Multiple octave shifts | Repeat marker | `c''` | Two octaves up |
| Rest | `r` | `r` | Rest |
| Spacer/invisible rest | `s` | `s` | Spacer (occupies time, invisible) |

**Octave reference:** The default octave (no markers) is octave 4 (middle C octave), matching LilyPond's relative octave convention is available as an option but by default the library uses absolute octave specification where `c` = C4, `c'` = C5, `c,` = C3.

### Duration Encoding

| Duration | Syntax | Notes |
|----------|--------|-------|
| Whole note | `1` | Semibreve |
| Half note | `2` | Minim |
| Quarter note | `4` | Crotchet |
| Eighth note | `8` | Quaver |
| Sixteenth note | `16` | Semiquaver |
| Thirty-second note | `32` | Demisemiquaver |
| Sixty-fourth note | `64` | Hemidemisemiquaver |
| Dotted | `.` suffix | `4.` = dotted quarter |
| Double-dotted | `..` suffix | `4..` = double-dotted quarter |
| **Duration stickiness** | If omitted, use previous duration | `c4 d e` = three quarter notes |

### Core Input Example

```typst
#import "@preview/sheet-music:0.1.0": score

// Simple melody — single staff
#score(
  title: "Ode to Joy",
  composer: "L. van Beethoven",
  key: "D",
  time: "4/4",
  staves: (
    (clef: "treble", music: "
      f#'4 f#' g' a' | a' g' f#' e' | d' d' e' f#' | f#'4. e'8 e'2 |
      f#'4 f#' g' a' | a' g' f#' e' | d' d' e' f#' | e'4. d'8 d'2
    "),
  ),
)
```

### Grand Staff (Piano)

```typst
#score(
  key: "C",
  time: "4/4",
  staves: (
    (clef: "treble", music: "c'4 e' g' c'' | c''1"),
    (clef: "bass",   music: "c2 g | c1"),
  ),
  staff-group: "grand",   // Draws brace + connects barlines
)
```

### SATB Choir

```typst
#score(
  key: "G",
  time: "4/4",
  staves: (
    (clef: "treble", label: "S", music: "b'4 b' a' g'  | a'2 b'"),
    (clef: "treble", label: "A", music: "g'4 g' f#' e' | f#'2 g'"),
    (clef: "treble-8", label: "T", music: "d'4 d' d' b  | d'2 d'"),
    (clef: "bass",   label: "B", music: "g4 g d e     | d2 g"),
  ),
  staff-group: "choir",   // Draws bracket + individual barlines
  lyrics: (
    (staff: 1, text: "Glo -- ry, glo -- ry, hal -- le -- lu -- jah!"),
  ),
)
```

### Lyrics Syntax

Lyrics are attached to a staff by index (1-based) and use a simple space-separated format:

| Element | Syntax | Meaning |
|---------|--------|---------|
| Syllable | plain text | One syllable per note |
| Hyphen (syllable break) | ` -- ` (spaced double dash) | Connects syllables of one word with a hyphen |
| Melisma extender | `__` | Extends previous syllable across tied/slurred notes |
| Skip a note | `_` (single underscore) | No lyric for this note (used for melismas or instrumental notes) |
| Verse number | `1.` prefix on first word | Automatic verse numbering |

**Example:**
```typst
lyrics: (
  (staff: 1, text: "A -- ma -- zing grace, how sweet the sound"),
  (staff: 1, verse: 2, text: "'Twas grace that taught my heart to fear"),
)
```

### Chord Symbol Syntax

Chord symbols are placed above a staff, aligned to specific beats:

```typst
chords: (
  (staff: 1, symbols: "G | D/F# Em | C D | G"),
)
```

| Element | Syntax | Rendering |
|---------|--------|-----------|
| Major | `C` | C |
| Minor | `Cm` or `Cmin` | Cm |
| Seventh | `C7` | C7 |
| Major seventh | `Cmaj7` or `CM7` | Cmaj7 |
| Minor seventh | `Cm7` or `Cmin7` | Cm7 |
| Diminished | `Cdim` or `Co` | C° |
| Augmented | `Caug` or `C+` | C+ |
| Suspended | `Csus4`, `Csus2` | Csus4 |
| Add | `Cadd9` | Cadd9 |
| Slash (inversion) | `C/E` | C/E |
| No chord | `N.C.` | N.C. |
| Barline sync | `\|` | Aligns to barline in music |
| Beat align | space-separated | One symbol per beat (quarter note by default) |

---

## 6. Feature Requirements

### 6.1 Staff and System Layout

| ID | Requirement |
|----|-------------|
| S1 | Render standard 5-line staff with configurable line thickness and spacing. |
| S2 | Support arbitrarily many staves grouped vertically into a **system**. |
| S3 | Support **grand staff** grouping (brace on left, connected barlines) for piano/keyboard. |
| S4 | Support **choir** grouping (bracket on left, barlines may or may not connect depending on configuration). |
| S5 | Support **orchestral** grouping (bracket groups by instrument family, braces for keyboard). |
| S6 | Automatically break systems across lines when the music exceeds the available width, just as text wraps. |
| S7 | Draw system-initial clef, key signature, and time signature on every new line. Cautionary clef/key at end of preceding line when changes occur. |
| S8 | Configurable vertical spacing between staves within a system and between systems. |
| S9 | Optional staff labels on the first system (full name) and subsequent systems (abbreviation). |
| S10 | Automatic vertical beat alignment: notes/rests at the same beat position across all staves in a system share the same horizontal x-coordinate. |

### 6.2 Notes, Rests, and Durations

| ID | Requirement |
|----|-------------|
| N1 | Render noteheads for whole, half, quarter, eighth, sixteenth, thirty-second, and sixty-fourth notes. |
| N2 | Render corresponding rest symbols for each duration. |
| N3 | Dotted and double-dotted notes/rests. |
| N4 | Correct stem direction: up for notes below middle line, down for notes on or above. User override available. |
| N5 | Correct stem length (minimum 3.5 staff spaces, extended for notes far from staff). |
| N6 | Flags on unbeamed eighth notes and shorter (correct direction matching stem). |
| N7 | Ledger lines for notes above and below the staff, drawn with correct width and spacing. |
| N8 | Whole and double-whole (breve) rest centered in measure. |
| N9 | Multi-measure rests with measure count (e.g., the "H-bar" style). |

### 6.3 Accidentals and Key Signatures

| ID | Requirement |
|----|-------------|
| A1 | Render sharps, flats, naturals, double-sharps (×), and double-flats. |
| A2 | Accidentals placed to the left of the notehead, with correct spacing (no collision with adjacent notes). |
| A3 | Key signatures displayed at the start of each system and after key changes, using standard sharp/flat order and vertical placement per clef. |
| A4 | Courtesy (cautionary) accidentals: automatically add parenthesized accidental when a note was altered in the previous measure. Configurable (on/off/always). |
| A5 | Key signature cancellation naturals when key changes to fewer sharps/flats. |

### 6.4 Time Signatures

| ID | Requirement |
|----|-------------|
| T1 | Standard numeric time signatures: `4/4`, `3/4`, `6/8`, `2/2`, etc. |
| T2 | Common time (`C`) and cut time (`¢`) symbols. |
| T3 | Compound/complex time signatures: `5/4`, `7/8`, additive forms like `3+2/8`. |
| T4 | Time signature placed after clef and key signature at start of piece and after changes. |

### 6.5 Clefs

| ID | Requirement |
|----|-------------|
| CL1 | Treble clef (G clef). |
| CL2 | Bass clef (F clef). |
| CL3 | Alto clef (C clef, 3rd line). |
| CL4 | Tenor clef (C clef, 4th line). |
| CL5 | Treble clef 8va bassa (`treble-8`) — for tenor voice. |
| CL6 | Treble clef 8va alta (`treble+8`). |
| CL7 | Mid-measure clef changes rendered at reduced size. |
| CL8 | Percussion clef (two vertical bars). |

### 6.6 Measures and Barlines

| ID | Requirement |
|----|-------------|
| M1 | Single barline (default measure separator). |
| M2 | Double barline (section boundary). |
| M3 | Final barline (thin+thick at end of piece). |
| M4 | Repeat barlines (start repeat, end repeat, start+end repeat). |
| M5 | Dashed/dotted barline (for irregular groupings). |
| M6 | Barlines connect staves within a group (configurable per group type). |
| M7 | Beats per measure are **not enforced** — the renderer draws whatever notes the user places between barline markers. |
| M8 | Measure numbers displayed at the start of each system (configurable: every measure, every N measures, or off). |

### 6.7 Beaming

| ID | Requirement |
|----|-------------|
| B1 | Automatic beaming of eighth notes and shorter based on time signature grouping rules (e.g., 4/4 beams in groups of 2 beats, 6/8 in groups of 3 eighth-notes). |
| B2 | User override to force beam start/break: `[` begins a beam group, `]` ends it. |
| B3 | Cross-staff beaming for grand-staff writing (notes on one staff beamed to notes on another). |
| B4 | Correct beam slope calculation based on the pitches of the beamed group. |
| B5 | Secondary beam breaks (e.g., in 16th-note patterns, breaking the inner beam at eighth-note subdivisions). |

### 6.8 Chords (Harmonic)

Chords are multiple notes sounding simultaneously on the same staff.

| ID | Requirement |
|----|-------------|
| CH1 | Input syntax: notes enclosed in angle brackets `<c e g>4` = C major chord, quarter note. |
| CH2 | Correct notehead stacking: seconds displaced to alternate sides of the stem. |
| CH3 | Accidentals on chord tones arranged to avoid collisions (standard stacking algorithm). |
| CH4 | Single stem and flag/beam for the chord. |
| CH5 | Arpeggiated chord marking (wavy line). |

### 6.9 Chord Symbols (Lead Sheet)

| ID | Requirement |
|----|-------------|
| CS1 | Render chord symbols above the staff in a readable font (bold root, superscript extensions). |
| CS2 | Automatic alignment to beat positions within each measure. |
| CS3 | Slash chords (e.g., `C/E`) with bass note after slash. |
| CS4 | Support all common chord qualities: major, minor, diminished, augmented, 7th, maj7, min7, dim7, 9th, 11th, 13th, sus2, sus4, add, alt, no3, no5. |
| CS5 | `N.C.` (no chord) rendering. |
| CS6 | Chord symbols can have rhythm; they change at user-specified points, not necessarily every beat. |

### 6.10 Lyrics

| ID | Requirement |
|----|-------------|
| L1 | Lyrics placed below the staff, one syllable per note. |
| L2 | Syllable hyphens centered between syllables: `A - ma - zing`. |
| L3 | Melisma extender line drawn under notes where a syllable is held. |
| L4 | Multiple verses stacked vertically below the staff. |
| L5 | Lyrics alignment tracks the notehead position for correct visual association. |
| L6 | Skip markers (`_`) to leave a note without a lyric syllable. |
| L7 | Elision (two syllables under one note) using `~` between words. |

### 6.11 Ties and Slurs

| ID | Requirement |
|----|-------------|
| TS1 | Ties connect two notes of the same pitch: `c'4~ c'4` renders as a tie. |
| TS2 | Slurs encompass a phrase: `c'4( d' e' f')` renders a slur from first to last note. |
| TS3 | Ties and slurs rendered as smooth Bézier curves using CeTZ. |
| TS4 | Correct placement: ties near notehead level, slurs arching over/under the phrase. |
| TS5 | Ties and slurs that cross system breaks rendered as two partial arcs. |
| TS6 | Phrasing slurs `\(` and `\)` for outer phrasing vs inner slurs. |

### 6.12 Articulations and Ornaments

| ID | Requirement |
|----|-------------|
| AR1 | **Staccato**: dot above/below note. Syntax: `c'4.stac` or `c'4-. ` |
| AR2 | **Accent**: `>` or `-.>` |
| AR3 | **Tenuto**: `--` or `-.tenuto` |
| AR4 | **Marcato**: `^` |
| AR5 | **Staccatissimo**: `-.!` |
| AR6 | **Fermata**: `\fermata` placed above note. |
| AR7 | **Breath mark**: `\breathe` placed between notes. |
| AR8 | **Trill**: `\trill` above note, optionally with continuation wavy line. |
| AR9 | **Mordent** / **Turn** / **Inverted mordent**: `\mordent`, `\turn`, `\prall` |
| AR10 | **Bow markings**: up-bow `\upbow`, down-bow `\downbow`. |
| AR11 | **Pizzicato** / **Snap pizzicato** / **Harmonic**: `\pizz`, `\snap`, `\flageolet`. |
| AR12 | Articulations auto-placed on the correct side (above for stem-down, below for stem-up) with user override. |

### 6.13 Dynamics

| ID | Requirement |
|----|-------------|
| D1 | Standard dynamics: `ppp`, `pp`, `p`, `mp`, `mf`, `f`, `ff`, `fff`, `sfz`, `fp`, etc. |
| D2 | Rendered in italic bold music-notation font below the staff. |
| D3 | Hairpin crescendo/decrescendo: `\<` begins crescendo, `\>` begins decrescendo, `\!` terminates. |
| D4 | Text dynamics: `cresc.`, `dim.`, rendered in italic. |
| D5 | Dynamics vertically aligned across staves when feasible. |

### 6.14 Tempo and Rehearsal Marks

| ID | Requirement |
|----|-------------|
| TM1 | Metronome marks: `♩ = 120` format. Syntax: `\tempo "Allegro" 4=120`. |
| TM2 | Tempo text: `\tempo "Andante"`. |
| TM3 | Rehearsal marks: `\mark "A"` renders boxed or circled letter above staff. |
| TM4 | Automatic rehearsal mark sequencing: `\mark \default` auto-increments A, B, C... |
| TM5 | Segno and Coda symbols: `\segno`, `\coda`. |

### 6.15 Repeats and Endings

| ID | Requirement |
|----|-------------|
| R1 | Repeat barlines (`:||:` style). Syntax: `\repeat volta 2 { ... }` or inline `\|:` and `:\|` markers. |
| R2 | First and second endings (volta brackets): `\alternative { { ... } { ... } }` or `\1.` / `\2.` markers. |
| R3 | D.C. (Da Capo), D.S. (Dal Segno), "al Fine", "al Coda" text markings. |
| R4 | Percent repeat (single measure repeat `%`). |
| R5 | Simile marks (two-measure repeat). |

### 6.16 Grace Notes

| ID | Requirement |
|----|-------------|
| GR1 | **Acciaccatura** (slashed grace note): `\grace f#'8` or `\acciaccatura { f#'16 }`. |
| GR2 | **Appoggiatura** (unslashed grace note): `\appoggiatura { d'8 }`. |
| GR3 | Grace notes rendered at reduced size (~65%) with a slur to the main note. |
| GR4 | Multiple grace notes (grace note runs) supported. |
| GR5 | Grace notes do not consume metric time. |

### 6.17 Tuplets

| ID | Requirement |
|----|-------------|
| TU1 | Triplets and other tuplets: `\tuplet 3/2 { c'8 d' e' }` = three eighth notes in the space of two. |
| TU2 | Tuplet bracket with number rendered above or below the beam. |
| TU3 | Bracket hidden when all notes in the tuplet are beamed. Configurable. |
| TU4 | Nested tuplets supported. |

### 6.18 Mid-Staff Changes

| ID | Requirement |
|----|-------------|
| MC1 | Clef changes mid-measure: `\clef bass` inserts a small clef. |
| MC2 | Key changes mid-piece: `\key A \major` inserts new key signature (with cancellation naturals). |
| MC3 | Time signature changes mid-piece: `\time 3/4` inserts new time signature. |
| MC4 | All changes rendered correctly and propagated to subsequent systems. |

### 6.19 Transposition

| ID | Requirement |
|----|-------------|
| TR1 | API-level transposition function that shifts all pitches in a staff by a given interval. |
| TR2 | Useful for creating transposing parts (e.g., Bb clarinet from concert pitch). |

### 6.20 Multi-Voice Writing

| ID | Requirement |
|----|-------------|
| V1 | Support two or more independent voices on a single staff. |
| V2 | Voice 1 stems up, Voice 2 stems down (standard convention). |
| V3 | Syntax: `\voice1 { c'4 d' e' f' } \voice2 { a2 g }` or simultaneous delimiter `<< { upper } \\\\ { lower } >>`. |
| V4 | Rest positioning adjusted per voice to avoid collisions. |

### 6.21 Title and Header Block

| ID | Requirement |
|----|-------------|
| H1 | Optional header rendering above the first system with title, subtitle, composer, arranger, lyricist, and other fields. |
| H2 | Rendered using Typst text (not CeTZ) so it inherits document styling. |
| H3 | Copyright/footer text at bottom of first page. |

---

## 7. Rendering Engine Design

### Glyph Rendering Strategy

All musical symbols (noteheads, rests, clefs, accidentals, flags, dynamics, etc.) are rendered by placing characters from the **Bravura** (SMuFL) font at precise coordinates on the CeTZ canvas. This approach is key because:

1. **Scalable vectors** — font glyphs are outlines, not bitmaps.
2. **Consistent style** — all symbols come from a professionally designed, standardized music font.
3. **Precise metrics** — SMuFL metadata provides exact bounding boxes, stem attachment points, and anchors.

### Line/Curve Rendering

Staff lines, barlines, stems, beams, ties, slurs, hairpins, brackets, and braces are drawn as CeTZ primitives:

| Element | CeTZ Primitive |
|---------|---------------|
| Staff lines | `cetz.draw.line()` (5 horizontal lines) |
| Barlines | `cetz.draw.line()` (vertical) |
| Stems | `cetz.draw.line()` (vertical, attached to notehead) |
| Beams | `cetz.draw.rect()` or filled `cetz.draw.line()` with thickness |
| Ties & Slurs | `cetz.draw.bezier()` / cubic curves |
| Hairpins | Two `cetz.draw.line()` segments forming a wedge |
| Braces | Bravura brace glyph, scaled to system height |
| Brackets | `cetz.draw.line()` with serifs |
| Ledger lines | Short `cetz.draw.line()` segments |

### Coordinate System

- Origin `(0, 0)` is the **top-left** of the first staff's top line.
- X increases to the right (horizontal time axis).
- Y increases downward (matching Typst's and CeTZ's default).
- 1 **staff space** (distance between adjacent staff lines) is the fundamental vertical unit. Default: `1.75mm` (configurable).
- Horizontal spacing is measured in abstract "time units" that the layout engine converts to physical distances.

---

## 8. Layout Algorithm

### Horizontal Spacing

The layout engine uses a **proportional spacing** model modified by optical adjustments:

1. **Assign duration-based widths.** Each note/rest gets a base width proportional to `log2(duration)`. A quarter note gets width `w`, a half note gets `~1.5w`, a whole note `~2w`, an eighth note `~0.75w`. This logarithmic scaling matches established engraving practice.

2. **Add padding for pre-note elements.** Accidentals, dots, and grace notes add width before/after the notehead.

3. **Compute column positions.** Each "time column" (a unique rhythmic position across all simultaneous staves) receives a horizontal position equal to the maximum width needed by any staff at that column.

4. **Stretch or compress to fill the line.** After placing all columns in a system, distribute remaining horizontal space proportionally, or break to a new system if the content exceeds the available width.

### Vertical Beat Alignment

All staves in a system share the same set of time columns. The layout engine:

1. Collects all unique onset times across all staves.
2. Merges them into a single timeline.
3. Assigns each onset the x-position determined by step 3 above.
4. Each staff looks up its events at each time column and draws them at that x-position.

This guarantees that beats line up vertically, just like MuseScore and LilyPond.

### System Breaking (Line Breaking)

The system-break algorithm mirrors Typst's paragraph line-breaking approach:

1. Attempt to fit as many measures as possible on one line.
2. If the total width of the next measure would exceed the available width minus a tolerance, break to a new system.
3. Optionally, the user can force system breaks with `\break`.
4. At each new system, redraw initial clef, key signature, and (optionally) time signature.

### Page Breaking

Systems are emitted as Typst `block()` elements. Typst's native page-break algorithm handles page flow. The library avoids orphaned single-measure systems and keeps multi-staff systems together using `block(breakable: false)` for each system.

---

## 9. Music Font Strategy

### Bravura Font

- **Source:** [steinbergmedia/bravura](https://github.com/steinbergmedia/bravura) (or equivalent mirror).
- **License:** SIL Open Font License 1.1 — fully compatible with distribution in a Typst package.
- **Bundled files:**
  - `Bravura.otf` — the font file itself.
  - `bravura_metadata.json` — glyph metrics, anchors, bounding boxes.
  - `glyphnames.json` — SMuFL canonical glyph names → Unicode PUA codepoints.

### Glyph Access in Typst

Typst supports loading custom fonts bundled with a package. On the CeTZ canvas, we place text nodes using the Bravura font at specific Unicode codepoints (from SMuFL's Private Use Area, U+E000–U+F8FF).

Example of rendering a treble clef:
```typst
// Pseudocode within CeTZ canvas
cetz.draw.content(
  (x, y),
  text(font: "Bravura", size: staff-height)[#str.from-unicode(0xE050)]
)
```

Where `0xE050` is the SMuFL codepoint for `gClef`.

### Fallback Strategy

If Bravura is unavailable (unlikely since it's bundled), the library falls back to SVG asset files (similar to the `staves` package approach). However, the font-based approach is strongly preferred for simplicity and quality.

---

## 10. Project Structure

```
sheet-music/
├── typst.toml                     # Package manifest
├── LICENSE                        # MIT or Apache-2.0
├── README.md                      # Usage documentation
├── lib.typ                        # Main entry point — exports public API
│
├── src/
│   ├── parser.typ                 # Music string parser → event list
│   ├── parser-lyrics.typ          # Lyrics string parser
│   ├── parser-chords.typ          # Chord symbol string parser
│   ├── layout.typ                 # Horizontal/vertical layout engine
│   ├── layout-spacing.typ         # Duration → width calculations
│   ├── layout-beaming.typ         # Automatic beaming algorithm
│   ├── layout-breaks.typ          # System/line break algorithm
│   ├── renderer.typ               # Main CeTZ rendering orchestrator
│   ├── render-staff.typ           # Staff lines, barlines, braces/brackets
│   ├── render-notes.typ           # Noteheads, stems, flags, dots, ledger lines
│   ├── render-accidentals.typ     # Accidental placement logic
│   ├── render-beams.typ           # Beam drawing
│   ├── render-slurs-ties.typ      # Slur and tie curve rendering
│   ├── render-text.typ            # Lyrics, chord symbols, dynamics, tempo
│   ├── render-articulations.typ   # Articulation/ornament placement
│   ├── render-clef-key-time.typ   # Clef, key signature, time signature drawing
│   ├── render-repeats.typ         # Repeat barlines, volta brackets
│   ├── render-header.typ          # Title/composer header block
│   ├── model.typ                  # Data structures (event types, note, rest, chord, etc.)
│   ├── pitch.typ                  # Pitch arithmetic, transposition, interval calculation
│   ├── constants.typ              # SMuFL codepoints, key sig data, clef offsets
│   └── utils.typ                  # Shared utility functions
│
├── fonts/
│   └── Bravura.otf               # Bundled SMuFL font
│
├── data/
│   ├── bravura_metadata.json      # SMuFL glyph metrics
│   └── glyphnames.json            # SMuFL glyph names → codepoints
│
├── tests/
│   ├── test-parser.typ            # Parser unit tests
│   ├── test-layout.typ            # Layout algorithm tests
│   ├── test-render-basic.typ      # Visual regression: single staff, basic notes
│   ├── test-render-grand.typ      # Visual regression: grand staff
│   ├── test-render-satb.typ       # Visual regression: SATB choir
│   ├── test-render-chords.typ     # Chord symbol rendering
│   ├── test-render-lyrics.typ     # Lyrics rendering
│   ├── test-render-articulations.typ
│   ├── test-render-dynamics.typ
│   ├── test-render-repeats.typ
│   └── test-render-grace.typ
│
└── examples/
    ├── ode-to-joy.typ             # Simple melody
    ├── piano-prelude.typ          # Grand staff with two voices
    ├── amazing-grace-satb.typ     # SATB choir with lyrics
    ├── lead-sheet.typ             # Melody + chords + lyrics
    └── complex-orchestral.typ     # Multi-instrument score excerpt
```

---

## 11. API Surface

### Primary Function: `score()`

```typst
#let score(
  // --- Music Content ---
  staves: (),              // Array of staff dictionaries (see below)
  lyrics: (),              // Array of lyric dictionaries
  chords: (),              // Array of chord symbol dictionaries

  // --- Initial State ---
  key: "C",               // Key signature: "C", "G", "Bb", "f#" (lowercase = minor)
  time: "4/4",            // Time signature
  tempo: none,            // Tempo marking: "Allegro" or (text: "Allegro", bpm: 120, beat: 4)

  // --- Header ---
  title: none,            // Piece title
  subtitle: none,         // Subtitle
  composer: none,         // Composer name
  arranger: none,         // Arranger name
  lyricist: none,         // Lyricist name
  copyright: none,        // Copyright text (bottom of first page)

  // --- Layout ---
  staff-group: "none",    // "none", "grand", "choir", "orchestra", or custom grouping
  staff-size: 1.75mm,     // Staff space (distance between lines)
  system-spacing: 12mm,   // Vertical space between systems
  staff-spacing: 8mm,     // Vertical space between staves within a system
  width: auto,            // Explicit width or auto (fills available space)
  measure-numbers: "system", // "system" (start of each line), "every", "none"
  beam-exceptions: auto,  // Override beaming rules

  // --- Behavior ---
  relative-octave: false, // If true, use LilyPond-style relative octave entry
) = { ... }
```

### Staff Dictionary Schema

```typst
(
  clef: "treble",         // "treble", "bass", "alto", "tenor", "treble-8", "treble+8", "percussion"
  music: "c'4 d' e' f' | g'2 g' | ...",  // Music string
  label: none,            // Staff label: "Violin I" or (full: "Violin I", abbr: "Vln. I")
  voice: none,            // For multi-voice: "auto" or specific voice assignments
)
```

### Convenience Functions

```typst
// Quick single-staff melody
#melody(key: "G", time: "3/4", clef: "treble", music: "...")

// Lead sheet (melody + chords + lyrics on one staff)
#lead-sheet(
  key: "C", time: "4/4",
  music: "...",
  chords: "G | Am | F | C",
  lyrics: "Some -- where o -- ver the rain -- bow",
)

// Render just a chord symbol chart (Nashville-number style or with rhythms)
#chord-chart(
  key: "G", time: "4/4",
  chords: "G | G | C | C | D | D | G | G",
)
```

---

## 12. Error Handling

| Scenario | Behavior |
|----------|----------|
| Unknown note name | Typst `panic()` with descriptive message: `"sheet-music: unknown pitch 'x' at position 14 in music string"` |
| Invalid duration | Panic with message showing the invalid token. |
| Unmatched slur/tie | Warning (if possible via Typst's warning mechanism) or rendering without the slur, plus a console message. |
| Missing closing barline | Auto-insert final barline at end of music. |
| Staves with different total durations | Pad shorter staves with invisible rests to align with the longest staff. Issue a warning. |
| Unknown chord symbol | Render the raw text as-is with a warning. |
| Invalid key/time signature string | Panic with descriptive message. |

---

## 13. Testing Strategy

### Unit Tests

- **Parser tests:** Verify that music strings, lyric strings, and chord strings are correctly tokenized and converted to the internal model.
- **Pitch tests:** Test transposition, interval computation, enharmonic equivalences.
- **Layout tests:** Verify column assignment, spacing calculation, beaming group detection.

### Visual Regression Tests

- Render known scores and compare the output visually.
- Use Typst's deterministic rendering to generate PDF/PNG snapshots.
- Store reference images in `tests/ref/` and use a simple diff-based workflow.
- Can use tytanic (Typst test runner) for automated visual tests.

### Example-Based Tests

- Each file in `examples/` serves as an integration test — compile and visually inspect.

---

## 14. Performance Considerations

| Concern | Mitigation |
|---------|------------|
| Large scores (100+ measures) | The layout engine operates measure-by-measure, not globally. System breaks are decided greedily to avoid combinatorial explosion. |
| Many staves (10+) | Each staff is rendered independently within the shared column grid. Rendering is linear in the number of staves. |
| Font loading | Bravura is loaded once via Typst's `#set text(font: "Bravura")` within the CeTZ canvas scope. No repeated I/O. |
| JSON metadata parsing | `bravura_metadata.json` is loaded once at import time using `json()` and stored in a module-level variable. |
| CeTZ canvas size | Each system is its own canvas to avoid one massive canvas for the entire piece. |

---

## 15. Packaging and Distribution

### `typst.toml` Manifest

```toml
[package]
name = "sheet-music"
version = "0.1.0"
entrypoint = "lib.typ"
authors = ["Your Name"]
license = "MIT"
description = "Render professional sheet music directly in Typst documents."
repository = "https://github.com/your-username/typst-sheet-music"
keywords = ["music", "notation", "sheet-music", "score"]
categories = ["visualization", "components"]
disciplines = ["music"]
compiler = "0.14.0"

[template]
path = "examples"
entrypoint = "examples/ode-to-joy.typ"
```

### Dependencies

```toml
[dependencies]
cetz = "0.5.0"
```

### Font Bundling

Fonts placed in the `fonts/` directory of a Typst package are automatically available to documents using the package. The Bravura OTF file must be included here.

---

## 16. Phased Implementation Roadmap

### Phase 1 — Core Foundation (MVP)

**Goal:** Render a single staff with basic notes, rests, barlines, clef, key signature, and time signature.

- [ ] Project scaffolding, `typst.toml`, dependency setup
- [ ] Bundle Bravura font and load SMuFL metadata
- [ ] Music string parser (pitches, durations, barlines, rests)
- [ ] Internal model (`note`, `rest`, `barline`, `clef`, `key-sig`, `time-sig`)
- [ ] Basic horizontal spacing (fixed proportional)
- [ ] CeTZ renderer: staff lines, clef, key signature, time signature
- [ ] CeTZ renderer: noteheads, stems, flags, dots, ledger lines
- [ ] CeTZ renderer: barlines (single, double, final)
- [ ] Accidentals (sharps, flats, naturals)
- [ ] Basic `score()` and `melody()` API

**Deliverable:** Render "Ode to Joy" as a single-staff melody.

### Phase 2 — Multi-Staff and Alignment

**Goal:** Grand staff, SATB, vertical beat alignment, system breaks.

- [ ] Multi-staff systems with shared time grid
- [ ] Vertical beat alignment across staves
- [ ] Grand staff grouping (brace, connected barlines)
- [ ] Choir grouping (bracket)
- [ ] System breaking (line wrapping)
- [ ] Clef/key/time signature re-rendering at new system start
- [ ] Staff labels
- [ ] Measure numbers

**Deliverable:** Render a piano grand staff and SATB chorale.

### Phase 3 — Beaming, Ties, Slurs

**Goal:** Automatic beaming, ties, slurs, and polished rhythmic notation.

- [ ] Automatic beaming algorithm (per time signature)
- [ ] Manual beam override (`[`, `]`)
- [ ] Tie rendering (Bézier curves)
- [ ] Slur rendering (Bézier curves)
- [ ] Beam slope calculation
- [ ] Cross-system ties and slurs

**Deliverable:** Render beamed passages with slurs and ties.

### Phase 4 — Lyrics and Chord Symbols

**Goal:** Lead sheet capability.

- [ ] Lyrics parser (syllables, hyphens, melisma, skips)
- [ ] Lyrics renderer (placement below staff, hyphen lines, extender lines)
- [ ] Multiple lyric verses
- [ ] Chord symbol parser
- [ ] Chord symbol renderer (placement above staff)
- [ ] `lead-sheet()` convenience function

**Deliverable:** Render "Amazing Grace" lead sheet with melody, chords, and lyrics.

### Phase 5 — Articulations, Dynamics, and Expression

**Goal:** Full expression markings.

- [ ] Articulation parser and placement engine
- [ ] Staccato, accent, tenuto, marcato, staccatissimo
- [ ] Fermata and breath marks
- [ ] Dynamics (textual: p, f, mf, etc.)
- [ ] Hairpin crescendo/decrescendo
- [ ] Tempo markings
- [ ] Rehearsal marks

**Deliverable:** Expressive score rendering with all standard markings.

### Phase 6 — Advanced Notation

**Goal:** Grace notes, tuplets, repeats, ornaments, multi-voice.

- [ ] Grace notes (acciaccatura, appoggiatura)
- [ ] Tuplets (triplets and arbitrary)
- [ ] Repeat barlines and volta brackets
- [ ] D.C. / D.S. / Coda / Segno markings
- [ ] Ornaments (trill, mordent, turn)
- [ ] Multi-voice on a single staff
- [ ] Harmonic chords (angle bracket notation)
- [ ] Transposition function

**Deliverable:** Full-featured notation covering all requirements in this document.

### Phase 7 — Polish and Publishing

**Goal:** Publication readiness.

- [ ] Comprehensive documentation (README with examples)
- [ ] Complete test suite
- [ ] Performance optimization for large scores
- [ ] Advanced spacing refinements (optical adjustments)
- [ ] Edge case handling (extreme ledger lines, dense chords, etc.)
- [ ] Submit to Typst Universe (`@preview/sheet-music`)

---

## Appendix A — Full Syntax Reference

### Music String Grammar (Informal)

```
music       = (event | barline | command)*
event       = (note | rest | chord | spacer) duration? articulation*
note        = pitch octave_mark* duration? dot* articulation*
pitch       = [a-g] accidental?
accidental  = '#' | '##' | '&' | '&&' | '='
octave_mark = '\'' | ','
duration    = '1' | '2' | '4' | '8' | '16' | '32' | '64'
dot         = '.'
rest        = 'r' duration? dot*
spacer      = 's' duration? dot*
chord       = '<' note+ '>' duration? dot* articulation*
barline     = '|' | '||' | '|.' | '.|:' | ':|.' | ':|.||'
command     = '\\' identifier ('{' music '}' | argument*)

articulation = '-.' | '->' | '--' | '-^' | '-!' | '~' | '(' | ')'
tie          = '~'
slur_start   = '('
slur_end     = ')'
beam_start   = '['
beam_end     = ']'

// Commands
\\clef identifier                  // e.g., \clef bass
\\key pitch ('\\major' | '\\minor') // e.g., \key D \major
\\time string                      // e.g., \time 6/8
\\tempo string? beat=bpm?          // e.g., \tempo "Allegro" 4=120
\\mark string | \\mark \\default
\\fermata
\\breathe
\\trill
\\mordent \\turn \\prall
\\grace { music }
\\acciaccatura { music }
\\appoggiatura { music }
\\tuplet fraction { music }        // e.g., \tuplet 3/2 { c8 d e }
\\repeat volta n { music }
\\alternative { { music } { music } }
\\voice1 { music }
\\voice2 { music }
\\break                            // Force system break
\\< \\> \\!                         // Hairpin dynamics
\\p \\f \\mf \\mp \\pp \\ff etc.    // Dynamic marks
```

### Lyrics String Grammar

```
lyrics     = (syllable | hyphen | extender | skip)*
syllable   = word_characters+
hyphen     = ' -- '
extender   = '__'
skip       = '_'
elision    = '~'           // joins two syllables under one note
```

### Chord Symbol String Grammar

```
chords     = (chord_sym | barline | rest_marker)*
chord_sym  = root quality? extension* slash?
root       = [A-G] ('#' | 'b')?
quality    = 'maj' | 'min' | 'm' | 'dim' | 'aug' | '+' | 'o' | 'sus2' | 'sus4'
extension  = '7' | '9' | '11' | '13' | 'add9' | 'add11' | 'no3' | 'no5'  | 'alt'
slash      = '/' root
barline    = '|'
rest_marker= 'N.C.'
```

---

## Appendix B — SMuFL Glyph Subset

Below are the key SMuFL codepoints (from the Bravura font) that the library must use. Full list in `data/glyphnames.json`.

### Clefs

| Glyph | Codepoint | SMuFL Name |
|-------|-----------|------------|
| 𝄞 Treble | U+E050 | `gClef` |
| 𝄢 Bass | U+E062 | `fClef` |
| 𝄡 Alto/Tenor | U+E05C | `cClef` |
| Treble 8vb | U+E052 | `gClef8vb` |
| Percussion | U+E069 | `unpitchedPercussionClef1` |

### Noteheads

| Glyph | Codepoint | SMuFL Name |
|-------|-----------|------------|
| Whole | U+E0A2 | `noteheadWhole` |
| Half | U+E0A3 | `noteheadHalf` |
| Quarter/filled | U+E0A4 | `noteheadBlack` |

### Flags

| Glyph | Codepoint | SMuFL Name |
|-------|-----------|------------|
| 8th up | U+E240 | `flag8thUp` |
| 8th down | U+E241 | `flag8thDown` |
| 16th up | U+E242 | `flag16thUp` |
| 16th down | U+E243 | `flag16thDown` |
| 32nd up | U+E244 | `flag32ndUp` |
| 32nd down | U+E245 | `flag32ndDown` |

### Rests

| Glyph | Codepoint | SMuFL Name |
|-------|-----------|------------|
| Whole rest | U+E4E3 | `restWhole` |
| Half rest | U+E4E4 | `restHalf` |
| Quarter rest | U+E4E5 | `restQuarter` |
| 8th rest | U+E4E6 | `rest8th` |
| 16th rest | U+E4E7 | `rest16th` |
| 32nd rest | U+E4E8 | `rest32nd` |

### Accidentals

| Glyph | Codepoint | SMuFL Name |
|-------|-----------|------------|
| Sharp ♯ | U+E262 | `accidentalSharp` |
| Flat ♭ | U+E260 | `accidentalFlat` |
| Natural ♮ | U+E261 | `accidentalNatural` |
| Double sharp 𝄪 | U+E263 | `accidentalDoubleSharp` |
| Double flat 𝄫 | U+E264 | `accidentalDoubleFlat` |

### Time Signatures

| Glyph | Codepoint | SMuFL Name |
|-------|-----------|------------|
| Digits 0-9 | U+E080–U+E089 | `timeSig0`–`timeSig9` |
| Common time | U+E08A | `timeSigCommon` |
| Cut time | U+E08B | `timeSigCutCommon` |

### Dynamics

| Glyph | Codepoint | SMuFL Name |
|-------|-----------|------------|
| p | U+E520 | `dynamicPiano` |
| m | U+E521 | `dynamicMezzo` |
| f | U+E522 | `dynamicForte` |
| s | U+E524 | `dynamicSforzando` |
| z | U+E525 | `dynamicZ` |
| r | U+E523 | `dynamicRinforzando` |

### Articulations

| Glyph | Codepoint | SMuFL Name |
|-------|-----------|------------|
| Staccato | U+E4A2 | `articStaccatoAbove` |
| Accent | U+E4A0 | `articAccentAbove` |
| Tenuto | U+E4A4 | `articTenutoAbove` |
| Marcato | U+E4AC | `articMarcatoAbove` |
| Fermata | U+E4C0 | `fermataAbove` |

### Ornaments

| Glyph | Codepoint | SMuFL Name |
|-------|-----------|------------|
| Trill | U+E566 | `ornamentTrill` |
| Mordent | U+E56C | `ornamentMordent` |
| Turn | U+E567 | `ornamentTurn` |

### Other Symbols

| Glyph | Codepoint | SMuFL Name |
|-------|-----------|------------|
| Breath mark | U+E4CE | `breathMarkComma` |
| Segno | U+E047 | `segno` |
| Coda | U+E048 | `coda` |
| Repeat dot | U+E044 | `repeatDot` |
| Brace | U+E000 | `brace` |
| Bracket | U+E002 | `bracket` |

---

*End of Requirements Document*
