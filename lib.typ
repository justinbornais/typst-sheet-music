// lib.typ - Main entry point for the sheet-music package
//
// Exports the public API: score(), melody(), lead-sheet(), chord-chart()

#import "src/parser.typ": parse-music
#import "src/layout.typ": layout-staff, layout-score
#import "src/layout-spacing.typ": align-staves-by-beat
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
    let clef-name = s.at("clef", default: "treble")
    let base-oct = if clef-name == "bass" { 3 } else { 4 }
    parse-music(music-str, base-octave: base-oct)
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

    // System-break decisions are driven by the first staff.
    let first-clef = staves.at(0).at("clef", default: "treble")
    let prefix-first = prefix-width-sp(sp-unit, first-clef, true)
    let prefix-cont  = prefix-width-sp(sp-unit, first-clef, false)
    let first-avail  = if avail-width-mm != none { avail-width-mm / sp-unit - prefix-first - 1.0 } else { none }
    let cont-avail   = if avail-width-mm != none { avail-width-mm / sp-unit - prefix-cont  - 1.0 } else { none }

    // Compute systems-events arrays for the first staff, then mirror the same
    // break points onto every other staff.
    let first-events = staves-events.at(0)
    let systems-events-per-staff = staves-events.map(_ => ())  // will fill in below

    // Determine system event lists for staff 0
    let staff0-systems = ()
    if has-line-breaks(first-events) {
      staff0-systems = split-at-line-breaks(first-events)
    } else if measures-per-line != none {
      staff0-systems = compute-system-breaks(first-events, available-width: none, measures-per-line: measures-per-line)
    } else if avail-width-mm != none {
      let remaining = first-events
      let fb = compute-system-breaks(remaining, available-width: first-avail)
      if fb.len() > 0 {
        staff0-systems.push(fb.at(0))
        let rest = ()
        for i in range(1, fb.len()) { rest += fb.at(i) }
        remaining = rest
      }
      if remaining.len() > 0 {
        staff0-systems += compute-system-breaks(remaining, available-width: cont-avail)
      }
    } else {
      staff0-systems = (first-events,)
    }

    // Split every other staff into the same number of systems by counting measures.
    // We count how many measures (barlines) fall in each staff-0 system and apply
    // the identical measure counts to the other staves.
    let measure-counts = staff0-systems.map(sys => {
      sys.filter(ev => ev.type == "barline").len()
    })

    for si in range(staves-events.len()) {
      if si == 0 {
        systems-events-per-staff.at(si) = staff0-systems
      } else {
        let ev-list = staves-events.at(si)
        let split = ()
        let remaining = ev-list
        for (mc-idx, mc) in measure-counts.enumerate() {
          let is-last = mc-idx == measure-counts.len() - 1
          // Grab `mc` barlines worth of events from remaining.
          // On the last system, grab everything so trailing events after the
          // final barline are not silently dropped.
          let seg = ()
          let bars-seen = 0
          let j = 0
          while j < remaining.len() and (is-last or bars-seen < mc or mc == 0) {
            seg.push(remaining.at(j))
            if remaining.at(j).type == "barline" { bars-seen += 1 }
            j += 1
          }
          // Ensure j is at the end when mc == 0 or it's the last system
          if mc == 0 or is-last { j = remaining.len() }
          split.push(seg)
          remaining = remaining.slice(j)
        }
        // Any leftover goes into a final system
        if remaining.len() > 0 { split.push(remaining) }
        systems-events-per-staff.at(si) = split
      }
    }

    let num-systems = staff0-systems.len()

    // Track per-staff note offsets for fingering extraction
    let notes-before-per-staff = staves.map(_ => 0)
    // Track measure offset for chord-symbol extraction (measures before this system)
    let measures-before = 0

    for sys-idx in range(num-systems) {
      let is-first = sys-idx == 0

      // Lay out each staff for this system
      let laid-out-staves = ()
      for si in range(staves.len()) {
        let clef = staves.at(si).at("clef", default: "treble")
        let sys-evs = if sys-idx < systems-events-per-staff.at(si).len() {
          systems-events-per-staff.at(si).at(sys-idx)
        } else { () }
        laid-out-staves.push(layout-staff(sys-evs, clef: clef, staff-space: staff-size))
      }

      // Beat-align across staves so notes at the same beat share x positions.
      if laid-out-staves.len() > 1 {
        laid-out-staves = align-staves-by-beat(laid-out-staves)
      }

      // Fingerings: only for the first staff
      let sys-fingerings = none
      let fingering-list = staves.at(0).at("fingerings", default: none)
      if fingering-list != none {
        let nb = notes-before-per-staff.at(0)
        let sys-evs = if sys-idx < systems-events-per-staff.at(0).len() {
          systems-events-per-staff.at(0).at(sys-idx)
        } else { () }
        let nc = sys-evs.filter(ev => ev.type == "note").len()
        let end-idx = calc.min(nb + nc, fingering-list.len())
        if nb < fingering-list.len() {
          sys-fingerings = fingering-list.slice(nb, end-idx)
        }
      }

      // Chord symbols: per-measure arrays, sliced for this system.
      // Count measures by counting groups of note/rest/chord events between barlines.
      let sys-chord-symbols = none
      let chord-symbol-list = staves.at(0).at("chord-symbols", default: none)
      if chord-symbol-list != none {
        let sys-evs = if sys-idx < systems-events-per-staff.at(0).len() {
          systems-events-per-staff.at(0).at(sys-idx)
        } else { () }
        // Count measures: each group of note/chord/rest events between barlines is one measure.
        let n-measures = 0
        let in-measure = false
        for ev in sys-evs {
          if ev.type == "note" or ev.type == "chord" or ev.type == "rest" or ev.type == "spacer" {
            if not in-measure {
              n-measures += 1
              in-measure = true
            }
          } else if ev.type == "barline" {
            in-measure = false
          }
        }
        let end-idx = calc.min(measures-before + n-measures, chord-symbol-list.len())
        if measures-before < chord-symbol-list.len() {
          sys-chord-symbols = chord-symbol-list.slice(measures-before, end-idx)
        }
      }

      // Update note-offset counters
      for si in range(staves.len()) {
        let sys-evs = if sys-idx < systems-events-per-staff.at(si).len() {
          systems-events-per-staff.at(si).at(sys-idx)
        } else { () }
        notes-before-per-staff.at(si) += sys-evs.filter(ev => ev.type == "note").len()
      }

      // Update measure counter for chord-symbol slicing
      {
        let sys-evs = if sys-idx < systems-events-per-staff.at(0).len() {
          systems-events-per-staff.at(0).at(sys-idx)
        } else { () }
        let n-measures = 0
        let in-measure = false
        for ev in sys-evs {
          if ev.type == "note" or ev.type == "chord" or ev.type == "rest" or ev.type == "spacer" {
            if not in-measure {
              n-measures += 1
              in-measure = true
            }
          } else if ev.type == "barline" {
            in-measure = false
          }
        }
        measures-before += n-measures
      }

      render-score(
        laid-out-staves,
        key: key,
        time-upper: ts.upper,
        time-lower: ts.lower,
        time-symbol: ts.symbol,
        sp: staff-size,
        width: if avail-width-mm != none { avail-width-mm * 1mm } else { auto },
        staff-spacing: staff-spacing,
        staff-group: staff-group,
        title: if is-first { title } else { none },
        subtitle: if is-first { subtitle } else { none },
        composer: if is-first { composer } else { none },
        arranger: if is-first { arranger } else { none },
        lyricist: if is-first { lyricist } else { none },
        show-time: is-first,
        fingerings: sys-fingerings,
        chord-symbols: sys-chord-symbols,
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
  chord-symbols: none,
  measures-per-line: none,
) = {
  score(
    staves: ((clef: clef, music: music, fingerings: fingerings, chord-symbols: chord-symbols),),
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
#let lead-sheet(
  music: "",
  chords: none,
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
    staves: ((clef: clef, music: music, chord-symbols: chords),),
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
