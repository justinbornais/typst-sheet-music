// Mary & Hot Cross Buns - simple melody examples

#import "../lib.typ": melody, score

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

= Simple Melodies

== Mary Had a Little Lamb

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music: "
  e4 d c d | e e e2 | d4 d d2 | e4 g g2 |
  e4 d c d | e e e e | d d e d | c1 ||
  ",
)

#v(1cm)

== Hot Cross Buns (Half-width, centered)

#align(center)[
  #melody(
    key: "C",
    time: "4/4",
    clef: "treble",
    music: "e4n[3] d c2 | e4 d c2 | c8 c c c d d d d | e4 d c2",
    width: 130mm,
  )
]

== Hot Cross Buns (Smaller staff)

#align(center)[
  #melody(
    key: "C",
    time: "4/4",
    clef: "treble",
    music: "e4n[3] d c2 | e4 d c2 | c8 c c c d d d d | e4 d c2",
    width: 75mm,
    staff-size: 1.0mm
  )
]

== Hot Cross Buns (Large Staff)

#align(center)[
  #melody(
    key: "C",
    time: "4/4",
    clef: "treble",
    music: "e4n[3] d c2 | e4 d c2 | c8 c c c d d d d | e4 d c2",
    staff-size: 2.5mm
  )
]

== Hot Cross Buns (Melody Only)

#align(center)[
  #melody(
    music: "e4n[3] d c2 | e4 d c2 | c8 c c c d d d d | e4 d c2",
    staff-size: 2.5mm
  )
]