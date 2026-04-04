// Octave-line rendering test

#import "../lib.typ": melody

#set page(margin: 1.5cm)

= Octave Line Tests

== 8va above (continuation across systems)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music: "c4 d e f g a b c' 8a{d e f g |
  a b c' d'} e f g a b c' d' e' f' g' a' b' c''",
)

#v(1cm)

== 15mb below

#melody(
  key: "C",
  time: "4/4",
  clef: "bass",
  music: "c,4 d e f g a b c 15b{c, d e f g a b c} d e f g a b c",
)
