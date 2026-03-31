#import "../lib.typ": score

#set page(margin: 1.5cm)

#score(
  title: "Grand Staff Test",
  key: "C",
  time: "4/4",
  staff-group: "grand",
  staff-spacing: 6mm,
  staves: (
    (clef: "treble", music: "e4 f g a | b c' d' e' | e'2 d'2"),
    (clef: "bass",   music: "c4 d e f | g a b c'  | c'2 b2"),
  ),
)
