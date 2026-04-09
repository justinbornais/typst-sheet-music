#import "../lib.typ": score

#set page(margin: 1.5cm)

#score(
  title: "Techniques Demonstration",
  composer: "Various Techniques",
  staves: (
    (
      clef: "treble",
      music: "
      c16n[1] d e fn[1] g a b c'n[1] d' e' f'n[1] g' a' b' c''n[5] b' a' g' f' e'n[3] d' c' bn[4] a g f en[3] d c4
      <c e g>4n[1 3 5] <e g c'>n[1 2 5] <g c' e'>n[1 3 5] <c' e' g'>n[1 3 5] <g c' e'>n[1 3 5] <e g c'>n[1 2 5] <c e g>1n[1 3 5]
      ",
    ),
    (
      clef: "bass",
      music: "
      c16n[1] d e fn[1] g a b c'n[1] treble d e fn[1] g a b c'n[5] b a g f en[3] bass d' c' bn[4] a g f en[3] d c4
      <c e g>4n_[5 3 1] <e g c'>n_[5 3 1] <g c' e'>n_[5 2 1] <c' e' g'>n_[5 3 1] <g c' e'>n_[5 2 1] <e g c'>n_[5 3 1] <c e g>1n_[5 3 1]
      "
    )
  ),
  key: "C",
  time: "4/4",
)