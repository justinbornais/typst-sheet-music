// test-parser.typ - Parser unit tests

#import "../src/parser.typ": parse-music
#import "../src/pitch.typ": pitch-to-diatonic, staff-position, key-sig-accidentals

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

= Parser Unit Tests

#let assert-eq(a, b, msg: "") = {
  if a != b {
    text(fill: red, weight: "bold")[FAIL: #msg - got #repr(a), expected #repr(b)]
  } else {
    text(fill: green)[PASS: #msg]
  }
}

== Note Parsing

#let events = parse-music("c'4")
#assert-eq(events.len(), 1, msg: "single note count")
#assert-eq(events.at(0).type, "note", msg: "note type")
#assert-eq(events.at(0).name, "c", msg: "note name")
#assert-eq(events.at(0).octave, 5, msg: "note octave c'")
#assert-eq(events.at(0).duration, 4, msg: "note duration")

#let events2 = parse-music("f#'4 g'8")
#assert-eq(events2.len(), 2, msg: "two notes count")
#assert-eq(events2.at(0).name, "f", msg: "first note name")
#assert-eq(events2.at(0).accidental, "sharp", msg: "sharp accidental")
#assert-eq(events2.at(1).name, "g", msg: "second note name")
#assert-eq(events2.at(1).duration, 8, msg: "eighth note duration")

== Duration Stickiness

#let events3 = parse-music("c'4 d' e'")
#assert-eq(events3.len(), 3, msg: "sticky duration count")
#assert-eq(events3.at(1).duration, 4, msg: "sticky duration value")
#assert-eq(events3.at(2).duration, 4, msg: "sticky duration value 2")

== Rests

#let events4 = parse-music("c'4 r4 e'4")
#assert-eq(events4.len(), 3, msg: "notes with rest count")
#assert-eq(events4.at(1).type, "rest", msg: "rest type")
#assert-eq(events4.at(1).duration, 4, msg: "rest duration")

== Barlines

#let events5 = parse-music("c'4 d' | e' f'")
#assert-eq(events5.len(), 5, msg: "barline count")
#assert-eq(events5.at(2).type, "barline", msg: "barline type")
#assert-eq(events5.at(2).style, "single", msg: "barline style")

== Double Barline

#let events6 = parse-music("c'4 d' || e' f'")
#assert-eq(events6.at(2).style, "double", msg: "double barline")

== Final Barline

#let events7 = parse-music("c'4 d' |.")
#assert-eq(events7.at(2).style, "final", msg: "final barline")

== Octave Markers

#let events8 = parse-music("c4 c'4 c''4 c,4")
#assert-eq(events8.at(0).octave, 4, msg: "default octave")
#assert-eq(events8.at(1).octave, 5, msg: "one octave up")
#assert-eq(events8.at(2).octave, 6, msg: "two octaves up")
#assert-eq(events8.at(3).octave, 3, msg: "one octave down")

== Dotted Notes

#let events9 = parse-music("c'4. d'8")
#assert-eq(events9.at(0).dots, 1, msg: "single dot")

== Flats

#let events10 = parse-music("b&'4 e&'4")
#assert-eq(events10.at(0).accidental, "flat", msg: "b flat")
#assert-eq(events10.at(1).accidental, "flat", msg: "e flat")

== Inline Clef Changes

#let events11 = parse-music("f e d c bass b a g")
#assert-eq(events11.len(), 8, msg: "inline clef event count")
#assert-eq(events11.at(4).type, "clef", msg: "inline clef type")
#assert-eq(events11.at(4).clef, "bass", msg: "inline clef value")
#assert-eq(events11.at(5).octave, 3, msg: "bass clef resets default octave")

#let events12 = parse-music("g a b c' treble d e f g", base-octave: 3)
#assert-eq(events12.at(4).type, "clef", msg: "treble clef change inserted")
#assert-eq(events12.at(5).octave, 4, msg: "treble clef raises default octave")

== Inline Time Signature Changes

#let events13 = parse-music("c e g c' 3/4 g g c")
#assert-eq(events13.len(), 8, msg: "inline time signature event count")
#assert-eq(events13.at(4).type, "time-sig", msg: "inline time signature type")
#assert-eq(events13.at(4).upper, 3, msg: "inline time signature upper")
#assert-eq(events13.at(4).lower, 4, msg: "inline time signature lower")

#let events14 = parse-music("c e | 2/4 g a")
#assert-eq(events14.at(3).type, "time-sig", msg: "post-barline time signature type")
#assert-eq(events14.at(3).upper, 2, msg: "post-barline time signature upper")

== Pitch Calculations

#assert-eq(pitch-to-diatonic("c", 4), 28, msg: "C4 diatonic")
#assert-eq(pitch-to-diatonic("f", 5), 38, msg: "F5 diatonic")
#assert-eq(staff-position("f", 5, clef: "treble"), 0, msg: "F5 treble top line")
#assert-eq(staff-position("e", 4, clef: "treble"), 8, msg: "E4 treble bottom line")
#assert-eq(staff-position("b", 4, clef: "treble"), 4, msg: "B4 treble middle line")
#assert-eq(staff-position("c", 4, clef: "treble"), 10, msg: "C4 below treble staff")

== Key Signature Accidentals

#let d-major = key-sig-accidentals("D")
#assert-eq(d-major.len(), 2, msg: "D major has 2 sharps")
#assert-eq("f" in d-major, true, msg: "D major has F#")
#assert-eq("c" in d-major, true, msg: "D major has C#")

#let bb-major = key-sig-accidentals("Bb")
#assert-eq(bb-major.len(), 2, msg: "Bb major has 2 flats")
#assert-eq("b" in bb-major, true, msg: "Bb major has Bb")
#assert-eq("e" in bb-major, true, msg: "Bb major has Eb")

#v(1cm)
#text(size: 14pt, weight: "bold")[All parser tests complete.]
