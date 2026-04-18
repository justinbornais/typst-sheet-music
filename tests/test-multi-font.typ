// Multi-font comprehensive feature test
// Tests: accidentals, dynamics, hairpins, fingerings, chord symbols,
// 16th/32nd notes, all rest values, and octave lines (8va)
// for every supported font: Bravura, Leipzig, Leland, Petaluma,
// Sebastian, Finale Ash, Finale Broadway, Finale Engraver,
// Finale Jazz, Finale Legacy, and Finale Maestro.

#import "../lib.typ": score, melody

#set page(width: 210mm, height: 297mm, margin: 1.5cm)
#set text(size: 10pt)

= Multi-Font Feature Test

// ── Shared test music ─────────────────────────────────────
//
// Feature showcase – 5 measures of 4/4:
//   m1: accidentals (eb, f#, ab) + dynamic (mf) + chord symbols
//   m2: chord notes + fingerings + articulations (accent, staccato) + chord symbol
//   m3: 16th-note run under a crescendo hairpin
//   m4: 32nd-note run under a decrescendo hairpin
//   m5: 8va octave line
#let feature-music = "
  c4v[mf][C] eb4[Bb/D] f#4[G7] ab4[Ab] |
  <c e g>4n[1 3 5][C] e4>n[3] g4*n[5] c'4n[1] |
  cresc{c16 d e f g a b c'} r2 |
  decresc{c32 d e f g a b c'} r2. |
  8a{c'4 d' e' f'} g4 r4 r2 ||
"

// Rest showcase – 6 measures of 4/4, one per rest duration
// (whole · half · quarter · eighth · 16th · 32nd)
#let rest-music = "
  r1 | r2 r2 | r4 r4 r2 | r8 r8 r4 r2 | r16 r16 r8 r4 r2 | r32 r32 r16 r8 r4 r2 ||
"

#let font-test(font-name) = {
  [== #font-name]

  score(
    key: "C",
    time: "4/4",
    music-font: font-name,
    measure-numbers: "none",
    staves: (
      (
        clef: "treble",
        music: feature-music,
      ),
    ),
  )

  v(1mm)

  melody(
    key: "C",
    time: "4/4",
    clef: "treble",
    music-font: font-name,
    measures-per-line: 6,
    music: rest-music,
  )

  v(5mm)
}

#font-test("Bravura")
#font-test("Leipzig")
#font-test("Leland")
#font-test("Petaluma")
#font-test("Sebastian")
#font-test("Finale Ash")
#font-test("Finale Broadway")
#font-test("Finale Engraver")
#font-test("Finale Jazz")
#font-test("Finale Legacy")
#font-test("Finale Maestro")
