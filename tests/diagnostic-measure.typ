// Diagnostic: measure text element sizes
#set page(width: 200mm, height: 100mm, margin: 1cm)

#let sp = 1.75  // mm  
#let fsize = 4.0 * sp * 1mm  // 7mm

#let measure-glyph(name, glyph) = {
  let t = text(font: "Bravura", size: fsize, glyph)
  context {
    let m = measure(t)
    [#name: width=#m.width, height=#m.height \ ]
  }
}

= Bravura Glyph Measurements at size #fsize

#measure-glyph("noteheadBlack", "\u{E0A4}")
#measure-glyph("noteheadHalf", "\u{E0A3}")
#measure-glyph("noteheadWhole", "\u{E0A2}")
#measure-glyph("gClef (treble)", "\u{E050}")
#measure-glyph("fClef (bass)", "\u{E062}")
#measure-glyph("accidentalSharp", "\u{E262}")
#measure-glyph("accidentalFlat", "\u{E260}")
#measure-glyph("timeSig4", "\u{E084}")
#measure-glyph("restQuarter", "\u{E4E5}")
#measure-glyph("rest8th", "\u{E4E6}")
#measure-glyph("flag8thUp", "\u{E240}")

Expected notehead height: ~#(sp)mm (1 staff-space)
Expected notehead width: ~#(1.18 * sp)mm (1.18 staff-spaces from metadata)

Staff-space: #(sp)mm
Font em: 4 × #(sp)mm = #(4 * sp)mm
