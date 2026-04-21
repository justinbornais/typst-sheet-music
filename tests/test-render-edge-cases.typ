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

#v(1cm)

== Test 2: Multi-System First Ending with 8va, 15ma, Chords, and Fingerings

#score(
  key: "D",
  time: "4/4",
  width: 165mm,
  measures-per-line: 1,
  staves: (
    (
      clef: "treble",
      fingering-position: "above",
      music: "
        |: d'4[A] e' f#' a' |
        end{1.: 8a{<d'' f#'' a''>4n[1 *3* 5] <e'' g'' b''>4n[1 2 *5*] <f#'' a'' c#'''>4n[*1* 3 5] <g'' b'' d'''>4n[1 *2* 5] |
        <a'' c#''' e'''>4n[1 3 *5*] <b'' d''' f#'''>4n[*1* 2 5]} 15a{<c#''' e''' g'''>4n[1 *3* 5] <d''' f#''' a'''>4n[*1* 3 5]}} :|
        end{2.: d''2[A] f#'' | d''1}
      ",
    ),
  ),
)

#v(1cm)

== Test 3: Nested Tuplets Inside Opposing Voices

#score(
  key: "C",
  time: "6/8",
  staves: (
    (
      clef: "treble",
      music: "v{{2,3:c''8 b' a'} g'8 r e'';c'4. <g b d'>4.} | v{<a' c''>8 <b' d''> <c'' e''>  {2,3:d''16 e'' f''};r8 g4 r8}",
    ),
  ),
)

#v(1cm)

== Test 4: Dense Chords with Accidentals, Slurs, Ties, and Text Stacks

#score(
  key: "F",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "<b& d' f' a'>4([Bbmaj7]n[1 2 *4* 5] <c#' e' g' b'>4text[cluster] <d' f#' a' c''>4~ <d' f#' a' c''>4) | <e&' g&' b&' d''>2v[ff]exp[poco ten.] <f' a' c'' e''>2n[*1* 2 3 5]",
    ),
  ),
)

#v(1cm)

== Test 5: Inline Clef and Meter Changes Under Manual Spacing

#score(
  key: "C",
  time: "5/8",
  staves: (
    (
      clef: "treble",
      music: "c'8 d'   e' f' g' | 7/8 a'8 b' c'' d'' e'' f'' g'' | bass c,8 d, e, f, g, a, b, | 3/4 treble c''4 b' a'",
    ),
  ),
)

#v(1cm)

== Test 6: Grace Bursts, Slashes, Markers, and Hairpins

#score(
  key: "A",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "grace{c#''32 d'' e'' f#''/} g''4bm cresc{a''8 b'' c#''' d'''} | grace{g'16 a' b'/} c''4// decresc{b'8 a' g' f#'} | e'2tr ds e'coda r4",
    ),
  ),
)

#v(1cm)

== Test 7: Separate Staves with Synchronized But Unconnected Barlines

#score(
  key: "B&",
  time: "3/4",
  width: 160mm,
  staff-group: "separate",
  staves: (
    (
      clef: "treble",
      music: "8a{<f'' a'' c'''>4n[1 3 5] <e&'' g'' b&''> <d'' f'' a''>} | v{c,2 b&,,4;f,,4 r f,,} | <b& d' f'>2.",
    ),
    (
      clef: "alto",
      music: "v{f'4 g' a';<b& d' f'>2.} | c'4text[inner] d,, e' | <e& g b&>2.",
    ),
    (
      clef: "bass",
      music: "b&,2. | f,4 c f | b&,2.",
    ),
  ),
)

#v(1cm)

== Test 8: Cross-Staff Alignment with Long Durations and Tiny Notes

#score(
  key: "E",
  time: "4/4",
  staff-group: "grand",
  staves: (
    (
      clef: "treble",
      music: "e''32 f#'' g#'' a'' b'' c#''' d#''' e''' r16 e''8. | <g#' b' e''>breve",
    ),
    (
      clef: "bass",
      music: "e,1 | <e, b, e>breve",
    ),
  ),
)

#v(1cm)

== Test 9: Lyrics, Fingerings, Dynamics, and Repeat Barlines in One Line

#score(
  key: "G",
  time: "4/4",
  staves: (
    (
      clef: "treble",
      music: "|: g4l[Odd-]n[1] a4l[ly]v[p] b4l[spaced_]n[*3*] c'4l[text] | d'4l e'4l[lands]v[mf] f#'4l[on] g'4l[chords] :|: <b d' g'>2n[1 *3* 5]l[stacked] <a c' f#'>2l[words] :|",
    ),
  ),
)

#v(1cm)

== Test 10: Low Octave Line, Chord Seconds, and Below Fingerings

#score(
  key: "C",
  time: "4/4",
  staves: (
    (
      clef: "bass",
      fingering-position: "below",
      music: "15b{<c, d, e,>4n_[5 4 *2*] <d, e, f,>4n_[5 *3* 2] <e, f, g,>4n_[4 3 1] <f, g, a,>4n_[*5* 2 1]} | <g, a, b,>2n_[5 3 1] <c d e>2n_[*4* 2 1]",
    ),
  ),
)
