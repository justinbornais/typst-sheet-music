// parser.typ - Music string parser
//
// Converts a music string like "c'4 d' e' f' | g'2 g'" into an array
// of event dictionaries (notes, rests, barlines, etc.)

#import "model.typ": make-note, make-rest, make-spacer, make-barline, make-line-break
#import "utils.typ": is-digit, is-lower, is-whitespace

/// Main entry: parse a music string into an array of events.
#let parse-music(input) = {
  let events = ()
  let pos = 0
  let len = input.len()
  let last-duration = 4   // sticky duration
  let last-dots = 0

  // Tuplet state: track open "{n" blocks
  let tuplet-start-idx = none
  let tuplet-n = none
  let tuplet-m = none

  // Helper: peek at current character (returns none at end)
  let peek(p) = {
    if p < len { input.at(p) } else { none }
  }

  while pos < len {
    let ch = input.at(pos)

    // --- Skip whitespace ---
    if ch == " " or ch == "\t" or ch == "\r" {
      pos += 1
      continue
    }

    // --- Newlines signal system breaks ---
    if ch == "\n" {
      pos += 1
      // Skip subsequent whitespace/blank lines
      while pos < len {
        let nc = input.at(pos)
        if nc == " " or nc == "\t" or nc == "\r" or nc == "\n" {
          pos += 1
        } else {
          break
        }
      }
      // Only emit a line-break if there's more content ahead and we already have events
      if pos < len and events.len() > 0 {
        events.push(make-line-break())
      }
      continue
    }

    // --- Barlines ---
    // Check multi-character barlines first
    if ch == "|" {
      // Look ahead for double, final, repeat barlines
      let next = peek(pos + 1)
      if next == "|" {
        // "||" - double barline
        events.push(make-barline(style: "double"))
        pos += 2
        continue
      } else if next == "." {
        // "|." - final barline
        events.push(make-barline(style: "final"))
        pos += 2
        continue
      } else {
        // "|" - single barline
        events.push(make-barline(style: "single"))
        pos += 1
        continue
      }
    }

    // --- Notes (a-g) ---
    if ch >= "a" and ch <= "g" and ch != "b" {
      // Definitely a note (a, c, d, e, f, g)
      let name = ch
      pos += 1
      let accidental = none
      let octave = 4

      // Parse accidental
      let ac = peek(pos)
      if ac == "#" {
        pos += 1
        if peek(pos) == "#" {
          accidental = "double-sharp"
          pos += 1
        } else {
          accidental = "sharp"
        }
      } else if ac == "&" {
        pos += 1
        if peek(pos) == "&" {
          accidental = "double-flat"
          pos += 1
        } else {
          accidental = "flat"
        }
      } else if ac == "=" {
        accidental = "natural"
        pos += 1
      }

      // If a single digit immediately follows (before any tick/comma) and the
      // character after that digit is ' or ,, treat the digit as an absolute
      // octave number.  Otherwise fall through to the relative-tick path.
      let abs-oct-char = peek(pos)
      if abs-oct-char != none and is-digit(abs-oct-char) {
        let next-after = peek(pos + 1)
        if next-after == "'" or next-after == "," {
          octave = int(abs-oct-char)
          pos += 1
          // Absorb any following ticks/commas as visual separators (do not
          // shift the octave since it was specified absolutely).
          while peek(pos) == "'" or peek(pos) == "," { pos += 1 }
        } else {
          // No absolute octave - parse relative tick/comma modifiers.
          while peek(pos) == "'" { octave += 1; pos += 1 }
          while peek(pos) == "," { octave -= 1; pos += 1 }
        }
      } else {
        // No digit at all - parse relative tick/comma modifiers.
        while peek(pos) == "'" { octave += 1; pos += 1 }
        while peek(pos) == "," { octave -= 1; pos += 1 }
      }

      // Parse duration
      let duration = last-duration
      let dur-str = ""
      while peek(pos) != none and is-digit(peek(pos)) {
        dur-str += peek(pos)
        pos += 1
      }
      if dur-str.len() > 0 {
        duration = int(dur-str)
      }

      // Parse dots
      let dots = 0
      while peek(pos) == "." {
        dots += 1
        pos += 1
      }

      // Parse tie
      let tie = false
      if peek(pos) == "~" {
        tie = true
        pos += 1
      }

      // Parse slur start/end
      let slur-start = false
      let slur-end = false
      if peek(pos) == "(" {
        slur-start = true
        pos += 1
      }
      if peek(pos) == ")" {
        slur-end = true
        pos += 1
      }

      // Parse beam markers
      let beam-start = false
      let beam-end = false
      if peek(pos) == "[" {
        beam-start = true
        pos += 1
      }
      if peek(pos) == "]" {
        beam-end = true
        pos += 1
      }

      last-duration = duration
      last-dots = dots

      events.push(make-note(
        name,
        accidental: accidental,
        octave: octave,
        duration: duration,
        dots: dots,
        tie: tie,
        slur-start: slur-start,
        slur-end: slur-end,
        beam-start: beam-start,
        beam-end: beam-end,
      ))
      continue
    }

    // --- Handle "b" which is ambiguous (note B or flat indicator) ---
    if ch == "b" {
      // "b" is the note B - flats use "&" prefix in this syntax
      let name = "b"
      pos += 1
      let accidental = none
      let octave = 4

      // Parse accidental
      let ac = peek(pos)
      if ac == "#" {
        pos += 1
        if peek(pos) == "#" {
          accidental = "double-sharp"
          pos += 1
        } else {
          accidental = "sharp"
        }
      } else if ac == "&" {
        pos += 1
        if peek(pos) == "&" {
          accidental = "double-flat"
          pos += 1
        } else {
          accidental = "flat"
        }
      } else if ac == "=" {
        accidental = "natural"
        pos += 1
      }

      // Absolute octave: single digit before tick/comma → absolute octave.
      let abs-oct-char = peek(pos)
      if abs-oct-char != none and is-digit(abs-oct-char) {
        let next-after = peek(pos + 1)
        if next-after == "'" or next-after == "," {
          octave = int(abs-oct-char)
          pos += 1
          while peek(pos) == "'" or peek(pos) == "," { pos += 1 }
        } else {
          while peek(pos) == "'" { octave += 1; pos += 1 }
          while peek(pos) == "," { octave -= 1; pos += 1 }
        }
      } else {
        while peek(pos) == "'" { octave += 1; pos += 1 }
        while peek(pos) == "," { octave -= 1; pos += 1 }
      }

      // Parse duration
      let duration = last-duration
      let dur-str = ""
      while peek(pos) != none and is-digit(peek(pos)) {
        dur-str += peek(pos)
        pos += 1
      }
      if dur-str.len() > 0 {
        duration = int(dur-str)
      }

      // Parse dots
      let dots = 0
      while peek(pos) == "." {
        dots += 1
        pos += 1
      }

      // Parse tie
      let tie = false
      if peek(pos) == "~" {
        tie = true
        pos += 1
      }

      // Parse slur
      let slur-start = false
      let slur-end = false
      if peek(pos) == "(" {
        slur-start = true
        pos += 1
      }
      if peek(pos) == ")" {
        slur-end = true
        pos += 1
      }

      // Parse beam markers
      let beam-start = false
      let beam-end = false
      if peek(pos) == "[" {
        beam-start = true
        pos += 1
      }
      if peek(pos) == "]" {
        beam-end = true
        pos += 1
      }

      last-duration = duration
      last-dots = dots

      events.push(make-note(
        name,
        accidental: accidental,
        octave: octave,
        duration: duration,
        dots: dots,
        tie: tie,
        slur-start: slur-start,
        slur-end: slur-end,
        beam-start: beam-start,
        beam-end: beam-end,
      ))
      continue
    }

    // --- Rests ---
    if ch == "r" {
      pos += 1
      let duration = last-duration
      let dur-str = ""
      while peek(pos) != none and is-digit(peek(pos)) {
        dur-str += peek(pos)
        pos += 1
      }
      if dur-str.len() > 0 {
        duration = int(dur-str)
      }
      let dots = 0
      while peek(pos) == "." {
        dots += 1
        pos += 1
      }
      last-duration = duration
      events.push(make-rest(duration: duration, dots: dots))
      continue
    }

    // --- Spacers (invisible rests) ---
    if ch == "s" {
      pos += 1
      let duration = last-duration
      let dur-str = ""
      while peek(pos) != none and is-digit(peek(pos)) {
        dur-str += peek(pos)
        pos += 1
      }
      if dur-str.len() > 0 {
        duration = int(dur-str)
      }
      let dots = 0
      while peek(pos) == "." {
        dots += 1
        pos += 1
      }
      last-duration = duration
      events.push(make-spacer(duration: duration, dots: dots))
      continue
    }

    // --- Slur start/end (when not after a note) ---
    if ch == "(" {
      // Attach to previous note if possible
      if events.len() > 0 {
        let last = events.last()
        if last.type == "note" {
          events.at(events.len() - 1).slur-start = true
        }
      }
      pos += 1
      continue
    }
    if ch == ")" {
      if events.len() > 0 {
        let last = events.last()
        if last.type == "note" {
          events.at(events.len() - 1).slur-end = true
        }
      }
      pos += 1
      continue
    }

    // --- Tuplet start: "{n" or "{n:m" ---
    if ch == "{" {
      pos += 1
      // Skip optional whitespace
      while pos < len and (input.at(pos) == " " or input.at(pos) == "\t") { pos += 1 }
      // Parse n (required digit string)
      let n-str = ""
      while pos < len and is-digit(input.at(pos)) {
        n-str += input.at(pos)
        pos += 1
      }
      if n-str.len() > 0 {
        let tn = int(n-str)
        // Parse optional :m
        let tm = none
        if pos < len and input.at(pos) == ":" {
          pos += 1
          let m-str = ""
          while pos < len and is-digit(input.at(pos)) {
            m-str += input.at(pos)
            pos += 1
          }
          if m-str.len() > 0 { tm = int(m-str) }
        }
        if tm == none {
          // Default m: largest power of 2 strictly less than n
          let m = 1
          while m * 2 < tn { m = m * 2 }
          tm = m
        }
        // Skip whitespace after header
        while pos < len and (input.at(pos) == " " or input.at(pos) == "\t") { pos += 1 }
        tuplet-start-idx = events.len()
        tuplet-n = tn
        tuplet-m = tm
      }
      continue
    }

    // --- Tuplet end: "}" ---
    if ch == "}" {
      if tuplet-start-idx != none {
        let end-idx = events.len()
        for i in range(tuplet-start-idx, end-idx) {
          events.at(i).tuplet-n = tuplet-n
          events.at(i).tuplet-m = tuplet-m
          if i == tuplet-start-idx { events.at(i).tuplet-start = true }
          if i == end-idx - 1 { events.at(i).tuplet-end = true }
        }
        tuplet-start-idx = none
        tuplet-n = none
        tuplet-m = none
      }
      pos += 1
      continue
    }

    // --- Unknown character: skip ---
    pos += 1
  }

  events
}
