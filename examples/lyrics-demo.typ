#import "../lib.typ": score

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

#score(
  title: "Lyrics Demo",
  subtitle: "Inline lyric syllables, hyphens, melismas, and stacked verses",
  key: "C",
  time: "4/4",
  measures-per-line: 2,
  staves: (
    (
      clef: "treble",
      music: "
      c4l[Hel-]l[How-] dll[dy] el[lo]l[sold-] fl[there]l[ier!] | g4l[Hold_] al bl c'l[on]
      e4l[1. Ev-]l[2. Why_]l[3. In] dlll[You,] el['ry]l[do]l[O] fl[night]l[I]l[Lord,] | g4l[Sing-]l[Keep]l[Praise] alll[me] bl[with]l[watch-]l[to] c'l[joy]l[ing]l[God]
      ",
    ),
  ),
)
