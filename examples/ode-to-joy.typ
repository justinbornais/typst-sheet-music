// Ode to Joy - Full melody example
// Demonstrates system breaks, key signatures, fingering numbers, and chord symbols.

#import "../lib.typ": score

#set page(margin: 1.5cm)

#score(
  title: "Ode to Joy",
  composer: "L. van Beethoven",
  key: "D",
  time: "4/4",
  staff-group: "grand",
  // staff-spacing: 6mm,
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a8 b g4 f# e | d d e f# | f#4. e8 e2 |
        f#4 f# g a | a g f# e | d d e f# | e4. d8 d2 |
        |: e4 e f# d | e f#8 g8 f#4 d | e f#8 g8 f#4 e | d e a4 d |
        f#4 f# g a | a g f# e | d d e f# | e4. d8 d2 :|
      ",
      fingerings: (
        // Line 1: measures 1-4
        2, 2, 4, 5,  5, 4, 2, 1,  1, 1, 2, 4,  4, 2, 2,
        // Line 2: measures 5-8
        2, 2, 4, 5,  5, 4, 2, 1,  1, 1, 2, 4,  2, 1, 1,
        // Line 3: measures 9-12
        2, 2, 4, 1,  2, 4, 4, 1,  2, 4, 4, 2,  1, 2, 5,
        // Line 4: measures 13-16
        2, 2, 4, 5,  5, 4, 2, 1,  1, 1, 2, 4,  2, 1, 1,
      ),
      chord-symbols: (
        // Line 1: measures 1-4
        ("D",), ("D/A", "A"), ("D",), ("A",),
        // Line 2: measures 5-8
        ("D",), ("A",), ("D",), ("A", "D"),
        // Line 3: measures 9-12
        ("A", "D"), ("A", "D"), ("A", "F#"), ("Bm", "Em", "A"),
        // Line 4: measures 13-16
        ("D",), ("A",), ("D",), ("A", "D"),
      ),
    ),
    (
      clef: "bass",
      music: "
        d1 | a, | d | a, |
        d | a, | d | a,2 d4 r |
        |: a,2 d | a, d | a, f#, | b,4 e, a,2 |
        d1 | a, | d | a,2 d :|"
    )
  ),
)

#score(
  title: "Ode to Joy",
  composer: "L. van Beethoven",
  key: "D",
  time: "4/4",
  // staff-spacing: 6mm,
  staves: (
    (
      clef: "treble",
      music: "
        f#4 f# g a | a8 b g4 f# e | d d e f# | f#4. e8 e2 |
        f#4 f# g a | a g f# e | d d e f# | e4. d8 d2 |
        e4 e f# d | e f#8 g8 f#4 d | e f#8 g8 f#4 e | d e a4 d |
        f#4 f# g a | a g f# e | d d e f# | e4. d8 d2
      ",
      fingerings: (
        // Line 1: measures 1-4
        2, 2, 4, 5,  5, 4, 2, 1,  1, 1, 2, 4,  4, 2, 2,
        // Line 2: measures 5-8
        2, 2, 4, 5,  5, 4, 2, 1,  1, 1, 2, 4,  2, 1, 1,
        // Line 3: measures 9-12
        2, 2, 4, 1,  2, 4, 4, 1,  2, 4, 4, 2,  1, 2, 5,
        // Line 4: measures 13-16
        2, 2, 4, 5,  5, 4, 2, 1,  1, 1, 2, 4,  2, 1, 1,
      ),
      chord-symbols: (
        // Line 1: measures 1-4
        ("D",), ("D/A", "A"), ("D",), ("A",),
        // Line 2: measures 5-8
        ("D",), ("A",), ("D",), ("A", "D"),
        // Line 3: measures 9-12
        ("A", "D"), ("A", "D"), ("A", "F#"), ("Bm", "Em", "A"),
        // Line 4: measures 13-16
        ("D",), ("A",), ("D",), ("A", "D"),
      ),
    ),
    (
      clef: "bass",
      music: "
        d1 | a, | d | a, |
        d | a, | d | a,2 d |
        a, d | a, d | a, f#, | b,4 e, a,2 |
        d1 | a, | d | a,2 d"
    )
  ),
)