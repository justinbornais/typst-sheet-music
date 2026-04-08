#import "../lib.typ": melody, score

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

= Large Score (Many Lines)
This includes tuplets.

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
      c4 {2,3:d8n[2] cn[1] dn[2]} e4 f | g a b c' | c' b a g | f e d c
      d {3:e8 r e} f#4 g | a b c#' d' | d' c' b a | g f# e d
      e f# g# a | b c#' d#' e' | e' d#' c#' b | a g# f# e
      f g a b& | c' d' e' f' | f' e' d' c' | b& a g f
      {8,5:c3 d e& c d} e= f | g a b c' | c' b a g | f e d c
      cresc{d e f# g | a b c#' d'} | decresc{d' c' b a | g f# e d}
      cresc{e f# g# a | b c#' d#' e'} | decresc{e' d#' c#' b | a g# f# e
      f g a b& | c' d' e' f'} | f' e' d' c' | b& a g f
      c4 d e f | g a b c' | c' b a g | f e d c
      d e f# g | a b c#' d' | d' c' b a | g f# e d
      e f# g# a | b c#' d#' e' | e' d#' c#' b | a g# f# e
      f g a b& | c' d' e' f' | f' e' d' c' | b& a g f
      c4 d e f | g a b c' | c' b a g | f e d c
      d e f# g | a b c#' d' | d' c' b a | g f# e d
      e f# g# a | b c#' d#' e' | e' d#' c#' b | a g# f# e
      "
    ),
  )
)
