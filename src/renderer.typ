// renderer.typ - Main CeTZ rendering orchestrator
//
// Takes laid-out events and draws the complete score using CeTZ.

#import "@preview/cetz:0.4.2"
#import "constants.typ": *
#import "render-staff.typ": draw-staff-lines, draw-barline
#import "render-clef-key-time.typ": draw-clef, draw-key-signature, draw-time-signature, clef-advance, key-sig-advance, time-sig-advance
#import "render-notes.typ": draw-note, draw-rest

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

  // Draw opening (initial) barline
  draw-barline(0.2 * sp, y-top, y-bottom, style: "single", sp: sp)

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

  // Draw all music events
  let note-idx = 0  // Track which note we're on for fingerings
  for item in items {
    let event = item.event
    let x = music-start-x + item.x * scale-x * sp
    let y = item.y * sp

    if event.type == "note" {
      draw-note(
        x, y-top + y, event,
        item.stem-dir, y-top + item.stem-y-end * sp,
        y-top,
        clef: clef-name,
        sp: sp,
      )

      // Draw fingering number above/below the note
      if fingerings != none and note-idx < fingerings.len() {
        let fng = fingerings.at(note-idx)
        if fng != none and fng != 0 {
          let fng-str = str(fng)
          let fng-y = y-top + 1.5 * sp  // Above the staff
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
      // Draw barline at ~1/3 into its slot so more space falls after it
      draw-barline(x + 0.5 * sp, y-top, y-bottom, style: event.style, sp: sp)
    }
    // spacers: invisible, nothing to draw
  }

  // Draw final barline at the end (if the music doesn't end with one)
  let last-is-barline = items.len() > 0 and items.last().event.type == "barline"
  if not last-is-barline {
    draw-barline(total-width * sp - 0.2 * sp, y-top, y-bottom, style: "final", sp: sp)
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
