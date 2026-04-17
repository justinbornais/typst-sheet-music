// ─── Glyph metadata (zero-allocation match-based lookups) ──────────────

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

#[inline]
pub fn advance_width(glyph_name: &str) -> f64 {
    match glyph_name {
        "mensuralWhiteLonga" => 1.3,
        "noteheadDoubleWhole" => 2.396,
        "noteheadWhole" => 1.688,
        "noteheadHalf" | "noteheadBlack" => 1.18,
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

#[inline]
pub fn bbox(glyph_name: &str) -> Option<BBox> {
    let b = |sw_x, sw_y, ne_x, ne_y| {
        Some(BBox {
            sw_x,
            sw_y,
            ne_x,
            ne_y,
        })
    };
    match glyph_name {
        "mensuralWhiteLonga" => b(0.0, -3.548, 1.3, 0.684),
        "noteheadDoubleWhole" => b(0.0, -0.62, 2.396, 0.62),
        "noteheadWhole" => b(0.0, -0.5, 1.688, 0.5),
        "noteheadHalf" | "noteheadBlack" => b(0.0, -0.5, 1.18, 0.5),
        "restLonga" => b(0.0, -0.996, 0.5, 1.0),
        "restDoubleWhole" => b(0.0, 0.0, 0.5, 1.0),
        "restWhole" => b(0.0, -0.54, 1.128, 0.036),
        "restHalf" => b(0.0, -0.008, 1.128, 0.568),
        "restQuarter" => b(0.004, -1.5, 1.08, 1.492),
        "rest8th" => b(0.0, -1.004, 0.988, 0.696),
        "rest16th" => b(0.0, -2.0, 1.28, 0.716),
        "rest32nd" => b(0.0, -2.0, 1.452, 1.704),
        "rest64th" => b(0.0, -3.012, 1.692, 1.72),
        "flag8thUp" => b(0.0, -3.240768, 1.056, 0.035212),
        "flag8thDown" => b(0.0, -0.057567, 1.224, 3.232897),
        "flag16thUp" => b(0.0, -3.252, 1.116, 0.008),
        "flag16thDown" => b(0.0, -0.036011, 1.164, 3.248026),
        "flag32ndUp" => b(0.0, -3.248, 1.044, 0.596),
        "flag32ndDown" => b(0.0, -0.687477, 1.092, 3.248),
        "flag64thUp" => b(0.0, -3.248, 1.044, 1.387108),
        "flag64thDown" => b(0.0, -1.504026, 1.092, 3.248),
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

#[inline]
pub fn anchor(glyph_name: &str, anchor_name: &str) -> Option<Anchor> {
    match (glyph_name, anchor_name) {
        ("noteheadBlack", "stemDownNW") => Some(Anchor { x: 0.0, y: -0.168 }),
        ("noteheadBlack", "stemUpSE") => Some(Anchor { x: 1.18, y: 0.168 }),
        ("noteheadHalf", "stemDownNW") => Some(Anchor { x: 0.0, y: -0.168 }),
        ("noteheadHalf", "stemUpSE") => Some(Anchor { x: 1.18, y: 0.168 }),
        _ => None,
    }
}
