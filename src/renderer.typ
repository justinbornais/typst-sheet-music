// renderer.typ - Main CeTZ rendering orchestrator
//
// Takes laid-out events and draws the complete score using CeTZ.

#import "@preview/cetz:0.4.2"
#import "constants.typ": *
#import "render-staff.typ": draw-staff-lines, draw-barline
#import "render-clef-key-time.typ": draw-clef, draw-key-signature, draw-time-signature, clef-advance, key-sig-advance, time-sig-advance
#import "render-notes.typ": draw-note, draw-rest, note-stem-x
#import "render-beams.typ": draw-beam-group

/// Render a single system (one line of music) for one staff.
///
/// Parameters:
/// - laid-out: the layout result from layout-staff (items, total-width, clef)
/// - key: key signature string
/// - time-upper: time sig numerator
/// - time-lower: time sig denominator
/// - time-symbol: "common", "cut", or none
/// - sp: staff space in absolute units (e.g., 1.75mm)
/// - width: available width for the system
/// - show-clef: whether to draw the clef
/// - show-key: whether to draw the key sig
/// - show-time: whether to draw the time sig
/// - fingerings: optional array of fingering values (one per note in this system)
#let render-system(
  laid-out,
  key: "C",
  time-upper: 4,
  time-lower: 4,
  time-symbol: none,
  sp: 1.0,
  width: none,
  show-clef: true,
  show-key: true,
  show-time: true,
  fingerings: none,
) = {
  import cetz.draw: *

  let clef-name = laid-out.clef
  let items = laid-out.items
  let total-layout-width = laid-out.total-width

  // Y coordinates
  let y-top = 0.0   // Top staff line
  let y-bottom = y-top - 4.0 * sp  // Bottom staff line (4 staff spaces down)

  // Compute prefix width (clef + key sig + time sig)
  let prefix-x = 0.5 * sp  // Left margin
  let clef-w = 0.0
  let key-w = 0.0
  let time-w = 0.0

  if show-clef {
    clef-w = clef-advance(clef-name: clef-name, sp: sp)
  }
  if show-key {
    key-w = key-sig-advance(key, sp: sp)
  }
  if show-time {
    time-w = time-sig-advance(time-upper, time-lower, symbol: time-symbol, sp: sp)
  }

  let music-start-x = prefix-x + clef-w + key-w + time-w + 1.0 * sp

  // Add extra space if the first music event has an accidental
  let first-note = items.find(item => item.event.type == "note")
  if first-note != none and first-note.event.accidental != none {
    music-start-x += 1.0 * sp
  }

  // Compute scaling: fit events into available width
  let available-music-width = if width != none {
    width / sp - music-start-x / sp - 1.0  // Reserve right margin
  } else {
    total-layout-width + 2.0
  }

  let scale-x = if total-layout-width > 0 {
    available-music-width / total-layout-width
  } else {
    1.0
  }

  let total-width = if width != none { width / sp } else { music-start-x / sp + total-layout-width * scale-x + 1.0 }

  // Draw staff lines across full width
  draw-staff-lines(0.0, total-width * sp, y-top, sp: sp)

  // Draw opening (initial) barline flush with the left edge of the staff.
  draw-barline(0.0, y-top, y-bottom, style: "single", sp: sp)

  // Draw clef
  let cx = prefix-x
  if show-clef {
    draw-clef(cx, y-top, clef-name, sp: sp)
    cx += clef-w
  }

  // Draw key signature
  if show-key {
    draw-key-signature(cx, y-top, key, clef-name, sp: sp)
    cx += key-w
  }

  // Draw time signature
  if show-time {
    draw-time-signature(cx, y-top, time-upper, time-lower, symbol: time-symbol, sp: sp)
    cx += time-w
  }

  // ── Pre-compute per-item x positions (needed for beam geometry) ─────────
  let item-xs = items.map(item => music-start-x + item.x * scale-x * sp)

  // ── Auto-beaming: group consecutive notes with duration ≥ 8 ─────────────
  // Groups are broken by barlines, rests, notes with duration < 8, or when
  // the group reaches 4 notes (one half-measure in 4/4 time).
  let raw-beam-groups = ()
  let cur-beam = ()
  for (i, item) in items.enumerate() {
    let ev = item.event
    if ev.type == "note" and ev.duration >= 8 {
      // Flush at 4 notes and start a new group
      if cur-beam.len() == 4 {
        raw-beam-groups.push(cur-beam)
        cur-beam = ()
      }
      cur-beam.push(i)
    } else {
      if cur-beam.len() >= 2 { raw-beam-groups.push(cur-beam) }
      cur-beam = ()
    }
  }
  if cur-beam.len() >= 2 { raw-beam-groups.push(cur-beam) }

  // Compute beam geometry: adjusted stem ends + beam-note records.
  let adj-stem-ends = (:)   // str(i) → stem-y in staff-sp units
  let adj-stem-dirs = (:)   // str(i) → stem-dir string
  let beam-groups-data = () // array of beam-note arrays for draw-beam-group

  for group in raw-beam-groups {
    let stem-dir = items.at(group.first()).stem-dir
    let x0 = item-xs.at(group.first())
    let xn = item-xs.at(group.last())
    let sy0 = items.at(group.first()).stem-y-end   // staff-sp units
    let syn = items.at(group.last()).stem-y-end

    let beam-note-data = ()
    for idx in group {
      let item = items.at(idx)
      let xi = item-xs.at(idx)
      // Linearly interpolate the beam y at this note's x
      let t = if xn != x0 { (xi - x0) / (xn - x0) } else { 0.0 }
      let by-staff = sy0 + t * (syn - sy0)   // staff-sp units
      let by-abs   = y-top + by-staff * sp   // absolute canvas y
      let sx = note-stem-x(xi, item.event.duration, stem-dir, sp: sp)
      beam-note-data.push((stem-x: sx, beam-y: by-abs, duration: item.event.duration, stem-dir: stem-dir))
      adj-stem-ends.insert(str(idx), by-staff)
      adj-stem-dirs.insert(str(idx), stem-dir)
    }
    beam-groups-data.push(beam-note-data)
  }

  // ── Find tuplet groups (from tuplet-start / tuplet-end flags) ────────────
  let tuplet-groups = ()
  let cur-tup-indices = ()
  let cur-tup-n = 1
  let cur-tup-m = 1
  for (i, item) in items.enumerate() {
    let ev = item.event
    if ev.type == "note" or ev.type == "rest" {
      let tn = ev.at("tuplet-n", default: 1)
      if tn > 1 {
        let tm = ev.at("tuplet-m", default: 1)
        if ev.at("tuplet-start", default: false) {
          cur-tup-indices = (i,)
          cur-tup-n = tn
          cur-tup-m = tm
        } else if cur-tup-indices.len() > 0 {
          cur-tup-indices.push(i)
        }
        if ev.at("tuplet-end", default: false) and cur-tup-indices.len() > 0 {
          tuplet-groups.push((indices: cur-tup-indices, n: cur-tup-n, m: cur-tup-m))
          cur-tup-indices = ()
        }
      }
    }
  }

  // ── Draw all music events ────────────────────────────────────────────────
  let note-idx = 0  // Track which note we're on for fingerings
  for (i, item) in items.enumerate() {
    let event = item.event
    let x = item-xs.at(i)
    let y = item.y * sp

    if event.type == "note" {
      // Use beam-adjusted stem end and direction if this note is beamed
      let stem-end-override = adj-stem-ends.at(str(i), default: none)
      let stem-dir-override = adj-stem-dirs.at(str(i), default: none)
      let actual-stem-end = if stem-end-override != none {
        y-top + stem-end-override * sp
      } else {
        y-top + item.stem-y-end * sp
      }
      let actual-stem-dir = if stem-dir-override != none { stem-dir-override } else { item.stem-dir }
      let is-beamed = stem-end-override != none

      draw-note(
        x, y-top + y, event,
        actual-stem-dir, actual-stem-end,
        y-top,
        clef: clef-name,
        sp: sp,
        beamed: is-beamed,
      )

      // Draw fingering number above the note
      if fingerings != none and note-idx < fingerings.len() {
        let fng = fingerings.at(note-idx)
        if fng != none and fng != 0 {
          let fng-str = str(fng)
          let note-center-y = y-top + y
          let fng-y = calc.max(y-top + 1.5 * sp, note-center-y + 1.0 * sp)
          content(
            (x, fng-y),
            anchor: "south",
            text(size: 7pt, weight: "regular", fng-str),
          )
        }
      }
      note-idx += 1
    } else if event.type == "rest" {
      draw-rest(x, y-top + y, event.duration, dots: event.dots, sp: sp)
    } else if event.type == "barline" {
      // All barlines except the very last item are drawn at their layout position.
      // The last barline is drawn at the right edge (handled below).
      if i < items.len() - 1 {
        draw-barline(x + 0.5 * sp, y-top, y-bottom, style: event.style, sp: sp)
      }
    }
    // spacers: invisible, nothing to draw
  }

  // ── Draw final barline at right edge (always) ────────────────────────────
  // Use the style from the last event if it is a barline; otherwise "final".
  let final-style = if items.len() > 0 and items.last().event.type == "barline" {
    items.last().event.style
  } else {
    "final"
  }
  // Position the closing barline so its rightmost visual edge is flush with
  // the right end of the staff lines.
  let final-x = if final-style == "final" or final-style == "repeat-end" or final-style == "repeat-both" {
    total-width * sp - default-thick-barline / 2.0 * sp
  } else {
    total-width * sp - default-barline-thickness / 2.0 * sp
  }
  draw-barline(final-x, y-top, y-bottom, style: final-style, sp: sp)

  // ── Draw beams ───────────────────────────────────────────────────────────
  for beam-data in beam-groups-data {
    draw-beam-group(beam-data, sp: sp)
  }

  // ── Draw tuplet brackets ─────────────────────────────────────────────────
  for tup in tuplet-groups {
    let indices = tup.indices
    let tn = tup.n
    if indices.len() == 0 { continue }

    // Collect x positions and stem ends for the tuplet notes
    let tup-xs = indices.map(idx => item-xs.at(idx))
    let tup-stem-ends = indices.map(idx => {
      let override = adj-stem-ends.at(str(idx), default: none)
      if override != none {
        y-top + override * sp
      } else {
        y-top + items.at(idx).stem-y-end * sp
      }
    })

    let x-first = tup-xs.first()
    let x-last  = tup-xs.last()
    let stem-dir = adj-stem-dirs.at(str(indices.first()), default: items.at(indices.first()).stem-dir)

    // Place bracket on same side as beam (stem-tip side)
    let bracket-y = if stem-dir == "up" {
      // up-stems: bracket above stem tips
      tup-stem-ends.fold(tup-stem-ends.first(), calc.max) + 0.6 * sp
    } else {
      // down-stems: bracket below stem tips
      tup-stem-ends.fold(tup-stem-ends.first(), calc.min) - 0.6 * sp
    }
    let tick-len = 0.4 * sp
    let tick-dir = if stem-dir == "up" { -1.0 } else { 1.0 }   // toward noteheads

    // Draw bracket: horizontal line + end ticks
    line(
      (x-first, bracket-y), (x-last, bracket-y),
      stroke: (thickness: 0.12 * sp * 1mm, paint: black),
    )
    line(
      (x-first, bracket-y), (x-first, bracket-y + tick-dir * tick-len),
      stroke: (thickness: 0.12 * sp * 1mm, paint: black),
    )
    line(
      (x-last, bracket-y), (x-last, bracket-y + tick-dir * tick-len),
      stroke: (thickness: 0.12 * sp * 1mm, paint: black),
    )
    // Tuplet number centered on bracket
    let mid-x = (x-first + x-last) / 2.0
    let num-anchor = if stem-dir == "up" { "south" } else { "north" }
    content(
      (mid-x, bracket-y),
      anchor: num-anchor,
      text(size: 7pt, weight: "regular", style: "italic", str(tn)),
    )
  }
}

/// Render a complete score as a CeTZ canvas block.
///
/// Parameters:
/// - laid-out-staves: array of layout results (one per staff)
/// - key, time-upper, time-lower, time-symbol: initial signatures
/// - sp: staff space (length, e.g., 1.75mm)
/// - width: available width (length or auto)
/// - staff-spacing: vertical space between staves
/// - title, subtitle, composer, arranger, lyricist: header fields
/// - show-time: whether to render time signature
/// - fingerings: optional array of fingering values (one per note)
#let render-score(
  laid-out-staves,
  key: "C",
  time-upper: 4,
  time-lower: 4,
  time-symbol: none,
  sp: default-staff-space,
  width: auto,
  staff-spacing: 8mm,
  title: none,
  subtitle: none,
  composer: none,
  arranger: none,
  lyricist: none,
  show-time: true,
  fingerings: none,
) = {
  // Use a unit staff-space internally for CeTZ coordinates
  // Then scale the entire canvas
  let unit = sp / 1mm  // Convert to mm number

  // The available width in mm
  let avail-width = if width == auto { none } else { width / 1mm }

  // Render header (Typst content, not CeTZ)
  import "render-header.typ": render-header
  render-header(
    title: title,
    subtitle: subtitle,
    composer: composer,
    arranger: arranger,
    lyricist: lyricist,
  )

  // Render the music in a CeTZ canvas
  let num-staves = laid-out-staves.len()
  // Total height: each staff is 4*sp, plus spacing between staves
  let staff-height-mm = 4.0 * unit
  let spacing-mm = staff-spacing / 1mm
  let total-height-mm = num-staves * staff-height-mm + calc.max(0, num-staves - 1) * spacing-mm + 4.0 * unit  // Extra padding

  cetz.canvas(
    length: 1mm,
    {
      import cetz.draw: *

      for (i, laid-out) in laid-out-staves.enumerate() {
        // Y offset for this staff
        let y-offset = -i * (staff-height-mm + spacing-mm)

        // Translate to staff position
        set-origin((0, y-offset))

        render-system(
          laid-out,
          key: key,
          time-upper: time-upper,
          time-lower: time-lower,
          time-symbol: time-symbol,
          sp: unit,
          width: avail-width,
          show-clef: true,
          show-key: true,
          show-time: show-time,
          fingerings: if i == 0 { fingerings } else { none },
        )
      }
    },
  )
}
