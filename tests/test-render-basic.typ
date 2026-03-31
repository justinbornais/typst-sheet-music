// Basic rendering test - verifies core functionality

#import "../lib.typ": score, melody

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

= Sheet Music Library - Phase 1 Tests

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

== Test 14: Chords — basic block chords

#melody(
  key: "C",
  time: "4/4",
  music: "<c e g>4 <d f a> <e g b> <f a c'>",
)

#v(1cm)

== Test 15: Chords — mixed with single notes

#melody(
  key: "C",
  time: "4/4",
  music: "c4 <e g b> c' | <c e g>2 <f a c'>2",
)

#v(1cm)

== Test 16: Chords — dotted and eighth chords

#melody(
  key: "C",
  time: "4/4",
  music: "<c e>4. <d f>8 <e g>4 r | <c g>8 <d a> <e b> <f c'> <g d'> <a e'> <b f'> <c' g'>",
)

#v(1cm)

== Test 17: Chords — with accidentals

#melody(
  key: "C",
  time: "4/4",
  music: "<c e& g>4 <d f# a> <e g b&> <f# a c'>",
)

#v(1cm)

== Test 18: Fingerings — skip a note (none) and multi-finger chord

// Fingerings: note 1 = finger 1, note 2 = skipped, note 3 = fingers (1,3) stacked, note 4 = finger 5
#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f",
  fingerings: (1, none, (1, 3), 5),
)

#v(1cm)

== Test 19: Fingerings — chords with skip and stacked fingerings

#score(
  key: "C",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "<c e g>4 <d f a> <e g b> <f a c'>",
      fingerings: ((1, 3, 5), none, (1, 3, 5), (1, 3)),
    ),
  ),
)

#v(1cm)

== Test 20: Chord Symbols — one chord per measure (4/4)

One chord per measure places it on beat 1:

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f | g a b c' | c'2 g | c'1",
  chord-symbols: (
    ("C",),
    ("G",),
    ("Am",),
    ("C",),
  ),
)

#v(1cm)

== Test 21: Chord Symbols — two chords per measure (4/4)

Two chords → beat 1 and beat 3 in 4/4:

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f | g a b c'",
  chord-symbols: (
    ("C", "F"),
    ("G", "C"),
  ),
)

#v(1cm)

== Test 22: Chord Symbols — three chords per measure (4/4)

Three chords → beats 1, 2, 3 in 4/4:

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f | g a b c'",
  chord-symbols: (
    ("Am", "Dm", "G"),
    ("C", "F", "G7"),
  ),
)

#v(1cm)

== Test 23: Chord Symbols — two chords per measure (3/4)

Two chords → beat 1 and beat 2 in 3/4:

#melody(
  key: "C",
  time: "3/4",
  music: "c4 d e | f g a",
  chord-symbols: (
    ("C", "G"),
    ("F", "Am"),
  ),
)

#v(1cm)

== Test 24: Chord Symbols — slash chords and accidentals

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f | g a b c'",
  chord-symbols: (
    ("Bb/F",),
    ("F#m7",),
  ),
)

#v(1cm)

== Test 25: Chord Symbols — with fingerings (chord above fingering)

Chord symbols should appear above fingering numbers:

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f | g a b c'",
  fingerings: (1, 2, 3, 4, 5, 4, 3, 2),
  chord-symbols: (
    ("C", "F"),
    ("G", "C"),
  ),
)

#v(1cm)

== Test 26: Chord Symbols — four chords per measure (4/4, one per beat)

Four chords in 4/4 → one per beat:

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f | g a b c'",
  chord-symbols: (
    ("C", "Dm", "Em", "F"),
    ("G", "Am", "Bdim", "C"),
  ),
)

#v(1cm)

== Test 27: Chord Symbols — empty measures and none placeholders

Some measures have no chords (empty array):

#melody(
  key: "C",
  time: "4/4",
  music: "c4 d e f | g a b c' | c'2 g | c'1",
  chord-symbols: (
    ("C",),
    (),
    ("Am", "G"),
    ("C",),
  ),
)

#v(1cm)

== Test 28: Chord Symbols — 6/8 time, two chords

Two chords in 6/8 → beat 1 and beat 4 (middle of compound time):

#score(
  key: "C",
  time: "6/8",
  staves: (
    (
      clef: "treble",
      music: "c8 d e f g a | b c' d' e' f' g'",
      chord-symbols: (
        ("C", "F"),
        ("G", "C"),
      ),
    ),
  ),
)