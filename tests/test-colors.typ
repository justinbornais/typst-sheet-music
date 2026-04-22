// Color rendering test coverage

#import "../lib.typ": score, melody, red, blue, gold, sky-blue, purple, bronze, charcoal, navy

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

= Color Functionality Tests

== Test 1: Global melody color

#melody(
  key: "C",
  time: "4/4",
  color: "#b91c1c",
  music: "c4 d e f | g a b c'",
)

#v(1cm)

== Test 2: Global score color with title and composer

#score(
  title: "Global Color",
  composer: "Scorify",
  key: "G",
  time: "3/4",
  color: "#0f766e",
  staves: (
    (clef: "treble", music: "g4 a b | c' d' e'"),
    (clef: "bass", music: "g,2 d | g,2."),
  ),
  staff-group: "grand",
)

#v(1cm)

== Test 3: Per-staff colors inside one score

#score(
  key: "C",
  time: "4/4",
  staves: (
    (clef: "treble", color: "#1d4ed8", music: "c'4 d' e' f' | g' a' b' c''"),
    (clef: "bass", color: "#7c3aed", music: "c,2 g, | c,2 r2"),
  ),
  staff-group: "grand",
)

#v(1cm)

== Test 4: Selection color wrapper over notes and slurs

#melody(
  key: "C",
  time: "4/4",
  music: "c4 color{#dc2626:d8 e f g | e4( d) c2}",
)

#v(1cm)

== Test 5: Selection color wrapper with dynamics, text, and lyrics

#melody(
  key: "C",
  time: "4/4",
  music: "color{#0ea5e9:c4v[mf]text[Solo]l[La] d e f | g[Am] a bm c'}",
)

#v(1cm)

== Test 6: Selection color wrapper should not recolor barlines or staff lines

#melody(
  key: "C",
  time: "4/4",
  music: "color{#ea580c:c4 d e f | g a b c'}",
)

#v(1cm)

== Test 7: Local note color override

#melody(
  key: "C",
  time: "4/4",
  music: "c4color{#ef4444} d e f | g a b c'",
)

#v(1cm)

== Test 8: Local tie-only color override

#melody(
  key: "C",
  time: "4/4",
  music: "c4~color{#2563eb} c4 e2",
)

#v(1cm)

== Test 9: Local slur-only color override

#melody(
  key: "C",
  time: "4/4",
  music: "c4(color{#16a34a} d e) f",
)

#v(1cm)

== Test 10: Local chord-note color override

#melody(
  key: "C",
  time: "4/4",
  music: "<c ecolor{#db2777} g>1",
)

#v(1cm)

== Test 11: Local articulation and dynamic color overrides

#melody(
  key: "C",
  time: "4/4",
  music: "c4>color{#f97316} dv[mf]color{#0891b2} e- f*",
)

#v(1cm)

== Test 12: Local chord symbol, staff text, and lyric color overrides

#melody(
  key: "C",
  time: "4/4",
  music: "c4[C]color{#a16207} dtext[Solo]color{#0369a1} el[La]color{#be123c} f",
)

#v(1cm)

== Test 13: Mixed global, staff, selection, and local colors together

#score(
  key: "D",
  time: "4/4",
  color: "#334155",
  staves: (
    (
      clef: "treble",
      color: "#7c3aed",
      music: "color{#ef4444:f#4 a b c#'} d'~color{#2563eb} d' | <d' f#' acolor{#f59e0b}>2 g'2",
    ),
    (
      clef: "bass",
      music: "d,2 a, | d color{#16a34a:f# a d'}",
    ),
  ),
  staff-group: "grand",
)

#v(1cm)

== Test 14: Fingering color override with one colored mark in a three-fingering chord

#melody(
  key: "C",
  time: "4/4",
  music: "c4n[2] d e | <f a c'>2n[1 color{#dc2626:3} 5] g2n[4]",
)

#v(1cm)

== Test 15: Named preset global colors across a grand staff

#score(
  title: "Preset Palette",
  composer: "Scorify",
  key: "D",
  time: "4/4",
  color: sky-blue,
  staves: (
    (clef: "treble", color: purple, music: "d'4 e' f#' g' | a' b' c#'' d''"),
    (clef: "bass", color: bronze, music: "d,2 a, | d,2 r2"),
  ),
  staff-group: "grand",
)

#v(1cm)

== Test 16: Named preset inline wrappers and local overrides

#melody(
  key: "C",
  time: "4/4",
  color: charcoal,
  music: "color{gold:c4 d e f} | gcolor{red} acolor{blue} bcolor{green} c'",
)

#v(1cm)

== Test 17: Metallic and neutral presets on text, chords, and fingerings

#melody(
  key: "G",
  time: "4/4",
  color: navy,
  music: "g4text[Solo]color{platinum} a[Em]color{silver} b l[La]color{orange} c' | <d' f#' a'>2n[1 color{yellow:2} 4] color{gold:g'2}",
)

#v(1cm)

== Test 18: Melody note-colors map

#melody(
  key: "C",
  time: "4/4",
  color: charcoal,
  note-colors: ("c": red, "e": blue, "g": gold, "c'": purple),
  music: "c4 d e f | g a b c'
  c8 d e c d e f d | e4 d c2",
)

#v(1cm)

== Test 19: Score and per-staff note-colors maps

#score(
  key: "C",
  time: "4/4",
  note-colors: ("c": red, "g": sky-blue),
  staves: (
    (
      clef: "treble",
      note-colors: ("c": purple, "e'": bronze),
      music: "c4 d e f | g a b c'",
    ),
    (
      clef: "bass",
      music: "c,4 g, c d | e f g c",
    ),
  ),
  staff-group: "grand",
)

#v(1cm)

== Test 20: Note-colors map should yield to inline color wrappers

#melody(
  key: "C",
  time: "4/4",
  note-colors: ("c": red, "d": blue),
  music: "ccolor{#22c55e} d color{gold:e f}",
)

#v(1cm)

== Test 21: Note-colors map should apply across octaves with explicit split points

#melody(
  key: "C",
  time: "4/4",
  note-colors: ("c": red, "c'": green),
  music: "c,,8 c, c c' c'' c''' c'' c' | c c, c,, c,,,",
)