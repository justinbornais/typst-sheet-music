// Diagnostic: test CeTZ glyph placement strategies
#import "@preview/cetz:0.4.2"

#set page(width: 200mm, height: 297mm, margin: 1cm)

#let sp = 1.75  // staff-space in mm
#let fsize = 4.0 * sp * 1mm  // SMuFL: em = 4 staff-spaces

= Glyph Placement Diagnostic

== Strategy 1: anchor "west" (current approach)
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  // Draw staff lines
  for i in range(5) { line((0, -i * sp), (60, -i * sp), stroke: 0.2mm + black) }
  // Red dot at target position (G4 = second line from bottom = y = -3*sp)
  circle((10, -3 * sp), radius: 0.5, fill: red, stroke: none)
  // Treble clef with anchor: "west"
  content((10, -3 * sp), anchor: "west", text(font: "Bravura", size: fsize, "\u{E050}"))
  // Notehead at B4 (middle line = y = -2*sp) 
  circle((30, -2 * sp), radius: 0.3, fill: red, stroke: none)
  content((30, -2 * sp), anchor: "west", text(font: "Bravura", size: fsize, "\u{E0A4}"))
  // Notehead at G4 (line 2 from bottom = y = -3*sp)
  circle((40, -3 * sp), radius: 0.3, fill: red, stroke: none)
  content((40, -3 * sp), anchor: "west", text(font: "Bravura", size: fsize, "\u{E0A4}"))
})

== Strategy 2: anchor "mid" (used for noteheads currently)
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  for i in range(5) { line((0, -i * sp), (60, -i * sp), stroke: 0.2mm + black) }
  circle((10, -3 * sp), radius: 0.5, fill: red, stroke: none)
  content((10, -3 * sp), anchor: "mid", text(font: "Bravura", size: fsize, "\u{E050}"))
  circle((30, -2 * sp), radius: 0.3, fill: red, stroke: none)
  content((30, -2 * sp), anchor: "mid", text(font: "Bravura", size: fsize, "\u{E0A4}"))
  circle((40, -3 * sp), radius: 0.3, fill: red, stroke: none)
  content((40, -3 * sp), anchor: "mid", text(font: "Bravura", size: fsize, "\u{E0A4}"))
})

== Strategy 3: zero-height box with baseline control
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  for i in range(5) { line((0, -i * sp), (60, -i * sp), stroke: 0.2mm + black) }
  // The idea: wrap text in a box where we control the baseline
  // box(height: 0pt, clip: false) makes the box invisible to layout
  // The text's baseline = glyph origin in SMuFL
  circle((10, -3 * sp), radius: 0.5, fill: red, stroke: none)
  content(
    (10, -3 * sp),
    anchor: "south-west",
    box(width: 0pt, height: 0pt, clip: false, 
      align(bottom + left, text(font: "Bravura", size: fsize, "\u{E050}"))
    ),
  )
  circle((30, -2 * sp), radius: 0.3, fill: red, stroke: none)
  content(
    (30, -2 * sp),
    anchor: "south-west",
    box(width: 0pt, height: 0pt, clip: false,
      align(bottom + left, text(font: "Bravura", size: fsize, "\u{E0A4}"))
    ),
  )
})

== Strategy 4: Use place() for absolute positioning
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  for i in range(5) { line((0, -i * sp), (60, -i * sp), stroke: 0.2mm + black) }
  circle((10, -3 * sp), radius: 0.5, fill: red, stroke: none)
  content(
    (10, -3 * sp),
    text(font: "Bravura", size: fsize, baseline: 0pt, "\u{E050}"),
  )
  circle((30, -2 * sp), radius: 0.3, fill: red, stroke: none)
  content(
    (30, -2 * sp),
    text(font: "Bravura", size: fsize, baseline: 0pt, "\u{E0A4}"),
  )
})

== Strategy 5: Default anchor (no anchor specified)
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  for i in range(5) { line((0, -i * sp), (60, -i * sp), stroke: 0.2mm + black) }
  circle((10, -3 * sp), radius: 0.5, fill: red, stroke: none)
  content((10, -3 * sp), text(font: "Bravura", size: fsize, "\u{E050}"))
  circle((30, -2 * sp), radius: 0.3, fill: red, stroke: none)
  content((30, -2 * sp), text(font: "Bravura", size: fsize, "\u{E0A4}"))
})

#pagebreak()

== Strategy 6: Using move() to shift from baseline
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  for i in range(5) { line((0, -i * sp), (60, -i * sp), stroke: 0.2mm + black) }
  
  // Try different baseline offsets for the treble clef
  for (idx, offset) in (0mm, 2mm, 4mm, 6mm, 8mm, 10mm, 12mm).enumerate() {
    let xx = 5 + idx * 8
    content(
      (xx, -3 * sp),
      anchor: "west",
      move(dy: offset, text(font: "Bravura", size: fsize, "\u{E050}")),
    )
  }
})

== Strategy 7: Using top-edge alignment
#cetz.canvas(length: 1mm, {
  import cetz.draw: *
  for i in range(5) { line((0, -i * sp), (60, -i * sp), stroke: 0.2mm + black) }
  
  // top-edge = ascender of glyph. For gClef, bboxNE.y = 4.392 staff-spaces above origin.
  // If we use anchor: "north-west", the top of text box is at our coord.
  // gClef origin should be 4.392 staff-spaces below the top of the glyph.
  // But the text box includes the full font ascent, not just this glyph's top.
  
  // Let's try both anchor: "north-west" and custom offset
  circle((10, -3 * sp), radius: 0.5, fill: red, stroke: none)
  content((10, -3 * sp), anchor: "north-west", text(font: "Bravura", size: fsize, "\u{E050}"))
  
  circle((30, -3 * sp), radius: 0.5, fill: red, stroke: none)
  content((30, -3 * sp), anchor: "south-west", text(font: "Bravura", size: fsize, "\u{E050}"))
})

== Glyph sizes: Visual check  
#text(font: "Bravura", size: fsize)[\u{E050}] (treble clef at #fsize)

Staff space = #(sp)mm, font size = #fsize

== Individual glyph rendering at correct size
Notehead (should be 1 staff-space = #(sp)mm tall): #box(stroke: 0.1pt + red, text(font: "Bravura", size: fsize, "\u{E0A4}"))

Quarter rest: #box(stroke: 0.1pt + red, text(font: "Bravura", size: fsize, "\u{E4E5}"))

Sharp: #box(stroke: 0.1pt + red, text(font: "Bravura", size: fsize, "\u{E262}"))

Treble clef: #box(stroke: 0.1pt + red, text(font: "Bravura", size: fsize, "\u{E050}"))

Time sig 4: #box(stroke: 0.1pt + red, text(font: "Bravura", size: fsize, "\u{E084}"))
