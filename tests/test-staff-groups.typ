// Staff grouping test - verifies per-staff barline, bracket, and brace groups

#import "../lib.typ": score

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

= Staff Grouping Tests

== Per-Staff Group Ranges

#score(
  title: "Per-Staff Grouping",
  key: "G",
  time: "4/4",
  width: 170mm,
  staff-spacing: 7mm,
  staves: (
    (
      clef: "treble",
      instrument-name: "Flute",
      barline-group-start: true,
      music: "g'4 a' b' c'' | d''2 b'2",
    ),
    (
      clef: "treble",
      instrument-name: "Oboe",
      barline-group-end: true,
      music: "e'4 f#' g' a' | b'2 g'2",
    ),
    (
      clef: "treble",
      instrument-name: "Violin",
      bracket-start: true,
      music: "b'4 c'' d'' e'' | d''2 b'2",
    ),
    (
      clef: "alto",
      instrument-name: "Viola",
      music: "d'4 e' f#' g' | a'2 f#'2",
    ),
    (
      clef: "bass",
      instrument-name: "Cello",
      bracket-end: true,
      music: "g,4 d g b | g2 d2",
    ),
    (
      clef: "treble",
      instrument-name: "Piano",
      brace-start: true,
      music: "<g' b' d''>2 <a' c'' e''> | <b' d'' f#''>1",
    ),
    (
      clef: "bass",
      instrument-name-shared: true,
      brace-end: true,
      music: "g,2 d | g,1",
    ),
  ),
)

