// parser.typ - Music string parser
//
// Converts a music string like "c'4 d' e' f' | g'2 g'" into an array
// of event dictionaries (notes, rests, barlines, etc.)

#import "model.typ": make-note, make-rest, make-spacer, make-barline, make-line-break, make-chord, make-clef
#import "constants.typ": supported-clefs, clef-default-base-octave
#import "utils.typ": is-digit, is-lower, is-whitespace

/// Main entry: parse a music string into an array of events.
/// - base-octave: the default octave number when no ' or , markers are given.
///   Use 4 for treble clef (C4 = middle C), 3 for bass clef (C3 = low C).
#let parse-music(input, base-octave: 4) = {
  let events = ()
  let pos = 0
  let len = input.len()
  let last-duration = 4   // sticky duration
  let last-dots = 0
  let current-base-octave = base-octave

  // Tuplet state: track open "{n" blocks
  let tuplet-start-idx = none
  let tuplet-n = none
  let tuplet-m = none

  // Octave-line state: track open "n[a|b]{" blocks (e.g. 8a{...}, 15b{...})
  let octline-start-idx = none
  let octline-number = none
  let octline-dir = none

  // Helper: peek at current character (returns none at end)
  let peek(p) = {
    if p < len { input.at(p) } else { none }
  }

  let is-word-char(ch) = {
    ch != none and (is-lower(ch) or is-digit(ch) or ch == "-")
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
        if peek(pos + 2) == ":" {
          // "||:" - repeat start
          events.push(make-barline(style: "repeat-start"))
          pos += 3
        } else {
          // "||" - double barline
          events.push(make-barline(style: "double"))
          pos += 2
        }
      } else if next == ":" {
        // "|:" - repeat start
        events.push(make-barline(style: "repeat-start"))
        pos += 2
      } else if next == "." {
        // "|." - final barline
        events.push(make-barline(style: "final"))
        pos += 2
      } else {
        // "|" - single barline
        events.push(make-barline(style: "single"))
        pos += 1
      }
      continue
    }

    // --- Repeat-end barlines starting with ":" ---
    if ch == ":" {
      let next = peek(pos + 1)
      if next == "|" {
        if peek(pos + 2) == "|" {
          // ":||" - repeat end
          events.push(make-barline(style: "repeat-end"))
          pos += 3
        } else {
          // ":|" - repeat end
          events.push(make-barline(style: "repeat-end"))
          pos += 2
        }
      } else {
        // Unknown colon - skip
        pos += 1
      }
      continue
    }

    // --- Chords: <note1 note2 ...>duration ---
    if ch == "<" {
      pos += 1
      let chord-notes = ()
      while pos < len and input.at(pos) != ">" {
        let c = input.at(pos)
        if c == " " or c == "\t" or c == "\r" or c == "\n" { pos += 1; continue }
        if (c >= "a" and c <= "g") {
          let cname = c
          pos += 1
          let caccidental = none
          let coctave = current-base-octave
          // Parse accidental
          let cac = peek(pos)
          if cac == "#" {
            pos += 1
            if peek(pos) == "#" { caccidental = "double-sharp"; pos += 1 }
            else { caccidental = "sharp" }
          } else if cac == "&" {
            pos += 1
            if peek(pos) == "&" { caccidental = "double-flat"; pos += 1 }
            else { caccidental = "flat" }
          } else if cac == "=" { caccidental = "natural"; pos += 1 }
          // Parse octave markers
          while peek(pos) == "'" { coctave += 1; pos += 1 }
          while peek(pos) == "," { coctave -= 1; pos += 1 }
          chord-notes.push((name: cname, accidental: caccidental, octave: coctave))
        } else {
          pos += 1
        }
      }
      // Consume the closing ">"
      if pos < len and input.at(pos) == ">" { pos += 1 }

      // Parse duration
      let duration = last-duration
      let dur-str = ""
      while peek(pos) != none and is-digit(peek(pos)) { dur-str += peek(pos); pos += 1 }
      if dur-str.len() > 0 { duration = int(dur-str) }
      // Parse dots
      let dots = 0
      while peek(pos) == "." { dots += 1; pos += 1 }
      // Parse tie
      let tie = false
      if peek(pos) == "~" { tie = true; pos += 1 }

      // Parse articulations: > (accent), * (staccato), - (tenuto), _ (fermata)
      let articulations = ()
      while peek(pos) == ">" or peek(pos) == "*" or peek(pos) == "-" or peek(pos) == "_" {
        let ac = peek(pos)
        if ac == ">" { articulations.push("accent") }
        else if ac == "*" { articulations.push("staccato") }
        else if ac == "-" { articulations.push("tenuto") }
        else if ac == "_" { articulations.push("fermata") }
        pos += 1
      }

      // Parse dynamic: v[text] e.g. v[mf], v[ff]
      let dynamic = none
      if peek(pos) == "v" and pos + 1 < len and input.at(pos + 1) == "[" {
        pos += 2 // skip "v["
        let dyn-str = ""
        while pos < len and input.at(pos) != "]" {
          dyn-str += input.at(pos)
          pos += 1
        }
        if pos < len { pos += 1 } // skip "]"
        if dyn-str.len() > 0 {
          dynamic = dyn-str
        }
      }

      // Allow tie after articulations/dynamics (e.g. c4v[pp]~ c)
      if not tie and peek(pos) == "~" {
        tie = true
        pos += 1
      }

      // Parse slur
      let slur-start = false
      let slur-end = false
      if peek(pos) == "(" { slur-start = true; pos += 1 }
      if peek(pos) == ")" { slur-end = true; pos += 1 }
      // Parse beam markers ([ is beam-start unless followed by A-G = chord symbol)
      let beam-start = false
      let beam-end = false
      if peek(pos) == "[" {
        let nxt = if pos + 1 < len { input.at(pos + 1) } else { none }
        if nxt == none or not (nxt >= "A" and nxt <= "G") {
          beam-start = true
          pos += 1
        }
      }
      if peek(pos) == "]" { beam-end = true; pos += 1 }

      // Parse inline chord symbol [text] and fingering n[digits] or n_[digits]
      let chord-symbol = none
      let fingering = none
      let fingering-position = "above"
      while peek(pos) == "[" or (peek(pos) == "n" and pos + 1 < len and (input.at(pos + 1) == "[" or (input.at(pos + 1) == "_" and pos + 2 < len and input.at(pos + 2) == "["))) {
        if peek(pos) == "n" and pos + 1 < len and (input.at(pos + 1) == "[" or (input.at(pos + 1) == "_" and pos + 2 < len and input.at(pos + 2) == "[")) {
          if input.at(pos + 1) == "_" {
            fingering-position = "below"
            pos += 3
          } else {
            pos += 2
          }
          let fng-str = ""
          while pos < len and input.at(pos) != "]" {
            fng-str += input.at(pos)
            pos += 1
          }
          if pos < len { pos += 1 }
          let parts = fng-str.split(" ").filter(s => s.len() > 0)
          if parts.len() == 1 {
            fingering = int(parts.at(0))
          } else if parts.len() > 1 {
            fingering = parts.map(s => int(s))
          }
        } else if peek(pos) == "[" {
          pos += 1
          let sym-str = ""
          while pos < len and input.at(pos) != "]" {
            sym-str += input.at(pos)
            pos += 1
          }
          if pos < len { pos += 1 }
          if sym-str.len() > 0 {
            chord-symbol = sym-str
          }
        }
      }

      last-duration = duration
      last-dots = dots

      if chord-notes.len() > 0 {
        events.push(make-chord(
          chord-notes,
          duration: duration,
          dots: dots,
          tie: tie,
          slur-start: slur-start,
          slur-end: slur-end,
          beam-start: beam-start,
          beam-end: beam-end,
          articulations: articulations,
          dynamic: dynamic,
          fingering: fingering,
          fingering-position: fingering-position,
          chord-symbol: chord-symbol,
        ))
      }
      continue
    }

    // --- Inline clef changes ---
    if is-lower(ch) {
      let word-end = pos
      while word-end < len and is-word-char(input.at(word-end)) {
        word-end += 1
      }
      let token = input.slice(pos, word-end)
      if supported-clefs.contains(token) {
        events.push(make-clef(token))
        current-base-octave = clef-default-base-octave(token)
        pos = word-end
        continue
      }
    }

    // --- Notes (a-g) ---
    if ch >= "a" and ch <= "g" and ch != "b" {
      // Definitely a note (a, c, d, e, f, g)
      let name = ch
      pos += 1
      let accidental = none
      let octave = current-base-octave

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

      // Parse octave markers
      while peek(pos) == "'" {
        octave += 1
        pos += 1
      }
      while peek(pos) == "," {
        octave -= 1
        pos += 1
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
      // If no dots specified but duration wasn't given, use last dots? No, dots don't stick.
      // Actually per the spec, only duration is sticky. Dots are per-note.

      // Parse tie
      let tie = false
      if peek(pos) == "~" {
        tie = true
        pos += 1
      }

      // Parse articulations: > (accent), * (staccato), - (tenuto), _ (fermata)
      let articulations = ()
      while peek(pos) == ">" or peek(pos) == "*" or peek(pos) == "-" or peek(pos) == "_" {
        let ac = peek(pos)
        if ac == ">" { articulations.push("accent") }
        else if ac == "*" { articulations.push("staccato") }
        else if ac == "-" { articulations.push("tenuto") }
        else if ac == "_" { articulations.push("fermata") }
        pos += 1
      }

      // Parse dynamic: v[text] e.g. v[mf], v[ff]
      let dynamic = none
      if peek(pos) == "v" and pos + 1 < len and input.at(pos + 1) == "[" {
        pos += 2 // skip "v["
        let dyn-str = ""
        while pos < len and input.at(pos) != "]" {
          dyn-str += input.at(pos)
          pos += 1
        }
        if pos < len { pos += 1 } // skip "]"
        if dyn-str.len() > 0 {
          dynamic = dyn-str
        }
      }

      // Allow tie after articulations/dynamics (e.g. c4v[pp]~ c)
      if not tie and peek(pos) == "~" {
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

      // Parse beam markers ([ is beam-start unless followed by A-G = chord symbol)
      let beam-start = false
      let beam-end = false
      if peek(pos) == "[" {
        let nxt = if pos + 1 < len { input.at(pos + 1) } else { none }
        if nxt == none or not (nxt >= "A" and nxt <= "G") {
          beam-start = true
          pos += 1
        }
      }
      if peek(pos) == "]" {
        beam-end = true
        pos += 1
      }

      // Parse inline chord symbol [text] and fingering n[digits] or n_[digits]
      // These can appear in any order: c4[C/E]n[3] or c4n[3][C/E]
      let chord-symbol = none
      let fingering = none
      let fingering-position = "above"
      while peek(pos) == "[" or (peek(pos) == "n" and pos + 1 < len and (input.at(pos + 1) == "[" or (input.at(pos + 1) == "_" and pos + 2 < len and input.at(pos + 2) == "["))) {
        if peek(pos) == "n" and pos + 1 < len and (input.at(pos + 1) == "[" or (input.at(pos + 1) == "_" and pos + 2 < len and input.at(pos + 2) == "[")) {
          // Fingering: n[...] = above, n_[...] = below
          if input.at(pos + 1) == "_" {
            fingering-position = "below"
            pos += 3 // skip "n_["
          } else {
            pos += 2 // skip "n["
          }
          let fng-str = ""
          while pos < len and input.at(pos) != "]" {
            fng-str += input.at(pos)
            pos += 1
          }
          if pos < len { pos += 1 } // skip "]"
          let parts = fng-str.split(" ").filter(s => s.len() > 0)
          if parts.len() == 1 {
            fingering = int(parts.at(0))
          } else if parts.len() > 1 {
            fingering = parts.map(s => int(s))
          }
        } else if peek(pos) == "[" {
          // Chord symbol: [...]
          pos += 1 // skip "["
          let sym-str = ""
          while pos < len and input.at(pos) != "]" {
            sym-str += input.at(pos)
            pos += 1
          }
          if pos < len { pos += 1 } // skip "]"
          if sym-str.len() > 0 {
            chord-symbol = sym-str
          }
        }
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
        articulations: articulations,
        dynamic: dynamic,
        fingering: fingering,
        fingering-position: fingering-position,
        chord-symbol: chord-symbol,
      ))
      continue
    }

    // --- Handle "b" which is ambiguous (note B or flat indicator) ---
    if ch == "b" {
      // "b" is the note B - flats use "&" prefix in this syntax
      let name = "b"
      pos += 1
      let accidental = none
      let octave = current-base-octave

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

      // Parse octave markers
      while peek(pos) == "'" {
        octave += 1
        pos += 1
      }
      while peek(pos) == "," {
        octave -= 1
        pos += 1
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

      // Parse articulations: > (accent), * (staccato), - (tenuto), _ (fermata)
      let articulations = ()
      while peek(pos) == ">" or peek(pos) == "*" or peek(pos) == "-" or peek(pos) == "_" {
        let ac = peek(pos)
        if ac == ">" { articulations.push("accent") }
        else if ac == "*" { articulations.push("staccato") }
        else if ac == "-" { articulations.push("tenuto") }
        else if ac == "_" { articulations.push("fermata") }
        pos += 1
      }

      // Parse dynamic: v[text] e.g. v[mf], v[ff]
      let dynamic = none
      if peek(pos) == "v" and pos + 1 < len and input.at(pos + 1) == "[" {
        pos += 2 // skip "v["
        let dyn-str = ""
        while pos < len and input.at(pos) != "]" {
          dyn-str += input.at(pos)
          pos += 1
        }
        if pos < len { pos += 1 } // skip "]"
        if dyn-str.len() > 0 {
          dynamic = dyn-str
        }
      }

      // Allow tie after articulations/dynamics (e.g. b4v[pp]~ b)
      if not tie and peek(pos) == "~" {
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

      // Parse beam markers ([ is beam-start unless followed by A-G = chord symbol)
      let beam-start = false
      let beam-end = false
      if peek(pos) == "[" {
        let nxt = if pos + 1 < len { input.at(pos + 1) } else { none }
        if nxt == none or not (nxt >= "A" and nxt <= "G") {
          beam-start = true
          pos += 1
        }
      }
      if peek(pos) == "]" {
        beam-end = true
        pos += 1
      }

      // Parse inline chord symbol [text] and fingering n[digits] or n_[digits]
      let chord-symbol = none
      let fingering = none
      let fingering-position = "above"
      while peek(pos) == "[" or (peek(pos) == "n" and pos + 1 < len and (input.at(pos + 1) == "[" or (input.at(pos + 1) == "_" and pos + 2 < len and input.at(pos + 2) == "["))) {
        if peek(pos) == "n" and pos + 1 < len and (input.at(pos + 1) == "[" or (input.at(pos + 1) == "_" and pos + 2 < len and input.at(pos + 2) == "[")) {
          if input.at(pos + 1) == "_" {
            fingering-position = "below"
            pos += 3
          } else {
            pos += 2
          }
          let fng-str = ""
          while pos < len and input.at(pos) != "]" {
            fng-str += input.at(pos)
            pos += 1
          }
          if pos < len { pos += 1 }
          let parts = fng-str.split(" ").filter(s => s.len() > 0)
          if parts.len() == 1 {
            fingering = int(parts.at(0))
          } else if parts.len() > 1 {
            fingering = parts.map(s => int(s))
          }
        } else if peek(pos) == "[" {
          pos += 1
          let sym-str = ""
          while pos < len and input.at(pos) != "]" {
            sym-str += input.at(pos)
            pos += 1
          }
          if pos < len { pos += 1 }
          if sym-str.len() > 0 {
            chord-symbol = sym-str
          }
        }
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
        articulations: articulations,
        dynamic: dynamic,
        fingering: fingering,
        fingering-position: fingering-position,
        chord-symbol: chord-symbol,
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

    // --- Tuplet start: "{n,m:" ---
    // n = number of beats this tuplet occupies for spacing
    // m = the tuplet number displayed on the bracket
    if ch == "{" {
      pos += 1
      // Skip optional whitespace
      while pos < len and (input.at(pos) == " " or input.at(pos) == "\t") { pos += 1 }
      // Parse n (beats, required digit string)
      let n-str = ""
      while pos < len and is-digit(input.at(pos)) {
        n-str += input.at(pos)
        pos += 1
      }
      if n-str.len() > 0 {
        let tb = int(n-str)  // tuplet-beats
        let tn = tb           // tuplet-number defaults to beats if not specified
        // Parse ",m" (required comma then tuplet number)
        if pos < len and input.at(pos) == "," {
          pos += 1
          let m-str = ""
          while pos < len and is-digit(input.at(pos)) {
            m-str += input.at(pos)
            pos += 1
          }
          if m-str.len() > 0 { tn = int(m-str) }
        }
        // Skip colon separator
        if pos < len and input.at(pos) == ":" { pos += 1 }
        // Skip whitespace after header
        while pos < len and (input.at(pos) == " " or input.at(pos) == "\t") { pos += 1 }
        tuplet-start-idx = events.len()
        tuplet-n = tn    // tuplet number (displayed)
        tuplet-m = tb     // tuplet beats (spacing)
      }
      continue
    }

    // --- Octave-line start: "<number>a{" or "<number>b{" ---
    // Examples: 8a{ ... }  15b{ ... }
    if is-digit(ch) {
      let p = pos
      let nstr = ""
      while p < len and is-digit(input.at(p)) { nstr += input.at(p); p += 1 }
      if nstr.len() > 0 and p < len {
        let suf = input.at(p)
        if suf == "a" or suf == "b" {
          // Allow optional whitespace between suffix and "{"
          let q = p + 1
          while q < len and (input.at(q) == " " or input.at(q) == "\t") { q += 1 }
          if q < len and input.at(q) == "{" {
            octline-start-idx = events.len()
            octline-number = int(nstr)
            octline-dir = if suf == "a" { "above" } else { "below" }
            pos = q + 1
            continue
          }
        }
      }
    }

    // --- Tuplet end: "}" ---
    if ch == "}" {
      if tuplet-start-idx != none {
        let end-idx = events.len()
        let count = end-idx - tuplet-start-idx
        for i in range(tuplet-start-idx, end-idx) {
          events.at(i).tuplet-beats = tuplet-m
          events.at(i).tuplet-number = tuplet-n
          events.at(i).tuplet-count = count
          if i == tuplet-start-idx { events.at(i).tuplet-start = true }
          if i == end-idx - 1 { events.at(i).tuplet-end = true }
        }
        tuplet-start-idx = none
        tuplet-n = none
        tuplet-m = none
      }
      // Also allow closing octave-line blocks (started with e.g. 8a{ )
      else if octline-start-idx != none {
        let end-idx = events.len()
        for i in range(octline-start-idx, end-idx) {
          events.at(i).octave-line-number = octline-number
          events.at(i).octave-line-direction = octline-dir
          if i == octline-start-idx { events.at(i).octave-line-start = true }
          if i == end-idx - 1 { events.at(i).octave-line-end = true }
        }
        octline-start-idx = none
        octline-number = none
        octline-dir = none
      }
      pos += 1
      continue
    }

    // --- Unknown character: skip ---
    pos += 1
  }

  events
}
