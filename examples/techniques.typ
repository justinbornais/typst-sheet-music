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
      c16n[1] en[2] gn[3] c'n[1] e' g' c''n[5] g' e' c'n[1] gn[3] e c4 | e16n[1] gn[2] c'n[4] e'n[1] g' c'' e''n[5] c'' g' e' c'n[4] g e4 | g16n[1] c'n[2] e'n[4] g'n[1] c'' e'' g''n[5] e'' c'' g' e'n[4] c' g4
      ",
    ),
    (
      clef: "bass",
      music: "
      c16n[1] d e fn[1] g a b c'n[1] treble d e fn[1] g a b c'n[5] b a g f en[3] bass d' c' bn[4] a g f en[3] d c4
      <c e g>4n_[5 3 1] <e g c'>n_[5 3 1] <g c' e'>n_[5 2 1] <c' e' g'>n_[5 3 1] <g c' e'>n_[5 2 1] <e g c'>n_[5 3 1] <c e g>1n_[5 3 1]
      c16 e g c' treble e g c' g bass e' c' g e c4 | e16 g c' e' treble g c' e' c' bass g' e' c' g e4 | g16 c' e' g' treble c' e' g' e' c' g e c bass g4
      "
    )
  ),
  key: "C",
  time: "4/4",
)