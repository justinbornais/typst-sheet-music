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

#v(1cm)

== Test 11: First Ending Spanning Four Forced Staff Lines

#score(
  key: "C",
  time: "4/4",
  width: 122mm,
  measures-per-line: 1,
  staves: (
    (
      clef: "treble",
      fingering-position: "above",
      music: "
        |: c'4[A] d' e' f' |
        end{1.-long: c''4text[start high] b' a' g' |
        8a{<e'' g'' c'''>4n[1 3 5] <f'' a'' d'''>4n[1 *3* 5] <g'' b'' e'''>4n[*1* 2 5] <a'' c''' f'''>4n[1 2 *5*]} |
        15a{b''4 c''' d''' e'''} |
        f'''1text[far end]} :|
        end{2.: c'1text[short exit]}
      ",
    ),
  ),
)

#v(1cm)

== Test 12: Deliberately Awkward Nested Tuplets and Voice Tuplets

#score(
  key: "C",
  time: "4/4",
  width: 150mm,
  staves: (
    (
      clef: "treble",
      music: "{3,2:c''8 {5,4:d''16 e'' f'' g'' a''} b'8} | v{{7,4:c'''16 b'' a'' g'' f'' e'' d''};{3,2:c'8 e' g'}} | {5,4:<c' e' g'>16 <d' f' a'> <e' g' b'> <f' a' c''> <g' b' d''>}",
    ),
  ),
)

#v(1cm)

== Test 13: Mixed Nested Spans Over an Ending

#score(
  key: "D",
  time: "4/4",
  width: 150mm,
  staves: (
    (
      clef: "treble",
      music: "
        |: d'4 e' f#' a' |
        end{1. stack: 15a{tr{c'''4text[top trill] d''' e''' f'''}} |
        cresc{8a{<g'' b'' d'''>4[A7]n[1 2 5] <a'' c''' e'''> <b'' d''' f#'''> <c''' e''' g'''>}} |
        grace{d'''32 c''' b'' a''/} g''2v[ff]exp[under] f#''2} :|
        end{2.: d''1v[p]}
      ",
    ),
  ),
)

#v(1cm)

== Test 14: Dense Single-Staff Annotation Collision Stack

#score(
  key: "F",
  time: "4/4",
  width: 145mm,
  staves: (
    (
      clef: "treble",
      fingering-position: "above",
      music: "<b& d' f#' a&' c'' e&''>8([C13b9]n[1 *2* 3 4 5]l[too-]text[above chord]v[ffff] <c#' e' g' b' d'' f''>8)l[many_]exp[below stack]   r8text[rest text]v[pppp] s8 | <d&' e&' f' g&' a' b&'>4[A cluster]n[*1* *2* *3* *4* *5*] <e' f' g' a' b' c''>4text[seconds]exp[collision] <f' a' c'' e''>2l[end]",
    ),
  ),
)

#v(1cm)

== Test 15: Extreme Ledger Lines, Inline Clefs, and Opposed Octave Lines

#score(
  key: "C",
  time: "4/4",
  width: 150mm,
  staves: (
    (
      clef: "treble",
      music: "8a{c''''32 d'''' e'''' f'''' g'''' a'''' b'''' c'''''} | bass 15b{c,,,,32 d,,,, e,,,, f,,,, g,,,, a,,,, b,,,, c,,,} | treble <c,,,, c''''>1text[split register]",
    ),
  ),
)

#v(1cm)

== Test 16: Grand Staff Ending Across Systems With Opposite Vertical Pressure

#score(
  key: "G",
  time: "4/4",
  width: 135mm,
  measures-per-line: 1,
  staff-group: "grand",
  staff-spacing: 5mm,
  staves: (
    (
      clef: "treble",
      music: "
        |: g'4 b' d'' g'' |
        end{1.: 8a{<b'' d''' g'''>2text[upper volta] <a'' c''' f#'''>2} |
        tr{g''4 f#'' e'' d''} |
        c''1text[end high]} :|
        end{2.: g'1}
      ",
    ),
    (
      clef: "bass",
      fingering-position: "below",
      music: "
        |: g,1 |
        end{1.: 15b{<g,, d, b,>2n_[5 2 1] <a,, e, c>2n_[5 3 1]} |
        decresc{g,,4 f,, e,, d,,} |
        c,,1exp[low end]} :|
        end{2.: g,1}
      ",
    ),
  ),
)

#v(1cm)

== Test 17: Bracketed Four-Staff System With Crowded Above and Below Content

#score(
  key: "B&",
  time: "3/4",
  width: 160mm,
  staff-group: "bracket",
  staff-spacing: 4mm,
  staves: (
    (
      clef: "treble",
      instrument-name: "Tiny top text",
      music: "c''4text[very high label][Bb] d''v[fff] e''tr | cresc{{3,2:f''8 g'' a''} b''4} c'''4",
    ),
    (
      clef: "alto",
      instrument-name: "Middle seconds",
      music: "v{<b& d' f'>2.text[upper voice];<c e& g&>4exp[lower text] r <d f a&>} | end{1.: <e& g& b&>2.text[inner ending]}",
    ),
    (
      clef: "tenor",
      instrument-name: "Tenor switches",
      music: "tenor f4 g a | treble c'4text[clef jump] bass f, exp[return] c",
    ),
    (
      clef: "bass",
      instrument-name: "Lowest marks",
      fingering-position: "below",
      music: "15b{b&,,4n_[5] a,,n_[4] g,,n_[3]} | decresc{f,,4v[pp] e,, d,,}",
    ),
  ),
)

#v(1cm)

== Test 18: Multi-Staff Alignment With Maxima, Breves, Spacers, and Tiny Runs

#score(
  key: "E",
  time: "4/4",
  width: 155mm,
  staff-group: "separate",
  staves: (
    (
      clef: "treble",
      music: "e'''64 d''' c#''' b'' a'' g#'' f#'' e'' d'' c#'' b' a' g#' f#' e' d' | c#'maxima",
    ),
    (
      clef: "alto",
      music: "s1 | <e g# b>breve text[breve spacer stress] sbreve",
    ),
    (
      clef: "bass",
      music: "e,,1 | rmaxima exp[huge silent duration]",
    ),
  ),
)

#v(1cm)

== Test 19: Multi-Staff Voice Groups Containing Tuplets and Nested Components

#score(
  key: "A",
  time: "6/8",
  width: 150mm,
  staff-group: "grand",
  staves: (
    (
      clef: "treble",
      music: "v{{3,2:c''8 d'' e''} 8a{f''8 g'' a''};c'4. {5,4:e'16 f#' g#' a' b'}} | v{tr{c'''8 b'' a''};15b{a,8 g, f,}}",
    ),
    (
      clef: "bass",
      music: "v{c,4. g,4.;{7,4:c,,16 d,, e,, f,, g,, a,, b,,} c,8 r} | cresc{<a,, e, c>8 <b,, f# b,> <c, g c'> <d, a d'> <e, b, e> <f, c f>}",
    ),
  ),
)

#v(1cm)

== Test 20: Repeat Endings Over Rests, Spacers, Clef Changes, and Meter Changes

#score(
  key: "C",
  time: "5/8",
  width: 140mm,
  measures-per-line: 2,
  staves: (
    (
      clef: "treble",
      music: "
        |: c'8 d' e' f' g' |
        end{1. rests: r8text[rest start] s8 bass c,8 d, e, | 7/8 treble f'8 g' a' b' c'' d'' e''} :|
        end{2. clefs: alto c'4text[alto inside] d'8 tenor e'4 | 3/4 treble <f' a' c''>2.[last chord]}
      ",
    ),
  ),
)
