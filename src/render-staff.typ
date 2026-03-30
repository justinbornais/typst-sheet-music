// render-staff.typ - Draw staff lines, barlines, braces, and brackets

#import "@preview/cetz:0.4.2"
#import "constants.typ": *

/// Draw the five staff lines.
/// - ctx: CeTZ draw context (caller does `import cetz.draw: *`)
/// - x-start: left x position
/// - x-end: right x position
/// - y-top: y position of the top staff line
/// - sp: staff space size (1.0 in staff-space units)
#let draw-staff-lines(x-start, x-end, y-top, sp: 1.0) = {
  import cetz.draw: *
  let thickness = default-staff-line-thickness * sp
  for i in range(5) {
    let y = y-top - i * sp
    line(
      (x-start, y), (x-end, y),
      stroke: thickness * 1mm + black,
    )
  }
}

/// Draw a single barline.
/// - x: horizontal position
/// - y-top: y of top staff line
/// - y-bottom: y of bottom staff line
/// - style: "single", "double", "final", "repeat-start", "repeat-end"
/// - sp: staff space
#let draw-barline(x, y-top, y-bottom, style: "single", sp: 1.0) = {
  import cetz.draw: *
  let thin = default-barline-thickness * sp
  let thick = default-thick-barline * sp

  if style == "single" {
    line(
      (x, y-top), (x, y-bottom),
      stroke: thin * 1mm + black,
    )
  } else if style == "double" {
    line(
      (x - 0.3 * sp, y-top), (x - 0.3 * sp, y-bottom),
      stroke: thin * 1mm + black,
    )
    line(
      (x, y-top), (x, y-bottom),
      stroke: thin * 1mm + black,
    )
  } else if style == "final" {
    line(
      (x - 0.4 * sp, y-top), (x - 0.4 * sp, y-bottom),
      stroke: thin * 1mm + black,
    )
    line(
      (x, y-top), (x, y-bottom),
      stroke: thick * 1mm + black,
    )
  } else if style == "repeat-start" {
    line(
      (x, y-top), (x, y-bottom),
      stroke: thick * 1mm + black,
    )
    line(
      (x + 0.4 * sp, y-top), (x + 0.4 * sp, y-bottom),
      stroke: thin * 1mm + black,
    )
    // Dots
    let dot-y1 = y-top - 1.5 * sp
    let dot-y2 = y-top - 2.5 * sp
    circle((x + 0.8 * sp, dot-y1), radius: 0.15 * sp, fill: black, stroke: none)
    circle((x + 0.8 * sp, dot-y2), radius: 0.15 * sp, fill: black, stroke: none)
  } else if style == "repeat-end" {
    // Dots
    let dot-y1 = y-top - 1.5 * sp
    let dot-y2 = y-top - 2.5 * sp
    circle((x - 0.8 * sp, dot-y1), radius: 0.15 * sp, fill: black, stroke: none)
    circle((x - 0.8 * sp, dot-y2), radius: 0.15 * sp, fill: black, stroke: none)
    line(
      (x - 0.4 * sp, y-top), (x - 0.4 * sp, y-bottom),
      stroke: thin * 1mm + black,
    )
    line(
      (x, y-top), (x, y-bottom),
      stroke: thick * 1mm + black,
    )
  }
}
