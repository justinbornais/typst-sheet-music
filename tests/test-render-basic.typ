// Basic rendering test - verifies core functionality

#import "../lib.typ": score, melody

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

= Sheet Music Library - Tests

== Test 1: Simple C major scale (treble clef)

#melody(
  key: "C",
  time: "4/4",
  clef: "treble",
  music: "c4 d e f | g a b c'",
)

#v(1cm)

== Test 2: D major key signature

#melody(
  key: "D",
  time: "4/4",
  music: "d4 e f# g | a b c#' d'",
)

#v(1cm)

== Test 3: Rests

#melody(
  key: "C",
  time: "4/4",
  music: "c4 r4 e r | r2 g2",
)

#v(1cm)

== Test 4: Different durations

#melody(
  key: "C",
  time: "4/4",
  music: "c1 | c2 d2 | c4 d e f | c8 d e f g a b c'",
)

#v(1cm)

== Test 5: Dotted notes

#melody(
  key: "C",
  time: "4/4",
  music: "c4. d8 e4 f | g2. r4",
)

#v(1cm)

== Test 6: Accidentals

#melody(
  key: "C",
  time: "4/4",
  music: "c4 c# d d& | e& e f# g",
)

#v(1cm)

== Test 7: Ledger lines (high and low notes)

#melody(
  key: "C",
  time: "4/4",
  music: "d4 c b, a, | g' a' b' c''",
)

#v(1cm)

== Test 8: Bass clef

#score(
  key: "C",
  time: "4/4",
  lyric-line-spacing: 5mm,
  staves: (
    (clef: "bass", music: "c4 d e f | g a b c'"),
  ),
)

#v(1cm)

== Test 9: Flat key signatures

#melody(
  key: "Bb",
  time: "3/4",
  music: "b&4 c' d' | e&' f' g'",
)

#v(1cm)

== Test 10: Title and composer

#score(
  title: "Test Piece",
  composer: "Test Composer",
  key: "G",
  time: "3/4",
  staves: (
    (clef: "treble", music: "g4 a b | c' d' e' | d'2."),
  ),
)

#v(1cm)

== Test 11: Ledger line edge cases

Treble: G5 (no ledger), A5 (1 ledger), D4 (no ledger), C4 (1 ledger):
#melody(
  key: "C",
  time: "4/4",
  music: "g'4 a' d c",
)

#v(0.5cm)

Bass: B3 (no ledger), C4 (1 ledger), F2 (no ledger), E2 (1 ledger):
#score(
  key: "C",
  time: "4/4",
  staves: (
    (clef: "bass", music: "b4 c' f, e,"),
  ),
)

#v(1cm)

== Test 12: Sharp on first note spacing

#melody(
  key: "C",
  time: "4/4",
  music: "f#4 g a b",
)

== Test 13: 16th Notes

#melody(
  key: "C",
  time: "5/4",
  music: "c16 d e f g a b c' d' e' f' g' a' b' c'' d'' e''4",
)

#v(1cm)

== Test 14: Chords - basic block chords

#melody(
  key: "C",
  time: "4/4",
  music: "<c e g>4 <d f a> <e g b> <f a c'>",
)

#v(1cm)

== Test 15: Chords - mixed with single notes

#melody(
  key: "C",
  time: "4/4",
  music: "c4 <e g b> c' | <c e g>2 <f a c'>2",
)

#v(1cm)

== Test 16: Chords - dotted and eighth chords

#melody(
  key: "C",
  time: "4/4",
  music: "<c e>4. <d f>8 <e g>4 r | <c g>8 <d a> <e b> <f c'> <g d'> <a e'> <b f'> <c' g'>",
)

#v(1cm)

== Test 17: Chords - with accidentals

#melody(
  key: "C",
  time: "4/4",
  music: "<c e& g>4 <d f# a> <e g b&> <f# a c'>",
)

#v(1cm)

== Test 18: Fingerings - inline notation (above)

// Fingerings: note 1 = finger 1, note 2 = skipped, note 3 = fingers (1,3) stacked, note 4 = finger 5
#melody(
  key: "C",
  time: "4/4",
  music: "c4n[1] d en[1 3] fn[5]",
)

#v(1cm)

== Test 18b: Fingerings - below staff

// Fingerings below: n_[digit] places fingering below the staff
#melody(
  key: "C",
  time: "4/4",
  music: "c4n_[1] dn_[2] en_[3] fn_[4]",
)

#v(1cm)

== Test 18c: Fingerings - mixed above and below

#melody(
  key: "C",
  time: "4/4",
  music: "c4n[1] dn_[2] en[3] fn_[4] | gn_[5] an[4] bn_[3] c'n[2]",
)

#v(1cm)

== Test 19: Fingerings - chords with stacked fingerings

#score(
  key: "C",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "<c e g>4n[1 3 5] <d f a> <e g b>n_[1 3 5] <f a c'>n[1 3]",
    ),
  ),
)

#v(1cm)

== Test 20: Chord Symbols - one chord per measure (4/4)

One chord per measure on beat 1:

#melody(
  key: "C",
  time: "4/4",
  music: "c4[C] d e f | g[G] a b c' | c'2[Am] g | c'1[C]",
)

#v(1cm)

== Test 21: Chord Symbols - two chords per measure (4/4)

Two chords on beats 1 and 3 in 4/4:

#melody(
  key: "C",
  time: "4/4",
  music: "c4[C] d e[F] f | g[G] a b[C] c'",
)

#v(1cm)

== Test 22: Chord Symbols - three chords per measure (4/4)

Three chords spread across 4/4 measure:

#melody(
  key: "C",
  time: "4/4",
  music: "c4[Am] d[Dm] e f[G] | g[C] a[F] b c'[G7]",
)

#v(1cm)

== Test 23: Chord Symbols - two chords per measure (3/4)

Two chords in 3/4:

#melody(
  key: "C",
  time: "3/4",
  music: "c4[C] d[G] e | f[F] g[Am] a",
)

#v(1cm)

== Test 24: Chord Symbols - slash chords and accidentals

#melody(
  key: "C",
  time: "4/4",
  music: "c4[Bb/F] d e f | g[F#m7] a b c'",
)

#v(1cm)

== Test 25: Chord Symbols - with fingerings (chord above fingering)

Chord symbols should appear above fingering numbers:

#melody(
  key: "C",
  time: "4/4",
  music: "c4n[1][C] dn[2] en[3][F] fn[4] | gn[5][G] an[4] bn[3][C] c'n[2]",
)

#v(1cm)

== Test 26: Chord Symbols - four chords per measure (4/4, one per beat)

Four chords in 4/4 - one per beat:

#melody(
  key: "C",
  time: "4/4",
  music: "c4[C] d[Dm] e[Em] f[F] | g[G] a[Am] b[Bdim] c'[C]",
)

#v(1cm)

== Test 27: Chord Symbols - empty measures and inline placement

Some measures have no chords:

#melody(
  key: "C",
  time: "4/4",
  music: "c4[C] d e f | g a b c' | c'2[Am] g[G] | c'1[C]",
)

#v(1cm)

== Test 28: Chord Symbols - 6/8 time, two chords

Two chords in 6/8:

#score(
  key: "C",
  time: "6/8",
  staves: (
    (
      clef: "treble",
      music: "c8[C] d e f[F] g a | b[G] c' d' e'[C] f' g'",
    ),
  ),
)

#v(1cm)

== Test 29: Dynamics - basic markings

Dynamic markings rendered below the staff:

#melody(
  key: "C",
  time: "4/4",
  music: "c4v[pp] d ev[mf] f | gv[f] a bv[ff] c'",
)

#v(1cm)

== Test 30: Dynamics - all standard markings

Every standard dynamic marking:

#melody(
  key: "C",
  time: "4/4",
  music: "c4v[ppp] dv[pp] ev[p] fv[mp] | gv[mf] av[f] bv[ff] c'v[fff]",
)

#v(1cm)

== Test 31: Dynamics - sf, sfz, fp

Special dynamics:

#melody(
  key: "C",
  time: "4/4",
  music: "c4v[sf] dv[sfz] ev[fp] f",
)

#v(1cm)

== Test 32: Articulations - accent

Accent marks on notes:

#melody(
  key: "C",
  time: "4/4",
  music: "c4> d> e> f> | g> a> b> c'>",
)

#v(1cm)

== Test 33: Articulations - staccato

Staccato dots on notes:

#melody(
  key: "C",
  time: "4/4",
  music: "c4* d* e* f* | g* a* b* c'*",
)

#v(1cm)

== Test 34: Articulations - tenuto

Tenuto marks on notes:

#melody(
  key: "C",
  time: "4/4",
  music: "c4- d- e- f- | g- a- b- c'-",
)

#v(1cm)

== Test 35: Articulations - fermata

Fermata on final note:

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f | g a b c'_",
)

#v(1cm)

== Test 36: Articulations - combined (accent + staccato)

Multiple articulations on the same note:

#melody(
  key: "C",
  time: "4/4",
  music: "c4>* d>* e f | g a b c'>*",
)

#v(1cm)

== Test 37: Articulations + Dynamics combined

Articulations and dynamics on the same note:

#melody(
  key: "C",
  time: "4/4",
  music: "c4>*v[f] d ev[p] f_ | g>-v[ff] a b c'_*v[pp]",
)

#v(1cm)

== Test 38: Articulations with fermata and other marks

Fermata combined with other articulations:

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f>_ | g a b c'>*_",
)

#v(1cm)

== Test 39: Ties - basic tie between two notes

A simple tie connecting two quarter notes of the same pitch:

#melody(
  key: "C",
  time: "4/4",
  music: "c4~ c4 e2",
)

#v(1cm)

== Test 40: Ties - across a barline

Tie that spans a barline:

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f~ | f e d c",
)

#v(1cm)

== Test 41: Ties - half notes and whole notes

Longer tied notes:

#melody(
  key: "C",
  time: "4/4",
  music: "c2~ c2 | g2~ g4 e4",
)

#v(1cm)

== Test 42: Ties - high and low notes (stem direction)

Verify tie curves respect stem direction (above for stem-down, below for stem-up):

#melody(
  key: "C",
  time: "4/4",
  music: "g'4~ g' c~ c",
)

#v(1cm)

== Test 43: Ties - multiple ties in sequence

Several ties in a row:

#melody(
  key: "C",
  time: "4/4",
  music: "c4~ c~ c~ c",
)

#v(1cm)

== Test 44: Ties - with accidentals

Tied notes with accidentals:

#melody(
  key: "C",
  time: "4/4",
  music: "f#4~ f# d&~ d&",
)

#v(1cm)

== Test 45: Ties - chord blocks

Tied chords:

#melody(
  key: "C",
  time: "4/4",
  music: "<c e g>4~ <c e g>4 <d f a>2",
)

#v(1cm)

== Test 46: Slurs - basic two-note slur

Simple slur over two notes:

#melody(
  key: "C",
  time: "4/4",
  music: "c4( d) e( f)",
)

#v(1cm)

== Test 47: Slurs - multi-note phrase

Slur spanning four notes:

#melody(
  key: "C",
  time: "4/4",
  music: "c4( d e f) g2",
)

#v(1cm)

== Test 48: Slurs - ascending and descending

Slurs over ascending and descending passages:

#melody(
  key: "C",
  time: "4/4",
  music: "c4( d e f) | g( f e d)",
)

#v(1cm)

== Test 49: Slurs - across barline

Slur that crosses a barline:

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e( f | g) a b c'",
)

#v(1cm)

== Test 50: Slurs - high notes (stem down, slur above)

Slur on notes above the staff:

#melody(
  key: "C",
  time: "4/4",
  music: "g'4( a' b' c'')",
)

#v(1cm)

== Test 51: Slurs - low notes (stem up, slur below)

Slur on notes below the staff:

#melody(
  key: "C",
  time: "4/4",
  music: "c4( d e f)",
)

#v(1cm)

== Test 52: Slurs + Ties combined

A slurred passage with a tie inside:

#melody(
  key: "C",
  time: "4/4",
  music: "c4( d e~ e) | f2 g",
)

#v(1cm)

== Test 53: Slurs + Articulations combined

Slur with staccato and accent:

#melody(
  key: "C",
  time: "4/4",
  music: "c4>( d* e f*) | g a b c'",
)

#v(1cm)

== Test 54: Ties + Dynamics combined

Tied note with a dynamic marking:

#melody(
  key: "C",
  time: "4/4",
  music: "c4v[pp]~ c e2v[ff]",
)

#v(1cm)

== Test 55: Slurs with eighth notes

Slur over beamed eighth notes:

#melody(
  key: "C",
  time: "4/4",
  music: "c8( d e f) g( a b c')",
)

#v(1cm)

== Test 56: Inline clef changes

Treble to bass and bass to treble switches within a single staff:

#score(
  key: "C",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "f4 e d c bass b a g | g a b c treble d e f g",
    ),
  ),
)

#v(1cm)

== Test 57: Inline time signature changes

Mid-measure, post-barline, and line-end time signature changes:

#score(
  key: "C",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "c4 e g c' 3/4 g4 g c | 2/4 d4 e | \n5/4 f4 g a b c'",
    ),
    (
      clef: "bass",
      music: "c,4 g, c e 3/4 g,4 c e | 2/4 f g | \n5/4 c,4 d, e, f, g,",
    ),
  ),
  staff-group: "grand",
)

#v(1cm)

== Test 58: Repeat-both barlines

Mid-system `:||:` should render with dots on both sides, and a line-ending
`repeat-both` should render as `:||` plus `||:` at the next system start:

#score(
  key: "C",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "c4 d e f :|: g a b c' | d' c' b a",
    ),
    (
      clef: "bass",
      music: "c,4 d, e, f, :|: g, a, b, c | d c b, a,",
    ),
  ),
  staff-group: "grand",
)

#v(1cm)

#score(
  key: "C",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "c4 d e f :||: g a b c'",
    ),
    (
      clef: "bass",
      music: "c,4 d, e, f, :||: g, a, b, c",
    ),
  ),
  staff-group: "grand",
  measures-per-line: 1,
)

#v(1cm)

== Test 59: Crescendo and decrescendo hairpins

#melody(
  key: "C",
  time: "4/4",
  music: "c4 e g c | cresc{c e g c} | decresc{c' b a g}",
)

#v(1cm)

== Test 60: Trill symbols and trill lines

#melody(
  key: "C",
  time: "4/4",
  music: "c4tr d e f | tr{g4} a b c' | tr{d'4 e' f' g' |
  a' b' c'' d''} e''1",
)

#v(1cm)

== Test 61: Tuplets containing rests

#melody(
  key: "C",
  time: "4/4",
  music: "{2,3:c8 r8 e8} g4 | c'4 {2,3:r8 d8 e8} g4",
)

#v(1cm)

== Test 62: First and second endings

#score(
  key: "C",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "|: c4 e g c | end{1.:f4n[2][C] d e c | g[G] g c c} :| | end{2.:g4[G] g g g | b[Bdim] b c' c'} |.",
    ),
  ),
)

#v(1cm)

== Test 63: Lyrics - hyphens, extenders, carries, and stacked verses

#score(
  key: "C",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "c4l[Hel-] dl el[lo] fl[there] | g4l[Hold_] al bl c'l[on] |
      e4l[1. Ev-]l[2. Why_]l[3. In] dlll[You,] el['ry]l[do]l[O] fl[night]l[I]l[Lord,]",
    ),
  ),
)
