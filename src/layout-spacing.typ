// layout-spacing.typ - Duration-to-width calculations
//
// Converts event durations into horizontal spacing values.

#import "utils.typ": duration-to-beats, duration-spacing-factor
#import "constants.typ": default-note-spacing-base

/// Compute the horizontal width (in staff-spaces) for an event's duration.
#let event-width(event, base-width: default-note-spacing-base) = {
  if event.type == "barline" {
    // Barlines have padding on both sides
    2.5
  } else if event.type == "clef" or event.type == "key-sig" or event.type == "time-sig" {
    // Non-rhythmic events: fixed width
    2.0
  } else {
    // Notes, rests, spacers, chords: duration-proportional
    let dur = event.at("duration", default: 4)
    let dots = event.at("dots", default: 0)
    let factor = duration-spacing-factor(dur, dots: dots)
    let w = base-width * factor
    // Extra space for notes with accidentals
    if event.type == "note" and event.at("accidental", default: none) != none {
      w += 0.5
    }
    w
  }
}

/// Given an array of events, compute an array of x-positions.
/// Returns an array of (x, width) pairs.
#let compute-event-positions(events, base-width: default-note-spacing-base) = {
  let positions = ()
  let x = 0.0
  for event in events {
    let w = event-width(event, base-width: base-width)
    positions.push((x: x, width: w))
    x += w
  }
  positions
}

/// Compute total width of all events.
#let total-events-width(events, base-width: default-note-spacing-base) = {
  let total = 0.0
  for event in events {
    total += event-width(event, base-width: base-width)
  }
  total
}
