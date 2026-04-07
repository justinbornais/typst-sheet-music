#import "../lib.typ": score

#set page(margin: 1.5cm)

#score(
  title: "Inline Clef Changes",
  subtitle: "Clef switches embedded directly in the music string",
  key: "C",
  time: "8/4",
  staves: (
    (
      clef: "bass",
      music: "g4 a b c' treble d e f g | a g f e d c b a |
      g2 c bass g c | c8 d e f g a b c' treble c d e f g a b c' |
      c4 e g c f d e c | c e g c bass g g c2",
    ),
    (
      clef: "treble",
      music: "f4 e d c bass b a g. f8 | f4 g a b treble c d e f |
      g,2 c g c | c1 c bass |
      c4 e g c treble f f g c bass | c2 c g c",
    ),
  ),
  staff-group: "grand",
)

#v(12mm)

#score(
  title: "Inline Time Signature Changes",
  subtitle: "Mid-measure, post-barline, and line-end time signatures",
  key: "C",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "c4 e g c' 3/4 g4 g c | 2/4 d4 e |
      5/4 f4 g a b c' | c'4 3/4 b a g",
    ),
    (
      clef: "bass",
      music: "c,4 g, c e 3/4 g,4 c e | 2/4 f4 g |
      5/4 c,4 d, e, f, g, | c,4 3/4 g, c e",
    ),
  ),
  staff-group: "grand",
  measures-per-line: 2,
)
