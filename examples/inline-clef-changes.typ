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
