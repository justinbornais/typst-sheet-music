// Diagnostic: test bounds-based glyph placement
#import "@preview/cetz:0.4.2"

#set page(width: 200mm, height: 200mm, margin: 1cm)

#let sp = 1.75  // staff-space in mm
#let fsize = 4.0 * sp * 1mm  // 7mm = SMuFL em for this staff-size

// Load metadata for bbox offsets
#let _meta = json("../data/bravura_metadata.json")
#let bbox(name) = {
  let b = _meta.glyphBBoxes.at(name)
  (sw: (x: float(b.bBoxSW.at(0)), y: float(b.bBoxSW.at(1))),
   ne: (x: float(b.bBoxNE.at(0)), y: float(b.bBoxNE.at(1))))
}

// Helper: place a SMuFL glyph with its origin at exact canvas coordinates (x, y).
// Uses top-edge/bottom-edge "bounds" so the text box = glyph ink bbox.
// Then uses anchor "south-west" and offsets by the glyph's SW corner.
#let place-glyph(x, y, glyph-char, glyph-name, sp) = {
  import cetz.draw: *
  let fsize = 4.0 * sp * 1mm
  let bb = bbox(glyph-name)
  // Box SW corner goes at (x + sw.x*sp, y + sw.y*sp)
  // This makes glyph origin land at exactly (x, y)
  let px = x + bb.sw.x * sp
  let py = y + bb.sw.y * sp
  content(
    (px, py),
    anchor: "south-west",
    text(font: "Bravura", size: fsize, top-edge: "bounds", bottom-edge: "bounds", glyph-char),
  )
}

= Bounds-based placement test

== Treble clef, noteheads, sharp - origin at red dots
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  // Staff lines
  for i in range(5) { line((0, -i * sp), (80, -i * sp), stroke: 0.23mm + black) }

  // Treble clef: origin = G4 line = 2nd line from bottom
  // In our coords: line 0 (top) = y=0, line 4 (bottom) = y=-4*sp
  // G4 = 2nd from bottom = 4th from top = y = -3*sp
  let clef-y = -3.0 * sp
  circle((3, clef-y), radius: 0.4, fill: red, stroke: none)
  place-glyph(3, clef-y, "\u{E050}", "gClef", sp)

  // Notehead on B4 (middle line, y = -2*sp)
  let note1-y = -2.0 * sp
  circle((25, note1-y), radius: 0.3, fill: red, stroke: none)
  place-glyph(25, note1-y, "\u{E0A4}", "noteheadBlack", sp)

  // Notehead on F5 (top line, y = 0)
  let note2-y = 0.0
  circle((35, note2-y), radius: 0.3, fill: red, stroke: none)
  place-glyph(35, note2-y, "\u{E0A4}", "noteheadBlack", sp)

  // Notehead on E4 (bottom line, y = -4*sp)
  let note3-y = -4.0 * sp
  circle((45, note3-y), radius: 0.3, fill: red, stroke: none)
  place-glyph(45, note3-y, "\u{E0A4}", "noteheadBlack", sp)

  // Sharp accidental at D5 (y = -0.5*sp, space between lines 1 and 2)
  let sharp-y = -0.5 * sp
  circle((53, sharp-y), radius: 0.25, fill: red, stroke: none)
  place-glyph(53, sharp-y, "\u{E262}", "accidentalSharp", sp)
  // And the notehead it belongs to
  place-glyph(55, sharp-y, "\u{E0A4}", "noteheadBlack", sp)

  // Time signature: "4" on upper half, "4" on lower half
  // Upper "4" origin = between lines 1-2 (y = -1*sp)
  // Lower "4" origin = between lines 3-4 (y = -3*sp)
  place-glyph(15, -1.0 * sp, "\u{E084}", "timeSig4", sp)
  place-glyph(15, -3.0 * sp, "\u{E084}", "timeSig4", sp)
})

#v(1cm)
== Bass clef test
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  for i in range(5) { line((0, -i * sp), (60, -i * sp), stroke: 0.23mm + black) }

  // Bass clef: origin = F3 line = 2nd line from top = y = -1*sp
  let fclef-y = -1.0 * sp
  circle((3, fclef-y), radius: 0.4, fill: red, stroke: none)
  place-glyph(3, fclef-y, "\u{E062}", "fClef", sp)

  // Notehead on A3 (top line of bass staff, y = 0)
  place-glyph(25, 0, "\u{E0A4}", "noteheadBlack", sp)

  // Flat accidental
  place-glyph(33, -1.5 * sp, "\u{E260}", "accidentalFlat", sp)
  place-glyph(36, -1.5 * sp, "\u{E0A4}", "noteheadBlack", sp)
})

#v(1cm)
== Rest glyphs
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  for i in range(5) { line((0, -i * sp), (80, -i * sp), stroke: 0.23mm + black) }

  // Whole rest: hangs below line 2 (y = -1*sp), origin at top of glyph
  place-glyph(10, -1.0 * sp, "\u{E4E3}", "restWhole", sp)

  // Half rest: sits on line 3 (y = -2*sp), origin at bottom of glyph
  place-glyph(25, -2.0 * sp, "\u{E4E4}", "restHalf", sp)

  // Quarter rest: centered on staff
  place-glyph(40, -2.0 * sp, "\u{E4E5}", "restQuarter", sp)

  // Eighth rest
  place-glyph(55, -2.0 * sp, "\u{E4E6}", "rest8th", sp)
})

#v(1cm)
== Half notehead, flag
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  for i in range(5) { line((0, -i * sp), (60, -i * sp), stroke: 0.23mm + black) }

  // Half note on B4 (middle line)
  place-glyph(10, -2.0 * sp, "\u{E0A3}", "noteheadHalf", sp)

  // Whole note on D5
  place-glyph(25, -0.5 * sp, "\u{E0A2}", "noteheadWhole", sp)

  // Flag (8th up) at stem tip position
  place-glyph(40, 1.5 * sp, "\u{E240}", "flag8thUp", sp)

  // Flag (8th down)
  place-glyph(50, -5.5 * sp, "\u{E241}", "flag8thDown", sp)
})
