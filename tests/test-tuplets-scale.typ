// Tuplet scaling test

#import "../lib.typ": melody

#set page(width: 210mm, height: 100mm, margin: 1.5cm)

= Tuplet Scaling Test

#melody(
  key: "C",
  time: "2/4",
  clef: "treble",
  music: "c4 {2,3:d4 e d} | c2",
  staff-size: 2.5mm,
)

#v(1cm)

#melody(
  key: "C",
  time: "2/4",
  clef: "treble",
  music: "c4 {2,3:d4 e d} | c2",
  staff-size: 1.2mm,
)
