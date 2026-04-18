// Multi-font rendering test
// Renders the same music with Bravura, Leipzig, Leland, Petaluma,
// Sebastian, Finale Ash, Finale Broadway, Finale Engraver,
// Finale Jazz, Finale Legacy, and Finale Maestro
// to verify spacing, stem attachment, and overall calibration.

#import "../lib.typ": score, melody

#set page(width: 210mm, height: 297mm, margin: 1.5cm)
#set text(size: 10pt)

= Multi-Font Rendering Test

// ── Bravura (default, bundled) ────────────────────────────

== Bravura

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)

// ── Leipzig ───────────────────────────────────────────────

== Leipzig

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  music-font: "Leipzig",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music-font: "Leipzig",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music-font: "Leipzig",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)

// ── Leland ────────────────────────────────────────────────

== Leland

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  music-font: "Leland",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music-font: "Leland",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music-font: "Leland",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)

// ── Petaluma ──────────────────────────────────────────────

== Petaluma

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  music-font: "Petaluma",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music-font: "Petaluma",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music-font: "Petaluma",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)

// ── Sebastian ─────────────────────────────────────────────

== Sebastian

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  music-font: "Sebastian",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music-font: "Sebastian",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music-font: "Sebastian",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)

// ── Finale Ash ────────────────────────────────────────────

== Finale Ash

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  music-font: "Finale Ash",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music-font: "Finale Ash",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music-font: "Finale Ash",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)

// ── Finale Broadway ───────────────────────────────────────

== Finale Broadway

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  music-font: "Finale Broadway",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music-font: "Finale Broadway",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music-font: "Finale Broadway",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)

// ── Finale Engraver ───────────────────────────────────────

== Finale Engraver

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  music-font: "Finale Engraver",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music-font: "Finale Engraver",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music-font: "Finale Engraver",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)

// ── Finale Jazz ───────────────────────────────────────────

== Finale Jazz

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  music-font: "Finale Jazz",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music-font: "Finale Jazz",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music-font: "Finale Jazz",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)

// ── Finale Legacy ─────────────────────────────────────────

== Finale Legacy

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  music-font: "Finale Legacy",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music-font: "Finale Legacy",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music-font: "Finale Legacy",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)

// ── Finale Maestro ────────────────────────────────────────

== Finale Maestro

#score(
  title: "Ode to Joy (excerpt)",
  key: "D",
  time: "4/4",
  music-font: "Finale Maestro",
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a g f# e | d d e f# | f#4. e8 e2 ||
      ",
    ),
  ),
)

#v(4mm)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music-font: "Finale Maestro",
  music: "c8 d e f g a b c' | c'4 b8 a g4 f8 e | d4. e8 f4 g | c1 ||",
)

#v(4mm)

#melody(
  key: "F",
  time: "3/4",
  clef: "bass",
  music-font: "Finale Maestro",
  music: "c,4 f, a, | c e g | f4. e8 d4 | c,2. ||",
)
