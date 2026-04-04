#import "../lib.typ": score

#set page(margin: 1.5cm)

// Demonstrate alto clef with sharps in different time signatures
#score(
  title: "Alto Clef Examples",
  composer: "Demonstrating Alt/Tenor Clefs",
  staff-group: "grand",
  staves: (
    (
      clef: "alto",
      music: "
        c4 d e f | g a b c' |
        c b, a, g, | f, e, d, c,
      ",
    ),
    (
      clef: "tenor",
      music: "
        c4 d e f | g a b c' |
        c b, a, g, | f, e, d, c,
      ",
    ),
  ),
  key: "Cb",
  time: "C",
)

// Test cut time in tenor clef
#score(
  title: "Cut Time in Tenor Clef",
  staves: (
    (
      clef: "tenor",
      music: "c4 d e f | g a b c'",
    ),
  ),
  key: "C#",
  time: "cut",
)

// Multi-staff example with different clefs
#score(
  title: "Grand Staff with Alto/Tenor",
  staff-group: "grand",
  staves: (
    (
      clef: "alto",
      music: "e4 f g a | b2 c'2",
    ),
    (
      clef: "tenor",
      music: "g,4 a, b, c | d2 e2",
    ),
  ),
  key: "Bb",
  time: "4/4",
)

// Multi-staff example with different clefs.
#score(
  title: "Grand Staff with Treble / Treble-8b",
  staff-group: "grand",
  staves: (
    (
      clef: "treble",
      music: "e4 f g a | b2 c'2",
    ),
    (
      clef: "treble-8b",
      music: "g4 a b c' | d'2 e'2",
    ),
  ),
  key: "Bb",
  time: "4/4",
)
