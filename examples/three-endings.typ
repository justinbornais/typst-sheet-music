#import "../lib.typ": score

#set page(margin: 1.5cm)

#score(
  title: "Three Endings Demo",
  subtitle: "Volta brackets with three alternate endings",
  key: "G",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "
        |: g4[D] a b c' | d'4[C] b a g |
        end{1.: g4[D] a b c' | d'4[C] c' b a} :|
        end{2.: e'4[Em] d' c' b | a4[D] b c' d'} :|
        end{3.: g'4[G] f#' e' d' | c'4[D] b a g}
      ",
    ),
    (
      clef: "bass",
      music: "
        |: g,1 | d1 |
        end{1.: g,2 c | d1} :|
        end{2.: e,2 e | d1} :|
        end{3.: g,2 d | g,1}
      ",
    ),
  ),
  staff-group: "grand",
)
