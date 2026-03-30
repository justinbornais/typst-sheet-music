// layout.typ - Layout engine
//
// Takes parsed events + configuration and produces laid-out events
// with x,y positions for the renderer.

#import "model.typ": make-laid-out-event
#import "pitch.typ": staff-position, auto-stem-direction, compute-stem-end-y
#import "layout-spacing.typ": compute-event-positions, total-events-width
#import "constants.typ": *

/// Layout a single staff's events.
///
/// Parameters:
/// - events: array of parsed events
/// - clef: the initial clef string
/// - staff-space: size of one staff space (length)
/// - available-width: available horizontal width (in staff spaces, or none for unlimited)
///
/// Returns: dictionary with:
///   - items: array of laid-out events (each has x, y, event, stem-dir, stem-y-end)
///   - total-width: total width in staff spaces
#let layout-staff(
  events,
  clef: "treble",
  staff-space: default-staff-space,
  available-width: none,
) = {
  let positions = compute-event-positions(events)
  let items = ()
  let current-clef = clef

  for (i, event) in events.enumerate() {
    let pos-info = positions.at(i)
    let x = pos-info.x
    let y = 0.0
    let stem-dir = none
    let stem-y-end = none

    if event.type == "note" {
      // Compute staff position and Y coordinate
      let sp = staff-position(event.name, event.octave, clef: current-clef)
      // Y = position in staff spaces (positive = upward from top line in CeTZ coords)
      // In our system: position 0 = top line = y 0
      //                position 8 = bottom line = y -4*staff-space
      // Y goes negative downward in staff-space units
      y = -sp / 2.0   // Convert half-spaces to staff-spaces (negative = down)

      // Stem direction
      stem-dir = auto-stem-direction(sp)

      // Stem end Y
      stem-y-end = compute-stem-end-y(y, sp, stem-dir, 1.0)  // 1.0 = staff-space unit
    } else if event.type == "rest" {
      // Rests are centered vertically on the staff
      // Whole rest hangs from line 2 (position 2), y = -1.0
      // Half rest sits on line 3 (position 4), y = -2.0
      // Quarter rest spans middle area, y = -2.0 (center of staff)
      if event.duration == 1 {
        y = -1.0   // Whole rest: hangs from 4th line (2nd from top)
      } else if event.duration == 2 {
        y = -2.0   // Half rest: sits on middle line
      } else {
        y = -2.0   // Others: centered on staff
      }
    } else if event.type == "clef" {
      current-clef = event.clef
    }

    items.push(make-laid-out-event(
      event,
      x: x,
      y: y,
      stem-dir: stem-dir,
      stem-y-end: stem-y-end,
    ))
  }

  let tw = total-events-width(events)

  (
    items: items,
    total-width: tw,
    clef: clef,
  )
}

/// Layout an entire score (multiple staves).
/// For Phase 1, this just lays out each staff independently.
/// Phase 2 will add vertical beat alignment across staves.
#let layout-score(
  staves-events,
  staves-config,
  staff-space: default-staff-space,
  available-width: none,
) = {
  let laid-out-staves = ()

  for (i, events) in staves-events.enumerate() {
    let config = staves-config.at(i)
    let clef = config.at("clef", default: "treble")
    let result = layout-staff(
      events,
      clef: clef,
      staff-space: staff-space,
      available-width: available-width,
    )
    laid-out-staves.push(result)
  }

  laid-out-staves
}
