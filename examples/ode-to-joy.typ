// Ode to Joy - Full melody example
// Demonstrates system breaks, key signatures, and fingering numbers.

#import "../lib.typ": score

#set page(margin: 1.5cm)

#score(
  title: "Ode to Joy",
  composer: "L. van Beethoven",
  key: "D",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "
        |: f#'4 f#' g' a' | a'8 b' g'4 f#' e' | d' d' e' f#' | f#'4. e'8 e'2 |
        f#'4 f#' g' a' | a' g' f#' e' | d' d' e' f#' | e'4. d'8 d'2 |
        e'4 e' f#' d' | e' f#'8 g'8 f#'4 d' | e' f#'8 g'8 f#'4 e' | d' e' a4'2 |
        f#'4 f#' g' a' | a' g' f#' e' | d' d' e' f#' | e'4. d'8 d'2 :|
      ",
      fingerings: (
        // Line 1: measures 1-4
        2, 2, 4, 5,  5, 4, 2, 1,  1, 1, 2, 4,  4, 2, 2,
        // Line 2: measures 5-8
        2, 2, 4, 5,  5, 4, 2, 1,  1, 1, 2, 4,  2, 1, 1,
        // Line 4: measures 9-12
        2, 2, 4, 1,  2, 4, 4, 1,  2, 4, 4, 2,  1, 2, 5,
        // Line 5: measures 14-16
        2, 2, 4, 5,  5, 4, 2, 1,  1, 1, 2, 4,  2, 1, 1,
      ),
    ),
  ),
)
