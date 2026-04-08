#import "../lib.typ": melody

#set page(margin: 1.5cm)

#set text(font: "Libertinus Serif")

#show heading.where(level: 1): it => block(above: 0pt, below: 6pt, text(16pt, weight: "bold", it.body))
#show heading.where(level: 2): it => block(above: 0pt, below: 4pt, text(11pt, style: "italic", it.body))

= Trills

== Standalone trill symbols and trill lines

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music: "c4tr d e f | tr{g4} a b c' | tr{d'4 e' f' g' |
  a' b' c'' d''} e''1",
  width: 145mm,
)
