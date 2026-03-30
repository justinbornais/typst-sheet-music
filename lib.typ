// lib.typ - Main entry point for the sheet-music package
//
// Exports the public API: score(), melody(), lead-sheet(), chord-chart()

#import "src/parser.typ": parse-music
#import "src/layout.typ": layout-staff, layout-score
#import "src/layout-breaks.typ": compute-system-breaks, split-at-line-breaks, has-line-breaks
#import "src/renderer.typ": render-score
#import "src/render-clef-key-time.typ": clef-advance, key-sig-advance, time-sig-advance
#import "src/constants.typ": default-staff-space

/// Parse a time signature string like "4/4", "3/4", "6/8" into (upper, lower, symbol).
#let parse-time-sig(ts) = {
  if ts == "C" or ts == "c" {
    (upper: 4, lower: 4, symbol: "common")
  } else if ts == "C|" or ts == "c|" {
    (upper: 2, lower: 2, symbol: "cut")
  } else {
    // Parse "N/D" format
    let parts = ts.split("/")
    if parts.len() == 2 {
      (upper: int(parts.at(0).trim()), lower: int(parts.at(1).trim()), symbol: none)
    } else {
      // Default fallback
      (upper: 4, lower: 4, symbol: none)
    }
  }
}

/// Render a complete music score.
///
/// This is the primary entry point for the sheet-music library.
///
/// Parameters:
/// - staves: array of staff dictionaries, each with:
///     - clef: "treble", "bass", "alto", "tenor", "treble-8", "percussion"
///     - music: music string (see syntax reference)
///     - label: optional staff label
/// - lyrics: array of lyric dictionaries (Phase 4 - not yet implemented)
/// - chords: array of chord symbol dictionaries (Phase 4 - not yet implemented)
/// - key: key signature string ("C", "G", "D", "Bb", "f#", etc.)
/// - time: time signature string ("4/4", "3/4", "6/8", "C", "C|")
/// - tempo: tempo marking (not yet implemented)
/// - title: piece title
/// - subtitle: subtitle
/// - composer: composer name
/// - arranger: arranger name
/// - lyricist: lyricist name
/// - copyright: copyright text (not yet implemented)
/// - staff-group: "none", "grand", "choir", "orchestra" (Phase 2)
/// - staff-size: staff space distance (default 1.75mm)
/// - system-spacing: vertical space between systems (Phase 2)
/// - staff-spacing: vertical space between staves within a system
/// - width: explicit width or auto
/// - measure-numbers: "system", "every", "none" (Phase 2)
/// - relative-octave: if true, use relative octave entry (Phase 2)
/// - measures-per-line: if set, force this many measures per system line
#let score(
  staves: (),
  lyrics: (),
  chords: (),
  key: "C",
  time: "4/4",
  tempo: none,
  title: none,
  subtitle: none,
  composer: none,
  arranger: none,
  lyricist: none,
  copyright: none,
  staff-group: "none",
  staff-size: default-staff-space,
  system-spacing: 12mm,
  staff-spacing: 8mm,
  width: auto,
  measure-numbers: "system",
  relative-octave: false,
  measures-per-line: none,
) = {
  // Handle convenience: if staves is empty but there's something to render, return empty
  if staves.len() == 0 { return }

  // Parse time signature
  let ts = parse-time-sig(time)

  // Parse music for each staff
  let staves-events = staves.map(s => {
    let music-str = s.at("music", default: "")
    parse-music(music-str)
  })

  // Internal helper: compute prefix width in staff-space units for a given system
  let prefix-width-sp(sp-unit, clef-name, show-time) = {
    let pf = 0.5 // left margin
    pf += clef-advance(clef-name: clef-name, sp: 1.0)
    pf += key-sig-advance(key, sp: 1.0)
    if show-time {
      pf += time-sig-advance(ts.upper, ts.lower, symbol: ts.symbol, sp: 1.0)
    }
    pf += 1.0  // music-start padding
    pf
  }

  // Resolve width: we need the page width in mm for system-breaking calculations
  let render-inner(avail-width-mm) = {
    let sp-unit = staff-size / 1mm

    // For each staff, compute system breaks and lay out each system
    // For Phase 1 (single staff), we break the first staff's events
    let all-systems = ()  // array of arrays of laid-out staves

    // Compute how much width is available for music content (in staff-space units)
    let first-clef = staves.at(0).at("clef", default: "treble")
    let first-system-prefix = prefix-width-sp(sp-unit, first-clef, true)
    let continuation-prefix = prefix-width-sp(sp-unit, first-clef, false)

    // Break events using the first staff (Phase 1: single staff)
    let first-events = staves-events.at(0)
    let first-avail = if avail-width-mm != none { avail-width-mm / sp-unit - first-system-prefix - 1.0 } else { none }
    let cont-avail = if avail-width-mm != none { avail-width-mm / sp-unit - continuation-prefix - 1.0 } else { none }

    let systems-events = ()

    if has-line-breaks(first-events) {
      // Newlines in the music string define system breaks
      systems-events = split-at-line-breaks(first-events)
    } else if measures-per-line != none {
      // Fixed measures per line: split by measure count
      systems-events = compute-system-breaks(first-events, available-width: none, measures-per-line: measures-per-line)
    } else {
      // Width-based breaking
      let remaining = first-events

      // First system
      let first-breaks = compute-system-breaks(remaining, available-width: first-avail)
      if first-breaks.len() > 0 {
        systems-events.push(first-breaks.at(0))
        let rest-events = ()
        for i in range(1, first-breaks.len()) {
          rest-events += first-breaks.at(i)
        }
        remaining = rest-events
      }

      // Continuation systems
      if remaining.len() > 0 {
        let cont-breaks = compute-system-breaks(remaining, available-width: cont-avail)
        systems-events += cont-breaks
      }
    }

    // Parse fingerings if provided
    let fingering-list = staves.at(0).at("fingerings", default: none)

    // Lay out and render each system
    for (sys-idx, sys-events) in systems-events.enumerate() {
      let is-first = sys-idx == 0
      let clef = staves.at(0).at("clef", default: "treble")

      let laid-out = layout-staff(sys-events, clef: clef, staff-space: staff-size)

      // Compute fingerings for this system's notes
      let sys-fingerings = none
      if fingering-list != none {
        // Count how many notes came before this system
        let notes-before = 0
        for prev-idx in range(sys-idx) {
          for ev in systems-events.at(prev-idx) {
            if ev.type == "note" { notes-before += 1 }
          }
        }
        // Extract fingerings for this system's notes
        let sys-note-count = sys-events.filter(ev => ev.type == "note").len()
        let end-idx = calc.min(notes-before + sys-note-count, fingering-list.len())
        if notes-before < fingering-list.len() {
          sys-fingerings = fingering-list.slice(notes-before, end-idx)
        }
      }

      render-score(
        (laid-out,),
        key: key,
        time-upper: ts.upper,
        time-lower: ts.lower,
        time-symbol: ts.symbol,
        sp: staff-size,
        width: if avail-width-mm != none { avail-width-mm * 1mm } else { auto },
        staff-spacing: staff-spacing,
        title: if is-first { title } else { none },
        subtitle: if is-first { subtitle } else { none },
        composer: if is-first { composer } else { none },
        arranger: if is-first { arranger } else { none },
        lyricist: if is-first { lyricist } else { none },
        show-time: is-first,
        fingerings: sys-fingerings,
      )
      v(system-spacing)
    }
  }

  // Resolve width
  if width == auto {
    layout(size => {
      render-inner(size.width / 1mm)
    })
  } else {
    render-inner(width / 1mm)
  }
}

/// Quick single-staff melody rendering.
///
/// A convenience wrapper around `score()` for simple melodies.
#let melody(
  music: "",
  key: "C",
  time: "4/4",
  clef: "treble",
  title: none,
  composer: none,
  staff-size: default-staff-space,
  width: auto,
  fingerings: none,
  measures-per-line: none,
) = {
  score(
    staves: ((clef: clef, music: music, fingerings: fingerings),),
    key: key,
    time: time,
    title: title,
    composer: composer,
    staff-size: staff-size,
    width: width,
    measures-per-line: measures-per-line,
  )
}

/// Lead sheet rendering (melody + chords + lyrics).
/// Phase 4 stub - currently renders only the melody.
#let lead-sheet(
  music: "",
  chords: "",
  lyrics: "",
  key: "C",
  time: "4/4",
  clef: "treble",
  title: none,
  composer: none,
  staff-size: default-staff-space,
  width: auto,
) = {
  score(
    staves: ((clef: clef, music: music),),
    key: key,
    time: time,
    title: title,
    composer: composer,
    staff-size: staff-size,
    width: width,
  )
}

/// Chord chart rendering.
/// Phase 4 stub.
#let chord-chart(
  chords: "",
  key: "C",
  time: "4/4",
  title: none,
  width: auto,
) = {
  // Stub - will be implemented in Phase 4
}
