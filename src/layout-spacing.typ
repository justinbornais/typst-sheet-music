// layout-spacing.typ - Duration-to-width calculations
//
// Converts event durations into horizontal spacing values.

#import "utils.typ": duration-to-beats, duration-spacing-factor
#import "constants.typ": default-note-spacing-base
#import "render-clef-key-time.typ": clef-advance

/// Compute the horizontal width (in staff-spaces) for an event's duration.
#let event-width(event, base-width: default-note-spacing-base) = {
  if event.type == "barline" {
    // Barlines have padding on both sides
    2.5
  } else if event.type == "clef" {
    clef-advance(clef-name: event.clef, sp: 1.0)
  } else if event.type == "key-sig" or event.type == "time-sig" {
    // Non-rhythmic events: fixed width
    2.0
  } else {
    // Notes, rests, spacers, chords: duration-proportional
    let dur = event.at("duration", default: 4)
    let dots = event.at("dots", default: 0)
    let factor = duration-spacing-factor(dur, dots: dots)
    let w = base-width * factor
    // Extra space for notes/chords with accidentals
    if event.type == "note" and event.at("accidental", default: none) != none {
      w += 0.5
    }
    if event.type == "chord" {
      let any-acc = event.at("notes", default: ()).any(n => n.at("accidental", default: none) != none)
      if any-acc { w += 0.5 }
    }
    // Tuplet notes: total group width = width of tuplet-beats, divided among notes
    let tb = event.at("tuplet-beats", default: 0)
    let tc = event.at("tuplet-count", default: 0)
    if tb > 0 and tc > 0 {
      let equiv-dur = 4.0 / tb
      let total-w = base-width * duration-spacing-factor(equiv-dur)
      w = total-w / tc
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

/// Align multiple staves' layouts so events at the same beat position share
/// the same x coordinate. Uses a distributed-width approach: each event's
/// width is spread evenly across the beat columns it spans, so a whole note
/// sharing beat 0 with a quarter note does not inflate that column.
///
/// Boundary events like barlines and inline clefs reserve a shared column
/// across all staves, even if only one staff contains the event.
#let align-staves-by-beat(laid-out-staves) = {
  if laid-out-staves.len() <= 1 { return laid-out-staves }

  let barline-epsilon = 0.000001
  let is-rhythmic-event(ev) = {
    ev.type == "note" or ev.type == "rest" or ev.type == "spacer" or ev.type == "chord"
  }
  let is-boundary-event(ev) = {
    ev.type == "barline" or ev.type == "clef" or ev.type == "key-sig" or ev.type == "time-sig"
  }
  let is-pre-barline-clef(items, idx) = {
    idx + 1 < items.len() and items.at(idx).event.type == "clef" and items.at(idx + 1).event.type == "barline"
  }
  let rounded-beat(beat) = calc.round(beat, digits: 6)
  let beat-key(beat) = str(rounded-beat(beat))

  // 1. For each beat boundary, compute the maximum number of non-rhythmic
  //    columns that occur before the next rhythmic event on any staff.
  let beat-boundary-widths = (:)
  for laid-out in laid-out-staves {
    let beat = 0.0
    let boundary-count = 0
    let items = laid-out.items
    for (ii, item) in items.enumerate() {
      let ev = item.event
      let key = beat-key(beat)
      if is-pre-barline-clef(items, ii) {
        continue
      } else if is-boundary-event(ev) {
        boundary-count += 1
        let current = beat-boundary-widths.at(key, default: 0)
        if boundary-count > current {
          beat-boundary-widths.insert(key, boundary-count)
        }
      } else if is-rhythmic-event(ev) {
        let current = beat-boundary-widths.at(key, default: 0)
        if boundary-count > current {
          beat-boundary-widths.insert(key, boundary-count)
        }
        boundary-count = 0

        let dur = ev.at("duration", default: 4)
        let dots = ev.at("dots", default: 0)
        let dur-beats = duration-to-beats(dur, dots: dots)
        let tb = ev.at("tuplet-beats", default: 0)
        let tc = ev.at("tuplet-count", default: 0)
        if tb > 0 and tc > 0 {
          dur-beats = tb / tc
        }
        beat += dur-beats
      }
    }

    let final-key = beat-key(beat)
    let current = beat-boundary-widths.at(final-key, default: 0)
    if boundary-count > current {
      beat-boundary-widths.insert(final-key, boundary-count)
    }
  }

  // 2. Compute cumulative beat offsets for every item in every staff.
  //    Rhythmic events start after the maximum boundary width at that beat,
  //    so a clef on one staff reserves space on every other staff too.
  let staves-beats = ()
  for laid-out in laid-out-staves {
    let beats = ()
    let beat = 0.0
    let boundary-phase = 0
    let items = laid-out.items
    for (ii, item) in items.enumerate() {
      let ev = item.event
      let rb = rounded-beat(beat)
      let boundary-width = beat-boundary-widths.at(beat-key(beat), default: 0)

      if is-pre-barline-clef(items, ii) {
        beats.push(calc.round(rb - barline-epsilon, digits: 6))
      } else if is-boundary-event(ev) {
        beats.push(calc.round(rb + boundary-phase * barline-epsilon, digits: 6))
        boundary-phase += 1
      } else if is-rhythmic-event(ev) {
        beats.push(calc.round(rb + boundary-width * barline-epsilon, digits: 6))

        let dur = ev.at("duration", default: 4)
        let dots = ev.at("dots", default: 0)
        let dur-beats = duration-to-beats(dur, dots: dots)
        let tb = ev.at("tuplet-beats", default: 0)
        let tc = ev.at("tuplet-count", default: 0)
        if tb > 0 and tc > 0 {
          dur-beats = tb / tc
        }
        beat += dur-beats
        boundary-phase = 0
      } else {
        beats.push(calc.round(rb + boundary-width * barline-epsilon, digits: 6))
      }
    }
    staves-beats.push(beats)
  }

  // 3. Sorted unique beat positions.
  let beat-set = (:)
  for staff-beats in staves-beats {
    for b in staff-beats {
      beat-set.insert(str(b), b)
    }
  }
  let all-beats = beat-set.values().sorted()

  // 4. Beat -> column index map.
  let beat-to-col = (:)
  for (ci, b) in all-beats.enumerate() {
    beat-to-col.insert(str(b), ci)
  }
  let n-cols = all-beats.len()

  // 5. Compute column widths using the distributed-width approach.
  let col-widths = range(n-cols).map(_ => 0.0)

  for (si, laid-out) in laid-out-staves.enumerate() {
    let staff-beats = staves-beats.at(si)
    let items = laid-out.items
    for (ii, item) in items.enumerate() {
      let start-col = beat-to-col.at(str(staff-beats.at(ii)))
      let end-col = if ii + 1 < items.len() {
        beat-to-col.at(str(staff-beats.at(ii + 1)))
      } else {
        start-col + 1
      }
      let span = calc.max(end-col - start-col, 1)
      let w = event-width(item.event)
      let distributed = w / span
      for c in range(start-col, calc.min(end-col, n-cols)) {
        if distributed > col-widths.at(c) {
          col-widths.at(c) = distributed
        }
      }
    }
  }

  // 6. Cumulative x positions per column.
  let col-xs = ()
  let x = 0.0
  for w in col-widths {
    col-xs.push(x)
    x += w
  }
  let total-w = x

  // 7. Reassign x to each item based on its column.
  let result = ()
  for (si, laid-out) in laid-out-staves.enumerate() {
    let staff-beats = staves-beats.at(si)
    let new-items = ()
    for (ii, item) in laid-out.items.enumerate() {
      let ci = beat-to-col.at(str(staff-beats.at(ii)))
      let new-item = (
        event: item.event,
        x: col-xs.at(ci),
        y: item.y,
        stem-dir: item.stem-dir,
        stem-y-end: item.stem-y-end,
      )
      let chord-ys = item.at("chord-ys", default: none)
      let chord-staff-positions = item.at("chord-staff-positions", default: none)
      if chord-ys != none { new-item.insert("chord-ys", chord-ys) }
      if chord-staff-positions != none {
        new-item.insert("chord-staff-positions", chord-staff-positions)
      }
      new-items.push(new-item)
    }
    result.push((
      items: new-items,
      total-width: total-w,
      clef: laid-out.clef,
    ))
  }

  result
}
