#import "../lib.typ": score

#set page(width: 210mm, height: 120mm, margin: 12mm)

= Grace Notes

#text(style: "italic")[Appoggiaturas and acciaccaturas with tight spacing]

#v(6mm)

#score(
  key: "C",
  time: "4/4",
  staff-group: "grand",
  staves: (
    (
      clef: "treble",
      music: "grace{c16 d e} f4 g a b | grace{f#16 g a/} b4 c' d' e' |
      grace{d16 e f} g4 a b c' | grace{c''16 b' a' g'/} f'1",
    ),
    (
      clef: "bass",
      music: "c4 g, c g, | c g, c g, |
      c g treble c g | c g, bass c2"
    )
  ),
  width: 175mm,
)
