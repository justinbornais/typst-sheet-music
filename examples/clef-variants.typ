// Examples: Clef variants (8va/8vb and 15ma/15mb)
#import "../lib.typ": score, melody

#set page(margin: 1.5cm)

// Single-staff melody examples for each clef variant
#score(
  title: "Clef Variants — Single Staves",
  staves: (
    (clef: "treble", music: "g4 a b c' | d' e' f#' g'"),
    (clef: "treble-8a", music: "g4 a b c' | d' e' f#' g'"),
    (clef: "treble-8b", music: "g4 a b c' | d' e' f#' g'"),
    (clef: "treble-15a", music: "g4 a b c' | d' e' f#' g'"),
    (clef: "treble-15b", music: "g4 a b c' | d' e' f#' g'"),
    (clef: "alto", music: "c4 d e f | g a b c'"),
    (clef: "tenor", music: "c4 d e f | g a b c'"),
    (clef: "bass", music: "c,4 d, e, f, | g, a, b, c"),
    (clef: "bass-8a", music: "c,4 d, e, f, | g, a, b, c"),
    (clef: "bass-8b", music: "c,4 d, e, f, | g, a, b, c"),
    (clef: "bass-15a", music: "c,4 d, e, f, | g, a, b, c"),
    (clef: "bass-15b", music: "c,4 d, e, f, | g, a, b, c"),
  ),
  key: "G",
  time: "4/4",
  staff-size: 1.75mm,
  staff-spacing: 10mm
)

// Multi-staff example showing mixed clefs and key signatures
#score(
  title: "Mixed Clefs and Key Signatures",
  staff-group: "grand",
  staves: (
    (clef: "treble-8b", music: "g4 a b c' | d' e' f#' g'"),
    (clef: "bass-8b", music: "c,4 d, e, f, | g, a, b, c"),
  ),
  key: "Bb",
  time: "common",
)

// Demonstrate the 'a' variants (alta) with cut time
#score(
  title: "Alta Variants (8a / 15a)",
  staves: (
    (clef: "treble", music: "c4 d e f | g a b c'"),
    (clef: "treble", music: "c4 d e f | g a b c'"),
    (clef: "bass", music: "c,4 d, e, f, | g, a, b, c"),
    (clef: "bass", music: "c,4 d, e, f, | g, a, b, c"),
  ),
  key: "D",
  time: "cut",
)
