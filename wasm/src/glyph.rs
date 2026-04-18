// ─── Glyph metadata (zero-allocation match-based lookups) ──────────────
// Supports Bravura (default), Leipzig, Leland, and Petaluma fonts.

#[derive(Debug, Clone, Copy)]
pub struct BBox {
    pub sw_x: f64,
    pub sw_y: f64,
    pub ne_x: f64,
    pub ne_y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Anchor {
    pub x: f64,
    pub y: f64,
}

/// Per-font engraving defaults from SMuFL metadata.
#[derive(Debug, Clone, Copy)]
pub struct EngravingDefaults {
    pub staff_line_thickness: f64,
    pub stem_thickness: f64,
    pub beam_thickness: f64,
    pub beam_spacing: f64,
    pub thin_barline_thickness: f64,
    pub thick_barline_thickness: f64,
    pub ledger_line_extension: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontId {
    Bravura,
    Leipzig,
    Leland,
    Petaluma,
}

impl FontId {
    pub fn from_name(name: &str) -> Self {
        match name {
            "Leipzig" | "leipzig" => FontId::Leipzig,
            "Leland" | "leland" => FontId::Leland,
            "Petaluma" | "petaluma" => FontId::Petaluma,
            _ => FontId::Bravura,
        }
    }
}

pub fn engraving_defaults(font: FontId) -> EngravingDefaults {
    match font {
        FontId::Bravura => EngravingDefaults {
            staff_line_thickness: 0.13,
            stem_thickness: 0.12,
            beam_thickness: 0.5,
            beam_spacing: 0.25,
            thin_barline_thickness: 0.16,
            thick_barline_thickness: 0.5,
            ledger_line_extension: 0.4,
        },
        FontId::Leipzig => EngravingDefaults {
            staff_line_thickness: 0.08,
            stem_thickness: 0.076,
            beam_thickness: 0.5,
            beam_spacing: 0.25,
            thin_barline_thickness: 0.15,
            thick_barline_thickness: 0.5,
            ledger_line_extension: 0.27,
        },
        FontId::Leland => EngravingDefaults {
            staff_line_thickness: 0.11,
            stem_thickness: 0.1,
            beam_thickness: 0.5,
            beam_spacing: 0.25,
            thin_barline_thickness: 0.18,
            thick_barline_thickness: 0.55,
            ledger_line_extension: 0.33,
        },
        FontId::Petaluma => EngravingDefaults {
            staff_line_thickness: 0.13,
            stem_thickness: 0.2,
            beam_thickness: 0.5,
            beam_spacing: 0.25,
            thin_barline_thickness: 0.16,
            thick_barline_thickness: 0.5,
            ledger_line_extension: 0.4,
        },
    }
}

// ─── Advance width ─────────────────────────────────────────────────────

#[inline]
pub fn advance_width(glyph_name: &str) -> f64 {
    advance_width_for(FontId::Bravura, glyph_name)
}

#[inline]
pub fn advance_width_for(font: FontId, glyph_name: &str) -> f64 {
    match font {
        FontId::Bravura => advance_width_bravura(glyph_name),
        FontId::Leipzig => advance_width_leipzig(glyph_name),
        FontId::Leland => advance_width_leland(glyph_name),
        FontId::Petaluma => advance_width_petaluma(glyph_name),
    }
}

fn advance_width_bravura(g: &str) -> f64 {
    match g {
        "mensuralWhiteMaxima" => 2.5,
        "mensuralWhiteLonga" => 1.3,
        "noteheadDoubleWhole" => 2.396,
        "noteheadWhole" => 1.688,
        "noteheadHalf" | "noteheadBlack" => 1.18,
        "restMaxima" => 1.524,
        "restLonga" => 0.5,
        "restDoubleWhole" => 0.504,
        "restWhole" | "restHalf" => 1.132,
        "restQuarter" => 1.08,
        "rest8th" => 1.0,
        "rest16th" => 1.28,
        "rest32nd" => 1.452,
        "rest64th" => 1.696,
        "flag8thUp" => 1.056,
        "flag8thDown" => 1.224,
        "flag16thUp" => 1.116,
        "flag16thDown" => 1.168,
        "flag32ndUp" => 1.048,
        "flag32ndDown" => 1.096,
        "flag64thUp" => 1.048,
        "flag64thDown" => 1.1,
        "accidentalSharp" => 0.996,
        "accidentalFlat" => 0.904,
        "accidentalNatural" => 0.672,
        "accidentalDoubleSharp" => 1.0,
        "accidentalDoubleFlat" => 1.652,
        "gClef" | "gClef8va" | "gClef15ma" | "gClef15mb" => 2.684,
        "gClef8vb" => 2.656,
        "fClef" | "fClef8va" | "fClef8vb" | "fClef15ma" | "fClef15mb" => 2.736,
        "cClef" => 2.796,
        "timeSig0" | "timeSig4" => 1.88,
        "timeSig1" => 1.336,
        "timeSig2" => 1.784,
        "timeSig3" => 1.684,
        "timeSig5" => 1.612,
        "timeSig6" | "timeSig9" => 1.736,
        "timeSig7" => 1.764,
        "timeSig8" => 1.744,
        "timeSigCommon" => 1.696,
        "timeSigCutCommon" => 1.668,
        "ornamentTrill" => 2.084,
        "wiggleTrill" => 0.948,
        "breathMarkComma" => 0.612,
        "caesura" => 1.54,
        "segno" => 2.228,
        "coda" => 3.816,
        "brace" => 0.336,
        "unpitchedPercussionClef1" => 1.528,
        _ => 0.0,
    }
}

fn advance_width_leipzig(g: &str) -> f64 {
    match g {
        "noteheadDoubleWhole" => 2.18,
        "noteheadWhole" => 1.62,
        "noteheadHalf" | "noteheadBlack" => 1.256,
        "restMaxima" => 1.5,
        "restLonga" => 0.5,
        "restDoubleWhole" => 0.5,
        "restWhole" | "restHalf" => 1.2,
        "restQuarter" => 1.22,
        "rest8th" => 1.104,
        "rest16th" => 1.3,
        "rest32nd" => 1.596,
        "rest64th" => 1.92,
        "flag8thUp" | "flag8thDown" => 1.104,
        "flag16thUp" | "flag16thDown" => 1.104,
        "flag32ndUp" | "flag32ndDown" => 1.104,
        "flag64thUp" | "flag64thDown" => 1.104,
        "accidentalSharp" => 0.788,
        "accidentalFlat" => 0.792,
        "accidentalNatural" => 0.628,
        "accidentalDoubleSharp" => 1.028,
        "accidentalDoubleFlat" => 1.552,
        "gClef" | "gClef8va" | "gClef8vb" | "gClef15mb" => 2.584,
        "gClef15ma" => 2.6,
        "fClef" | "fClef8va" | "fClef8vb" | "fClef15ma" | "fClef15mb" => 2.792,
        "cClef" => 2.424,
        "timeSig0" => 1.736,
        "timeSig1" => 1.264,
        "timeSig2" => 1.688,
        "timeSig3" => 1.568,
        "timeSig4" => 1.628,
        "timeSig5" => 1.508,
        "timeSig6" => 1.608,
        "timeSig7" => 1.696,
        "timeSig8" => 1.536,
        "timeSig9" => 1.62,
        "timeSigCommon" => 1.656,
        "timeSigCutCommon" => 1.668,
        "ornamentTrill" => 1.728,
        "wiggleTrill" => 0.841,
        "breathMarkComma" => 0.72,
        "caesura" => 1.2,
        "segno" => 1.644,
        "coda" => 2.148,
        "brace" => 0.371,
        "unpitchedPercussionClef1" => 1.3,
        // Fallback to Bravura for mensural glyphs not present in Leipzig
        "mensuralWhiteMaxima" | "mensuralWhiteLonga" => advance_width_bravura(g),
        _ => 0.0,
    }
}

fn advance_width_leland(g: &str) -> f64 {
    match g {
        "noteheadDoubleWhole" => 2.152,
        "noteheadWhole" => 1.492,
        "noteheadHalf" | "noteheadBlack" => 1.3,
        "restMaxima" => 1.799,
        "restLonga" => 0.5,
        "restDoubleWhole" => 0.501,
        "restWhole" | "restHalf" => 1.3,
        "restQuarter" => 0.94,
        "rest8th" => 1.104,
        "rest16th" => 1.376,
        "rest32nd" => 1.563,
        "rest64th" => 1.69,
        "flag8thUp" => 1.157,
        "flag8thDown" => 1.235,
        "flag16thUp" => 1.116,
        "flag16thDown" => 1.235,
        "flag32ndUp" => 1.117,
        "flag32ndDown" => 1.235,
        "flag64thUp" => 1.117,
        "flag64thDown" => 1.235,
        "accidentalSharp" => 0.976,
        "accidentalFlat" => 0.812,
        "accidentalNatural" => 0.684,
        "accidentalDoubleSharp" => 1.1,
        "accidentalDoubleFlat" => 1.484,
        "gClef" | "gClef8va" | "gClef8vb" | "gClef15ma" | "gClef15mb" => 2.56,
        "fClef" | "fClef8vb" => 2.656,
        "fClef8va" | "fClef15ma" => 2.66,
        "fClef15mb" => 2.652,
        "cClef" => 2.508,
        "timeSig0" => 1.556,
        "timeSig1" => 1.344,
        "timeSig2" => 1.508,
        "timeSig3" => 1.457,
        "timeSig4" => 1.768,
        "timeSig5" => 1.448,
        "timeSig6" | "timeSig9" => 1.548,
        "timeSig7" => 1.462,
        "timeSig8" => 1.572,
        "timeSigCommon" => 1.852,
        "timeSigCutCommon" => 1.852,
        "ornamentTrill" => 1.532,
        "wiggleTrill" => 0.948,
        "breathMarkComma" => 0.764,
        "caesura" => 1.52,
        "segno" => 2.616,
        "coda" => 2.98,
        "brace" => 0.248,
        "unpitchedPercussionClef1" => 1.408,
        "mensuralWhiteMaxima" | "mensuralWhiteLonga" => advance_width_bravura(g),
        _ => 0.0,
    }
}

fn advance_width_petaluma(g: &str) -> f64 {
    match g {
        "noteheadDoubleWhole" => 2.457,
        "noteheadWhole" => 1.521,
        "noteheadHalf" => 1.336,
        "noteheadBlack" => 1.336,
        "restMaxima" => 1.464,
        "restLonga" => 0.556,
        "restDoubleWhole" => 0.488,
        "restWhole" => 2.0,
        "restHalf" => 2.074,
        "restQuarter" => 1.052,
        "rest8th" => 1.156,
        "rest16th" => 1.332,
        "rest32nd" => 1.388,
        "rest64th" => 1.712,
        "flag8thUp" | "flag8thDown" => 1.044,
        "flag16thUp" | "flag16thDown" => 1.309,
        "flag32ndUp" | "flag32ndDown" => 1.072,
        "flag64thUp" => 1.128,
        "flag64thDown" => 1.124,
        "accidentalSharp" => 1.56,
        "accidentalFlat" => 0.836,
        "accidentalNatural" => 0.854,
        "accidentalDoubleSharp" => 1.148,
        "accidentalDoubleFlat" => 1.436,
        "gClef" | "gClef8va" | "gClef8vb" | "gClef15ma" | "gClef15mb" => 2.656,
        "fClef" | "fClef8va" | "fClef8vb" | "fClef15ma" | "fClef15mb" => 3.104,
        "cClef" => 2.924,
        "timeSig0" => 2.052,
        "timeSig1" => 1.132,
        "timeSig2" => 2.642,
        "timeSig3" => 2.16,
        "timeSig4" => 2.532,
        "timeSig5" => 2.332,
        "timeSig6" => 2.26,
        "timeSig7" => 2.368,
        "timeSig8" => 2.017,
        "timeSig9" => 1.976,
        "timeSigCommon" => 2.34,
        "timeSigCutCommon" => 2.78,
        "ornamentTrill" => 2.204,
        "wiggleTrill" => 1.473,
        "breathMarkComma" => 0.592,
        "caesura" => 2.262,
        "segno" => 3.192,
        "coda" => 5.52,
        "brace" => 0.312,
        "unpitchedPercussionClef1" => 1.517,
        "mensuralWhiteMaxima" | "mensuralWhiteLonga" => advance_width_bravura(g),
        _ => 0.0,
    }
}

// ─── Bounding boxes ────────────────────────────────────────────────────

#[inline]
pub fn bbox(glyph_name: &str) -> Option<BBox> {
    bbox_for(FontId::Bravura, glyph_name)
}

#[inline]
pub fn bbox_for(font: FontId, glyph_name: &str) -> Option<BBox> {
    match font {
        FontId::Bravura => bbox_bravura(glyph_name),
        FontId::Leipzig => bbox_leipzig(glyph_name),
        FontId::Leland => bbox_leland(glyph_name),
        FontId::Petaluma => bbox_petaluma(glyph_name),
    }
}

fn bbox_bravura(g: &str) -> Option<BBox> {
    let b = |sw_x, sw_y, ne_x, ne_y| Some(BBox { sw_x, sw_y, ne_x, ne_y });
    match g {
        "mensuralWhiteMaxima" => b(0.0, -3.548, 2.5, 0.684),
        "mensuralWhiteLonga" => b(0.0, -3.548, 1.3, 0.684),
        "noteheadDoubleWhole" => b(0.0, -0.62, 2.396, 0.62),
        "noteheadWhole" => b(0.0, -0.5, 1.688, 0.5),
        "noteheadHalf" | "noteheadBlack" => b(0.0, -0.5, 1.18, 0.5),
        "restMaxima" => b(0.0, -0.996, 1.524, 1.0),
        "restLonga" => b(0.0, -0.996, 0.5, 1.0),
        "restDoubleWhole" => b(0.0, 0.0, 0.5, 1.0),
        "restWhole" => b(0.0, -0.54, 1.128, 0.036),
        "restHalf" => b(0.0, -0.008, 1.128, 0.568),
        "restQuarter" => b(0.004, -1.5, 1.08, 1.492),
        "rest8th" => b(0.0, -1.004, 0.988, 0.696),
        "rest16th" => b(0.0, -2.0, 1.28, 0.716),
        "rest32nd" => b(0.0, -2.0, 1.452, 1.704),
        "rest64th" => b(0.0, -3.012, 1.692, 1.72),
        "flag8thUp" => b(0.0, -3.241, 1.056, 0.035),
        "flag8thDown" => b(0.0, -0.058, 1.224, 3.233),
        "flag16thUp" => b(0.0, -3.252, 1.116, 0.008),
        "flag16thDown" => b(0.0, -0.036, 1.164, 3.248),
        "flag32ndUp" => b(0.0, -3.248, 1.044, 0.596),
        "flag32ndDown" => b(0.0, -0.687, 1.092, 3.248),
        "flag64thUp" => b(0.0, -3.248, 1.044, 1.387),
        "flag64thDown" => b(0.0, -1.504, 1.092, 3.248),
        "accidentalSharp" => b(0.0, -1.392, 0.996, 1.4),
        "accidentalFlat" => b(0.0, -0.7, 0.904, 1.756),
        "accidentalNatural" => b(0.0, -1.34, 0.672, 1.364),
        "accidentalDoubleSharp" => b(0.0, -0.5, 0.988, 0.508),
        "accidentalDoubleFlat" => b(0.0, -0.7, 1.644, 1.748),
        "gClef" => b(0.0, -2.632, 2.684, 4.392),
        "fClef" => b(-0.02, -2.54, 2.736, 1.048),
        "cClef" => b(0.0, -2.024, 2.796, 2.024),
        "gClef8va" => b(0.0, -2.632, 2.684, 5.28),
        "gClef8vb" => b(0.0, -3.512, 2.684, 4.392),
        "gClef15ma" => b(0.0, -2.632, 2.684, 5.276),
        "gClef15mb" => b(0.0, -3.524, 2.684, 4.392),
        "fClef8va" => b(-0.02, -2.54, 2.736, 1.98),
        "fClef8vb" => b(-0.02, -2.976, 2.736, 1.048),
        "fClef15ma" => b(-0.02, -2.54, 2.736, 1.984),
        "fClef15mb" => b(-0.02, -2.968, 2.736, 1.048),
        "timeSig0" => b(0.08, -1.0, 1.8, 1.004),
        "timeSig1" => b(0.08, -1.0, 1.256, 1.004),
        "timeSig2" => b(0.08, -1.028, 1.704, 1.016),
        "timeSig3" => b(0.08, -1.004, 1.604, 0.996),
        "timeSig4" => b(0.08, -1.0, 1.8, 1.004),
        "timeSig5" => b(0.08, -1.004, 1.532, 0.984),
        "timeSig6" => b(0.08, -0.996, 1.656, 1.004),
        "timeSig7" => b(0.08, -1.0, 1.684, 0.996),
        "timeSig8" => b(0.08, -1.036, 1.664, 1.036),
        "timeSig9" => b(0.08, -0.996, 1.656, 1.004),
        "timeSigCommon" => b(0.02, -0.996, 1.696, 1.004),
        "timeSigCutCommon" => b(0.0, -1.436, 1.672, 1.444),
        "ornamentTrill" => b(0.0, -0.04, 2.084, 1.56),
        "wiggleTrill" => b(-0.144, 0.392, 1.08, 0.836),
        "breathMarkComma" => b(0.004, 0.008, 0.608, 1.004),
        "caesura" => b(0.0, -0.004, 1.536, 2.128),
        "segno" => b(0.016, -0.108, 2.2, 3.036),
        "coda" => b(-0.016, -0.632, 3.82, 3.592),
        "brace" => b(0.008, 0.0, 0.328, 3.988),
        "unpitchedPercussionClef1" => b(0.0, -1.0, 1.528, 1.0),
        _ => None,
    }
}

fn bbox_leipzig(g: &str) -> Option<BBox> {
    let b = |sw_x, sw_y, ne_x, ne_y| Some(BBox { sw_x, sw_y, ne_x, ne_y });
    match g {
        "noteheadDoubleWhole" => b(0.0, -0.68, 2.18, 0.68),
        "noteheadWhole" => b(0.0, -0.532, 1.62, 0.532),
        "noteheadHalf" => b(0.0, -0.528, 1.256, 0.552),
        "noteheadBlack" => b(0.0, -0.532, 1.256, 0.532),
        "restMaxima" => b(0.0, -1.0, 1.5, 1.0),
        "restLonga" => b(0.0, -1.0, 0.5, 1.0),
        "restDoubleWhole" => b(0.0, 0.0, 0.5, 1.0),
        "restWhole" => b(0.0, -0.5, 1.2, 0.0),
        "restHalf" => b(0.0, 0.0, 1.2, 0.5),
        "restQuarter" => b(0.0, -1.552, 1.22, 1.488),
        "rest8th" => b(0.0, -1.0, 1.104, 0.732),
        "rest16th" => b(0.004, -1.968, 1.3, 0.696),
        "rest32nd" => b(0.004, -1.94, 1.596, 1.652),
        "rest64th" => b(0.008, -2.872, 1.92, 1.652),
        "flag8thUp" => b(0.0, -2.776, 1.104, 0.0),
        "flag8thDown" => b(0.0, 0.0, 1.104, 2.776),
        "flag16thUp" => b(0.0, -3.116, 1.104, 0.0),
        "flag16thDown" => b(0.0, 0.0, 1.104, 3.044),
        "flag32ndUp" => b(0.0, -3.116, 1.104, 0.76),
        "flag32ndDown" => b(0.0, -0.76, 1.104, 3.04),
        "flag64thUp" => b(0.0, -3.116, 1.104, 1.52),
        "flag64thDown" => b(0.0, -1.52, 1.104, 3.04),
        "accidentalSharp" => b(0.0, -1.356, 0.788, 1.42),
        "accidentalFlat" => b(0.0, -0.7, 0.792, 1.876),
        "accidentalNatural" => b(0.0, -1.404, 0.628, 1.404),
        "accidentalDoubleSharp" => b(0.0, -0.48, 1.028, 0.48),
        "accidentalDoubleFlat" => b(0.0, -0.7, 1.552, 1.876),
        "gClef" => b(-0.004, -2.62, 2.584, 4.332),
        "fClef" => b(0.016, -2.324, 2.792, 1.004),
        "cClef" => b(0.0, -2.008, 2.424, 2.008),
        "gClef8va" => b(-0.004, -2.62, 2.584, 5.268),
        "gClef8vb" => b(-0.004, -3.56, 2.584, 4.332),
        "gClef15ma" => b(-0.004, -2.62, 2.6, 5.252),
        "gClef15mb" => b(-0.004, -3.572, 2.584, 4.332),
        "fClef8va" => b(0.016, -2.324, 2.792, 1.868),
        "fClef8vb" => b(0.016, -3.0, 2.792, 1.004),
        "fClef15ma" => b(0.016, -2.324, 2.792, 1.94),
        "fClef15mb" => b(0.016, -3.02, 2.792, 1.004),
        "timeSig0" => b(0.08, -1.0, 1.736, 1.004),
        "timeSig1" => b(0.08, -1.0, 1.264, 1.0),
        "timeSig2" => b(0.08, -1.004, 1.688, 1.004),
        "timeSig3" => b(0.08, -1.008, 1.568, 1.0),
        "timeSig4" => b(0.08, -1.0, 1.628, 1.004),
        "timeSig5" => b(0.08, -1.0, 1.508, 1.0),
        "timeSig6" => b(0.08, -1.0, 1.608, 1.0),
        "timeSig7" => b(0.08, -1.0, 1.696, 1.004),
        "timeSig8" => b(0.04, -1.0, 1.536, 0.997),
        "timeSig9" => b(0.08, -1.0, 1.62, 1.0),
        "timeSigCommon" => b(0.0, -0.996, 1.656, 1.004),
        "timeSigCutCommon" => b(0.0, -1.276, 1.668, 1.272),
        "ornamentTrill" => b(0.0, 0.0, 1.728, 1.456),
        "wiggleTrill" => b(-0.049, 0.0, 0.841, 0.448),
        "breathMarkComma" => b(0.0, 0.0, 0.72, 1.2),
        "caesura" => b(0.0, 0.0, 1.2, 2.0),
        "segno" => b(0.0, 0.0, 1.644, 2.052),
        "coda" => b(0.0, -0.292, 2.148, 2.32),
        "brace" => b(0.0, 0.005, 0.371, 3.995),
        "unpitchedPercussionClef1" => b(0.0, -1.0, 1.3, 1.0),
        // Fallback for mensural glyphs
        "mensuralWhiteMaxima" | "mensuralWhiteLonga" => bbox_bravura(g),
        _ => None,
    }
}

fn bbox_leland(g: &str) -> Option<BBox> {
    let b = |sw_x, sw_y, ne_x, ne_y| Some(BBox { sw_x, sw_y, ne_x, ne_y });
    match g {
        "noteheadDoubleWhole" => b(0.0, -0.712, 2.152, 0.716),
        "noteheadWhole" => b(0.0, -0.536, 1.492, 0.544),
        "noteheadHalf" => b(0.0, -0.532, 1.3, 0.528),
        "noteheadBlack" => b(0.0, -0.532, 1.3, 0.528),
        "restMaxima" => b(-0.001, -1.0, 1.799, 1.0),
        "restLonga" => b(0.0, -1.001, 0.5, 0.999),
        "restDoubleWhole" => b(0.001, -0.044, 0.501, 1.048),
        "restWhole" => b(0.0, -0.524, 1.3, 0.02),
        "restHalf" => b(0.0, -0.016, 1.3, 0.528),
        "restQuarter" => b(0.0, -1.325, 0.94, 1.605),
        "rest8th" => b(0.0, -1.022, 1.104, 0.814),
        "rest16th" => b(0.004, -2.029, 1.376, 0.815),
        "rest32nd" => b(0.0, -2.029, 1.563, 1.839),
        "rest64th" => b(-0.002, -3.032, 1.69, 1.855),
        "flag8thUp" => b(0.001, -3.268, 1.157, 0.048),
        "flag8thDown" => b(0.0, -0.048, 1.235, 3.268),
        "flag16thUp" => b(0.0, -3.281, 1.116, 0.049),
        "flag16thDown" => b(0.0, -0.104, 1.235, 3.213),
        "flag32ndUp" => b(0.0, -3.285, 1.117, 0.745),
        "flag32ndDown" => b(0.0, -0.863, 1.235, 3.214),
        "flag64thUp" => b(0.0, -3.287, 1.117, 1.493),
        "flag64thDown" => b(0.0, -1.61, 1.235, 3.214),
        "accidentalSharp" => b(0.0, -1.332, 0.976, 1.336),
        "accidentalFlat" => b(0.0, -0.704, 0.812, 1.812),
        "accidentalNatural" => b(0.0, -1.292, 0.684, 1.3),
        "accidentalDoubleSharp" => b(0.0, -0.548, 1.1, 0.552),
        "accidentalDoubleFlat" => b(0.0, -0.704, 1.484, 1.812),
        "gClef" => b(0.0, -2.666, 2.56, 4.449),
        "fClef" => b(0.001, -2.468, 2.656, 1.004),
        "cClef" => b(0.0, -1.92, 2.508, 1.928),
        "gClef8va" => b(0.0, -2.664, 2.56, 5.35),
        "gClef8vb" => b(0.0, -3.59, 2.56, 4.45),
        "gClef15ma" => b(0.0, -2.668, 2.56, 5.351),
        "gClef15mb" => b(0.0, -3.591, 2.56, 4.449),
        "fClef8va" => b(0.001, -2.465, 2.66, 1.952),
        "fClef8vb" => b(0.001, -2.968, 2.656, 1.004),
        "fClef15ma" => b(0.001, -2.465, 2.66, 1.968),
        "fClef15mb" => b(0.001, -2.976, 2.652, 1.004),
        "timeSig0" => b(0.06, -1.016, 1.556, 1.02),
        "timeSig1" => b(0.06, -0.972, 1.344, 0.98),
        "timeSig2" => b(0.06, -0.972, 1.508, 0.98),
        "timeSig3" => b(0.06, -0.976, 1.457, 0.976),
        "timeSig4" => b(0.055, -0.992, 1.768, 0.996),
        "timeSig5" => b(0.06, -0.976, 1.448, 0.984),
        "timeSig6" => b(0.06, -0.976, 1.548, 0.98),
        "timeSig7" => b(0.058, -1.0, 1.462, 1.004),
        "timeSig8" => b(0.06, -0.992, 1.572, 0.984),
        "timeSig9" => b(0.06, -0.976, 1.548, 0.98),
        "timeSigCommon" => b(0.0, -1.024, 1.852, 1.032),
        "timeSigCutCommon" => b(0.0, -1.504, 1.852, 1.552),
        "ornamentTrill" => b(-0.048, 0.0, 1.532, 1.532),
        "wiggleTrill" => b(-0.108, 0.468, 0.948, 1.124),
        "breathMarkComma" => b(0.0, 0.001, 0.764, 1.285),
        "caesura" => b(0.0, 0.0, 1.52, 1.96),
        "segno" => b(0.0, 0.0, 2.616, 3.476),
        "coda" => b(0.0, -0.452, 2.98, 2.94),
        "brace" => b(-0.002, 0.019, 0.248, 3.98),
        "unpitchedPercussionClef1" => b(0.0, -1.008, 1.408, 1.008),
        "mensuralWhiteMaxima" | "mensuralWhiteLonga" => bbox_bravura(g),
        _ => None,
    }
}

fn bbox_petaluma(g: &str) -> Option<BBox> {
    let b = |sw_x, sw_y, ne_x, ne_y| Some(BBox { sw_x, sw_y, ne_x, ne_y });
    match g {
        "noteheadDoubleWhole" => b(-0.002, -0.892, 2.457, 0.892),
        "noteheadWhole" => b(0.0, -0.745, 1.521, 0.66),
        "noteheadHalf" => b(0.0, -0.696, 1.336, 0.7),
        "noteheadBlack" => b(0.0, -0.656, 1.336, 0.656),
        "restMaxima" => b(0.0, -1.02, 1.464, 1.02),
        "restLonga" => b(0.0, -0.997, 0.556, 0.99),
        "restDoubleWhole" => b(0.0, 0.0, 0.488, 0.996),
        "restWhole" => b(0.004, -0.48, 2.0, 0.056),
        "restHalf" => b(0.0, 0.0, 2.074, 0.648),
        "restQuarter" => b(-0.002, -1.66, 1.052, 1.66),
        "rest8th" => b(0.0, -1.04, 1.156, 1.044),
        "rest16th" => b(0.0, -1.313, 1.332, 0.976),
        "rest32nd" => b(0.0, -1.99, 1.388, 1.932),
        "rest64th" => b(0.0, -2.88, 1.712, 1.895),
        "flag8thUp" => b(0.0, -3.276, 1.044, 0.0),
        "flag8thDown" => b(0.0, 0.0, 1.044, 3.276),
        "flag16thUp" => b(0.0, -3.278, 1.309, 0.0),
        "flag16thDown" => b(0.0, 0.0, 1.309, 3.276),
        "flag32ndUp" => b(-0.001, -3.398, 1.072, 0.596),
        "flag32ndDown" => b(-0.011, -0.676, 1.072, 3.318),
        "flag64thUp" => b(-0.016, -3.294, 1.128, 1.388),
        "flag64thDown" => b(-0.02, -1.5, 1.124, 3.182),
        "accidentalSharp" => b(-0.088, -1.536, 1.56, 1.532),
        "accidentalFlat" => b(0.004, -0.832, 0.836, 1.888),
        "accidentalNatural" => b(0.0, -1.824, 0.854, 1.848),
        "accidentalDoubleSharp" => b(0.0, -0.636, 1.148, 0.636),
        "accidentalDoubleFlat" => b(0.0, -0.88, 1.436, 1.784),
        "gClef" => b(0.0, -2.236, 2.656, 4.036),
        "fClef" => b(0.0, -1.984, 3.104, 0.864),
        "cClef" => b(0.0, -2.172, 2.924, 2.172),
        "gClef8va" => b(0.0, -2.236, 2.656, 5.253),
        "gClef8vb" => b(0.0, -3.36, 2.656, 4.036),
        "gClef15ma" => b(0.0, -2.236, 2.656, 5.403),
        "gClef15mb" => b(0.0, -3.54, 2.656, 4.036),
        "fClef8va" => b(0.0, -1.984, 3.104, 2.229),
        "fClef8vb" => b(0.0, -3.256, 3.104, 0.864),
        "fClef15ma" => b(0.0, -1.984, 3.104, 2.259),
        "fClef15mb" => b(0.0, -3.424, 3.104, 0.864),
        "timeSig0" => b(0.08, -1.433, 2.052, 1.436),
        "timeSig1" => b(0.08, -1.456, 1.132, 1.464),
        "timeSig2" => b(0.08, -1.515, 2.642, 1.528),
        "timeSig3" => b(0.079, -1.568, 2.16, 1.568),
        "timeSig4" => b(0.08, -1.958, 2.532, 1.965),
        "timeSig5" => b(0.079, -1.556, 2.332, 1.556),
        "timeSig6" => b(0.08, -1.452, 2.26, 1.452),
        "timeSig7" => b(0.081, -1.38, 2.368, 1.38),
        "timeSig8" => b(0.08, -1.712, 2.017, 1.712),
        "timeSig9" => b(0.08, -1.684, 1.976, 1.684),
        "timeSigCommon" => b(0.08, -1.192, 2.34, 1.178),
        "timeSigCutCommon" => b(0.08, -2.748, 2.78, 2.748),
        "ornamentTrill" => b(0.004, -0.072, 2.204, 2.156),
        "wiggleTrill" => b(-0.225, 0.439, 1.473, 1.144),
        "breathMarkComma" => b(0.0, 0.0, 0.592, 1.1),
        "caesura" => b(0.0, 0.0, 2.262, 2.25),
        "segno" => b(0.0, -0.26, 3.192, 3.224),
        "coda" => b(0.0, -0.784, 5.52, 4.148),
        "brace" => b(0.0, 0.0, 0.312, 3.994),
        "unpitchedPercussionClef1" => b(0.0, -1.0, 1.517, 1.0),
        "mensuralWhiteMaxima" | "mensuralWhiteLonga" => bbox_bravura(g),
        _ => None,
    }
}

// ─── Anchors ───────────────────────────────────────────────────────────

#[inline]
pub fn anchor(glyph_name: &str, anchor_name: &str) -> Option<Anchor> {
    anchor_for(FontId::Bravura, glyph_name, anchor_name)
}

#[inline]
pub fn anchor_for(font: FontId, glyph_name: &str, anchor_name: &str) -> Option<Anchor> {
    match font {
        FontId::Bravura => anchor_bravura(glyph_name, anchor_name),
        FontId::Leipzig => anchor_leipzig(glyph_name, anchor_name),
        FontId::Leland => anchor_leland(glyph_name, anchor_name),
        FontId::Petaluma => anchor_petaluma(glyph_name, anchor_name),
    }
}

fn anchor_bravura(glyph_name: &str, anchor_name: &str) -> Option<Anchor> {
    match (glyph_name, anchor_name) {
        ("noteheadBlack", "stemDownNW") => Some(Anchor { x: 0.0, y: -0.168 }),
        ("noteheadBlack", "stemUpSE") => Some(Anchor { x: 1.18, y: 0.168 }),
        ("noteheadHalf", "stemDownNW") => Some(Anchor { x: 0.0, y: -0.168 }),
        ("noteheadHalf", "stemUpSE") => Some(Anchor { x: 1.18, y: 0.168 }),
        _ => None,
    }
}

fn anchor_leipzig(glyph_name: &str, anchor_name: &str) -> Option<Anchor> {
    match (glyph_name, anchor_name) {
        ("noteheadBlack", "stemDownNW") => Some(Anchor { x: 0.0, y: -0.156 }),
        ("noteheadBlack", "stemUpSE") => Some(Anchor { x: 1.256, y: 0.156 }),
        ("noteheadHalf", "stemDownNW") => Some(Anchor { x: 0.0, y: -0.144 }),
        ("noteheadHalf", "stemUpSE") => Some(Anchor { x: 1.256, y: 0.164 }),
        _ => None,
    }
}

fn anchor_leland(glyph_name: &str, anchor_name: &str) -> Option<Anchor> {
    match (glyph_name, anchor_name) {
        ("noteheadBlack", "stemDownNW") => Some(Anchor { x: 0.0, y: -0.168 }),
        ("noteheadBlack", "stemUpSE") => Some(Anchor { x: 1.3, y: 0.16 }),
        ("noteheadHalf", "stemDownNW") => Some(Anchor { x: 0.0, y: -0.168 }),
        ("noteheadHalf", "stemUpSE") => Some(Anchor { x: 1.3, y: 0.16 }),
        _ => None,
    }
}

fn anchor_petaluma(glyph_name: &str, anchor_name: &str) -> Option<Anchor> {
    match (glyph_name, anchor_name) {
        ("noteheadBlack", "stemDownNW") => Some(Anchor { x: 0.0, y: -0.236 }),
        ("noteheadBlack", "stemUpSE") => Some(Anchor { x: 1.336, y: 0.288 }),
        ("noteheadHalf", "stemDownNW") => Some(Anchor { x: 0.0, y: -0.252 }),
        ("noteheadHalf", "stemUpSE") => Some(Anchor { x: 1.312, y: 0.284 }),
        _ => None,
    }
}

