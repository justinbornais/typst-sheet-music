// Basic rendering test - verifies core functionality

#import "../lib.typ": score, melody

#set page(width: 210mm, height: 297mm, margin: 1.5cm)

= Sheet Music Library - Edge Case Tests
This document provides a series of tests to verify the rendering of edge cases in the sheet music library. Each test focuses on a specific aspect of music notation that may present challenges in rendering, as well as specific examples of buggy outputs provided from other developers.

== Test 1: Dotted Note and Last 16th Note

#melody(
  clef: "treble",
  key: "G",
  music: "e8. b16 | a8. b16 | c'16. d'32 e'16. c'32 | c'16. d'32 e' c'16. | c'16. d'32  e' c'16. | g16 f e d c8"
)