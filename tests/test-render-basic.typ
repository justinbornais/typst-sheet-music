// Basic rendering test - verifies core functionality

#import "../lib.typ": score, melody

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

= Sheet Music Library - Phase 1 Tests

== Test 1: Simple C major scale (treble clef)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music: "c4'4 d' e' f' | g' a' b' c''",
)

#v(1cm)

== Test 2: D major key signature

#melody(
  key: "D",
  time: "4/4",
  music: "d'4 e' f#' g' | a' b' c#'' d''",
)

#v(1cm)

== Test 3: Rests

#melody(
  key: "C",
  time: "4/4",
  music: "c'4 r4 e'4 r4 | r2 g'2",
)

#v(1cm)

== Test 4: Different durations

#melody(
  key: "C",
  time: "4/4",
  music: "c4'1 | c'2 d'2 | c'4 d' e' f' | c'8 d' e' f' g' a' b' c''",
)

#v(1cm)

== Test 5: Dotted notes

#melody(
  key: "C",
  time: "4/4",
  music: "c'4. d'8 e'4 f' | g'2. r4",
)

#v(1cm)

== Test 6: Accidentals

#melody(
  key: "C",
  time: "4/4",
  music: "c'4 c#' d' d&' | e&' e' f#' g'",
)

#v(1cm)

== Test 7: Ledger lines (high and low notes)

#melody(
  key: "C",
  time: "4/4",
  music: "c2'4 d3' e3' f3' | g5' a' b' c'' | d'' e'' f'' g''",
)

#v(1cm)

== Test 8: Bass clef

#score(
  key: "C",
  time: "4/4",
  staves: (
    (clef: "bass", music: "c3 d e f | g a b c'"),
  ),
)

#v(1cm)

== Test 9: Flat key signatures

#melody(
  key: "Bb",
  time: "3/4",
  music: "b&'4 c'' d'' | e&'' f'' g''",
)

#v(1cm)

== Test 10: Title and composer

#score(
  title: "Test Piece",
  composer: "Test Composer",
  key: "G",
  time: "3/4",
  staves: (
    (clef: "treble", music: "g'4 a' b' | c'' d'' e'' | d''2."),
  ),
)

#v(1cm)

== Test 11: Ledger line edge cases

Treble: G5 (no ledger), A5 (1 ledger), D4 (no ledger), C4 (1 ledger):
#melody(
  key: "C",
  time: "4/4",
  music: "g''4 a'' d'4 c'",
)

#v(0.5cm)

Bass: B3 (no ledger), C4 (1 ledger), F2 (no ledger), E2 (1 ledger):
#score(
  key: "C",
  time: "4/4",
  staves: (
    (clef: "bass", music: "b4 c' f,,, e,,,"),
  ),
)

#v(1cm)

== Test 12: Sharp on first note spacing

#melody(
  key: "C",
  time: "4/4",
  music: "f#'4 g' a' b'",
)

== Test 13: 16th Notes

#melody(
  key: "C",
  time: "5/4",
  music: "c16 d e f g a b c' d' e' f' g' a' b' c'' d'' e''4",
)