#import "../lib.typ": score

#set page(margin: 1.5cm, height: auto)

= Vertical Spacing Preset Comparison

== 1. Tight Vertical Spacing (`vertical-spacing: "tight"`)

#score(
  title: "Tight Spacing Demo",
  composer: "Test Composer",
  key: "G",
  time: "4/4",
  vertical-spacing: "tight",
  staves: (
    (
      clef: "treble",
      music: "
        g4n[1] a[D] b c' | d'4[C] b a g |
        end{1.: g4[D] a b c' | d'4[C] c' b a} :|
        end{2.: e'4[Em] d' c' b | a4[D] b c' d'} :|
        end{3.: g'4[G] f#' e' d' | c'4[D] b a g}
      ",
    ),
  ),
)

#v(1cm)

== 2. Regular Vertical Spacing (`vertical-spacing: "regular"`)

#score(
  title: "Regular Spacing Demo",
  composer: "Test Composer",
  key: "G",
  time: "4/4",
  vertical-spacing: "regular",
  staves: (
    (
      clef: "treble",
      music: "
        g4n[1] a[D] b c' | d'4[C] b a g |
        end{1.: g4[D] a b c' | d'4[C] c' b a} :|
        end{2.: e'4[Em] d' c' b | a4[D] b c' d'} :|
        end{3.: g'4[G] f#' e' d' | c'4[D] b a g}
      ",
    ),
  ),
)
