// Ode to Joy - Full melody example
// Demonstrates system breaks, key signatures, fingering numbers, chord symbols,
// dynamics, and articulations.

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
        f#4n[3][D] f# g a | a8[D/A] b g4 f#[A] e | d[D] d e f# | f#4.[A] e8 e2 |
        f#4[D] f# g a | a[A] g f# e | d[D] d e f# | e4.[A] d8 d2[D] |
        |: e4[A] e f#[D] d | e[A] f#8 g8 f#4[D] d | e[A] f#8 g8 f#4[F#] e | d[Bm] e[Em] a4[A] d |
        f#4[D] f# g a | a[A] g f# e | d[D] d e f# | e4.[A] d8 d2[D] :|
      ",
    ),
    (
      clef: "bass",
      fingering-position: "below",
      music: "
        d1n[1] | a, | d | a, |
        d | a, | d | a,2 d4 r |
        |: a,2 d | a, d | a, f#, | b,4 en[1], a,2 |
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
        f#4(mf)[D] f# g a | a8[D/A] b g4 f#[A] e | d[D] d e f# | f#4.[A] e8 e2 |
        f#4(f)[D] f# g a | a>[A] g f# e | d[D] d e f# | e4.[A] d8 d2[D] |
        e4(p)[A] e f#[D] d | e[A] f#8 g8 f#4[D] d | e[A] f#8 g8 f#4[F#] e | d[Bm] e[Em] a4_(ff)[A] d |
        f#4[D] f# g a | a>[A] g* f# e | d[D] d e f# | e4.[A] d8 d2_[D]
      ",
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