#import "../lib.typ": melody, score

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

= Large Score (Many Lines)

#score(
  key: "C",
  time: "4/4",
  system-spacing: -4mm,
  staves: (
    (
      clef: "treble",
      music: "
      c4 d e f | g a b c' | c' b a g | f e d c
      d e f# g | a b c#' d' | d' c' b a | g f# e d
      e f# g# a | b c#' d#' e' | e' d#' c#' b | a g# f# e
      f g a b& | c' d' e' f' | f' e' d' c' | b& a g f
      c4 d e f | g a b c' | c' b a g | f e d c
      d e f# g | a b c#' d' | d' c' b a | g f# e d
      e f# g# a | b c#' d#' e' | e' d#' c#' b | a g# f# e
      f g a b& | c' d' e' f' | f' e' d' c' | b& a g f
      c4 d e f | g a b c' | c' b a g | f e d c
      d e f# g | a b c#' d' | d' c' b a | g f# e d
      e f# g# a | b c#' d#' e' | e' d#' c#' b | a g# f# e
      f g a b& | c' d' e' f' | f' e' d' c' | b& a g f
      c4 d e f | g a b c' | c' b a g | f e d c
      d e f# g | a b c#' d' | d' c' b a | g f# e d
      e f# g# a | b c#' d#' e' | e' d#' c#' b | a g# f# e
      f g a b& | c' d' e' f' | f' e' d' c' | b& a g f
      c4 d e f | g a b c' | c' b a g | f e d c
      d e f# g | a b c#' d' | d' c' b a | g f# e d
      e f# g# a | b c#' d#' e' | e' d#' c#' b | a g# f# e
      f g a b& | c' d' e' f' | f' e' d' c' | b& a g f
      "
    ),
  )
)