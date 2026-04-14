use std::collections::HashMap;
use std::sync::LazyLock;

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

static ADVANCE_WIDTHS: LazyLock<HashMap<&'static str, f64>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("noteheadWhole", 1.688);
    m.insert("noteheadHalf", 1.18);
    m.insert("noteheadBlack", 1.18);
    m.insert("restWhole", 1.132);
    m.insert("restHalf", 1.132);
    m.insert("restQuarter", 1.08);
    m.insert("rest8th", 1.0);
    m.insert("rest16th", 1.28);
    m.insert("rest32nd", 1.452);
    m.insert("rest64th", 1.696);
    m.insert("flag8thUp", 1.056);
    m.insert("flag8thDown", 1.224);
    m.insert("flag16thUp", 1.116);
    m.insert("flag16thDown", 1.168);
    m.insert("flag32ndUp", 1.048);
    m.insert("flag32ndDown", 1.096);
    m.insert("flag64thUp", 1.048);
    m.insert("flag64thDown", 1.1);
    m.insert("accidentalSharp", 0.996);
    m.insert("accidentalFlat", 0.904);
    m.insert("accidentalNatural", 0.672);
    m.insert("accidentalDoubleSharp", 1.0);
    m.insert("accidentalDoubleFlat", 1.652);
    m.insert("gClef", 2.684);
    m.insert("fClef", 2.736);
    m.insert("cClef", 2.796);
    m.insert("gClef8va", 2.684);
    m.insert("gClef8vb", 2.656);
    m.insert("gClef15ma", 2.684);
    m.insert("gClef15mb", 2.684);
    m.insert("fClef8va", 2.736);
    m.insert("fClef8vb", 2.736);
    m.insert("fClef15ma", 2.736);
    m.insert("fClef15mb", 2.736);
    m.insert("timeSig0", 1.88);
    m.insert("timeSig1", 1.336);
    m.insert("timeSig2", 1.784);
    m.insert("timeSig3", 1.684);
    m.insert("timeSig4", 1.88);
    m.insert("timeSig5", 1.612);
    m.insert("timeSig6", 1.736);
    m.insert("timeSig7", 1.764);
    m.insert("timeSig8", 1.744);
    m.insert("timeSig9", 1.736);
    m.insert("timeSigCommon", 1.696);
    m.insert("timeSigCutCommon", 1.668);
    m.insert("ornamentTrill", 2.084);
    m.insert("wiggleTrill", 0.948);
    m.insert("breathMarkComma", 0.612);
    m.insert("caesura", 1.54);
    m.insert("segno", 2.228);
    m.insert("coda", 3.816);
    m.insert("brace", 0.336);
    m.insert("unpitchedPercussionClef1", 1.528);
    m
});

static BBOXES: LazyLock<HashMap<&'static str, BBox>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    let b = |sw_x, sw_y, ne_x, ne_y| BBox { sw_x, sw_y, ne_x, ne_y };
    m.insert("noteheadWhole", b(0.0, -0.5, 1.688, 0.5));
    m.insert("noteheadHalf", b(0.0, -0.5, 1.18, 0.5));
    m.insert("noteheadBlack", b(0.0, -0.5, 1.18, 0.5));
    m.insert("restWhole", b(0.0, -0.54, 1.128, 0.036));
    m.insert("restHalf", b(0.0, -0.008, 1.128, 0.568));
    m.insert("restQuarter", b(0.004, -1.5, 1.08, 1.492));
    m.insert("rest8th", b(0.0, -1.004, 0.988, 0.696));
    m.insert("rest16th", b(0.0, -2.0, 1.28, 0.716));
    m.insert("rest32nd", b(0.0, -2.0, 1.452, 1.704));
    m.insert("rest64th", b(0.0, -3.012, 1.692, 1.72));
    m.insert("flag8thUp", b(0.0, -3.240768, 1.056, 0.035212));
    m.insert("flag8thDown", b(0.0, -0.057567, 1.224, 3.232897));
    m.insert("flag16thUp", b(0.0, -3.252, 1.116, 0.008));
    m.insert("flag16thDown", b(0.0, -0.036011, 1.164, 3.248026));
    m.insert("flag32ndUp", b(0.0, -3.248, 1.044, 0.596));
    m.insert("flag32ndDown", b(0.0, -0.687477, 1.092, 3.248));
    m.insert("flag64thUp", b(0.0, -3.248, 1.044, 1.387108));
    m.insert("flag64thDown", b(0.0, -1.504026, 1.092, 3.248));
    m.insert("accidentalSharp", b(0.0, -1.392, 0.996, 1.4));
    m.insert("accidentalFlat", b(0.0, -0.7, 0.904, 1.756));
    m.insert("accidentalNatural", b(0.0, -1.34, 0.672, 1.364));
    m.insert("accidentalDoubleSharp", b(0.0, -0.5, 0.988, 0.508));
    m.insert("accidentalDoubleFlat", b(0.0, -0.7, 1.644, 1.748));
    m.insert("gClef", b(0.0, -2.632, 2.684, 4.392));
    m.insert("fClef", b(-0.02, -2.54, 2.736, 1.048));
    m.insert("cClef", b(0.0, -2.024, 2.796, 2.024));
    m.insert("gClef8va", b(0.0, -2.632, 2.684, 5.28));
    m.insert("gClef8vb", b(0.0, -3.512, 2.684, 4.392));
    m.insert("gClef15ma", b(0.0, -2.632, 2.684, 5.276));
    m.insert("gClef15mb", b(0.0, -3.524, 2.684, 4.392));
    m.insert("fClef8va", b(-0.02, -2.54, 2.736, 1.98));
    m.insert("fClef8vb", b(-0.02, -2.976, 2.736, 1.048));
    m.insert("fClef15ma", b(-0.02, -2.54, 2.736, 1.984));
    m.insert("fClef15mb", b(-0.02, -2.968, 2.736, 1.048));
    m.insert("timeSig0", b(0.08, -1.0, 1.8, 1.004));
    m.insert("timeSig1", b(0.08, -1.0, 1.256, 1.004));
    m.insert("timeSig2", b(0.08, -1.028, 1.704, 1.016));
    m.insert("timeSig3", b(0.08, -1.004, 1.604, 0.996));
    m.insert("timeSig4", b(0.08, -1.0, 1.8, 1.004));
    m.insert("timeSig5", b(0.08, -1.004, 1.532, 0.984));
    m.insert("timeSig6", b(0.08, -0.996, 1.656, 1.004));
    m.insert("timeSig7", b(0.08, -1.0, 1.684, 0.996));
    m.insert("timeSig8", b(0.08, -1.036, 1.664, 1.036));
    m.insert("timeSig9", b(0.08, -0.996, 1.656, 1.004));
    m.insert("timeSigCommon", b(0.02, -0.996, 1.696, 1.004));
    m.insert("timeSigCutCommon", b(0.0, -1.436, 1.672, 1.444));
    m.insert("ornamentTrill", b(0.0, -0.04, 2.084, 1.56));
    m.insert("wiggleTrill", b(-0.144, 0.392, 1.08, 0.836));
    m.insert("breathMarkComma", b(0.004, 0.008, 0.608, 1.004));
    m.insert("caesura", b(0.0, -0.004, 1.536, 2.128));
    m.insert("segno", b(0.016, -0.108, 2.2, 3.036));
    m.insert("coda", b(-0.016, -0.632, 3.82, 3.592));
    m.insert("brace", b(0.008, 0.0, 0.328, 3.988));
    m.insert("unpitchedPercussionClef1", b(0.0, -1.0, 1.528, 1.0));
    m
});

static ANCHORS: LazyLock<HashMap<(&'static str, &'static str), Anchor>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    let a = |x, y| Anchor { x, y };
    m.insert(("noteheadBlack", "stemDownNW"), a(0.0, -0.168));
    m.insert(("noteheadBlack", "stemUpSE"), a(1.18, 0.168));
    m.insert(("noteheadHalf", "stemDownNW"), a(0.0, -0.168));
    m.insert(("noteheadHalf", "stemUpSE"), a(1.18, 0.168));
    m
});

pub fn advance_width(glyph_name: &str) -> f64 {
    *ADVANCE_WIDTHS.get(glyph_name).unwrap_or(&0.0)
}

pub fn bbox(glyph_name: &str) -> Option<BBox> {
    BBOXES.get(glyph_name).copied()
}

pub fn anchor(glyph_name: &str, anchor_name: &str) -> Option<Anchor> {
    ANCHORS.get(&(glyph_name, anchor_name)).copied()
}

// ─── SMuFL codepoints ──────────────────────────────────────────────────

pub fn notehead_codepoint(duration: i32) -> u32 {
    match duration {
        1 => 0xE0A2,  // whole
        2 => 0xE0A3,  // half
        _ => 0xE0A4,  // black (quarter and shorter)
    }
}

pub fn notehead_smufl_name(duration: i32) -> &'static str {
    match duration {
        1 => "noteheadWhole",
        2 => "noteheadHalf",
        _ => "noteheadBlack",
    }
}

pub fn rest_codepoint(duration: i32) -> u32 {
    match duration {
        1 => 0xE4E3,
        2 => 0xE4E4,
        4 => 0xE4E5,
        8 => 0xE4E6,
        16 => 0xE4E7,
        32 => 0xE4E8,
        64 => 0xE4E9,
        _ => 0xE4E5,
    }
}

pub fn rest_smufl_name(duration: i32) -> &'static str {
    match duration {
        1 => "restWhole",
        2 => "restHalf",
        4 => "restQuarter",
        8 => "rest8th",
        16 => "rest16th",
        32 => "rest32nd",
        64 => "rest64th",
        _ => "restQuarter",
    }
}

pub fn flag_codepoint(duration: i32, stem_dir: &str) -> Option<u32> {
    match (duration, stem_dir) {
        (8, "up") => Some(0xE240),
        (8, "down") => Some(0xE241),
        (16, "up") => Some(0xE242),
        (16, "down") => Some(0xE243),
        (32, "up") => Some(0xE244),
        (32, "down") => Some(0xE245),
        (64, "up") => Some(0xE246),
        (64, "down") => Some(0xE247),
        _ => None,
    }
}

pub fn flag_smufl_name(duration: i32, stem_dir: &str) -> Option<&'static str> {
    match (duration, stem_dir) {
        (8, "up") => Some("flag8thUp"),
        (8, "down") => Some("flag8thDown"),
        (16, "up") => Some("flag16thUp"),
        (16, "down") => Some("flag16thDown"),
        (32, "up") => Some("flag32ndUp"),
        (32, "down") => Some("flag32ndDown"),
        (64, "up") => Some("flag64thUp"),
        (64, "down") => Some("flag64thDown"),
        _ => None,
    }
}

pub fn accidental_codepoint(acc: &str) -> Option<u32> {
    match acc {
        "sharp" => Some(0xE262),
        "flat" => Some(0xE260),
        "natural" => Some(0xE261),
        "double-sharp" => Some(0xE263),
        "double-flat" => Some(0xE264),
        _ => None,
    }
}

pub fn accidental_smufl_name(acc: &str) -> Option<&'static str> {
    match acc {
        "sharp" => Some("accidentalSharp"),
        "flat" => Some("accidentalFlat"),
        "natural" => Some("accidentalNatural"),
        "double-sharp" => Some("accidentalDoubleSharp"),
        "double-flat" => Some("accidentalDoubleFlat"),
        _ => None,
    }
}

pub fn clef_smufl_name(clef: &str) -> &'static str {
    match clef {
        "treble" => "gClef",
        "bass" => "fClef",
        "alto" | "tenor" => "cClef",
        "treble-8a" | "treble8a" => "gClef8va",
        "treble-8b" | "treble8b" | "treble-8" | "treble8" => "gClef8vb",
        "treble-15a" => "gClef15ma",
        "treble-15b" => "gClef15mb",
        "bass-8a" | "bass8a" => "fClef8va",
        "bass-8b" | "bass8b" => "fClef8vb",
        "bass-15a" => "fClef15ma",
        "bass-15b" => "fClef15mb",
        "percussion" => "unpitchedPercussionClef1",
        _ => "gClef",
    }
}

pub fn clef_codepoint(clef: &str) -> u32 {
    match clef {
        "treble" => 0xE050,
        "bass" => 0xE062,
        "alto" | "tenor" => 0xE05C,
        "treble-8a" | "treble8a" => 0xE053,
        "treble-8b" | "treble8b" | "treble-8" | "treble8" => 0xE052,
        "treble-15a" => 0xE054,
        "treble-15b" => 0xE051,
        "bass-8a" | "bass8a" => 0xE065,
        "bass-8b" | "bass8b" => 0xE064,
        "bass-15a" => 0xE066,
        "bass-15b" => 0xE063,
        "percussion" => 0xE069,
        _ => 0xE050,
    }
}

pub fn clef_origin_offset(clef: &str) -> f64 {
    match clef {
        "treble" | "treble-8a" | "treble8a" | "treble-8b" | "treble8b"
        | "treble-8" | "treble8" | "treble-15a" | "treble-15b" => 3.0,
        "bass" | "bass-8a" | "bass8a" | "bass-8b" | "bass8b"
        | "bass-15a" | "bass-15b" => 1.0,
        "alto" => 4.0,
        "tenor" => 4.0,
        "percussion" => 4.0,
        _ => 3.0,
    }
}

pub fn time_sig_digit_codepoint(digit: u32) -> u32 {
    0xE080 + digit
}

pub const TIME_SIG_COMMON: u32 = 0xE08A;
pub const TIME_SIG_CUT: u32 = 0xE08B;

// Dynamic glyphs
pub fn dynamic_codepoint(ch: char) -> Option<u32> {
    match ch {
        'p' => Some(0xE520),
        'm' => Some(0xE521),
        'f' => Some(0xE522),
        'r' => Some(0xE523),
        's' => Some(0xE524),
        'z' => Some(0xE525),
        _ => None,
    }
}

// Articulation glyphs
pub fn articulation_codepoint(art: &str, above: bool) -> Option<u32> {
    match (art, above) {
        ("staccato", true) => Some(0xE4A2),
        ("staccato", false) => Some(0xE4A3),
        ("accent", true) => Some(0xE4A0),
        ("accent", false) => Some(0xE4A1),
        ("tenuto", true) => Some(0xE4A4),
        ("tenuto", false) => Some(0xE4A5),
        ("fermata", _) => Some(0xE4C0), // always above
        _ => None,
    }
}

pub const ORNAMENT_TRILL: u32 = 0xE566;
pub const WIGGLE_TRILL: u32 = 0xEAA4;
pub const BREATH_MARK: u32 = 0xE4CE;
pub const CAESURA: u32 = 0xE4D1;
pub const SEGNO: u32 = 0xE047;
pub const CODA: u32 = 0xE048;
pub const BRACE: u32 = 0xE000;

pub fn staff_marker_codepoint(kind: &str) -> Option<u32> {
    match kind {
        "breath-mark" => Some(BREATH_MARK),
        "caesura" => Some(CAESURA),
        "dal-segno" => Some(SEGNO),
        "coda" => Some(CODA),
        _ => None,
    }
}

pub fn staff_marker_smufl_name(kind: &str) -> Option<&'static str> {
    match kind {
        "breath-mark" => Some("breathMarkComma"),
        "caesura" => Some("caesura"),
        "dal-segno" => Some("segno"),
        "coda" => Some("coda"),
        _ => None,
    }
}
