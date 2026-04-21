// Orchestra ensemble example - demonstrates independent, bracketed, and braced staff groups

#import "../lib.typ": score

#set page(width: 210mm, height: 297mm, margin: 1.2cm)

= Orchestra Ensemble

#score(
  title: "Small Orchestra Ensemble",
  subtitle: "Per-staff grouping demo",
  key: "D",
  time: "4/4",
  width: 180mm,
  staff-spacing: 7mm,
  system-spacing: 10mm,
  staves: (
    (
      clef: "treble",
      instrument-name: "Flute",
      instrument-name-cont: "Fl.",
      barline-group-start: true,
      music: "d''4 e'' f#'' g'' | a''2 f#'' | e''4 d'' c#'' b' | a'1",
    ),
    (
      clef: "treble",
      instrument-name: "Oboe",
      instrument-name-cont: "Ob.",
      barline-group-end: true,
      music: "a'4 b' c#'' d'' | e''2 c#'' | b'4 a' g' f#' | e'1",
    ),
    (
      clef: "treble",
      instrument-name: "Clarinet",
      instrument-name-cont: "Cl.",
      music: "f#'4 g' a' b' | c#''2 a' | g'4 f#' e' d' | c#'1",
    ),
    (
      clef: "bass",
      instrument-name: "Bassoon",
      instrument-name-cont: "Bsn.",
      music: "d4 a, d f# | a2 d | b,4 d g f# | e1",
    ),
    (
      clef: "treble",
      instrument-name: "Violin I",
      instrument-name-cont: "Vln. I",
      bracket-start: true,
      brace-start: true,
      music: "a'4 b' c#'' d'' | e''2 d'' | f#''4 e'' d'' c#'' | d''1",
    ),
    (
      clef: "treble",
      instrument-name: "Violin II",
      instrument-name-cont: "Vln. II",
      brace-end: true,
      music: "f#'4 g' a' b' | c#''2 b' | d''4 c#'' b' a' | b'1",
    ),
    (
      clef: "alto",
      instrument-name: "Viola",
      instrument-name-cont: "Vla.",
      music: "d'4 e' f#' g' | a'2 g' | b'4 a' g' f#' | g'1",
    ),
    (
      clef: "bass",
      instrument-name: "Cello",
      instrument-name-cont: "Vc.",
      bracket-end: true,
      music: "d4 a, d f# | g2 d | e4 g b a | d1",
    ),
    (
      clef: "treble",
      instrument-name: "Piano",
      instrument-name-cont: "Pno.",
      brace-start: true,
      music: "<d' f#' a'>2 <e' g' b'> | <f#' a' c#''> <g' b' d''> | <a' c#'' e''> <g' b' d''> | <f#' a' d''>1",
    ),
    (
      clef: "bass",
      instrument-name-shared: true,
      instrument-name-cont: "Pno.",
      brace-end: true,
      music: "d,2 a, | d g, | a, d | d,1",
    ),
  ),
)

