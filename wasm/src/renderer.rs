use crate::glyph;
use crate::layout;
use crate::pitch;
use crate::types::*;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::sync::{Mutex, OnceLock};
use ttf_parser::{Face, GlyphId, OutlineBuilder};

// ─── Constants ─────────────────────────────────────────────────────────

const ACCIDENTAL_PADDING: f64 = 0.35;
const INLINE_CLEF_SCALE: f64 = 0.8;
const CLEF_PADDING: f64 = 0.5;
const GRACE_NOTE_SCALE: f64 = 0.68;
const GRACE_STEM_MIN_LENGTH: f64 = 3.0;
const MUSIC_START_PADDING: f64 = 1.55;
const LELAND_FONT: &[u8] = include_bytes!("../../fonts/Leland.otf");

// ─── SMuFL codepoint helpers ───────────────────────────────────────────

fn notehead_codepoint(duration: i32) -> u32 {
    match duration {
        DURATION_MAXIMA => 0xE95C, // mensuralWhiteMaxima
        DURATION_LONGA => 0xE95D,  // mensuralWhiteLonga
        DURATION_BREVE => 0xE0A0,  // noteheadDoubleWhole
        1 => 0xE0A2,               // noteheadWhole
        2 => 0xE0A3,               // noteheadHalf
        _ => 0xE0A4,               // noteheadBlack
    }
}

fn notehead_smufl(duration: i32) -> &'static str {
    match duration {
        DURATION_MAXIMA => "mensuralWhiteMaxima",
        DURATION_LONGA => "mensuralWhiteLonga",
        DURATION_BREVE => "noteheadDoubleWhole",
        1 => "noteheadWhole",
        2 => "noteheadHalf",
        _ => "noteheadBlack",
    }
}

fn rest_codepoint(duration: i32) -> u32 {
    match duration {
        DURATION_MAXIMA => 0xE4E0,
        DURATION_LONGA => 0xE4E1,
        DURATION_BREVE => 0xE4E2,
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

fn rest_smufl(duration: i32) -> &'static str {
    match duration {
        DURATION_MAXIMA => "restMaxima",
        DURATION_LONGA => "restLonga",
        DURATION_BREVE => "restDoubleWhole",
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

fn flag_codepoint(duration: i32, stem_dir: &str) -> Option<u32> {
    match (duration, stem_dir) {
        (8, "up") => Some(0xE240),
        (16, "up") => Some(0xE242),
        (32, "up") => Some(0xE244),
        (64, "up") => Some(0xE246),
        (8, "down") => Some(0xE241),
        (16, "down") => Some(0xE243),
        (32, "down") => Some(0xE245),
        (64, "down") => Some(0xE247),
        _ => None,
    }
}

fn flag_smufl(duration: i32, stem_dir: &str) -> Option<&'static str> {
    match (duration, stem_dir) {
        (8, "up") => Some("flag8thUp"),
        (16, "up") => Some("flag16thUp"),
        (32, "up") => Some("flag32ndUp"),
        (64, "up") => Some("flag64thUp"),
        (8, "down") => Some("flag8thDown"),
        (16, "down") => Some("flag16thDown"),
        (32, "down") => Some("flag32ndDown"),
        (64, "down") => Some("flag64thDown"),
        _ => None,
    }
}

fn accidental_codepoint(acc: &str) -> Option<u32> {
    match acc {
        "sharp" => Some(0xE262),
        "flat" => Some(0xE260),
        "natural" => Some(0xE261),
        "double-sharp" => Some(0xE263),
        "double-flat" => Some(0xE264),
        _ => None,
    }
}

fn accidental_smufl(acc: &str) -> Option<&'static str> {
    match acc {
        "sharp" => Some("accidentalSharp"),
        "flat" => Some("accidentalFlat"),
        "natural" => Some("accidentalNatural"),
        "double-sharp" => Some("accidentalDoubleSharp"),
        "double-flat" => Some("accidentalDoubleFlat"),
        _ => None,
    }
}

fn clef_smufl(clef: &str) -> &'static str {
    match clef {
        "treble" => "gClef",
        "bass" => "fClef",
        "alto" | "tenor" => "cClef",
        "treble-8a" | "treble8a" => "gClef8va",
        "treble-8b" | "treble8b" | "treble-8" | "treble8" => "gClef8vb",
        "bass-8a" | "bass8a" => "fClef8va",
        "bass-8b" | "bass8b" => "fClef8vb",
        "treble-15a" => "gClef15ma",
        "treble-15b" => "gClef15mb",
        "bass-15a" => "fClef15ma",
        "bass-15b" => "fClef15mb",
        "percussion" => "unpitchedPercussionClef1",
        _ => "gClef",
    }
}

fn clef_codepoint(clef: &str) -> u32 {
    match clef {
        "treble" => 0xE050,
        "bass" => 0xE062,
        "alto" | "tenor" => 0xE05C,
        "treble-8a" | "treble8a" => 0xE053,
        "treble-8b" | "treble8b" | "treble-8" | "treble8" => 0xE052,
        "bass-8a" | "bass8a" => 0xE065,
        "bass-8b" | "bass8b" => 0xE064,
        "treble-15a" => 0xE054,
        "treble-15b" => 0xE051,
        "bass-15a" => 0xE066,
        "bass-15b" => 0xE063,
        "percussion" => 0xE069,
        _ => 0xE050,
    }
}

fn clef_origin_offset(clef: &str) -> f64 {
    match clef {
        "treble" | "treble-8a" | "treble8a" | "treble-8b" | "treble8b" | "treble-8" | "treble8"
        | "treble-15a" | "treble-15b" => 3.0,
        "bass" | "bass-8a" | "bass8a" | "bass-8b" | "bass8b" | "bass-15a" | "bass-15b" => 1.0,
        "alto" => 2.0,
        "tenor" => 2.0,
        "percussion" => 2.0,
        _ => 3.0,
    }
}

fn time_digit_codepoint(d: u32) -> u32 {
    0xE080 + d
}

const TIME_SIG_NAMES: &[&str] = &[
    "timeSig0", "timeSig1", "timeSig2", "timeSig3", "timeSig4", "timeSig5", "timeSig6", "timeSig7",
    "timeSig8", "timeSig9",
];

fn time_sig_digits_width(digits: &str, sp: f64, font: glyph::FontId) -> f64 {
    digits
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .map(|d| glyph::advance_width_for(font, TIME_SIG_NAMES[d as usize]) * sp)
        .sum()
}

fn dynamic_codepoint(ch: char) -> Option<u32> {
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

fn articulation_codepoint(art: &str, above: bool) -> Option<u32> {
    match (art, above) {
        ("staccato", true) => Some(0xE4A2),
        ("staccato", false) => Some(0xE4A3),
        ("accent", true) => Some(0xE4A0),
        ("accent", false) => Some(0xE4A1),
        ("tenuto", true) => Some(0xE4A4),
        ("tenuto", false) => Some(0xE4A5),
        ("marcato", true) => Some(0xE4AC),
        ("marcato", false) => Some(0xE4AD),
        ("fermata", true) => Some(0xE4C0),
        ("fermata", false) => Some(0xE4C1),
        _ => None,
    }
}

fn staff_marker_codepoint(kind: &str) -> Option<u32> {
    match kind {
        "breath-mark" => Some(0xE4CE),
        "caesura" => Some(0xE4D1),
        "dal-segno" => Some(0xE047),
        "coda" => Some(0xE048),
        _ => None,
    }
}

// ─── Glyph placement helpers ──────────────────────────────────────────

/// Place a glyph using its SMuFL bounding-box SW corner as the south-west anchor.
/// The rendered origin (reference point) ends up at (x, y).
#[inline]
fn emit_glyph(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    y: f64,
    smufl_name: &str,
    codepoint: u32,
    sp: f64,
    font: glyph::FontId,
) {
    let fsize = 4.0 * sp;
    let bb = glyph::bbox_for(font, smufl_name);
    let (px, py) = if let Some(b) = bb {
        (x + b.sw_x * sp, y + b.sw_y * sp)
    } else {
        (x, y)
    };
    cmds.push(DrawCmd::Glyph {
        x: px,
        y: py,
        c: codepoint,
        s: fsize,
        a: "south-west".into(),
    });
}

/// Place a glyph with an explicit text anchor and NO bounding-box offset.
/// Use this for articulations and dynamics where the coordinate is the
/// desired glyph edge ("south" = bottom at y, "north" = top at y, etc.).
#[inline]
fn emit_glyph_anchored(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    y: f64,
    codepoint: u32,
    sp: f64,
    anchor: &'static str,
) {
    cmds.push(DrawCmd::Glyph {
        x,
        y,
        c: codepoint,
        s: 4.0 * sp,
        a: anchor.into(),
    });
}

fn emit_glyph_scaled(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    y: f64,
    smufl_name: &str,
    codepoint: u32,
    sp: f64,
    font: glyph::FontId,
) {
    emit_glyph(cmds, x, y, smufl_name, codepoint, sp, font);
}

#[inline]
fn emit_line(cmds: &mut Vec<DrawCmd>, x1: f64, y1: f64, x2: f64, y2: f64, w: f64) {
    cmds.push(DrawCmd::Line { x1, y1, x2, y2, w });
}

fn xml_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

fn svg_anchor_attrs(anchor: &str) -> (&'static str, &'static str) {
    let text_anchor = if anchor.contains("east") {
        "end"
    } else if anchor.contains("west") {
        "start"
    } else {
        "middle"
    };

    let baseline = if anchor.contains("north") {
        "text-before-edge"
    } else if anchor.contains("south") {
        "text-after-edge"
    } else {
        "central"
    };

    (text_anchor, baseline)
}

fn update_bounds(bounds: &mut (f64, f64, f64, f64), x: f64, y: f64) {
    bounds.0 = bounds.0.min(x);
    bounds.1 = bounds.1.min(y);
    bounds.2 = bounds.2.max(x);
    bounds.3 = bounds.3.max(y);
}

struct SvgPathBuilder<'a> {
    data: &'a mut String,
    sw_x: f64,
    sw_y: f64,
    scale: f64,
    x_offset: f64,
    y_offset: f64,
}

impl SvgPathBuilder<'_> {
    fn sx(&self, x: f32) -> f64 {
        self.sw_x + (x as f64 + self.x_offset) * self.scale
    }

    fn sy(&self, y: f32) -> f64 {
        self.sw_y - (y as f64 + self.y_offset) * self.scale
    }
}

impl OutlineBuilder for SvgPathBuilder<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        let _ = write!(self.data, "M{:.3} {:.3}", self.sx(x), self.sy(y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let _ = write!(self.data, "L{:.3} {:.3}", self.sx(x), self.sy(y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let _ = write!(
            self.data,
            "Q{:.3} {:.3} {:.3} {:.3}",
            self.sx(x1),
            self.sy(y1),
            self.sx(x),
            self.sy(y)
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let _ = write!(
            self.data,
            "C{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}",
            self.sx(x1),
            self.sy(y1),
            self.sx(x2),
            self.sy(y2),
            self.sx(x),
            self.sy(y)
        );
    }

    fn close(&mut self) {
        self.data.push('Z');
    }
}

fn anchor_sw(x: f64, y: f64, width: f64, height: f64, anchor: &str) -> (f64, f64) {
    let sw_x = if anchor.contains("east") {
        x - width
    } else if anchor.contains("west") {
        x
    } else {
        x - width / 2.0
    };

    let sw_y = if anchor.contains("north") {
        y - height
    } else if anchor.contains("south") {
        y
    } else {
        y - height / 2.0
    };

    (sw_x, sw_y)
}

fn glyph_id(face: &Face<'_>, ch: char) -> Option<GlyphId> {
    face.glyph_index(ch)
}

fn glyph_def_id(gid: GlyphId) -> String {
    format!("b{:x}", gid.0)
}

fn build_music_glyph_def(face: &Face<'_>, gid: GlyphId) -> Option<String> {
    let bbox = face.glyph_bounding_box(gid)?;
    let mut path = String::new();
    {
        let mut builder = SvgPathBuilder {
            data: &mut path,
            sw_x: 0.0,
            sw_y: 0.0,
            scale: 1.0,
            x_offset: -(bbox.x_min as f64),
            y_offset: -(bbox.y_min as f64),
        };
        face.outline_glyph(gid, &mut builder)?;
    }

    if path.is_empty() {
        None
    } else {
        Some(path)
    }
}

fn ensure_music_glyph_def(defs: &mut BTreeMap<u16, String>, face: &Face<'_>, gid: GlyphId) {
    if defs.contains_key(&gid.0) {
        return;
    }

    static GLYPH_DEFS: OnceLock<Mutex<BTreeMap<u16, String>>> = OnceLock::new();
    let cache = GLYPH_DEFS.get_or_init(|| Mutex::new(BTreeMap::new()));
    let path = {
        let mut cache = cache
            .lock()
            .expect("Bravura glyph definition cache poisoned");
        cache
            .entry(gid.0)
            .or_insert_with(|| build_music_glyph_def(face, gid).unwrap_or_default())
            .clone()
    };

    if !path.is_empty() {
        defs.insert(gid.0, path);
    }
}

fn collect_music_glyph_defs(face: &Face<'_>, cmds: &[DrawCmd]) -> BTreeMap<u16, String> {
    let mut defs = BTreeMap::new();

    for cmd in cmds {
        match cmd {
            DrawCmd::Glyph { c, .. } => {
                if let Some(ch) = char::from_u32(*c) {
                    if let Some(gid) = glyph_id(face, ch) {
                        ensure_music_glyph_def(&mut defs, face, gid);
                    }
                }
            }
            DrawCmd::MusicText { v, .. } => {
                for ch in v.chars() {
                    if let Some(gid) = glyph_id(face, ch) {
                        ensure_music_glyph_def(&mut defs, face, gid);
                    }
                }
            }
            _ => {}
        }
    }

    defs.retain(|_, path| !path.is_empty());
    defs
}

fn write_music_glyph_use(
    svg: &mut String,
    face: &Face<'_>,
    defs: &BTreeMap<u16, String>,
    x: f64,
    y: f64,
    c: u32,
    size_mm: f64,
    anchor: &str,
    tx: &impl Fn(f64) -> f64,
    ty: &impl Fn(f64) -> f64,
) -> bool {
    let Some(ch) = char::from_u32(c) else {
        return false;
    };
    let Some(gid) = glyph_id(face, ch) else {
        return false;
    };
    if !defs.contains_key(&gid.0) {
        return false;
    }
    let Some(bbox) = face.glyph_bounding_box(gid) else {
        return false;
    };

    let scale = size_mm / face.units_per_em() as f64;
    let width = (bbox.x_max - bbox.x_min) as f64 * scale;
    let height = (bbox.y_max - bbox.y_min) as f64 * scale;
    let (sw_x, sw_y) = anchor_sw(x, y, width, height, anchor);
    let _ = write!(
        svg,
        "<use href=\"#{id}\" transform=\"translate({x:.3} {y:.3}) scale({scale:.6})\" fill=\"black\"/>",
        id = glyph_def_id(gid),
        x = tx(sw_x),
        y = ty(sw_y),
        scale = scale,
    );
    true
}

fn write_music_text_uses(
    svg: &mut String,
    face: &Face<'_>,
    defs: &BTreeMap<u16, String>,
    x: f64,
    y: f64,
    value: &str,
    size_mm: f64,
    anchor: &str,
    tx: &impl Fn(f64) -> f64,
    ty: &impl Fn(f64) -> f64,
) -> bool {
    let scale = size_mm / face.units_per_em() as f64;
    let mut glyphs = Vec::new();
    let mut pen = 0.0_f64;
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for ch in value.chars() {
        let Some(gid) = glyph_id(face, ch) else {
            return false;
        };
        if !defs.contains_key(&gid.0) {
            return false;
        }
        let Some(bbox) = face.glyph_bounding_box(gid) else {
            return false;
        };
        min_x = min_x.min(pen + bbox.x_min as f64);
        min_y = min_y.min(bbox.y_min as f64);
        max_x = max_x.max(pen + bbox.x_max as f64);
        max_y = max_y.max(bbox.y_max as f64);
        glyphs.push((gid, pen, bbox));
        pen += face.glyph_hor_advance(gid).unwrap_or(0) as f64;
    }

    if glyphs.is_empty() || !min_x.is_finite() {
        return false;
    }

    let width = (max_x - min_x) * scale;
    let height = (max_y - min_y) * scale;
    let (sw_x, sw_y) = anchor_sw(x, y, width, height, anchor);
    let svg_sw_x = tx(sw_x);
    let svg_sw_y = ty(sw_y);

    for (gid, glyph_pen, bbox) in glyphs {
        let use_x = svg_sw_x + (glyph_pen + bbox.x_min as f64 - min_x) * scale;
        let use_y = svg_sw_y - (bbox.y_min as f64 - min_y) * scale;
        let _ = write!(
            svg,
            "<use href=\"#{id}\" transform=\"translate({x:.3} {y:.3}) scale({scale:.6})\" fill=\"black\"/>",
            id = glyph_def_id(gid),
            x = use_x,
            y = use_y,
            scale = scale,
        );
    }

    true
}

fn music_glyph_bounds(
    face: &Face<'_>,
    x: f64,
    y: f64,
    c: u32,
    size_mm: f64,
    anchor: &str,
) -> Option<(f64, f64, f64, f64)> {
    let ch = char::from_u32(c)?;
    let gid = glyph_id(face, ch)?;
    let bbox = face.glyph_bounding_box(gid)?;
    let scale = size_mm / face.units_per_em() as f64;
    let width = (bbox.x_max - bbox.x_min) as f64 * scale;
    let height = (bbox.y_max - bbox.y_min) as f64 * scale;
    let (sw_x, sw_y) = anchor_sw(x, y, width, height, anchor);
    Some((sw_x, sw_y, sw_x + width, sw_y + height))
}

fn music_text_bounds(
    face: &Face<'_>,
    x: f64,
    y: f64,
    value: &str,
    size_mm: f64,
    anchor: &str,
) -> Option<(f64, f64, f64, f64)> {
    let scale = size_mm / face.units_per_em() as f64;
    let mut pen = 0.0_f64;
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for ch in value.chars() {
        let gid = glyph_id(face, ch)?;
        if let Some(b) = face.glyph_bounding_box(gid) {
            min_x = min_x.min(pen + b.x_min as f64);
            min_y = min_y.min(b.y_min as f64);
            max_x = max_x.max(pen + b.x_max as f64);
            max_y = max_y.max(b.y_max as f64);
        }
        pen += face.glyph_hor_advance(gid).unwrap_or(0) as f64;
    }

    if !min_x.is_finite() {
        return None;
    }

    let width = (max_x - min_x) * scale;
    let height = (max_y - min_y) * scale;
    let (sw_x, sw_y) = anchor_sw(x, y, width, height, anchor);
    Some((sw_x, sw_y, sw_x + width, sw_y + height))
}

fn bbox(sw_x: f64, sw_y: f64, ne_x: f64, ne_y: f64) -> glyph::BBox {
    glyph::BBox {
        sw_x,
        sw_y,
        ne_x,
        ne_y,
    }
}

fn smufl_name_for_codepoint(codepoint: u32) -> Option<&'static str> {
    match codepoint {
        0xE0A0 => Some("noteheadDoubleWhole"),
        0xE0A2 => Some("noteheadWhole"),
        0xE0A3 => Some("noteheadHalf"),
        0xE0A4 => Some("noteheadBlack"),
        0xE95C => Some("mensuralWhiteMaxima"),
        0xE95D => Some("mensuralWhiteLonga"),
        0xE4E0 => Some("restMaxima"),
        0xE4E1 => Some("restLonga"),
        0xE4E2 => Some("restDoubleWhole"),
        0xE4E3 => Some("restWhole"),
        0xE4E4 => Some("restHalf"),
        0xE4E5 => Some("restQuarter"),
        0xE4E6 => Some("rest8th"),
        0xE4E7 => Some("rest16th"),
        0xE4E8 => Some("rest32nd"),
        0xE4E9 => Some("rest64th"),
        0xE240 => Some("flag8thUp"),
        0xE241 => Some("flag8thDown"),
        0xE242 => Some("flag16thUp"),
        0xE243 => Some("flag16thDown"),
        0xE244 => Some("flag32ndUp"),
        0xE245 => Some("flag32ndDown"),
        0xE246 => Some("flag64thUp"),
        0xE247 => Some("flag64thDown"),
        0xE260 => Some("accidentalFlat"),
        0xE261 => Some("accidentalNatural"),
        0xE262 => Some("accidentalSharp"),
        0xE263 => Some("accidentalDoubleSharp"),
        0xE264 => Some("accidentalDoubleFlat"),
        0xE050 => Some("gClef"),
        0xE051 => Some("gClef15mb"),
        0xE052 => Some("gClef8vb"),
        0xE053 => Some("gClef8va"),
        0xE054 => Some("gClef15ma"),
        0xE05C => Some("cClef"),
        0xE062 => Some("fClef"),
        0xE063 => Some("fClef15mb"),
        0xE064 => Some("fClef8vb"),
        0xE065 => Some("fClef8va"),
        0xE066 => Some("fClef15ma"),
        0xE069 => Some("unpitchedPercussionClef1"),
        0xE080..=0xE089 => TIME_SIG_NAMES.get((codepoint - 0xE080) as usize).copied(),
        0xE08A => Some("timeSigCommon"),
        0xE08B => Some("timeSigCutCommon"),
        0xE566 => Some("ornamentTrill"),
        0xEAA4 => Some("wiggleTrill"),
        0xE4CE => Some("breathMarkComma"),
        0xE4D1 => Some("caesura"),
        0xE047 => Some("segno"),
        0xE048 => Some("coda"),
        0xE000 => Some("brace"),
        _ => None,
    }
}

fn smufl_bbox_for_codepoint(codepoint: u32, font: glyph::FontId) -> Option<glyph::BBox> {
    if let Some(name) = smufl_name_for_codepoint(codepoint) {
        if let Some(bbox) = glyph::bbox_for(font, name) {
            return Some(bbox);
        }
    }

    match codepoint {
        0xE520 => Some(bbox(-0.356, -0.568, 1.464, 1.096)), // dynamicPiano
        0xE521 => Some(bbox(-0.08, -0.04, 1.784, 1.096)),   // dynamicMezzo
        0xE522 => Some(bbox(-0.564, -0.608, 1.456, 1.776)), // dynamicForte
        0xE523 => Some(bbox(-0.08, 0.0, 1.108, 1.096)),     // dynamicRinforzando
        0xE524 => Some(bbox(0.0, -0.04, 0.916, 1.092)),     // dynamicSforzando
        0xE525 => Some(bbox(-0.12, -0.04, 0.976, 1.072)),   // dynamicZ
        0xE4A0 => Some(bbox(0.0, 0.004, 1.356, 0.98)),      // articAccentAbove
        0xE4A1 => Some(bbox(0.0, -0.976, 1.356, 0.0)),      // articAccentBelow
        0xE4A2 => Some(bbox(0.0, 0.0, 0.336, 0.336)),       // articStaccatoAbove
        0xE4A3 => Some(bbox(0.0, -0.336, 0.336, 0.0)),      // articStaccatoBelow
        0xE4A4 => Some(bbox(-0.004, 0.0, 1.352, 0.192)),    // articTenutoAbove
        0xE4A5 => Some(bbox(-0.004, -0.192, 1.352, 0.0)),   // articTenutoBelow
        0xE4AC => Some(bbox(-0.004, -0.004, 0.94, 1.012)),  // articMarcatoAbove
        0xE4AD => Some(bbox(-0.004, -1.016, 0.94, 0.0)),    // articMarcatoBelow
        0xE4C0 => Some(bbox(0.012, -0.012, 2.42, 1.316)),   // fermataAbove
        0xE4C1 => Some(bbox(0.012, -1.328, 2.42, 0.0)),     // fermataBelow
        _ => None,
    }
}

fn smufl_advance_for_codepoint(codepoint: u32, font: glyph::FontId) -> Option<f64> {
    if let Some(name) = smufl_name_for_codepoint(codepoint) {
        let advance = glyph::advance_width_for(font, name);
        if advance > 0.0 {
            return Some(advance);
        }
    }

    match codepoint {
        0xE520 => Some(1.46),  // dynamicPiano
        0xE521 => Some(1.748), // dynamicMezzo
        0xE522 => Some(1.456), // dynamicForte
        0xE523 => Some(1.108), // dynamicRinforzando
        0xE524 => Some(0.916), // dynamicSforzando
        0xE525 => Some(0.976), // dynamicZ
        0xE4A0 | 0xE4A1 => Some(1.356),
        0xE4A2 | 0xE4A3 => Some(0.336),
        0xE4A4 | 0xE4A5 => Some(1.352),
        0xE4AC | 0xE4AD => Some(0.944),
        0xE4C0 | 0xE4C1 => Some(2.42),
        _ => None,
    }
}

fn music_text_bbox(value: &str, font: glyph::FontId) -> Option<glyph::BBox> {
    let mut pen = 0.0;
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for ch in value.chars() {
        let codepoint = ch as u32;
        let bbox = smufl_bbox_for_codepoint(codepoint, font)?;
        min_x = min_x.min(pen + bbox.sw_x);
        min_y = min_y.min(bbox.sw_y);
        max_x = max_x.max(pen + bbox.ne_x);
        max_y = max_y.max(bbox.ne_y);
        pen += smufl_advance_for_codepoint(codepoint, font)?;
    }

    if min_x.is_finite() {
        Some(bbox(min_x, min_y, max_x, max_y))
    } else {
        None
    }
}

fn anchored_text_origin(
    x: f64,
    y: f64,
    bbox: glyph::BBox,
    font_size_mm: f64,
    anchor: &str,
) -> (f64, f64) {
    let scale = font_size_mm / 4.0;
    let anchor_x = if anchor.contains("east") {
        bbox.ne_x
    } else if anchor.contains("west") {
        bbox.sw_x
    } else {
        (bbox.sw_x + bbox.ne_x) / 2.0
    };
    let anchor_y = if anchor.contains("north") {
        bbox.ne_y
    } else if anchor.contains("south") {
        bbox.sw_y
    } else {
        (bbox.sw_y + bbox.ne_y) / 2.0
    };

    (x - anchor_x * scale, y - anchor_y * scale)
}

fn svg_from_cmds(
    cmds: &[DrawCmd],
    width_mm: f64,
    fallback_height_mm: f64,
    music_font: &str,
) -> String {
    let mut bounds = (0.0, -fallback_height_mm, width_mm, 0.0);
    let mut ox = 0.0;
    let mut oy = 0.0;
    let font_id = glyph::FontId::from_name(music_font);
    let music_face = if music_font == "Leland" {
        Face::parse(LELAND_FONT, 0).ok()
    } else {
        None
    };

    for cmd in cmds {
        match cmd {
            DrawCmd::Line { x1, y1, x2, y2, w } => {
                let pad = *w * 0.5;
                update_bounds(&mut bounds, ox + x1 - pad, oy + y1 - pad);
                update_bounds(&mut bounds, ox + x2 + pad, oy + y2 + pad);
            }
            DrawCmd::Glyph { x, y, c, s, a } => {
                if let Some((x0, y0, x1, y1)) = music_face
                    .as_ref()
                    .and_then(|face| music_glyph_bounds(face, ox + x, oy + y, *c, *s, a))
                {
                    update_bounds(&mut bounds, x0, y0);
                    update_bounds(&mut bounds, x1, y1);
                } else if let Some(bbox) = smufl_bbox_for_codepoint(*c, font_id) {
                    let scale = *s / 4.0;
                    let (text_x, text_y) = anchored_text_origin(ox + x, oy + y, bbox, *s, a);
                    update_bounds(
                        &mut bounds,
                        text_x + bbox.sw_x * scale,
                        text_y + bbox.sw_y * scale,
                    );
                    update_bounds(
                        &mut bounds,
                        text_x + bbox.ne_x * scale,
                        text_y + bbox.ne_y * scale,
                    );
                } else {
                    let pad = *s;
                    update_bounds(&mut bounds, ox + x - pad, oy + y - pad);
                    update_bounds(&mut bounds, ox + x + pad, oy + y + pad);
                }
            }
            DrawCmd::MusicText { x, y, v, s, a } => {
                if let Some((x0, y0, x1, y1)) = music_face
                    .as_ref()
                    .and_then(|face| music_text_bounds(face, ox + x, oy + y, v, *s, a))
                {
                    update_bounds(&mut bounds, x0, y0);
                    update_bounds(&mut bounds, x1, y1);
                } else {
                    let pad_x = *s * v.chars().count().max(1) as f64;
                    update_bounds(&mut bounds, ox + x - pad_x, oy + y - *s);
                    update_bounds(&mut bounds, ox + x + pad_x, oy + y + *s);
                }
            }
            DrawCmd::Text { x, y, v, s, .. } => {
                let size_mm = *s * 25.4 / 72.0;
                let pad_x = size_mm * 0.65 * v.chars().count().max(1) as f64;
                update_bounds(&mut bounds, ox + x - pad_x, oy + y - size_mm);
                update_bounds(&mut bounds, ox + x + pad_x, oy + y + size_mm);
            }
            DrawCmd::Polygon { pts } => {
                let mut i = 0;
                while i + 1 < pts.len() {
                    update_bounds(&mut bounds, ox + pts[i], oy + pts[i + 1]);
                    i += 2;
                }
            }
            DrawCmd::BezierFill { pts } => {
                let mut i = 0;
                while i + 1 < pts.len() {
                    update_bounds(&mut bounds, ox + pts[i], oy + pts[i + 1]);
                    i += 2;
                }
            }
            DrawCmd::Circle { x, y, r } => {
                update_bounds(&mut bounds, ox + x - r, oy + y - r);
                update_bounds(&mut bounds, ox + x + r, oy + y + r);
            }
            DrawCmd::MoveOrigin { dx, dy } => {
                ox += dx;
                oy += dy;
            }
            DrawCmd::FlushContent => {}
        }
    }

    let margin = 1.5;
    let min_x = bounds.0.min(0.0) - margin;
    let max_x = bounds.2.max(width_mm) + margin;
    let min_y = bounds.1 - margin;
    let max_y = bounds.3 + margin;
    let vb_w = (max_x - min_x).max(1.0);
    let vb_h = (max_y - min_y).max(1.0);

    let tx = |x: f64| x - min_x;
    let ty = |y: f64| max_y - y;

    let escaped_music_font = xml_escape(music_font);
    let mut svg = String::with_capacity(cmds.len() * 96 + 256);
    let _ = write!(
        svg,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{:.3}mm\" height=\"{:.3}mm\" viewBox=\"0 0 {:.3} {:.3}\" overflow=\"visible\">",
        vb_w, vb_h, vb_w, vb_h
    );
    let music_defs = music_face
        .as_ref()
        .map(|face| collect_music_glyph_defs(face, cmds))
        .unwrap_or_default();
    if !music_defs.is_empty() {
        svg.push_str("<defs>");
        for (gid, path) in &music_defs {
            let _ = write!(svg, "<path id=\"b{:x}\" d=\"{}\"/>", gid, path);
        }
        svg.push_str("</defs>");
    }

    ox = 0.0;
    oy = 0.0;
    for cmd in cmds {
        match cmd {
            DrawCmd::Line { x1, y1, x2, y2, w } => {
                let _ = write!(
                    svg,
                    "<line x1=\"{:.3}\" y1=\"{:.3}\" x2=\"{:.3}\" y2=\"{:.3}\" stroke=\"black\" stroke-width=\"{:.3}\" stroke-linecap=\"butt\"/>",
                    tx(ox + x1), ty(oy + y1), tx(ox + x2), ty(oy + y2), w
                );
            }
            DrawCmd::Glyph { x, y, c, s, a } => {
                let rendered_as_path = music_face.as_ref().is_some_and(|face| {
                    write_music_glyph_use(
                        &mut svg,
                        face,
                        &music_defs,
                        ox + x,
                        oy + y,
                        *c,
                        *s,
                        a,
                        &tx,
                        &ty,
                    )
                });
                if !rendered_as_path {
                    if let Some(ch) = char::from_u32(*c) {
                        if let Some(bbox) = smufl_bbox_for_codepoint(*c, font_id) {
                            let (text_x, text_y) =
                                anchored_text_origin(ox + x, oy + y, bbox, *s, a);
                            let _ = write!(
                            svg,
                            "<text x=\"{:.3}\" y=\"{:.3}\" font-family=\"{}\" font-size=\"{:.3}\">{}</text>",
                            tx(text_x),
                            ty(text_y),
                            escaped_music_font,
                            s,
                            xml_escape(&ch.to_string())
                        );
                        } else {
                            let (text_anchor, baseline) = svg_anchor_attrs(a);
                            let _ = write!(
                            svg,
                            "<text x=\"{:.3}\" y=\"{:.3}\" font-family=\"{}\" font-size=\"{:.3}\" text-anchor=\"{}\" dominant-baseline=\"{}\">{}</text>",
                            tx(ox + x),
                            ty(oy + y),
                            escaped_music_font,
                            s,
                            text_anchor,
                            baseline,
                            xml_escape(&ch.to_string())
                        );
                        }
                    }
                }
            }
            DrawCmd::MusicText { x, y, v, s, a } => {
                let rendered_as_path = music_face.as_ref().is_some_and(|face| {
                    write_music_text_uses(
                        &mut svg,
                        face,
                        &music_defs,
                        ox + x,
                        oy + y,
                        v,
                        *s,
                        a,
                        &tx,
                        &ty,
                    )
                });
                if !rendered_as_path {
                    if let Some(bbox) = music_text_bbox(v, font_id) {
                        let (text_x, text_y) = anchored_text_origin(ox + x, oy + y, bbox, *s, a);
                        let _ = write!(
                        svg,
                        "<text x=\"{:.3}\" y=\"{:.3}\" font-family=\"{}\" font-size=\"{:.3}\">{}</text>",
                        tx(text_x),
                        ty(text_y),
                        escaped_music_font,
                        s,
                        xml_escape(v)
                    );
                    } else {
                        let (text_anchor, baseline) = svg_anchor_attrs(a);
                        let _ = write!(
                        svg,
                        "<text x=\"{:.3}\" y=\"{:.3}\" font-family=\"{}\" font-size=\"{:.3}\" text-anchor=\"{}\" dominant-baseline=\"{}\">{}</text>",
                        tx(ox + x),
                        ty(oy + y),
                        escaped_music_font,
                        s,
                        text_anchor,
                        baseline,
                        xml_escape(v)
                    );
                    }
                }
            }
            DrawCmd::Text {
                x,
                y,
                v,
                s,
                w,
                i,
                a,
            } => {
                let (text_anchor, baseline) = svg_anchor_attrs(a);
                let weight = if w.as_ref() == "bold" {
                    "bold"
                } else {
                    "normal"
                };
                let style = if *i { "italic" } else { "normal" };
                let size_mm = *s * 25.4 / 72.0;
                let _ = write!(
                    svg,
                    "<text x=\"{:.3}\" y=\"{:.3}\" font-size=\"{:.3}\" font-weight=\"{}\" font-style=\"{}\" text-anchor=\"{}\" dominant-baseline=\"{}\">{}</text>",
                    tx(ox + x), ty(oy + y), size_mm, weight, style, text_anchor, baseline, xml_escape(v)
                );
            }
            DrawCmd::Polygon { pts } => {
                svg.push_str("<path d=\"M");
                let mut i = 0;
                while i + 1 < pts.len() {
                    if i > 0 {
                        svg.push('L');
                    }
                    let _ = write!(svg, "{:.3} {:.3}", tx(ox + pts[i]), ty(oy + pts[i + 1]));
                    i += 2;
                }
                svg.push_str("Z\" fill=\"black\"/>");
            }
            DrawCmd::BezierFill { pts } => {
                if pts.len() >= 12 {
                    let _ = write!(
                        svg,
                        "<path d=\"M{:.3} {:.3} C{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} C{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}Z\" fill=\"black\"/>",
                        tx(ox + pts[0]), ty(oy + pts[1]),
                        tx(ox + pts[2]), ty(oy + pts[3]),
                        tx(ox + pts[4]), ty(oy + pts[5]),
                        tx(ox + pts[6]), ty(oy + pts[7]),
                        tx(ox + pts[8]), ty(oy + pts[9]),
                        tx(ox + pts[10]), ty(oy + pts[11]),
                        tx(ox + pts[0]), ty(oy + pts[1]),
                    );
                }
            }
            DrawCmd::Circle { x, y, r } => {
                let _ = write!(
                    svg,
                    "<circle cx=\"{:.3}\" cy=\"{:.3}\" r=\"{:.3}\" fill=\"black\"/>",
                    tx(ox + x),
                    ty(oy + y),
                    r
                );
            }
            DrawCmd::MoveOrigin { dx, dy } => {
                ox += dx;
                oy += dy;
            }
            DrawCmd::FlushContent => {}
        }
    }

    svg.push_str("</svg>");
    svg
}

// ─── Note stem x computation ──────────────────────────────────────────

fn note_stem_x(x: f64, duration: i32, stem_dir: &str, sp: f64, font: glyph::FontId) -> f64 {
    let smufl = notehead_smufl(duration);
    let nh_w = glyph::advance_width_for(font, smufl);
    let ed = glyph::engraving_defaults(font);
    let anchor_key = if stem_dir == "up" {
        "stemUpSE"
    } else {
        "stemDownNW"
    };
    let anch = glyph::anchor_for(font, smufl, anchor_key);
    let (att_x, _att_y) = if let Some(a) = anch {
        (a.x, a.y)
    } else if stem_dir == "up" {
        (nh_w, 0.168)
    } else {
        (0.0, -0.168)
    };
    let sx = x - nh_w / 2.0 * sp + att_x * sp;
    let half_thin = ed.stem_thickness / 2.0 * sp;
    sx + if stem_dir == "up" {
        -half_thin
    } else {
        half_thin
    }
}

fn augmentation_dot_radius(sp: f64) -> f64 {
    0.22 * sp
}

fn augmentation_dot_y(
    note_center_y: f64,
    staff_pos: i32,
    dot_radius: f64,
    sp: f64,
    font: glyph::FontId,
) -> f64 {
    if staff_pos % 2 != 0 {
        note_center_y + 0.12 * sp
    } else {
        let ed = glyph::engraving_defaults(font);
        note_center_y + dot_radius + ed.staff_line_thickness * sp / 2.0 + 0.04 * sp
    }
}

// ─── Chord notehead x offsets ──────────────────────────────────────────

fn chord_notehead_x_offsets(positions: &[i32], stem_dir: &str, nh_w: f64, lsp: f64) -> Vec<f64> {
    let n = positions.len();
    let mut offsets = vec![0.0; n];
    if n <= 1 {
        return offsets;
    }

    // Sort indices by staff position (stem-base first)
    let mut order: Vec<usize> = (0..n).collect();
    if stem_dir == "down" {
        order.sort_by(|&a, &b| positions[a].cmp(&positions[b]));
    } else {
        order.sort_by(|&a, &b| positions[b].cmp(&positions[a]));
    }

    // Reduce by ~0.1 sp so the displaced note sits flush against the stem
    // rather than leaving a small visual gap.
    let alt_offset = if stem_dir == "down" {
        -(nh_w - 0.075) * lsp
    } else {
        (nh_w - 0.075) * lsp
    };
    let mut side = 0;
    let mut prev_sp: Option<i32> = None;
    for &idx in &order {
        let current_sp = positions[idx];
        if let Some(prev) = prev_sp {
            if (current_sp - prev).abs() == 1 {
                side = 1 - side;
            } else {
                side = 0;
            }
        }
        if side == 1 {
            offsets[idx] = alt_offset;
        }
        prev_sp = Some(current_sp);
    }
    offsets
}

fn ranges_overlap(a0: f64, a1: f64, b0: f64, b1: f64, gap: f64) -> bool {
    a0 < b1 + gap && b0 < a1 + gap
}

fn chord_accidental_collision_left_edge(
    note_idx: usize,
    acc_x: f64,
    acc_y: f64,
    acc_smufl: &str,
    chord_x: f64,
    offsets: &[f64],
    chord_ys: &[f64],
    y_top: f64,
    notehead_smufl: &str,
    nh_w: f64,
    sp: f64,
    lsp: f64,
    font: glyph::FontId,
) -> Option<f64> {
    let Some(acc_bb) = glyph::bbox_for(font, acc_smufl) else {
        return None;
    };
    let note_bb = glyph::bbox_for(font, notehead_smufl);
    let acc_left = acc_x + acc_bb.sw_x * lsp;
    let acc_right = acc_x + acc_bb.ne_x * lsp;
    let acc_bottom = acc_y + acc_bb.sw_y * lsp;
    let acc_top = acc_y + acc_bb.ne_y * lsp;
    let gap = 0.06 * lsp;

    let mut target_left_edge: Option<f64> = None;
    for (other_idx, &offset) in offsets.iter().enumerate() {
        if other_idx == note_idx {
            continue;
        }

        let other_x = chord_x + offset;
        let other_y = y_top + chord_ys[other_idx] * sp;
        let origin_x = other_x - nh_w / 2.0 * lsp;
        let (head_left, head_right, head_bottom, head_top) = if let Some(bb) = note_bb {
            (
                origin_x + bb.sw_x * lsp,
                origin_x + bb.ne_x * lsp,
                other_y + bb.sw_y * lsp,
                other_y + bb.ne_y * lsp,
            )
        } else {
            (
                origin_x,
                origin_x + nh_w * lsp,
                other_y - 0.5 * lsp,
                other_y + 0.5 * lsp,
            )
        };

        if ranges_overlap(acc_left, acc_right, head_left, head_right, gap)
            && ranges_overlap(acc_bottom, acc_top, head_bottom, head_top, gap)
        {
            target_left_edge =
                Some(target_left_edge.map_or(head_left, |current| current.min(head_left)));
        }
    }

    target_left_edge
}

fn voice_stem_side_offset(stem_dir: &str, nh_w: f64, lsp: f64) -> f64 {
    if stem_dir == "down" {
        -(nh_w - 0.075) * lsp
    } else {
        (nh_w - 0.075) * lsp
    }
}

#[derive(Debug, Clone)]
struct NoteheadInfo {
    x: f64,
    staff_pos: i32,
    half_width: f64,
}

fn notehead_infos_for_item(
    item: &LaidOutItem,
    x: f64,
    stem_dir: &str,
    sp: f64,
    font: glyph::FontId,
) -> Vec<NoteheadInfo> {
    match &item.event {
        Event::Note(n) => {
            let note_scale = if n.grace { GRACE_NOTE_SCALE } else { 1.0 };
            let lsp = sp * note_scale;
            let nh_w = glyph::advance_width_for(font, notehead_smufl(n.duration)) * lsp;
            vec![NoteheadInfo {
                x,
                staff_pos: (-2.0 * item.y).round() as i32,
                half_width: nh_w / 2.0,
            }]
        }
        Event::Chord(c) => {
            let note_scale = if c.grace { GRACE_NOTE_SCALE } else { 1.0 };
            let lsp = sp * note_scale;
            let smufl = notehead_smufl(c.duration);
            let nh_w_sp = glyph::advance_width_for(font, smufl);
            let offsets =
                chord_notehead_x_offsets(&item.chord_staff_positions, stem_dir, nh_w_sp, lsp);
            let half_width = nh_w_sp * lsp / 2.0;
            item.chord_staff_positions
                .iter()
                .zip(offsets.iter())
                .map(|(&staff_pos, &offset)| NoteheadInfo {
                    x: x + offset,
                    staff_pos,
                    half_width,
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

fn stem_dir_for_item(
    item: &LaidOutItem,
    adj_stem_dirs: &std::collections::HashMap<usize, String>,
    idx: usize,
) -> String {
    adj_stem_dirs
        .get(&idx)
        .cloned()
        .or(item.stem_dir.clone())
        .unwrap_or_else(|| "up".to_string())
}

fn compute_cross_voice_notehead_offsets(
    items: &[LaidOutItem],
    item_xs: &[f64],
    sp: f64,
    font: glyph::FontId,
) -> Vec<f64> {
    let mut offsets = vec![0.0; items.len()];

    for (i, item) in items.iter().enumerate() {
        let Event::Note(n) = &item.event else {
            continue;
        };
        let Some(voice) = item.voice else {
            continue;
        };

        let staff_pos = (-2.0 * item.y).round() as i32;
        let mut collides = false;
        for (j, other) in items.iter().enumerate() {
            if i == j || other.voice.is_none() || other.voice == Some(voice) {
                continue;
            }
            if (other.x - item.x).abs() > 0.000001 {
                continue;
            }
            let other_stem_dir = other.stem_dir.as_deref().unwrap_or("up");
            let other_heads = notehead_infos_for_item(other, item_xs[j], other_stem_dir, sp, font);
            if other_heads
                .iter()
                .any(|head| (head.staff_pos - staff_pos).abs() <= 1)
            {
                collides = true;
                break;
            }
        }

        if collides {
            let note_scale = if n.grace { GRACE_NOTE_SCALE } else { 1.0 };
            let lsp = sp * note_scale;
            let nh_w = glyph::advance_width_for(font, notehead_smufl(n.duration));
            let stem_dir = item.stem_dir.as_deref().unwrap_or("up");
            offsets[i] = voice_stem_side_offset(stem_dir, nh_w, lsp);
        }
    }

    offsets
}

fn dot_x_base_avoiding_cross_voice_noteheads(
    items: &[LaidOutItem],
    item_xs: &[f64],
    idx: usize,
    staff_pos: i32,
    default_x: f64,
    dot_radius: f64,
    sp: f64,
    lsp: f64,
    adj_stem_dirs: &std::collections::HashMap<usize, String>,
    font: glyph::FontId,
) -> f64 {
    let Some(voice) = items[idx].voice else {
        return default_x;
    };

    let mut x = default_x;
    for (other_idx, other) in items.iter().enumerate() {
        if other_idx == idx || other.voice.is_none() || other.voice == Some(voice) {
            continue;
        }
        if (other.x - items[idx].x).abs() > 0.000001 {
            continue;
        }
        let other_stem_dir = stem_dir_for_item(other, adj_stem_dirs, other_idx);
        for head in notehead_infos_for_item(other, item_xs[other_idx], &other_stem_dir, sp, font) {
            if (head.staff_pos - staff_pos).abs() <= 1 {
                let candidate = head.x + head.half_width + dot_radius + 0.18 * lsp;
                if candidate > x {
                    x = candidate;
                }
            }
        }
    }
    x
}

// ─── Beam helpers ──────────────────────────────────────────────────────

fn resolve_chord_dot_collisions(
    dot_xs: &mut [f64],
    min_dot_xs: &[f64],
    dot_ys: &[f64],
    dot_radius: f64,
    lsp: f64,
) {
    if dot_xs.len() < 2 {
        return;
    }

    let min_dist = 2.0 * dot_radius + 0.22 * lsp;
    for _ in 0..3 {
        let mut changed = false;
        for a in 0..dot_xs.len() {
            for b in (a + 1)..dot_xs.len() {
                let dx = (dot_xs[a] - dot_xs[b]).abs();
                let dy = (dot_ys[a] - dot_ys[b]).abs();
                if dx * dx + dy * dy >= min_dist * min_dist {
                    continue;
                }

                let left = if dot_xs[a] <= dot_xs[b] { a } else { b };
                let right = if left == a { b } else { a };
                let needed_dx = (min_dist * min_dist - dy * dy).max(0.0).sqrt() + 0.02 * lsp;
                let target = dot_xs[right] - needed_dx;
                let adjusted = target.max(min_dot_xs[left]);
                if adjusted < dot_xs[left] {
                    dot_xs[left] = adjusted;
                    changed = true;
                }

                let dx_after = (dot_xs[left] - dot_xs[right]).abs();
                if dx_after * dx_after + dy * dy < min_dist * min_dist {
                    let adjusted_other = (dot_xs[left] + needed_dx).max(min_dot_xs[right]);
                    if adjusted_other > dot_xs[right] {
                        dot_xs[right] = adjusted_other;
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
}

fn beam_count(duration: i32) -> i32 {
    match duration {
        d if d >= 64 => 4,
        d if d >= 32 => 3,
        d if d >= 16 => 2,
        d if d >= 8 => 1,
        _ => 0,
    }
}

fn grace_slash_beam_extension(duration: i32) -> f64 {
    (beam_count(duration).saturating_sub(1) as f64) * 0.95
}

fn min_dur_for_level(level: i32) -> i32 {
    match level {
        1 => 8,
        2 => 16,
        3 => 32,
        _ => 64,
    }
}

// ─── Beam data structures ──────────────────────────────────────────────

struct BeamNote {
    stem_x: f64,
    beam_y: f64,
    duration: i32,
    stem_dir: String,
}

struct BeamGroupData {
    notes: Vec<BeamNote>,
    scale: f64,
}

// ─── Main rendering functions ──────────────────────────────────────────

/// Returns how far below `y_bottom` the given events' below-staff elements extend, in sp units.
/// Used to auto-expand inter-staff spacing within a system so low elements don't intrude into
/// the next staff's area.
fn compute_below_extent_sp(items: &[LaidOutItem]) -> f64 {
    let mut max_sp = 0.0_f64;
    for item in items {
        let ev = &item.event;
        let has_dynamic = ev.dynamic_mark().map_or(false, |d| !d.is_empty());
        let has_expression = ev.expression_text().map_or(false, |e| !e.is_empty());
        let has_hairpin = ev.hairpin().is_some();
        let lyric_count = ev.lyrics().iter().filter(|l| l.text.is_some()).count();
        // Dynamic glyphs normally sit at y_bottom-1 sp, but may be nudged lower
        // to clear low notes, below articulations, or below-staff slurs.
        // Expression text at y_bottom-3.5 sp (with dynamic) or y_bottom-2.0 sp (alone), ~1.5 sp tall.
        if has_dynamic && has_expression {
            max_sp = max_sp.max(7.0); // lowered dynamic + expression below that
        } else if has_dynamic {
            max_sp = max_sp.max(4.6);
        } else if has_expression {
            max_sp = max_sp.max(3.5);
        }
        if has_hairpin {
            max_sp = max_sp.max(3.0);
        }
        if lyric_count > 0 {
            max_sp = max_sp.max(3.1 + lyric_count as f64 * 1.75 + 2.0);
        }
    }
    max_sp
}

/// Returns how far above `y_top` the given events' above-staff elements extend, in sp units.
fn compute_above_extent_sp(items: &[LaidOutItem]) -> f64 {
    let mut max_sp = 0.0_f64;
    for item in items {
        let ev = &item.event;
        let has_chord = ev.chord_symbol().map_or(false, |c| !c.is_empty());
        let has_staff_text = ev.staff_text().map_or(false, |t| !t.is_empty());
        let has_ending = ev.ending().is_some();
        let has_fingering = ev.fingering().is_some();
        if has_ending {
            if has_chord || has_staff_text {
                max_sp = max_sp.max(9.0); // chord/text above bracket line + bracket height
            } else {
                max_sp = max_sp.max(5.0); // just the bracket
            }
        } else if has_staff_text || has_chord {
            max_sp = max_sp.max(5.5);
        } else if has_fingering {
            max_sp = max_sp.max(3.0);
        }
    }
    max_sp
}

pub fn render_system_group(
    laid_out_staves: &[LaidOutStaff],
    key: &str,
    time: &Option<TimeInfo>,
    sp_unit: f64,
    avail_width_mm: Option<f64>,
    staff_spacing_mm: f64,
    staff_group: &str,
    title: Option<&str>,
    subtitle: Option<&str>,
    composer: Option<&str>,
    arranger: Option<&str>,
    lyricist: Option<&str>,
    show_time: bool,
    fingering_positions: &[&str],
    music_font: &str,
) -> SystemOutput {
    let font = glyph::FontId::from_name(music_font);
    let ed = glyph::engraving_defaults(font);
    // Estimate ~20 draw commands per event (lines, glyphs, text, etc.)
    let estimated_cmds = laid_out_staves.iter().map(|s| s.items.len()).sum::<usize>() * 20 + 100;
    let mut cmds = Vec::with_capacity(estimated_cmds);
    let num_staves = laid_out_staves.len();
    let staff_height_mm = 4.0 * sp_unit;
    let use_spanning_barlines = num_staves > 1;

    // Compute shared prefix data
    let (shared_time_x, shared_music_start_x) =
        compute_shared_prefix(laid_out_staves, key, time, sp_unit, show_time);

    // Render header text
    let mut header_height = 0.0;
    if let Some(t) = title {
        if !t.is_empty() {
            let w = avail_width_mm.unwrap_or(200.0);
            cmds.push(DrawCmd::Text {
                x: w / 2.0,
                y: header_height + 5.0,
                v: t.to_string(),
                s: 16.0,
                w: "bold".into(),
                i: false,
                a: "south".into(),
            });
            header_height += 7.0;
        }
    }
    if let Some(s) = subtitle {
        if !s.is_empty() {
            let w = avail_width_mm.unwrap_or(200.0);
            cmds.push(DrawCmd::Text {
                x: w / 2.0,
                y: header_height + 3.0,
                v: s.to_string(),
                s: 11.0,
                w: "regular".into(),
                i: false,
                a: "south".into(),
            });
            header_height += 5.0;
        }
    }
    if let Some(c) = composer {
        if !c.is_empty() {
            let w = avail_width_mm.unwrap_or(200.0);
            cmds.push(DrawCmd::Text {
                x: w,
                y: header_height + 3.0,
                v: c.to_string(),
                s: 10.0,
                w: "regular".into(),
                i: true,
                a: "south-east".into(),
            });
            header_height = header_height.max(5.0);
        }
    }
    // arranger and lyricist similar to composer
    if let Some(a) = arranger {
        if !a.is_empty() {
            let w = avail_width_mm.unwrap_or(200.0);
            cmds.push(DrawCmd::Text {
                x: w,
                y: header_height + 3.0,
                v: format!("Arr. {}", a),
                s: 9.0,
                w: "regular".into(),
                i: true,
                a: "south-east".into(),
            });
        }
    }
    if let Some(l) = lyricist {
        if !l.is_empty() {
            cmds.push(DrawCmd::Text {
                x: 0.0,
                y: header_height + 3.0,
                v: format!("Lyrics: {}", l),
                s: 9.0,
                w: "regular".into(),
                i: true,
                a: "south-west".into(),
            });
        }
    }

    if header_height > 0.0 {
        header_height += 3.0;
    }

    let mut total_height = header_height;
    let mut y_offset = -header_height; // Renderer y goes up, but staff draws downward
    let mut staff_y_tops = Vec::with_capacity(num_staves);

    for (si, laid_out) in laid_out_staves.iter().enumerate() {
        if si > 0 {
            // Expand the gap if below-staff content of the upper staff or above-staff content
            // of the lower staff needs more room than the configured default spacing.
            let below_sp = compute_below_extent_sp(&laid_out_staves[si - 1].items);
            let above_sp = compute_above_extent_sp(&laid_out.items);
            let required_mm = (below_sp + above_sp + 0.5) * sp_unit;
            let spacing = staff_spacing_mm.max(required_mm);
            y_offset -= spacing;
            total_height += spacing;
        }

        let y_top = y_offset;
        staff_y_tops.push(y_top);
        let fng_pos = if si < fingering_positions.len() {
            fingering_positions[si]
        } else {
            "above"
        };

        render_system(
            &mut cmds,
            laid_out,
            key,
            time,
            sp_unit,
            avail_width_mm,
            show_time && si == 0,
            Some(shared_time_x),
            Some(shared_music_start_x),
            use_spanning_barlines, // all staves skip barlines when spanning
            fng_pos,
            y_top,
            font,
        );

        y_offset -= staff_height_mm;
        total_height += staff_height_mm;
    }

    // Draw multi-staff elements
    if num_staves > 1 {
        let first_y_top = -header_height;
        let last_y_bottom = y_offset + staff_height_mm - 4.0 * sp_unit;

        // Brace or bracket
        if staff_group == "grand" {
            let brace_cp = 0xE000u32;
            let span = first_y_top - last_y_bottom;
            if span > 0.0 {
                let nominal_h = 4.0 * sp_unit;
                let scale = span / nominal_h;
                let fsize = 4.0 * sp_unit * scale;
                let brace_w = glyph::advance_width_for(font, "brace") * sp_unit * scale;
                cmds.push(DrawCmd::Glyph {
                    x: -brace_w - 0.3 * sp_unit,
                    y: last_y_bottom,
                    c: brace_cp,
                    s: fsize,
                    a: "south-west".into(),
                });
            }
        } else if staff_group == "bracket" {
            let thick = 0.3 * sp_unit;
            let serif = 0.6 * sp_unit;
            let bx = -0.5 * sp_unit;
            emit_line(&mut cmds, bx, first_y_top, bx, last_y_bottom, thick);
            emit_line(&mut cmds, bx, first_y_top, bx + serif, first_y_top, thick);
            emit_line(
                &mut cmds,
                bx,
                last_y_bottom,
                bx + serif,
                last_y_bottom,
                thick,
            );
        }

        // Spanning barlines — opening, all internal measure barlines, and final,
        // all spanning the full height from the first staff top to the last staff bottom.
        {
            let staff0 = &laid_out_staves[0];
            let s0_total_w = staff0.total_width;
            let avail_music_w = if let Some(w) = avail_width_mm {
                w / sp_unit - shared_music_start_x / sp_unit - 1.0
            } else {
                s0_total_w + 2.0
            };
            let scale_x = if s0_total_w > 0.0 {
                avail_music_w / s0_total_w
            } else {
                1.0
            };
            let total_w = compute_total_width(
                laid_out_staves,
                sp_unit,
                avail_width_mm,
                shared_music_start_x,
            );

            // Opening barline (left edge)
            emit_line(
                &mut cmds,
                ed.thin_barline_thickness / 2.0 * sp_unit,
                first_y_top,
                ed.thin_barline_thickness / 2.0 * sp_unit,
                last_y_bottom,
                ed.thin_barline_thickness * sp_unit,
            );

            // Internal measure barlines and final barline.
            // Key rule: if the last item in staff0 IS a barline, that barline is the
            // system-closing barline (skip it in the internal loop, render it as final).
            // If the last item is NOT a barline (music ends mid-measure), render ALL
            // barlines as internal ones and append a synthetic final barline.
            let items = &staff0.items;
            let last_item_is_barline = items.last().map_or(false, |i| i.event.is_barline());
            let last_barline_idx: Option<usize> = if last_item_is_barline {
                items.iter().rposition(|it| it.event.is_barline())
            } else {
                None // all barlines are internal; final will be synthesised
            };
            for (idx, item) in items.iter().enumerate() {
                if let Event::Barline(b) = &item.event {
                    if Some(idx) == last_barline_idx {
                        continue;
                    } // handled below as final
                    let bx = shared_music_start_x + item.x * scale_x * sp_unit + 0.5 * sp_unit;
                    render_spanning_barline(
                        &mut cmds,
                        bx,
                        first_y_top,
                        last_y_bottom,
                        &staff_y_tops,
                        &b.style,
                        sp_unit,
                        font,
                    );
                }
            }

            // Final barline
            let raw_final_style = if last_item_is_barline {
                items
                    .last()
                    .and_then(|item| {
                        if let Event::Barline(b) = &item.event {
                            Some(b.style.as_str())
                        } else {
                            None
                        }
                    })
                    .unwrap_or("final")
            } else {
                "final" // music ends with a note — emit standard final barline
            };
            let final_style = if raw_final_style == "repeat-both" {
                "repeat-end"
            } else {
                raw_final_style
            };
            let final_x = if matches!(final_style, "final" | "repeat-end") {
                total_w * sp_unit - ed.thick_barline_thickness / 2.0 * sp_unit
            } else {
                total_w * sp_unit - ed.thin_barline_thickness / 2.0 * sp_unit
            };
            render_spanning_barline(
                &mut cmds,
                final_x,
                first_y_top,
                last_y_bottom,
                &staff_y_tops,
                final_style,
                sp_unit,
                font,
            );
        }
    }

    // Compute final dimensions
    let total_w_sp = compute_total_width(
        laid_out_staves,
        sp_unit,
        avail_width_mm,
        shared_music_start_x,
    );
    let width_mm = avail_width_mm.unwrap_or(total_w_sp * sp_unit);

    // Add below-staff content depth
    total_height += 1.75 * sp_unit; // baseline below depth

    let svg = svg_from_cmds(&cmds, width_mm, total_height, music_font);

    SystemOutput {
        width: width_mm,
        height: total_height,
        svg,
        cmds: Vec::new(),
    }
}

fn compute_shared_prefix(
    laid_out_staves: &[LaidOutStaff],
    key: &str,
    time: &Option<TimeInfo>,
    sp: f64,
    show_time: bool,
) -> (f64, f64) {
    let mut max_time_x: f64 = 0.0;
    let mut max_music_start: f64 = 0.0;

    for laid_out in laid_out_staves {
        let clef_name = laid_out.clef.as_deref();
        let clef_w = if let Some(c) = clef_name {
            layout::clef_advance_sp(c, sp)
        } else {
            0.0
        };
        let key_w = layout::key_sig_advance_sp(key, sp);
        let lt = laid_out.time.as_ref().or(time.as_ref());
        let show = laid_out.show_time_prefix || show_time;
        let time_w = if show {
            if let Some(t) = lt {
                layout::time_sig_advance_sp(t.upper, t.lower, t.symbol.as_deref(), sp)
            } else {
                0.0
            }
        } else {
            0.0
        };
        let prefix_x = 0.5 * sp;
        let local_time_x = prefix_x + clef_w + key_w;
        if show {
            max_time_x = max_time_x.max(local_time_x);
        }
        let local_music_start = prefix_x + clef_w + key_w + time_w + MUSIC_START_PADDING * sp;
        max_music_start = max_music_start.max(local_music_start);
    }

    (max_time_x, max_music_start)
}

fn compute_total_width(
    laid_out_staves: &[LaidOutStaff],
    sp: f64,
    avail_width_mm: Option<f64>,
    music_start_x: f64,
) -> f64 {
    if let Some(w) = avail_width_mm {
        return w / sp;
    }
    let max_tw = laid_out_staves
        .iter()
        .map(|s| s.total_width)
        .fold(0.0_f64, f64::max);
    music_start_x / sp + max_tw + 1.0
}

// ─── Single staff rendering ───────────────────────────────────────────

fn center_whole_measure_rests(
    items: &[LaidOutItem],
    item_xs: &mut [f64],
    music_start_x: f64,
    system_right_x: f64,
    sp: f64,
    font: glyph::FontId,
) {
    for i in 0..items.len() {
        let is_whole_rest = matches!(&items[i].event, Event::Rest(r) if r.duration == 1);
        if !is_whole_rest {
            continue;
        }

        let prev_barline = (0..i).rev().find(|&idx| items[idx].event.is_barline());
        let next_barline = (i + 1..items.len()).find(|&idx| items[idx].event.is_barline());
        let measure_start = prev_barline.map_or(music_start_x, |idx| item_xs[idx] + 0.5 * sp);
        let measure_end =
            next_barline.map_or(system_right_x - 0.5 * sp, |idx| item_xs[idx] + 0.5 * sp);

        if measure_end <= measure_start {
            continue;
        }

        let content_start = prev_barline.map_or(0, |idx| idx + 1);
        let content_end = next_barline.unwrap_or(items.len());
        let content_count = items[content_start..content_end]
            .iter()
            .filter(|item| {
                item.event.is_note()
                    || item.event.is_chord()
                    || item.event.is_rest()
                    || matches!(item.event, Event::Spacer(_))
            })
            .count();
        if content_count != 1 {
            continue;
        }

        let rest_center_offset = glyph::bbox_for(font, "restWhole")
            .map(|b| (b.sw_x + b.ne_x) * 0.5 * sp)
            .unwrap_or(0.5 * sp);
        item_xs[i] = (measure_start + measure_end) * 0.5 - rest_center_offset;
    }
}

fn expand_voice_group_items(items: &[LaidOutItem]) -> Vec<LaidOutItem> {
    fn push_expanded(out: &mut Vec<LaidOutItem>, item: &LaidOutItem, base_x: f64) {
        if item.voice_items.is_empty() {
            let mut cloned = item.clone();
            cloned.x += base_x;
            out.push(cloned);
        } else {
            for child in &item.voice_items {
                push_expanded(out, child, base_x + item.x);
            }
        }
    }

    let mut expanded = Vec::new();
    for item in items {
        push_expanded(&mut expanded, item, 0.0);
    }
    expanded
}

fn render_system(
    cmds: &mut Vec<DrawCmd>,
    laid_out: &LaidOutStaff,
    key: &str,
    time: &Option<TimeInfo>,
    sp: f64,
    avail_width_mm: Option<f64>,
    show_time: bool,
    forced_time_x: Option<f64>,
    forced_music_start_x: Option<f64>,
    skip_barlines: bool,
    fng_pos: &str,
    y_top_offset: f64,
    font: glyph::FontId,
) {
    let clef_name = laid_out.clef.as_deref();
    let opening_time = laid_out.time.as_ref().or(time.as_ref());
    let show_opening_time = laid_out.show_time_prefix || show_time;
    let expanded_items = expand_voice_group_items(&laid_out.items);
    let items = &expanded_items;
    let total_layout_width = laid_out.total_width;

    let y_top = y_top_offset;
    let y_bottom = y_top - 4.0 * sp;

    // Compute prefix
    let mut cx = 0.5 * sp;
    let ed = glyph::engraving_defaults(font);
    let mut clef_w = 0.0;
    if let Some(c) = clef_name {
        clef_w = layout::clef_advance_sp_font(c, sp, font);
    }
    let key_w = layout::key_sig_advance_sp_font(key, sp, font);
    let time_w = if show_opening_time {
        if let Some(t) = opening_time {
            layout::time_sig_advance_sp_font(t.upper, t.lower, t.symbol.as_deref(), sp, font)
        } else {
            0.0
        }
    } else {
        0.0
    };

    let music_start_x = forced_music_start_x.unwrap_or_else(|| {
        let mut msx = cx + clef_w + key_w + time_w + MUSIC_START_PADDING * sp;
        // Extra space for first accidental
        let first_has_acc = items
            .iter()
            .find(|i| i.event.is_note() || i.event.is_chord())
            .map_or(false, |i| match &i.event {
                Event::Note(n) => n.accidental.is_some(),
                Event::Chord(c) => c.notes.iter().any(|n| n.accidental.is_some()),
                _ => false,
            });
        if first_has_acc {
            msx += 1.0 * sp;
        }
        msx
    });

    let available_music_width = if let Some(w) = avail_width_mm {
        w / sp - music_start_x / sp - 1.0
    } else {
        total_layout_width + 2.0
    };

    let scale_x = if total_layout_width > 0.0 {
        available_music_width / total_layout_width
    } else {
        1.0
    };

    let total_width = if avail_width_mm.is_some() {
        avail_width_mm.unwrap() / sp
    } else {
        music_start_x / sp + total_layout_width * scale_x + 1.0
    };

    // Draw staff lines
    for i in 0..5 {
        let y = y_top - i as f64 * sp;
        emit_line(
            cmds,
            0.0,
            y,
            total_width * sp,
            y,
            ed.staff_line_thickness * sp,
        );
    }

    // Opening barline — skipped when the group renderer draws a spanning barline
    if !skip_barlines {
        emit_line(
            cmds,
            ed.thin_barline_thickness / 2.0 * sp,
            y_top,
            ed.thin_barline_thickness / 2.0 * sp,
            y_bottom,
            ed.thin_barline_thickness * sp,
        );
    }

    // Draw clef
    cx = 0.5 * sp;
    if let Some(c) = clef_name {
        let origin_offset = clef_origin_offset(c);
        let origin_y = y_top - origin_offset * sp;
        emit_glyph(
            cmds,
            cx,
            origin_y,
            clef_smufl(c),
            clef_codepoint(c),
            sp,
            font,
        );
        cx += clef_w;
    }

    // Draw key signature
    render_key_signature(cmds, cx, y_top, key, clef_name, sp, font);
    cx += key_w;

    // Draw time signature
    if show_opening_time {
        let time_x = forced_time_x.unwrap_or(cx);
        if let Some(t) = opening_time {
            render_time_signature(
                cmds,
                time_x,
                y_top,
                t.upper,
                t.lower,
                t.symbol.as_deref(),
                sp,
                font,
            );
        }
    }

    // Pre-compute item x positions
    let mut item_xs: Vec<f64> = items
        .iter()
        .map(|item| music_start_x + item.x * scale_x * sp)
        .collect();
    center_whole_measure_rests(
        items,
        &mut item_xs,
        music_start_x,
        total_width * sp,
        sp,
        font,
    );
    let cross_voice_offsets = compute_cross_voice_notehead_offsets(items, &item_xs, sp, font);
    for (x, offset) in item_xs.iter_mut().zip(cross_voice_offsets.iter()) {
        *x += offset;
    }

    // Compute notehead bbox
    let black_bb = glyph::bbox_for(font, "noteheadBlack");
    let black_top = black_bb.map_or(0.82, |b| b.ne_y);
    let black_bottom = black_bb.map_or(-0.82, |b| b.sw_y);

    // ── Auto-beaming ──
    let mut raw_beam_groups: Vec<Vec<usize>> = Vec::with_capacity(16);
    let mut cur_beam: Vec<usize> = Vec::with_capacity(8);
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        let beamable = (ev.is_note() || ev.is_chord()) && ev.duration() >= 8;
        let grace = ev.grace();
        if beamable {
            let same_grace =
                cur_beam.is_empty() || items[*cur_beam.first().unwrap()].event.grace() == grace;
            let same_voice =
                cur_beam.is_empty() || items[*cur_beam.first().unwrap()].voice == item.voice;
            if !same_grace || !same_voice {
                if cur_beam.len() >= 2 {
                    raw_beam_groups.push(cur_beam.clone());
                }
                cur_beam.clear();
            }
            let limit = if grace { 8 } else { 4 };
            if cur_beam.len() == limit {
                raw_beam_groups.push(cur_beam.clone());
                cur_beam.clear();
            }
            cur_beam.push(i);
        } else {
            if cur_beam.len() >= 2 {
                raw_beam_groups.push(cur_beam.clone());
            }
            cur_beam.clear();
        }
    }
    if cur_beam.len() >= 2 {
        raw_beam_groups.push(cur_beam);
    }

    // Compute beam geometry
    let mut adj_stem_ends: std::collections::HashMap<usize, f64> = std::collections::HashMap::new();
    let mut adj_stem_dirs: std::collections::HashMap<usize, String> =
        std::collections::HashMap::new();

    let mut beam_groups_data: Vec<BeamGroupData> = Vec::with_capacity(raw_beam_groups.len());

    for group in &raw_beam_groups {
        let group_is_grace = items[*group.first().unwrap()].event.grace();
        let beam_scale = if group_is_grace {
            GRACE_NOTE_SCALE
        } else {
            1.0
        };
        let stem_min_length = if group_is_grace {
            GRACE_STEM_MIN_LENGTH
        } else {
            3.5
        };

        // Unified stem direction
        let avg_y = group.iter().map(|&idx| items[idx].y).sum::<f64>() / group.len() as f64;
        let avg_staff_pos = -2.0 * avg_y;
        let forced_stem_dir = group.iter().find_map(|&idx| {
            items[idx]
                .stem_forced
                .then(|| items[idx].stem_dir.as_deref())
                .flatten()
        });
        let stem_dir = if let Some(dir) = forced_stem_dir {
            dir
        } else if group_is_grace {
            "up"
        } else if avg_staff_pos > 4.0 {
            "up"
        } else {
            "down"
        };

        let first = &items[*group.first().unwrap()];
        let last = &items[*group.last().unwrap()];
        let mut sy0 = pitch::compute_stem_end_y(
            first.y,
            (-2.0 * first.y).round() as i32,
            stem_dir,
            beam_scale,
            stem_min_length,
        );
        let mut syn = pitch::compute_stem_end_y(
            last.y,
            (-2.0 * last.y).round() as i32,
            stem_dir,
            beam_scale,
            stem_min_length,
        );

        let x0 = item_xs[*group.first().unwrap()];
        let xn = item_xs[*group.last().unwrap()];
        let ed = glyph::engraving_defaults(font);
        let beam_step_staff = (ed.beam_thickness + ed.beam_spacing) * beam_scale;
        let min_clearance = 0.25 * beam_scale;
        let mut required_shift: f64 = 0.0;

        for &idx in group {
            let item = &items[idx];
            let xi = item_xs[idx];
            let t = if xn != x0 { (xi - x0) / (xn - x0) } else { 0.0 };
            let by_staff = sy0 + t * (syn - sy0);
            let beam_levels = beam_count(item.event.duration());
            let nearest_edge = if stem_dir == "up" {
                by_staff
                    - (beam_levels - 1) as f64 * beam_step_staff
                    - ed.beam_thickness * beam_scale
            } else {
                by_staff
                    + (beam_levels - 1) as f64 * beam_step_staff
                    + ed.beam_thickness * beam_scale
            };
            let note_edge = if stem_dir == "up" {
                item.y + black_top * beam_scale
            } else {
                item.y + black_bottom * beam_scale
            };
            let actual_clearance = if stem_dir == "up" {
                nearest_edge - note_edge
            } else {
                note_edge - nearest_edge
            };
            if actual_clearance < min_clearance {
                let original_height = (by_staff - item.y).abs();
                let proportional_lift = 0.25 * original_height;
                let needed = (min_clearance - actual_clearance).max(proportional_lift);
                required_shift = required_shift.max(needed);
            }
        }

        if required_shift > 0.0 {
            let outward = if stem_dir == "up" {
                required_shift
            } else {
                -required_shift
            };
            sy0 += outward;
            syn += outward;
        }

        let mut beam_note_data = Vec::with_capacity(group.len());
        for &idx in group {
            let item = &items[idx];
            let xi = item_xs[idx];
            let t = if xn != x0 { (xi - x0) / (xn - x0) } else { 0.0 };
            let by_staff = sy0 + t * (syn - sy0);
            let sx = note_stem_x(xi, item.event.duration(), stem_dir, sp * beam_scale, font);
            beam_note_data.push(BeamNote {
                stem_x: sx,
                beam_y: y_top + by_staff * sp,
                duration: item.event.duration(),
                stem_dir: stem_dir.to_string(),
            });
            adj_stem_ends.insert(idx, by_staff);
            adj_stem_dirs.insert(idx, stem_dir.to_string());
        }
        beam_groups_data.push(BeamGroupData {
            notes: beam_note_data,
            scale: beam_scale,
        });
    }

    // ── Render note/chord/rest events (first pass: noteheads, rests, accidentals) ──
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        let x = item_xs[i];
        let y = item.y * sp;

        match ev {
            Event::Clef(c) => {
                let prev = if i > 0 {
                    Some(&items[i - 1].event)
                } else {
                    None
                };
                let next = items.get(i + 1).map(|i| &i.event);
                let offset = inline_clef_draw_offset(prev, next, sp);
                let clef_x = x - offset;
                let origin_y = y_top - clef_origin_offset(&c.clef) * sp;
                emit_glyph_scaled(
                    cmds,
                    clef_x,
                    origin_y,
                    clef_smufl(&c.clef),
                    clef_codepoint(&c.clef),
                    sp * INLINE_CLEF_SCALE,
                    font,
                );
            }
            Event::TimeSig(t) => {
                render_time_signature(
                    cmds,
                    x,
                    y_top,
                    t.upper,
                    t.lower,
                    t.symbol.as_deref(),
                    sp,
                    font,
                );
            }
            Event::Note(n) => {
                let note_scale = if n.grace { GRACE_NOTE_SCALE } else { 1.0 };
                let note_center_y = y_top + y;
                let staff_pos = (-2.0 * item.y).round() as i32;
                let lsp = sp * note_scale;
                let smufl = notehead_smufl(n.duration);
                let cp = notehead_codepoint(n.duration);
                let nh_w = glyph::advance_width_for(font, smufl);

                // Ledger lines
                render_ledger_lines(cmds, x, y_top, staff_pos, sp, note_scale, font);

                // Accidental
                if let Some(ref acc) = n.accidental {
                    if let (Some(acc_cp), Some(acc_sm)) =
                        (accidental_codepoint(acc), accidental_smufl(acc))
                    {
                        let acc_w = glyph::advance_width_for(font, acc_sm);
                        let acc_x = x - nh_w / 2.0 * lsp - ACCIDENTAL_PADDING * lsp - acc_w * lsp;
                        emit_glyph(cmds, acc_x, note_center_y, acc_sm, acc_cp, lsp, font);
                    }
                }

                // Notehead
                emit_glyph(
                    cmds,
                    x - nh_w / 2.0 * lsp,
                    note_center_y,
                    smufl,
                    cp,
                    lsp,
                    font,
                );
            }
            Event::Chord(c) => {
                let note_scale = if c.grace { GRACE_NOTE_SCALE } else { 1.0 };
                let lsp = sp * note_scale;
                let smufl = notehead_smufl(c.duration);
                let cp = notehead_codepoint(c.duration);
                let nh_w = glyph::advance_width_for(font, smufl);
                let stem_dir = adj_stem_dirs
                    .get(&i)
                    .cloned()
                    .or(item.stem_dir.clone())
                    .unwrap_or_else(|| "up".to_string());
                let offsets =
                    chord_notehead_x_offsets(&item.chord_staff_positions, &stem_dir, nh_w, lsp);

                for (ni, cn) in c.notes.iter().enumerate() {
                    let ny = y_top + item.chord_ys[ni] * sp;
                    let nsp = item.chord_staff_positions[ni];
                    let nx = x + offsets[ni];
                    render_ledger_lines(cmds, nx, y_top, nsp, sp, note_scale, font);
                    if let Some(ref acc) = cn.accidental {
                        if let (Some(acc_cp), Some(acc_sm)) =
                            (accidental_codepoint(acc), accidental_smufl(acc))
                        {
                            let acc_w = glyph::advance_width_for(font, acc_sm);
                            let note_left_edge = nx - nh_w / 2.0 * lsp;
                            let normal_acc_x =
                                note_left_edge - ACCIDENTAL_PADDING * lsp - acc_w * lsp;
                            let target_left_edge = chord_accidental_collision_left_edge(
                                ni,
                                normal_acc_x,
                                ny,
                                acc_sm,
                                x,
                                &offsets,
                                &item.chord_ys,
                                y_top,
                                smufl,
                                nh_w,
                                sp,
                                lsp,
                                font,
                            )
                            .unwrap_or(note_left_edge);
                            let acc_x = target_left_edge - ACCIDENTAL_PADDING * lsp - acc_w * lsp;
                            emit_glyph(cmds, acc_x, ny, acc_sm, acc_cp, lsp, font);
                        }
                    }
                    emit_glyph(cmds, nx - nh_w / 2.0 * lsp, ny, smufl, cp, lsp, font);
                }
            }
            Event::Rest(r) => {
                let note_scale = if r.grace { GRACE_NOTE_SCALE } else { 1.0 };
                let lsp = sp * note_scale;
                let rst_smufl = rest_smufl(r.duration);
                let rst_cp = rest_codepoint(r.duration);
                emit_glyph(cmds, x, y_top + y, rst_smufl, rst_cp, lsp, font);
                // Rest dots
                if r.dots > 0 {
                    let bb = glyph::bbox_for(font, rst_smufl);
                    let rest_right = bb.map_or(0.8 * lsp, |b| b.ne_x * lsp);
                    let dot_x_base = x + rest_right + 0.3 * lsp;
                    for d in 0..r.dots {
                        cmds.push(DrawCmd::Circle {
                            x: dot_x_base + d as f64 * 0.4 * lsp,
                            y: y_top + y + 0.15 * lsp,
                            r: 0.12 * lsp,
                        });
                    }
                }
            }
            Event::Barline(b) => {
                if !skip_barlines && i < items.len() - 1 {
                    render_barline(cmds, x + 0.5 * sp, y_top, y_bottom, &b.style, sp, font);
                }
            }
            _ => {}
        }
    }

    cmds.push(DrawCmd::FlushContent);

    // ── Second pass: stems, flags, dots, articulations, dynamics ──
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        let x = item_xs[i];
        let y = item.y * sp;

        match ev {
            Event::Note(n) => {
                let is_grace = n.grace;
                let note_scale = if is_grace { GRACE_NOTE_SCALE } else { 1.0 };
                let note_center_y = y_top + y;
                let lsp = sp * note_scale;
                let stem_dir = adj_stem_dirs
                    .get(&i)
                    .cloned()
                    .or(item.stem_dir.clone())
                    .unwrap_or_else(|| "up".to_string());
                let stem_end = adj_stem_ends
                    .get(&i)
                    .copied()
                    .map(|se| y_top + se * sp)
                    .or(item.stem_y_end.map(|se| y_top + se * sp));
                let is_beamed = adj_stem_ends.contains_key(&i);

                // Stem
                if n.duration >= 2 {
                    if let Some(stem_end_y) = stem_end {
                        let smufl_n = notehead_smufl(n.duration);
                        let nh_w = glyph::advance_width_for(font, smufl_n);
                        let anchor_key = if stem_dir == "up" {
                            "stemUpSE"
                        } else {
                            "stemDownNW"
                        };
                        let anch = glyph::anchor_for(font, smufl_n, anchor_key);
                        let (att_x, att_y) = if let Some(a) = anch {
                            (a.x, a.y)
                        } else if stem_dir == "up" {
                            (nh_w, 0.168)
                        } else {
                            (0.0, -0.168)
                        };
                        let stem_x = x - nh_w / 2.0 * lsp + att_x * lsp;
                        let ed = glyph::engraving_defaults(font);
                        let half_thin = ed.stem_thickness / 2.0 * lsp;
                        let stem_x = stem_x
                            + if stem_dir == "up" {
                                -half_thin
                            } else {
                                half_thin
                            };
                        let stem_start_y = note_center_y + att_y * lsp;
                        emit_line(
                            cmds,
                            stem_x,
                            stem_start_y,
                            stem_x,
                            stem_end_y,
                            ed.stem_thickness * lsp,
                        );

                        // Flag
                        if n.duration >= 8 && !is_beamed {
                            if let (Some(f_cp), Some(f_sm)) = (
                                flag_codepoint(n.duration, &stem_dir),
                                flag_smufl(n.duration, &stem_dir),
                            ) {
                                emit_glyph(cmds, stem_x, stem_end_y, f_sm, f_cp, lsp, font);
                            }
                        }

                        // Grace slash
                        if is_grace && n.grace_slash && (i == 0 || !items[i - 1].event.grace()) {
                            let thickness = 0.11 * lsp;
                            let beam_ext = grace_slash_beam_extension(n.duration);
                            let (x0, sl_y0, x1, sl_y1) = if stem_dir == "up" {
                                (
                                    stem_x - 0.45 * lsp,
                                    note_center_y + 1.02 * lsp,
                                    stem_x + (1.18 + beam_ext) * lsp,
                                    note_center_y + (2.64 + beam_ext) * lsp,
                                )
                            } else {
                                (
                                    stem_x - 0.45 * lsp,
                                    note_center_y - 2.18 * lsp,
                                    stem_x + (1.18 + beam_ext) * lsp,
                                    note_center_y + (-0.56 + beam_ext) * lsp,
                                )
                            };
                            emit_line(cmds, x0, sl_y0, x1, sl_y1, thickness);
                        }
                    }
                }

                // Dots
                if n.dots > 0 {
                    let nh_w = glyph::advance_width_for(font, notehead_smufl(n.duration));
                    let staff_pos = (-2.0 * item.y).round() as i32;
                    let dot_radius = augmentation_dot_radius(lsp);
                    let dot_x_base = dot_x_base_avoiding_cross_voice_noteheads(
                        items,
                        &item_xs,
                        i,
                        staff_pos,
                        x + nh_w / 2.0 * lsp + 0.76 * lsp,
                        dot_radius,
                        sp,
                        lsp,
                        &adj_stem_dirs,
                        font,
                    );
                    let dot_y = augmentation_dot_y(note_center_y, staff_pos, dot_radius, sp, font);
                    for d in 0..n.dots {
                        cmds.push(DrawCmd::Circle {
                            x: dot_x_base + d as f64 * 0.5 * lsp,
                            y: dot_y,
                            r: dot_radius,
                        });
                    }
                }

                // Articulations
                render_articulations(
                    cmds,
                    x,
                    note_center_y,
                    &n.articulations,
                    &stem_dir,
                    y_top,
                    sp,
                );

                // Dynamic
                if let Some(ref dyn_mark) = n.dynamic {
                    let dyn_y =
                        dynamic_anchor_y(items, &item_xs, i, &adj_stem_dirs, y_top, y_bottom, sp);
                    render_dynamic(cmds, x, dyn_y, dyn_mark, sp);
                }
            }
            Event::Chord(c) => {
                let is_grace = c.grace;
                let note_scale = if is_grace { GRACE_NOTE_SCALE } else { 1.0 };
                let lsp = sp * note_scale;
                let chord_ys_abs: Vec<f64> =
                    item.chord_ys.iter().map(|&vy| y_top + vy * sp).collect();
                let stem_dir = adj_stem_dirs
                    .get(&i)
                    .cloned()
                    .or(item.stem_dir.clone())
                    .unwrap_or_else(|| "up".to_string());
                let stem_end = adj_stem_ends
                    .get(&i)
                    .copied()
                    .map(|se| y_top + se * sp)
                    .or(item.stem_y_end.map(|se| y_top + se * sp));
                let is_beamed = adj_stem_ends.contains_key(&i);

                let top_y = chord_ys_abs
                    .iter()
                    .copied()
                    .fold(f64::NEG_INFINITY, f64::max);
                let _bottom_y = chord_ys_abs.iter().copied().fold(f64::INFINITY, f64::min);

                // Dots for each chord note
                let smufl_c = notehead_smufl(c.duration);
                let nh_w = glyph::advance_width_for(font, smufl_c);
                let offsets =
                    chord_notehead_x_offsets(&item.chord_staff_positions, &stem_dir, nh_w, lsp);
                if c.dots > 0 {
                    let mut dot_x_bases = Vec::with_capacity(chord_ys_abs.len());
                    for (ni, _) in chord_ys_abs.iter().enumerate() {
                        let nx = x + offsets[ni];
                        let staff_pos = item.chord_staff_positions[ni];
                        let dot_radius = augmentation_dot_radius(lsp);
                        dot_x_bases.push(dot_x_base_avoiding_cross_voice_noteheads(
                            items,
                            &item_xs,
                            i,
                            staff_pos,
                            nx + nh_w / 2.0 * lsp + 0.76 * lsp,
                            dot_radius,
                            sp,
                            lsp,
                            &adj_stem_dirs,
                            font,
                        ));
                    }
                    let farthest_dot_x = dot_x_bases
                        .iter()
                        .copied()
                        .fold(f64::NEG_INFINITY, f64::max);
                    let dot_radius = augmentation_dot_radius(lsp);
                    let mut dot_xs = Vec::with_capacity(chord_ys_abs.len());
                    let mut dot_ys = Vec::with_capacity(chord_ys_abs.len());
                    for (ni, &ny) in chord_ys_abs.iter().enumerate() {
                        let stagger = (ni as f64 % 2.0) * 0.08 * lsp;
                        dot_xs.push(dot_x_bases[ni].max(farthest_dot_x - 0.12 * lsp + stagger));
                        dot_ys.push(augmentation_dot_y(
                            ny,
                            item.chord_staff_positions[ni],
                            dot_radius,
                            sp,
                            font,
                        ));
                    }
                    resolve_chord_dot_collisions(
                        &mut dot_xs,
                        &dot_x_bases,
                        &dot_ys,
                        dot_radius,
                        lsp,
                    );

                    for (ni, &dot_y) in dot_ys.iter().enumerate() {
                        for d in 0..c.dots {
                            cmds.push(DrawCmd::Circle {
                                x: dot_xs[ni] + d as f64 * 0.5 * lsp,
                                y: dot_y,
                                r: dot_radius,
                            });
                        }
                    }
                }

                // Stem
                if c.duration >= 2 {
                    if let Some(stem_end_y) = stem_end {
                        let anchor_key = if stem_dir == "up" {
                            "stemUpSE"
                        } else {
                            "stemDownNW"
                        };
                        let anch = glyph::anchor_for(font, smufl_c, anchor_key);
                        let (att_x, att_y) = if let Some(a) = anch {
                            (a.x, a.y)
                        } else if stem_dir == "up" {
                            (nh_w, 0.168)
                        } else {
                            (0.0, -0.168)
                        };
                        let stem_x = x - nh_w / 2.0 * lsp + att_x * lsp;
                        let ed = glyph::engraving_defaults(font);
                        let half_thin = ed.stem_thickness / 2.0 * lsp;
                        let stem_x = stem_x
                            + if stem_dir == "up" {
                                -half_thin
                            } else {
                                half_thin
                            };
                        let primary_y_abs = if stem_dir == "up" {
                            chord_ys_abs.iter().copied().fold(f64::INFINITY, f64::min)
                        } else {
                            chord_ys_abs
                                .iter()
                                .copied()
                                .fold(f64::NEG_INFINITY, f64::max)
                        };
                        let stem_start_y = primary_y_abs + att_y * lsp;
                        emit_line(
                            cmds,
                            stem_x,
                            stem_start_y,
                            stem_x,
                            stem_end_y,
                            ed.stem_thickness * lsp,
                        );

                        // Flag
                        if c.duration >= 8 && !is_beamed {
                            if let (Some(f_cp), Some(f_sm)) = (
                                flag_codepoint(c.duration, &stem_dir),
                                flag_smufl(c.duration, &stem_dir),
                            ) {
                                emit_glyph(cmds, stem_x, stem_end_y, f_sm, f_cp, lsp, font);
                            }
                        }

                        // Grace slash
                        if is_grace && c.grace_slash && (i == 0 || !items[i - 1].event.grace()) {
                            let thickness = 0.11 * lsp;
                            let beam_ext = grace_slash_beam_extension(c.duration);
                            let (x0, sl_y0, x1, sl_y1) = if stem_dir == "up" {
                                (
                                    stem_x - 0.45 * lsp,
                                    primary_y_abs + 1.02 * lsp,
                                    stem_x + (1.18 + beam_ext) * lsp,
                                    primary_y_abs + (2.64 + beam_ext) * lsp,
                                )
                            } else {
                                (
                                    stem_x - 0.45 * lsp,
                                    primary_y_abs - 2.18 * lsp,
                                    stem_x + (1.18 + beam_ext) * lsp,
                                    primary_y_abs + (-0.56 + beam_ext) * lsp,
                                )
                            };
                            emit_line(cmds, x0, sl_y0, x1, sl_y1, thickness);
                        }
                    }
                }

                // Articulations
                let art_ref_y = if stem_dir == "down" { top_y } else { _bottom_y };
                render_articulations(cmds, x, art_ref_y, &c.articulations, &stem_dir, y_top, sp);

                // Dynamic
                if let Some(ref dyn_mark) = c.dynamic {
                    let dyn_y =
                        dynamic_anchor_y(items, &item_xs, i, &adj_stem_dirs, y_top, y_bottom, sp);
                    render_dynamic(cmds, x, dyn_y, dyn_mark, sp);
                }
            }
            _ => {}
        }

        // Fingering, chord symbol, staff text, expression text for notes and chords
        match ev {
            Event::Note(n) => {
                let note_center_y = y_top + item.y * sp;
                let stem_dir = adj_stem_dirs
                    .get(&i)
                    .cloned()
                    .or(item.stem_dir.clone())
                    .unwrap_or_else(|| "up".to_string());
                let stem_end = adj_stem_ends
                    .get(&i)
                    .copied()
                    .map(|se| y_top + se * sp)
                    .or(item.stem_y_end.map(|se| y_top + se * sp));
                let above_anchor = note_top_anchor_y(note_center_y, &stem_dir, stem_end, sp);
                render_inline_text(
                    cmds,
                    x,
                    ev,
                    above_anchor,
                    note_center_y,
                    y_top,
                    y_bottom,
                    sp,
                    fng_pos,
                    &stem_dir,
                    stem_end,
                    adj_stem_ends.contains_key(&i),
                    font,
                );

                // Staff markers
                render_staff_markers(cmds, x, &n.staff_markers, y_top, above_anchor, sp, font);
            }
            Event::Chord(c) => {
                let chord_ys_abs: Vec<f64> =
                    item.chord_ys.iter().map(|&vy| y_top + vy * sp).collect();
                let top_y = chord_ys_abs
                    .iter()
                    .copied()
                    .fold(f64::NEG_INFINITY, f64::max);
                let stem_dir = adj_stem_dirs
                    .get(&i)
                    .cloned()
                    .or(item.stem_dir.clone())
                    .unwrap_or_else(|| "up".to_string());
                let stem_end = adj_stem_ends
                    .get(&i)
                    .copied()
                    .map(|se| y_top + se * sp)
                    .or(item.stem_y_end.map(|se| y_top + se * sp));
                let above_anchor = chord_top_anchor_y(top_y, &stem_dir, stem_end, sp);
                render_inline_text(
                    cmds,
                    x,
                    ev,
                    above_anchor,
                    top_y,
                    y_top,
                    y_bottom,
                    sp,
                    fng_pos,
                    &stem_dir,
                    stem_end,
                    adj_stem_ends.contains_key(&i),
                    font,
                );
                render_staff_markers(cmds, x, &c.staff_markers, y_top, above_anchor, sp, font);
            }
            _ => {}
        }
    }

    cmds.push(DrawCmd::FlushContent);

    // ── Final barline ──
    if !skip_barlines {
        let raw_final_style = items
            .last()
            .and_then(|item| {
                if let Event::Barline(b) = &item.event {
                    Some(b.style.as_str())
                } else {
                    None
                }
            })
            .unwrap_or("final");
        let final_style = if raw_final_style == "repeat-both" {
            "repeat-end"
        } else {
            raw_final_style
        };
        let ed_final = glyph::engraving_defaults(font);
        let final_x = if matches!(final_style, "final" | "repeat-end" | "repeat-both") {
            total_width * sp - ed_final.thick_barline_thickness / 2.0 * sp
        } else {
            total_width * sp - ed_final.thin_barline_thickness / 2.0 * sp
        };
        render_barline(cmds, final_x, y_top, y_bottom, final_style, sp, font);
    }

    // ── Draw beams ──
    for beam_data in &beam_groups_data {
        render_beam_group(cmds, &beam_data.notes, sp * beam_data.scale, font);
    }

    // ── Tuplet brackets ──
    render_tuplets(
        cmds,
        items,
        &item_xs,
        &adj_stem_ends,
        &adj_stem_dirs,
        y_top,
        y_bottom,
        sp,
        font,
    );

    // ── Hairpins ──
    render_hairpins(
        cmds,
        items,
        &item_xs,
        &adj_stem_ends,
        &adj_stem_dirs,
        y_top,
        y_bottom,
        sp,
        music_start_x,
        total_width,
    );

    // ── Ties and slurs ──
    render_ties_and_slurs(cmds, items, &item_xs, &adj_stem_dirs, y_top, sp, font);

    // ── Trill lines ──
    render_trills(
        cmds,
        items,
        &item_xs,
        &adj_stem_ends,
        y_top,
        y_bottom,
        sp,
        music_start_x,
        total_width,
        font,
    );

    // ── Octave lines ──
    render_octave_lines(
        cmds,
        items,
        &item_xs,
        &adj_stem_ends,
        y_top,
        y_bottom,
        sp,
        music_start_x,
        total_width,
    );

    // ── Ending brackets (voltas) ──
    render_endings(
        cmds,
        items,
        &item_xs,
        &adj_stem_ends,
        &adj_stem_dirs,
        y_top,
        y_bottom,
        sp,
        total_width,
        font,
    );

    // ── Lyrics ──
    render_lyrics(
        cmds,
        items,
        &item_xs,
        &adj_stem_ends,
        &adj_stem_dirs,
        y_top,
        y_bottom,
        sp,
        &laid_out.lyric_prefix_states,
        music_start_x,
        total_width,
        fng_pos,
    );
}

// ─── Helper rendering functions ────────────────────────────────────────

fn render_key_signature(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    y_top: f64,
    key: &str,
    clef: Option<&str>,
    sp: f64,
    font: glyph::FontId,
) {
    let count = pitch::key_sig_accidental_count(key);
    if count == 0 {
        return;
    }
    let n = count.unsigned_abs() as usize;
    let use_clef = clef.unwrap_or("treble");
    let (acc_cp, acc_sm, positions) = if count > 0 {
        (
            0xE262u32,
            "accidentalSharp",
            pitch::key_sig_sharp_positions(use_clef),
        )
    } else {
        (
            0xE260u32,
            "accidentalFlat",
            pitch::key_sig_flat_positions(use_clef),
        )
    };
    let acc_w = glyph::advance_width_for(font, acc_sm);
    let acc_spacing = (acc_w + 0.2) * sp;
    for i in 0..n.min(positions.len()) {
        let staff_pos = positions[i];
        let acc_y = y_top - staff_pos as f64 * sp / 2.0;
        let acc_x = x + i as f64 * acc_spacing;
        emit_glyph(cmds, acc_x, acc_y, acc_sm, acc_cp, sp, font);
    }
}

fn render_time_signature(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    y_top: f64,
    upper: i32,
    lower: i32,
    symbol: Option<&str>,
    sp: f64,
    font: glyph::FontId,
) {
    match symbol {
        Some("common") => {
            emit_glyph(cmds, x, y_top - 2.0 * sp, "timeSigCommon", 0xE08A, sp, font);
        }
        Some("cut") => {
            emit_glyph(
                cmds,
                x,
                y_top - 2.0 * sp,
                "timeSigCutCommon",
                0xE08B,
                sp,
                font,
            );
        }
        _ => {
            // Upper digits
            let upper_s = upper.to_string();
            let lower_s = lower.to_string();
            let upper_w = time_sig_digits_width(&upper_s, sp, font);
            let lower_w = time_sig_digits_width(&lower_s, sp, font);
            let column_w = upper_w.max(lower_w);
            let mut dx = (column_w - upper_w) / 2.0;
            for ch in upper_s.chars() {
                if let Some(d) = ch.to_digit(10) {
                    let name = TIME_SIG_NAMES[d as usize];
                    let cp = time_digit_codepoint(d);
                    emit_glyph(cmds, x + dx, y_top - 1.0 * sp, name, cp, sp, font);
                    dx += glyph::advance_width_for(font, name) * sp;
                }
            }
            // Lower digits
            dx = (column_w - lower_w) / 2.0;
            for ch in lower_s.chars() {
                if let Some(d) = ch.to_digit(10) {
                    let name = TIME_SIG_NAMES[d as usize];
                    let cp = time_digit_codepoint(d);
                    emit_glyph(cmds, x + dx, y_top - 3.0 * sp, name, cp, sp, font);
                    dx += glyph::advance_width_for(font, name) * sp;
                }
            }
        }
    }
}

fn render_barline(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    y_top: f64,
    y_bottom: f64,
    style: &str,
    sp: f64,
    font: glyph::FontId,
) {
    let ed = glyph::engraving_defaults(font);
    let thin = ed.thin_barline_thickness * sp;
    let thick = ed.thick_barline_thickness * sp;

    let draw_bar = |cmds: &mut Vec<DrawCmd>, bx: f64, t: f64| {
        emit_line(cmds, bx, y_top, bx, y_bottom, t);
    };

    match style {
        "single" => draw_bar(cmds, x, thin),
        "double" => {
            draw_bar(cmds, x - 0.5 * sp, thin);
            draw_bar(cmds, x, thin);
        }
        "final" => {
            draw_bar(cmds, x - 0.5 * sp, thin);
            draw_bar(cmds, x, thick);
        }
        "repeat-start" => {
            draw_bar(cmds, x, thick);
            draw_bar(cmds, x + 0.5 * sp, thin);
            draw_repeat_dots(cmds, x + 1.0 * sp, y_top, sp);
        }
        "repeat-end" => {
            draw_repeat_dots(cmds, x - 1.0 * sp, y_top, sp);
            draw_bar(cmds, x - 0.5 * sp, thin);
            draw_bar(cmds, x, thick);
        }
        "repeat-both" => {
            draw_repeat_dots(cmds, x - 1.0 * sp, y_top, sp);
            draw_bar(cmds, x - 0.5 * sp, thin);
            draw_bar(cmds, x, thick);
            draw_bar(cmds, x + 0.5 * sp, thin);
            draw_repeat_dots(cmds, x + 1.0 * sp, y_top, sp);
        }
        _ => draw_bar(cmds, x, thin),
    }
}

fn draw_repeat_dots(cmds: &mut Vec<DrawCmd>, x: f64, staff_y_top: f64, sp: f64) {
    let dot_radius = 0.22 * sp;
    cmds.push(DrawCmd::Circle {
        x,
        y: staff_y_top - 1.5 * sp,
        r: dot_radius,
    });
    cmds.push(DrawCmd::Circle {
        x,
        y: staff_y_top - 2.5 * sp,
        r: dot_radius,
    });
}

fn render_spanning_barline(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    y_top: f64,
    y_bottom: f64,
    staff_y_tops: &[f64],
    style: &str,
    sp: f64,
    font: glyph::FontId,
) {
    let ed = glyph::engraving_defaults(font);
    let thin = ed.thin_barline_thickness * sp;
    let thick = ed.thick_barline_thickness * sp;

    let draw_bar = |cmds: &mut Vec<DrawCmd>, bx: f64, t: f64| {
        emit_line(cmds, bx, y_top, bx, y_bottom, t);
    };
    let draw_dots_on_staves = |cmds: &mut Vec<DrawCmd>, dx: f64| {
        for &staff_y_top in staff_y_tops {
            draw_repeat_dots(cmds, dx, staff_y_top, sp);
        }
    };

    match style {
        "repeat-start" => {
            draw_bar(cmds, x, thick);
            draw_bar(cmds, x + 0.5 * sp, thin);
            draw_dots_on_staves(cmds, x + 1.0 * sp);
        }
        "repeat-end" => {
            draw_dots_on_staves(cmds, x - 1.0 * sp);
            draw_bar(cmds, x - 0.5 * sp, thin);
            draw_bar(cmds, x, thick);
        }
        "repeat-both" => {
            draw_dots_on_staves(cmds, x - 1.0 * sp);
            draw_bar(cmds, x - 0.5 * sp, thin);
            draw_bar(cmds, x, thick);
            draw_bar(cmds, x + 0.5 * sp, thin);
            draw_dots_on_staves(cmds, x + 1.0 * sp);
        }
        _ => render_barline(cmds, x, y_top, y_bottom, style, sp, font),
    }
}

fn render_ledger_lines(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    y_top: f64,
    staff_pos: i32,
    sp: f64,
    note_scale: f64,
    font: glyph::FontId,
) {
    let info = pitch::ledger_lines_needed(staff_pos);
    if info.0 == 0 {
        return;
    }
    let ed = glyph::engraving_defaults(font);
    let lsp = sp * note_scale;
    let ext = ed.ledger_line_extension * lsp;
    let thickness = ed.staff_line_thickness * lsp;
    let nh_w = glyph::advance_width_for(font, "noteheadBlack");

    if info.1 == Some("above") {
        for i in 0..info.0 {
            let ledger_pos = -2 - i as i32 * 2;
            let ly = y_top - ledger_pos as f64 * sp / 2.0;
            emit_line(
                cmds,
                x - nh_w / 2.0 * lsp - ext,
                ly,
                x + nh_w / 2.0 * lsp + ext,
                ly,
                thickness,
            );
        }
    } else {
        for i in 0..info.0 {
            let ledger_pos = 10 + i as i32 * 2;
            let ly = y_top - ledger_pos as f64 * sp / 2.0;
            emit_line(
                cmds,
                x - nh_w / 2.0 * lsp - ext,
                ly,
                x + nh_w / 2.0 * lsp + ext,
                ly,
                thickness,
            );
        }
    }
}

fn render_articulations(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    note_y: f64,
    articulations: &[String],
    stem_dir: &str,
    y_top: f64,
    sp: f64,
) {
    if articulations.is_empty() {
        return;
    }
    let fermata: Vec<&String> = articulations
        .iter()
        .filter(|a| a.as_str() == "fermata")
        .collect();
    let non_fermata: Vec<&String> = articulations
        .iter()
        .filter(|a| a.as_str() != "fermata")
        .collect();
    let art_above = stem_dir == "down";
    // gap_above: positive = above note_center; gap_below: positive = below note_center
    let gap_above = 0.75 * sp; // first art starts 0.75sp above notehead center ("south" anchor)
    let gap_below = 1.0 * sp; // first art starts 1sp below notehead center ("north" anchor)
    let art_spacing = 1.0 * sp;

    if art_above {
        // Stem points down → articulations go ABOVE the note
        let mut cur_y = note_y + gap_above;
        for art in &non_fermata {
            if let Some(cp) = articulation_codepoint(art, true) {
                emit_glyph_anchored(cmds, x, cur_y, cp, sp, "south");
                cur_y += art_spacing;
            }
        }
        if !fermata.is_empty() {
            let fermata_y = cur_y.max(y_top + 0.5 * sp);
            emit_glyph_anchored(cmds, x, fermata_y, 0xE4C0, sp, "south");
        }
    } else {
        // Stem points up → articulations go BELOW the note
        let mut cur_y = note_y - gap_below;
        for art in &non_fermata {
            if let Some(cp) = articulation_codepoint(art, false) {
                emit_glyph_anchored(cmds, x, cur_y, cp, sp, "north");
                cur_y -= art_spacing;
            }
        }
        // Fermata always above, regardless of stem direction
        if !fermata.is_empty() {
            let fermata_y = (note_y + gap_above).max(y_top + 0.5 * sp);
            emit_glyph_anchored(cmds, x, fermata_y, 0xE4C0, sp, "south");
        }
    }
}

fn render_dynamic(cmds: &mut Vec<DrawCmd>, x: f64, y: f64, dynamic: &str, sp: f64) {
    if dynamic.is_empty() {
        return;
    }

    // Check if all chars are SMuFL dynamics
    let all_smufl = dynamic.chars().all(|ch| dynamic_codepoint(ch).is_some());
    if all_smufl {
        // Build a single Unicode string of all SMuFL codepoints and render it as one
        // music-font text element so the font handles kerning/ligatures (e.g. "mf", "mp").
        let dyn_str: String = dynamic
            .chars()
            .filter_map(|ch| dynamic_codepoint(ch))
            .filter_map(|cp| char::from_u32(cp))
            .collect();
        if !dyn_str.is_empty() {
            cmds.push(DrawCmd::MusicText {
                x,
                y,
                v: dyn_str,
                s: 4.0 * sp,
                a: "north".into(),
            });
        }
    } else {
        cmds.push(DrawCmd::Text {
            x,
            y,
            v: dynamic.to_string(),
            s: 8.0,
            w: "bold".into(),
            i: true,
            a: "north".into(),
        });
    }
}

fn dynamic_anchor_y(
    items: &[LaidOutItem],
    item_xs: &[f64],
    idx: usize,
    adj_stem_dirs: &std::collections::HashMap<usize, String>,
    y_top: f64,
    y_bottom: f64,
    sp: f64,
) -> f64 {
    let default_y = y_bottom - 1.0 * sp;
    let padding = 0.42 * sp;
    let mut safe_y = default_y;

    let mut consider_obstacle = |lowest_y: f64| {
        safe_y = safe_y.min(lowest_y - padding);
    };

    consider_obstacle(note_visual_bottom(&items[idx], y_top, sp));

    if let Some(lowest_y) = below_articulation_lowest_y(&items[idx], adj_stem_dirs, idx, y_top, sp)
    {
        consider_obstacle(lowest_y);
    }

    if let Some(lowest_y) = below_slur_lowest_y_at(items, item_xs, idx, adj_stem_dirs, y_top, sp) {
        consider_obstacle(lowest_y);
    }

    safe_y
}

fn note_visual_bottom(item: &LaidOutItem, y_top: f64, sp: f64) -> f64 {
    let bottom_scale = match &item.event {
        Event::Note(n) if n.grace => GRACE_NOTE_SCALE,
        Event::Chord(c) if c.grace => GRACE_NOTE_SCALE,
        _ => 1.0,
    };

    match &item.event {
        Event::Note(n) => {
            let note_y = y_top + item.y * sp;
            let glyph_bottom = glyph::bbox(notehead_smufl(n.duration)).map_or(-0.55, |b| b.sw_y)
                * sp
                * bottom_scale;
            note_y + glyph_bottom
        }
        Event::Chord(c) => {
            let lowest = item
                .chord_ys
                .iter()
                .map(|&vy| y_top + vy * sp)
                .fold(f64::INFINITY, f64::min);
            let glyph_bottom = glyph::bbox(notehead_smufl(c.duration)).map_or(-0.55, |b| b.sw_y)
                * sp
                * bottom_scale;
            lowest + glyph_bottom
        }
        _ => y_top + item.y * sp,
    }
}

fn below_articulation_lowest_y(
    item: &LaidOutItem,
    adj_stem_dirs: &std::collections::HashMap<usize, String>,
    idx: usize,
    y_top: f64,
    sp: f64,
) -> Option<f64> {
    let articulations = item.event.articulations();
    if articulations.is_empty() {
        return None;
    }

    let stem_dir = adj_stem_dirs
        .get(&idx)
        .map(|s| s.as_str())
        .or(item.stem_dir.as_deref())
        .unwrap_or("up");
    if stem_dir != "up" {
        return None;
    }

    let ref_y = match &item.event {
        Event::Note(_) => y_top + item.y * sp,
        Event::Chord(_) => item
            .chord_ys
            .iter()
            .map(|&vy| y_top + vy * sp)
            .fold(f64::INFINITY, f64::min),
        _ => return None,
    };

    let non_fermata_count = articulations
        .iter()
        .filter(|a| a.as_str() != "fermata")
        .count();
    if non_fermata_count == 0 {
        return None;
    }

    // Below-note articulations use a north anchor at note_y - 1sp and stack downward.
    Some(ref_y - (1.0 + non_fermata_count as f64) * sp)
}

fn below_slur_lowest_y_at(
    items: &[LaidOutItem],
    item_xs: &[f64],
    idx: usize,
    adj_stem_dirs: &std::collections::HashMap<usize, String>,
    y_top: f64,
    sp: f64,
) -> Option<f64> {
    let mut stack: Vec<usize> = Vec::with_capacity(4);

    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.is_note() && !ev.is_chord() {
            continue;
        }

        if ev.slur_start() {
            stack.push(i);
        }

        if ev.slur_end() {
            if let Some(start_idx) = stack.pop() {
                if idx < start_idx || idx > i {
                    continue;
                }

                let stem_dir = adj_stem_dirs
                    .get(&start_idx)
                    .map(|s| s.as_str())
                    .or(items[start_idx].stem_dir.as_deref())
                    .unwrap_or("up");
                if stem_dir != "up" {
                    continue;
                }

                let x1 = item_xs[start_idx];
                let x2 = item_xs[i];
                // Dynamics extend to the right of their anchor; sample slightly inside
                // that footprint so a below-slur starting on the same note still clears it.
                let target_x = (item_xs[idx] + 1.2 * sp).min(x1.max(x2));
                if target_x < x1.min(x2) || target_x > x1.max(x2) {
                    continue;
                }

                let y1 = y_top + items[start_idx].y * sp - 0.5 * sp;
                let y2 = y_top + item.y * sp - 0.5 * sp;
                let dx = x2 - x1;
                let arc_height = (dx.abs() * 0.3).clamp(0.8 * sp, 3.0 * sp);
                let t = if dx.abs() > f64::EPSILON {
                    ((target_x - x1) / dx).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let endpoint_y = y1 + (y2 - y1) * t;
                let local_depth = (std::f64::consts::PI * t).sin() * arc_height;

                return Some(endpoint_y - local_depth);
            }
        }
    }

    None
}

fn render_beam_group(
    cmds: &mut Vec<DrawCmd>,
    beam_notes: &[BeamNote],
    sp: f64,
    font: glyph::FontId,
) {
    let n = beam_notes.len();
    if n < 2 {
        return;
    }
    let stem_dir = &beam_notes[0].stem_dir;
    let sign = if stem_dir == "up" { -1.0 } else { 1.0 };
    let max_beams = beam_notes
        .iter()
        .map(|bn| beam_count(bn.duration))
        .max()
        .unwrap_or(0);
    let ed = glyph::engraving_defaults(font);
    let beam_step = (ed.beam_thickness + ed.beam_spacing) * sp;

    for level in 1..=max_beams {
        let y_offset = sign * (level - 1) as f64 * beam_step;
        let threshold = min_dur_for_level(level);
        let mut seg: Vec<&BeamNote> = Vec::new();

        let flush_seg =
            |cmds: &mut Vec<DrawCmd>, seg: &[&BeamNote], stem_dir: &str, sp: f64, y_offset: f64| {
                if seg.len() >= 2 {
                    let t = ed.beam_thickness * sp;
                    let (x0, y0) = (
                        seg.first().unwrap().stem_x,
                        seg.first().unwrap().beam_y + y_offset,
                    );
                    let (xn, yn) = (
                        seg.last().unwrap().stem_x,
                        seg.last().unwrap().beam_y + y_offset,
                    );
                    if stem_dir == "up" {
                        cmds.push(DrawCmd::Polygon {
                            pts: vec![x0, y0 - t, xn, yn - t, xn, yn, x0, y0],
                        });
                    } else {
                        cmds.push(DrawCmd::Polygon {
                            pts: vec![x0, y0, xn, yn, xn, yn + t, x0, y0 + t],
                        });
                    }
                } else if seg.len() == 1 {
                    let t = ed.beam_thickness * sp;
                    let sx = seg[0].stem_x;
                    let sy = seg[0].beam_y + y_offset;
                    let stub_w = 0.75 * sp;
                    if stem_dir == "up" {
                        cmds.push(DrawCmd::Polygon {
                            pts: vec![sx, sy - t, sx + stub_w, sy - t, sx + stub_w, sy, sx, sy],
                        });
                    } else {
                        cmds.push(DrawCmd::Polygon {
                            pts: vec![sx, sy, sx + stub_w, sy, sx + stub_w, sy + t, sx, sy + t],
                        });
                    }
                }
            };

        for bn in beam_notes {
            if bn.duration >= threshold {
                seg.push(bn);
            } else {
                flush_seg(cmds, &seg, stem_dir, sp, y_offset);
                seg.clear();
            }
        }
        flush_seg(cmds, &seg, stem_dir, sp, y_offset);
    }
}

fn note_top_anchor_y(note_y: f64, stem_dir: &str, stem_end: Option<f64>, sp: f64) -> f64 {
    let base = note_y + 1.0 * sp;
    if stem_dir == "up" {
        if let Some(se) = stem_end {
            base.max(se + 0.25 * sp)
        } else {
            base
        }
    } else {
        base
    }
}

fn chord_top_anchor_y(top_y: f64, stem_dir: &str, stem_end: Option<f64>, sp: f64) -> f64 {
    let base = top_y + 1.0 * sp;
    if stem_dir == "up" {
        if let Some(se) = stem_end {
            base.max(se + 0.25 * sp)
        } else {
            base
        }
    } else {
        base
    }
}

fn inline_clef_draw_offset(prev: Option<&Event>, next: Option<&Event>, sp: f64) -> f64 {
    let prev_is_music = prev.map_or(false, |p| {
        p.is_note() || p.is_chord() || p.is_rest() || matches!(p, Event::Spacer(_))
    });
    let next_is_music = next.map_or(false, |n| {
        n.is_note() || n.is_chord() || n.is_rest() || matches!(n, Event::Spacer(_))
    });
    if !prev_is_music || !next_is_music {
        return 0.0;
    }
    let base_shift = 0.5 * CLEF_PADDING * sp;
    let next_has_acc = next.map_or(false, |n| match n {
        Event::Note(note) => note.accidental.is_some(),
        Event::Chord(c) => c.notes.iter().any(|n| n.accidental.is_some()),
        _ => false,
    });
    if next_has_acc {
        base_shift + 0.1 * sp
    } else {
        base_shift
    }
}

fn render_inline_text(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    ev: &Event,
    above_anchor_y: f64,
    note_y: f64,
    y_top: f64,
    y_bottom: f64,
    sp: f64,
    fng_pos_default: &str,
    stem_dir: &str,
    stem_end: Option<f64>,
    is_beamed: bool,
    font: glyph::FontId,
) {
    let fng_stack_step = 1.35 * sp;
    let default_sp_numeric = 1.75; // default-staff-space in mm
    let fng_font_size = 7.85 * (sp / default_sp_numeric);
    let bold_fng_font_size = fng_font_size * 1.1;
    let ed = glyph::engraving_defaults(font);
    let beam_gap = (0.2 + 0.4 * ed.beam_thickness).max(0.35) * sp;

    // Track the topmost y of items placed above the staff so chord/staff-text
    // can stack above them with a clear gap.
    let mut above_stack_top = above_anchor_y;

    // Fingering
    if let Some(fng) = ev.fingering() {
        let event_fng_pos = ev.fingering_position();
        let fng_pos = if event_fng_pos == "below" {
            "below"
        } else {
            fng_pos_default
        };
        let marks = fng.marks();
        if fng_pos == "below" {
            let beam_clear_y = if is_beamed && stem_dir == "down" {
                stem_end.map(|se| se - beam_gap)
            } else {
                None
            };
            let mut fng_base_y = (y_bottom - 0.55 * sp).min(note_y - 0.85 * sp);
            if let Some(clear_y) = beam_clear_y {
                fng_base_y = fng_base_y.min(clear_y);
            }
            let mut cur_y = fng_base_y;
            for mark in &marks {
                if mark.value != 0 {
                    cmds.push(DrawCmd::Text {
                        x,
                        y: cur_y,
                        v: mark.value.to_string(),
                        s: if mark.bold {
                            bold_fng_font_size
                        } else {
                            fng_font_size
                        },
                        w: if mark.bold { "bold" } else { "regular" }.into(),
                        i: false,
                        a: "north".into(),
                    });
                    cur_y -= fng_stack_step;
                }
            }
        } else {
            let mut fng_base_y = (y_top + 0.8 * sp).max(note_y + 0.85 * sp);
            if is_beamed && stem_dir == "up" {
                if let Some(se) = stem_end {
                    fng_base_y = fng_base_y.max(se + beam_gap);
                }
            } else if stem_dir == "up" && ev.duration() >= 8 {
                fng_base_y = fng_base_y.max(above_anchor_y);
            }
            let mut cur_y = fng_base_y;
            for mark in &marks {
                if mark.value != 0 {
                    cmds.push(DrawCmd::Text {
                        x,
                        y: cur_y,
                        v: mark.value.to_string(),
                        s: if mark.bold {
                            bold_fng_font_size
                        } else {
                            fng_font_size
                        },
                        w: if mark.bold { "bold" } else { "regular" }.into(),
                        i: false,
                        a: "south".into(),
                    });
                    cur_y += fng_stack_step;
                }
            }
            // cur_y is now the y of the NEXT potential fingering slot — use it as
            // the new floor so chord/staff-text sit above the whole fingering stack.
            above_stack_top = above_stack_top.max(cur_y);
        }
    }

    // When this note is inside a volta/ending bracket, push the placement floor above
    // the bracket line so chord symbols and staff text don't collide with the bracket
    // frame or its label ("1.", "2.", etc.).  The bracket line lives at y_top + 3.5 sp.
    if ev.ending().is_some() {
        above_stack_top = above_stack_top.max(y_top + 3.5 * sp);
    }

    // Chord symbol — must clear the fingering stack with a visible gap.
    if let Some(cs) = ev.chord_symbol() {
        if !cs.is_empty() {
            let chord_base_y = (y_top + 2.5 * sp).max(above_stack_top + 1.5 * sp);
            cmds.push(DrawCmd::Text {
                x,
                y: chord_base_y,
                v: cs.to_string(),
                s: 10.0,
                w: "bold".into(),
                i: false,
                a: "south".into(),
            });
            // Chord text is ~10pt ≈ 3.5mm ≈ 2 sp — advance stack by that
            above_stack_top = above_stack_top.max(chord_base_y + 2.0 * sp);
        }
    }

    // Staff text — sits above chord symbols with a clear gap.
    if let Some(st) = ev.staff_text() {
        if !st.is_empty() {
            let staff_font_size = 12.0 * (sp / 1.75);
            // At least 1.0 sp above chord/fingering stack
            let staff_base_y = (y_top + 2.7 * sp).max(above_stack_top + 1.0 * sp);
            cmds.push(DrawCmd::Text {
                x,
                y: staff_base_y,
                v: st.to_string(),
                s: staff_font_size,
                w: "regular".into(),
                i: false,
                a: "south".into(),
            });
        }
    }

    // Expression text — below the staff, clear of dynamics (which sit at y_bottom - 1*sp).
    // When a dynamic is also present on this note, push expression text further down so
    // the two don't overlap (dynamics can extend ~2‑3 sp below their top anchor).
    if let Some(et) = ev.expression_text() {
        if !et.is_empty() {
            let exp_font_size = 8.75 * (sp / 1.75);
            let has_dynamic = ev.dynamic_mark().map_or(false, |d| !d.is_empty());
            let exp_base_y = if has_dynamic {
                y_bottom - 3.5 * sp
            } else {
                y_bottom - 2.0 * sp
            };
            cmds.push(DrawCmd::Text {
                x,
                y: exp_base_y,
                v: et.to_string(),
                s: exp_font_size,
                w: "regular".into(),
                i: true,
                a: "north".into(),
            });
        }
    }
}

fn render_staff_markers(
    cmds: &mut Vec<DrawCmd>,
    x: f64,
    markers: &[String],
    y_top: f64,
    above_anchor: f64,
    sp: f64,
    font: glyph::FontId,
) {
    let centered: Vec<&String> = markers
        .iter()
        .filter(|m| m.as_str() != "breath-mark" && m.as_str() != "caesura")
        .collect();
    let right: Vec<&String> = markers
        .iter()
        .filter(|m| m.as_str() == "breath-mark" || m.as_str() == "caesura")
        .collect();

    // Right-aligned markers (breath mark, caesura)
    for mk in &right {
        let marker_x = x + if mk.as_str() == "caesura" {
            1.75 * sp
        } else {
            1.55 * sp
        };
        // Caesura bbox sw_y ≈ 0, ne_y ≈ 2.13 sp — lower placement by 1 sp so it is
        // visually centred around the top staff line rather than sitting entirely above it.
        let marker_y = if mk.as_str() == "caesura" {
            y_top - 1.0 * sp
        } else {
            y_top + 0.12 * sp
        };
        if let Some(cp) = staff_marker_codepoint(mk) {
            emit_glyph(cmds, marker_x, marker_y, mk, cp, sp, font);
        }
    }

    // Centered markers
    let mut cur_y = (y_top + 1.9 * sp).max(above_anchor + 0.3 * sp);
    for mk in &centered {
        if let Some(cp) = staff_marker_codepoint(mk) {
            emit_glyph(cmds, x, cur_y, mk, cp, sp, font);
            cur_y += 1.7 * sp + 0.2 * sp;
        }
    }
}

fn render_tuplets(
    cmds: &mut Vec<DrawCmd>,
    items: &[LaidOutItem],
    item_xs: &[f64],
    adj_stem_ends: &std::collections::HashMap<usize, f64>,
    adj_stem_dirs: &std::collections::HashMap<usize, String>,
    y_top: f64,
    y_bottom: f64,
    sp: f64,
    font: glyph::FontId,
) {
    // Find tuplet groups
    let mut tuplet_groups: Vec<(Vec<usize>, i32)> = Vec::with_capacity(items.len() / 3);
    let mut cur_indices = Vec::with_capacity(8);
    let mut cur_number = 0;
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        if ev.is_note() || ev.is_rest() || ev.is_chord() {
            if ev.tuplet_beats() > 0.0 {
                if ev.tuplet_start() {
                    cur_indices = vec![i];
                    cur_number = ev.tuplet_number();
                } else if !cur_indices.is_empty() {
                    cur_indices.push(i);
                }
                if ev.tuplet_end() && !cur_indices.is_empty() {
                    tuplet_groups.push((cur_indices.clone(), cur_number));
                    cur_indices.clear();
                }
            }
        }
    }

    let tuplet_font_size = 7.75 * (sp / 1.75);

    for (indices, tn) in &tuplet_groups {
        if indices.is_empty() {
            continue;
        }

        // Determine stem direction
        let stem_ref: Vec<usize> = indices
            .iter()
            .copied()
            .filter(|&idx| items[idx].event.is_note() || items[idx].event.is_chord())
            .collect();
        let refs = if stem_ref.is_empty() {
            indices.clone()
        } else {
            stem_ref
        };
        let stem_dir = refs
            .iter()
            .find_map(|&idx| {
                adj_stem_dirs
                    .get(&idx)
                    .cloned()
                    .or(items[idx].stem_dir.clone())
            })
            .unwrap_or_else(|| "up".to_string());

        let tup_xs: Vec<f64> = indices
            .iter()
            .map(|&idx| {
                if items[idx].event.is_note() || items[idx].event.is_chord() {
                    note_stem_x(
                        item_xs[idx],
                        items[idx].event.duration(),
                        &stem_dir,
                        sp,
                        font,
                    )
                } else {
                    item_xs[idx]
                }
            })
            .collect();

        let tup_stem_ends: Vec<f64> = refs
            .iter()
            .map(|&idx| {
                if let Some(&se) = adj_stem_ends.get(&idx) {
                    y_top + se * sp
                } else if let Some(se) = items[idx].stem_y_end {
                    y_top + se * sp
                } else if stem_dir == "up" {
                    y_top + 1.6 * sp
                } else {
                    y_bottom - 1.6 * sp
                }
            })
            .collect();

        let pad = 0.26 * sp;
        let x_first = tup_xs.first().unwrap() - pad;
        let x_last = tup_xs.last().unwrap() + pad;

        let bracket_y = if stem_dir == "up" {
            tup_stem_ends
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max)
                + 0.6 * sp
        } else {
            tup_stem_ends.iter().copied().fold(f64::INFINITY, f64::min) - 0.6 * sp
        };

        let tick_len = 0.4 * sp;
        let tick_dir = if stem_dir == "up" { -1.0 } else { 1.0 };
        let line_w = 0.12 * sp;

        // Bracket lines
        emit_line(cmds, x_first, bracket_y, x_last, bracket_y, line_w);
        emit_line(
            cmds,
            x_first,
            bracket_y,
            x_first,
            bracket_y + tick_dir * tick_len,
            line_w,
        );
        emit_line(
            cmds,
            x_last,
            bracket_y,
            x_last,
            bracket_y + tick_dir * tick_len,
            line_w,
        );

        // Number
        let mid_x = (x_first + x_last) / 2.0;
        let num_offset = 0.25 * sp;
        let (num_y, anchor) = if stem_dir == "up" {
            (bracket_y + num_offset, "south")
        } else {
            (bracket_y - num_offset, "north")
        };
        cmds.push(DrawCmd::Text {
            x: mid_x,
            y: num_y,
            v: tn.to_string(),
            s: tuplet_font_size,
            w: "regular".into(),
            i: true,
            a: anchor.into(),
        });
    }
}

fn render_hairpins(
    cmds: &mut Vec<DrawCmd>,
    items: &[LaidOutItem],
    item_xs: &[f64],
    _adj_stem_ends: &std::collections::HashMap<usize, f64>,
    _adj_stem_dirs: &std::collections::HashMap<usize, String>,
    y_top: f64,
    y_bottom: f64,
    sp: f64,
    music_start_x: f64,
    total_width: f64,
) {
    struct HairpinGroup {
        indices: Vec<usize>,
        kind: String,
        starts_here: bool,
        ends_here: bool,
    }
    let mut groups: Vec<HairpinGroup> = Vec::with_capacity(4);
    let mut cur_indices = Vec::with_capacity(8);
    let mut cur_kind: Option<String> = None;

    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.is_anchor() {
            continue;
        }
        let hairpin = ev.hairpin().map(|s| s.to_string());

        if let Some(ref hp) = hairpin {
            if cur_indices.is_empty() || cur_kind.as_deref() == Some(hp) {
                cur_indices.push(i);
                cur_kind = Some(hp.clone());
            } else {
                if !cur_indices.is_empty() {
                    let first = cur_indices[0];
                    let last = *cur_indices.last().unwrap();
                    groups.push(HairpinGroup {
                        indices: cur_indices.clone(),
                        kind: cur_kind.unwrap().clone(),
                        starts_here: items[first].event.hairpin_start(),
                        ends_here: items[last].event.hairpin_end(),
                    });
                }
                cur_indices = vec![i];
                cur_kind = Some(hp.clone());
            }
            if ev.hairpin_end() && !cur_indices.is_empty() {
                let first = cur_indices[0];
                let last = *cur_indices.last().unwrap();
                groups.push(HairpinGroup {
                    indices: cur_indices.clone(),
                    kind: cur_kind.unwrap().clone(),
                    starts_here: items[first].event.hairpin_start(),
                    ends_here: items[last].event.hairpin_end(),
                });
                cur_indices = Vec::new();
                cur_kind = None;
            }
        } else if !cur_indices.is_empty() {
            let first = cur_indices[0];
            let last = *cur_indices.last().unwrap();
            groups.push(HairpinGroup {
                indices: cur_indices.clone(),
                kind: cur_kind.unwrap().clone(),
                starts_here: items[first].event.hairpin_start(),
                ends_here: items[last].event.hairpin_end(),
            });
            cur_indices = Vec::new();
            cur_kind = None;
        }
    }
    if !cur_indices.is_empty() {
        let first = cur_indices[0];
        let last = *cur_indices.last().unwrap();
        groups.push(HairpinGroup {
            indices: cur_indices,
            kind: cur_kind.unwrap(),
            starts_here: items[first].event.hairpin_start(),
            ends_here: items[last].event.hairpin_end(),
        });
    }

    for hg in &groups {
        if hg.indices.is_empty() {
            continue;
        }
        let continuation = !hg.starts_here;
        if continuation {
            // Only draw continued hairpins from the first anchor
            // (simplified - the original Typst code checks first_hairpin_anchor)
        }

        let x_first = item_xs[*hg.indices.first().unwrap()];
        let x_last = item_xs[*hg.indices.last().unwrap()];
        let x0 = if continuation {
            music_start_x
        } else {
            x_first + 0.25 * sp
        };
        let raw_x1 = if hg.ends_here {
            x_last + 0.95 * sp
        } else {
            total_width * sp - 1.0 * sp
        };
        let x1 = raw_x1.max(x0 + 1.5 * sp);

        // Compute the lowest note y in this group so the hairpin clears any ledger-line notes.
        let mut min_note_y = y_bottom;
        for &idx in &hg.indices {
            let item = &items[idx];
            match &item.event {
                Event::Note(_) => {
                    let ny = y_top + item.y * sp;
                    if ny < min_note_y {
                        min_note_y = ny;
                    }
                }
                Event::Chord(_) => {
                    for &cy in &item.chord_ys {
                        let ny = y_top + cy * sp;
                        if ny < min_note_y {
                            min_note_y = ny;
                        }
                    }
                }
                _ => {}
            }
        }
        // The hairpin must be at least 1.5 sp below the lowest notehead.
        let note_floor_y = min_note_y - 1.5 * sp;
        let baseline_y = (y_bottom - 1.9 * sp).min(note_floor_y);
        let y_center = baseline_y;

        let full_half = 0.55 * sp;
        let (start_h, end_h) = match hg.kind.as_str() {
            "cresc" => {
                let sh = if continuation { 0.18 * sp } else { 0.0 };
                (sh, full_half)
            }
            "decresc" => {
                let eh = if !hg.ends_here { 0.22 * sp } else { 0.0 };
                (full_half, eh)
            }
            _ => (0.0, 0.0),
        };

        let thickness = 0.14 * sp;
        emit_line(
            cmds,
            x0,
            y_center + start_h,
            x1,
            y_center + end_h,
            thickness,
        );
        emit_line(
            cmds,
            x0,
            y_center - start_h,
            x1,
            y_center - end_h,
            thickness,
        );
    }
}

fn render_ties_and_slurs(
    cmds: &mut Vec<DrawCmd>,
    items: &[LaidOutItem],
    item_xs: &[f64],
    adj_stem_dirs: &std::collections::HashMap<usize, String>,
    y_top: f64,
    sp: f64,
    font: glyph::FontId,
) {
    let get_stem_dir = |i: usize| -> String {
        adj_stem_dirs
            .get(&i)
            .cloned()
            .or(items[i].stem_dir.clone())
            .unwrap_or_else(|| "up".to_string())
    };

    // Ties
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.tie() {
            continue;
        }

        let mut j = i + 1;
        while j < items.len() {
            if items[j].event.is_note() || items[j].event.is_chord() {
                break;
            }
            j += 1;
        }
        if j >= items.len() {
            continue;
        }

        let stem_dir = get_stem_dir(i);
        let direction = if stem_dir == "up" { -1.0 } else { 1.0 };

        let nh_smufl = notehead_smufl(ev.duration());
        let nh_w = glyph::advance_width_for(font, nh_smufl) * sp;
        let next_nh_w =
            glyph::advance_width_for(font, notehead_smufl(items[j].event.duration())) * sp;

        let start_x = item_xs[i] + nh_w / 2.0 * 0.8;
        let end_x = item_xs[j] - next_nh_w / 2.0 * 0.8;
        let note_y = y_top + item.y * sp;
        let next_note_y = y_top + items[j].y * sp;
        let y_offset = direction * 0.35 * sp;

        render_arc(
            cmds,
            start_x,
            note_y + y_offset,
            end_x,
            next_note_y + y_offset,
            direction,
            sp,
            0.2,
            0.25,
        );
    }

    // Slurs
    let mut slur_starts: Vec<usize> = Vec::with_capacity(4);
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.is_note() && !ev.is_chord() {
            continue;
        }
        if ev.slur_start() {
            slur_starts.push(i);
        }
        if ev.slur_end() && !slur_starts.is_empty() {
            let start_idx = slur_starts.pop().unwrap();
            let stem_dir = get_stem_dir(start_idx);
            let direction = if stem_dir == "up" { -1.0 } else { 1.0 };

            let nh_w =
                glyph::advance_width_for(font, notehead_smufl(items[start_idx].event.duration()))
                    * sp;
            let next_nh_w = glyph::advance_width_for(font, notehead_smufl(ev.duration())) * sp;

            let start_x = item_xs[start_idx] + nh_w / 2.0 * 0.8;
            let end_x = item_xs[i] - next_nh_w / 2.0 * 0.8;
            let start_y = y_top + items[start_idx].y * sp + direction * 0.5 * sp;
            let end_y = y_top + item.y * sp + direction * 0.5 * sp;

            render_arc(
                cmds, start_x, start_y, end_x, end_y, direction, sp, 0.22, 0.3,
            );
        }
    }
}

fn render_arc(
    cmds: &mut Vec<DrawCmd>,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    direction: f64,
    sp: f64,
    max_thickness: f64,
    height_factor: f64,
) {
    let dx = x2 - x1;
    let arc_height = (dx.abs() * height_factor).clamp(0.8 * sp, 3.0 * sp);
    let half_thick = max_thickness * sp / 2.0;

    let outer_cp1_x = x1 + dx * 0.2;
    let outer_cp1_y = y1 + direction * (arc_height + half_thick) * 0.9;
    let outer_cp2_x = x1 + dx * 0.8;
    let outer_cp2_y = y2 + direction * (arc_height + half_thick) * 0.9;

    let inner_cp1_x = x1 + dx * 0.25;
    let inner_cp1_y = y1 + direction * (arc_height - half_thick).max(arc_height * 0.5) * 0.9;
    let inner_cp2_x = x1 + dx * 0.75;
    let inner_cp2_y = y2 + direction * (arc_height - half_thick).max(arc_height * 0.5) * 0.9;

    cmds.push(DrawCmd::BezierFill {
        pts: vec![
            x1,
            y1,
            outer_cp1_x,
            outer_cp1_y,
            outer_cp2_x,
            outer_cp2_y,
            x2,
            y2,
            inner_cp2_x,
            inner_cp2_y,
            inner_cp1_x,
            inner_cp1_y,
        ],
    });
}

fn render_trills(
    cmds: &mut Vec<DrawCmd>,
    items: &[LaidOutItem],
    item_xs: &[f64],
    _adj_stem_ends: &std::collections::HashMap<usize, f64>,
    y_top: f64,
    _y_bottom: f64,
    sp: f64,
    music_start_x: f64,
    total_width: f64,
    font: glyph::FontId,
) {
    let trill_cp = 0xE566u32;
    let wiggle_cp = 0xEAA4u32;
    let tr_width = glyph::advance_width_for(font, "ornamentTrill") * sp;
    let tr_min_y = y_top + 1.15 * sp;

    // Standalone trills
    for (idx, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.trill() || ev.trill_line() {
            continue;
        }
        let visual_top = note_visual_top(item, y_top, sp);
        let trill_y = (visual_top + 0.75 * sp).max(tr_min_y);
        emit_glyph(
            cmds,
            item_xs[idx] - 0.55 * tr_width,
            trill_y,
            "ornamentTrill",
            trill_cp,
            sp,
            font,
        );
    }

    // Trill line groups
    struct TrillLineGroup {
        indices: Vec<usize>,
        starts_here: bool,
        ends_here: bool,
    }
    let mut trill_groups: Vec<TrillLineGroup> = Vec::with_capacity(4);
    let mut cur_indices = Vec::with_capacity(8);
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        if ev.is_anchor() && ev.trill_line() {
            cur_indices.push(i);
        } else if !cur_indices.is_empty() {
            let first = cur_indices[0];
            let last = *cur_indices.last().unwrap();
            trill_groups.push(TrillLineGroup {
                indices: cur_indices.clone(),
                starts_here: items[first].event.trill_start(),
                ends_here: items[last].event.trill_end(),
            });
            cur_indices.clear();
        }
    }
    if !cur_indices.is_empty() {
        let first = cur_indices[0];
        let last = *cur_indices.last().unwrap();
        trill_groups.push(TrillLineGroup {
            indices: cur_indices,
            starts_here: items[first].event.trill_start(),
            ends_here: items[last].event.trill_end(),
        });
    }

    let tr_gap = 0.45 * sp; // enough space so "tr" glyph and wiggle line don't collide
    let wiggle_w = glyph::advance_width_for(font, "wiggleTrill") * sp;

    for tg in &trill_groups {
        if tg.indices.is_empty() {
            continue;
        }
        let line_top = tg
            .indices
            .iter()
            .map(|&idx| note_visual_top(&items[idx], y_top, sp))
            .fold(f64::NEG_INFINITY, f64::max);
        let trill_y = (line_top + 0.75 * sp).max(tr_min_y);

        if tg.starts_here {
            let symbol_x = item_xs[*tg.indices.first().unwrap()] - 0.55 * tr_width;
            emit_glyph(cmds, symbol_x, trill_y, "ornamentTrill", trill_cp, sp, font);
        }

        let wiggle_start = if tg.starts_here {
            item_xs[*tg.indices.first().unwrap()] - 0.55 * tr_width + tr_width + tr_gap
        } else {
            music_start_x
        };
        let wiggle_end = if !tg.ends_here {
            total_width * sp - 1.0 * sp
        } else {
            item_xs[*tg.indices.last().unwrap()] + 0.85 * sp
        }
        .max(wiggle_start + 0.4 * sp);

        if wiggle_w > 0.0 {
            let step = wiggle_w * 0.92;
            let mut cx = wiggle_start;
            while cx < wiggle_end {
                emit_glyph(
                    cmds,
                    cx,
                    trill_y + 0.02 * sp,
                    "wiggleTrill",
                    wiggle_cp,
                    sp,
                    font,
                );
                cx += step;
            }
        }
    }
}

fn render_octave_lines(
    cmds: &mut Vec<DrawCmd>,
    items: &[LaidOutItem],
    item_xs: &[f64],
    adj_stem_ends: &std::collections::HashMap<usize, f64>,
    y_top: f64,
    y_bottom: f64,
    sp: f64,
    music_start_x: f64,
    total_width: f64,
) {
    struct OctGroup {
        indices: Vec<usize>,
        number: i32,
        direction: String,
        starts_here: bool,
        ends_here: bool,
    }
    let mut groups: Vec<OctGroup> = Vec::with_capacity(4);
    let mut cur_indices = Vec::with_capacity(8);
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        if ev.is_anchor() && ev.octave_line_number() > 0 {
            cur_indices.push(i);
        } else if !cur_indices.is_empty() {
            let first = cur_indices[0];
            let last = *cur_indices.last().unwrap();
            groups.push(OctGroup {
                indices: cur_indices.clone(),
                number: items[first].event.octave_line_number(),
                direction: items[first]
                    .event
                    .octave_line_direction()
                    .unwrap_or("above")
                    .to_string(),
                starts_here: items[first].event.octave_line_start(),
                ends_here: items[last].event.octave_line_end(),
            });
            cur_indices.clear();
        }
    }
    if !cur_indices.is_empty() {
        let first = cur_indices[0];
        let last = *cur_indices.last().unwrap();
        groups.push(OctGroup {
            indices: cur_indices,
            number: items[first].event.octave_line_number(),
            direction: items[first]
                .event
                .octave_line_direction()
                .unwrap_or("above")
                .to_string(),
            starts_here: items[first].event.octave_line_start(),
            ends_here: items[last].event.octave_line_end(),
        });
    }

    let tuplet_font_size = 7.75 * (sp / 1.75);
    let line_w = 0.12 * sp;

    for og in &groups {
        if og.indices.is_empty() {
            continue;
        }
        let x_first = item_xs[*og.indices.first().unwrap()];
        let x_last = item_xs[*og.indices.last().unwrap()];
        let x0 = if og.starts_here {
            x_first
        } else {
            music_start_x
        };
        let x1 = if og.ends_here {
            x_last
        } else {
            total_width * sp - 1.0 * sp
        };

        if og.direction == "above" {
            let elem_ys: Vec<f64> = og
                .indices
                .iter()
                .map(|&idx| {
                    // Top edge of the highest notehead (centre + half notehead height).
                    let note_top = if items[idx].chord_ys.is_empty() {
                        y_top + items[idx].y * sp + 0.5 * sp
                    } else {
                        let max_y = items[idx]
                            .chord_ys
                            .iter()
                            .copied()
                            .fold(f64::NEG_INFINITY, f64::max);
                        y_top + max_y * sp + 0.5 * sp
                    };
                    // If the stem tip extends even higher, use that instead.
                    let stem_tip = adj_stem_ends
                        .get(&idx)
                        .copied()
                        .map(|se| y_top + se * sp)
                        .or_else(|| items[idx].stem_y_end.map(|se| y_top + se * sp));
                    stem_tip.map(|st| note_top.max(st)).unwrap_or(note_top)
                })
                .collect();
            let top_y = elem_ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            // Bracket sits above the highest element AND at least above the staff top.
            let bracket_y = top_y.max(y_top) + 0.75 * sp;
            let tick_len = 0.45 * sp;

            // Dashed line
            render_dashed_line(cmds, x0, x1, bracket_y, sp);
            if og.starts_here {
                emit_line(cmds, x0, bracket_y, x0, bracket_y - tick_len, line_w);
            }
            if og.ends_here {
                emit_line(cmds, x1, bracket_y, x1, bracket_y - tick_len, line_w);
            }

            // Label
            if og.starts_here {
                let suffix = if og.number == 15 { "ma" } else { "va" };
                cmds.push(DrawCmd::Text {
                    x: x0 + 0.3 * sp,
                    y: bracket_y + 0.45 * sp,
                    v: og.number.to_string(),
                    s: tuplet_font_size,
                    w: "bold".into(),
                    i: false,
                    a: "south".into(),
                });
                let offset_x = if og.number.to_string().len() > 1 {
                    1.3 * sp
                } else {
                    0.8 * sp
                };
                cmds.push(DrawCmd::Text {
                    x: x0 + 0.3 * sp + offset_x,
                    y: bracket_y + 0.45 * sp,
                    v: suffix.to_string(),
                    s: 0.55 * tuplet_font_size,
                    w: "bold".into(),
                    i: false,
                    a: "south".into(),
                });
            }
        } else {
            let elem_ys: Vec<f64> = og
                .indices
                .iter()
                .map(|&idx| {
                    // Bottom edge of the lowest notehead (centre - half notehead height).
                    let note_bottom = if items[idx].chord_ys.is_empty() {
                        y_top + items[idx].y * sp - 0.5 * sp
                    } else {
                        let min_y = items[idx]
                            .chord_ys
                            .iter()
                            .copied()
                            .fold(f64::INFINITY, f64::min);
                        y_top + min_y * sp - 0.5 * sp
                    };
                    // If the stem tip extends even lower, use that instead.
                    let stem_tip = adj_stem_ends
                        .get(&idx)
                        .copied()
                        .map(|se| y_top + se * sp)
                        .or_else(|| items[idx].stem_y_end.map(|se| y_top + se * sp));
                    stem_tip
                        .map(|st| note_bottom.min(st))
                        .unwrap_or(note_bottom)
                })
                .collect();
            let bot_y = elem_ys.iter().copied().fold(f64::INFINITY, f64::min);
            // Bracket sits below the lowest element AND at least below the staff bottom.
            let bracket_y = bot_y.min(y_bottom) - 0.75 * sp;
            let tick_len = 0.45 * sp;

            render_dashed_line(cmds, x0, x1, bracket_y, sp);
            if og.starts_here {
                emit_line(cmds, x0, bracket_y, x0, bracket_y + tick_len, line_w);
            }
            if og.ends_here {
                emit_line(cmds, x1, bracket_y, x1, bracket_y + tick_len, line_w);
            }

            if og.starts_here {
                let suffix = if og.number == 15 { "mb" } else { "vb" };
                cmds.push(DrawCmd::Text {
                    x: x0 + 0.3 * sp,
                    y: bracket_y - 0.45 * sp,
                    v: og.number.to_string(),
                    s: tuplet_font_size,
                    w: "bold".into(),
                    i: false,
                    a: "north".into(),
                });
                let offset_x = if og.number.to_string().len() > 1 {
                    1.3 * sp
                } else {
                    0.8 * sp
                };
                cmds.push(DrawCmd::Text {
                    x: x0 + 0.3 * sp + offset_x,
                    y: bracket_y - 0.45 * sp,
                    v: suffix.to_string(),
                    s: 0.55 * tuplet_font_size,
                    w: "bold".into(),
                    i: false,
                    a: "north".into(),
                });
            }
        }
    }
}

fn render_dashed_line(cmds: &mut Vec<DrawCmd>, x0: f64, x1: f64, y: f64, sp: f64) {
    let dash = 1.2 * sp;
    let gap = 0.8 * sp;
    let line_w = 0.12 * sp;
    let mut cur = x0;
    while cur < x1 {
        let seg_end = (cur + dash).min(x1);
        emit_line(cmds, cur, y, seg_end, y, line_w);
        cur += dash + gap;
    }
}

fn render_endings(
    cmds: &mut Vec<DrawCmd>,
    items: &[LaidOutItem],
    item_xs: &[f64],
    _adj_stem_ends: &std::collections::HashMap<usize, f64>,
    _adj_stem_dirs: &std::collections::HashMap<usize, String>,
    y_top: f64,
    _y_bottom: f64,
    sp: f64,
    total_width: f64,
    font: glyph::FontId,
) {
    struct EndingGroup {
        indices: Vec<usize>,
        label: String,
        starts_here: bool,
        ends_here: bool,
    }
    let mut groups: Vec<EndingGroup> = Vec::with_capacity(4);
    let mut cur_indices = Vec::with_capacity(8);
    let mut cur_label: Option<String> = None;

    for (i, item) in items.iter().enumerate() {
        let ending = item.event.ending().map(|s| s.to_string());
        if let Some(ref lbl) = ending {
            if cur_indices.is_empty() || cur_label.as_deref() == Some(lbl) {
                cur_indices.push(i);
                cur_label = Some(lbl.clone());
            } else {
                let first = cur_indices[0];
                let last = *cur_indices.last().unwrap();
                groups.push(EndingGroup {
                    indices: cur_indices.clone(),
                    label: cur_label.unwrap(),
                    starts_here: items[first].event.ending_start(),
                    ends_here: items[last].event.ending_end(),
                });
                cur_indices = vec![i];
                cur_label = Some(lbl.clone());
            }
            if item.event.ending_end() && !cur_indices.is_empty() {
                let first = cur_indices[0];
                let last = *cur_indices.last().unwrap();
                groups.push(EndingGroup {
                    indices: cur_indices.clone(),
                    label: cur_label.unwrap(),
                    starts_here: items[first].event.ending_start(),
                    ends_here: items[last].event.ending_end(),
                });
                cur_indices = Vec::new();
                cur_label = None;
            }
        } else if !cur_indices.is_empty() {
            let first = cur_indices[0];
            let last = *cur_indices.last().unwrap();
            groups.push(EndingGroup {
                indices: cur_indices.clone(),
                label: cur_label.unwrap(),
                starts_here: items[first].event.ending_start(),
                ends_here: items[last].event.ending_end(),
            });
            cur_indices = Vec::new();
            cur_label = None;
        }
    }
    if !cur_indices.is_empty() {
        let first = cur_indices[0];
        let last = *cur_indices.last().unwrap();
        groups.push(EndingGroup {
            indices: cur_indices,
            label: cur_label.unwrap(),
            starts_here: items[first].event.ending_start(),
            ends_here: items[last].event.ending_end(),
        });
    }

    let ed = glyph::engraving_defaults(font);
    let opening_barline_x = ed.thin_barline_thickness / 2.0 * sp;
    let final_barline_x = total_width * sp - ed.thick_barline_thickness / 2.0 * sp;
    let line_w = 0.12 * sp;
    let tuplet_font_size = 7.75 * (sp / 1.75);

    for eg in &groups {
        if eg.indices.is_empty() {
            continue;
        }
        let first = *eg.indices.first().unwrap();
        let last = *eg.indices.last().unwrap();

        // Find adjacent barlines for x coordinates
        let x0 = if eg.starts_here {
            // Find previous barline
            let mut prev_bar = None;
            let mut scan = first as i32 - 1;
            while scan >= 0 {
                if items[scan as usize].event.is_barline() {
                    prev_bar = Some(scan as usize);
                    break;
                }
                scan -= 1;
            }
            if let Some(pb) = prev_bar {
                if pb == items.len() - 1 {
                    final_barline_x
                } else {
                    item_xs[pb] + 0.5 * sp
                }
            } else {
                opening_barline_x
            }
        } else {
            opening_barline_x
        };

        let x1 = if eg.ends_here {
            let mut next_bar = None;
            let mut scan = last + 1;
            while scan < items.len() {
                if items[scan].event.is_barline() {
                    next_bar = Some(scan);
                    break;
                }
                scan += 1;
            }
            if let Some(nb) = next_bar {
                if nb == items.len() - 1 {
                    final_barline_x
                } else {
                    item_xs[nb] + 0.5 * sp
                }
            } else {
                final_barline_x
            }
        } else {
            final_barline_x
        };

        // Bracket sits high enough that chord symbols (which appear inside it
        // at y_top + 1.5 sp) are clearly below the bracket line.
        let bracket_y = y_top + 3.5 * sp;
        let hook_depth = 0.65 * sp;

        emit_line(cmds, x0, bracket_y, x1, bracket_y, line_w);
        emit_line(cmds, x0, bracket_y, x0, bracket_y - hook_depth, line_w);
        if eg.ends_here {
            emit_line(cmds, x1, bracket_y, x1, bracket_y - hook_depth, line_w);
        }

        if eg.starts_here && !eg.label.is_empty() {
            cmds.push(DrawCmd::Text {
                x: x0 + 0.45 * sp,
                y: bracket_y - 0.05 * sp,
                v: eg.label.clone(),
                s: tuplet_font_size * 1.15,
                w: "regular".into(),
                i: false,
                a: "north-west".into(),
            });
        }
    }
}

fn note_visual_top(item: &LaidOutItem, y_top: f64, sp: f64) -> f64 {
    let ev = &item.event;
    match ev {
        Event::Note(n) => {
            let note_y = y_top + item.y * sp;
            let glyph_top = glyph::bbox(notehead_smufl(n.duration)).map_or(1.0, |b| b.ne_y) * sp;
            (note_y + 0.9 * sp).max(note_y + glyph_top)
        }
        Event::Chord(c) => {
            let top_y = item
                .chord_ys
                .iter()
                .map(|&vy| y_top + vy * sp)
                .fold(f64::NEG_INFINITY, f64::max);
            let glyph_top = glyph::bbox(notehead_smufl(c.duration)).map_or(1.0, |b| b.ne_y) * sp;
            (top_y + 0.9 * sp).max(top_y + glyph_top)
        }
        _ => y_top + 1.0 * sp,
    }
}

fn render_lyrics(
    cmds: &mut Vec<DrawCmd>,
    items: &[LaidOutItem],
    item_xs: &[f64],
    _adj_stem_ends: &std::collections::HashMap<usize, f64>,
    _adj_stem_dirs: &std::collections::HashMap<usize, String>,
    _y_top: f64,
    y_bottom: f64,
    sp: f64,
    lyric_prefix_states: &[Option<String>],
    _music_start_x: f64,
    _total_width: f64,
    _fng_pos: &str,
) {
    let lyric_font_size = 9.25 * (sp / 1.75);
    let lyric_line_step = 1.75 * sp;
    let lyric_text_gap = 0.28 * sp;
    let lyric_extender_trim = 0.18 * sp;

    // Count lyric lines
    let lyric_line_count = items.iter().fold(lyric_prefix_states.len(), |count, item| {
        count.max(item.event.lyrics().len())
    });
    if lyric_line_count == 0 {
        return;
    }

    let lyric_top_y = (y_bottom - 3.1 * sp).min(y_bottom - 0.85 * sp);

    // Simple lyric rendering
    for (idx, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.is_note() && !ev.is_chord() {
            continue;
        }

        let lyrics = ev.lyrics();
        let x = item_xs[idx];

        for (li, entry) in lyrics.iter().enumerate() {
            if entry.carry {
                continue;
            }
            let top_y = lyric_top_y - li as f64 * lyric_line_step;

            if let Some(ref text) = entry.text {
                if !text.is_empty() {
                    cmds.push(DrawCmd::Text {
                        x,
                        y: top_y,
                        v: text.clone(),
                        s: lyric_font_size,
                        w: "regular".into(),
                        i: false,
                        a: "north".into(),
                    });
                }
            }
        }
    }

    // Hyphens and extenders between lyrics
    for li in 0..lyric_line_count {
        let top_y = lyric_top_y - li as f64 * lyric_line_step;
        let mut _prev_text_x: Option<f64> = None;
        let mut prev_continuation: Option<String> = None;
        let mut prev_right_x: Option<f64> = None;

        for (idx, item) in items.iter().enumerate() {
            let ev = &item.event;
            if !ev.is_note() && !ev.is_chord() {
                continue;
            }

            let lyrics = ev.lyrics();
            let x = item_xs[idx];
            let entry = lyrics.get(li);

            if let Some(entry) = entry {
                if entry.carry {
                    continue;
                }
                if let Some(ref text) = entry.text {
                    if !text.is_empty() {
                        // Draw continuation from previous
                        if let Some(ref cont) = prev_continuation {
                            if let Some(px) = prev_right_x {
                                if cont == "hyphen" {
                                    let mid_x = (px + x) / 2.0;
                                    cmds.push(DrawCmd::Text {
                                        x: mid_x,
                                        y: top_y,
                                        v: "-".into(),
                                        s: lyric_font_size,
                                        w: "regular".into(),
                                        i: false,
                                        a: "north".into(),
                                    });
                                } else if cont == "extender" {
                                    // Add extra padding at both ends so the underscore line
                                    // doesn't visually collide with the surrounding syllables.
                                    // Estimate the half-width of the NEXT syllable (text is
                                    // centred at x) and leave 0.4 sp of clear space before it.
                                    let next_half_w = text.len() as f64 * 0.25 * sp;
                                    let ext_start = px + lyric_extender_trim;
                                    let ext_end = x - next_half_w - 0.4 * sp - lyric_extender_trim;
                                    if ext_end > ext_start {
                                        let ext_y = top_y - 1.45 * sp;
                                        emit_line(
                                            cmds,
                                            ext_start,
                                            ext_y,
                                            ext_end,
                                            ext_y,
                                            0.09 * sp,
                                        );
                                    }
                                }
                            }
                        }
                        _prev_text_x = Some(x);
                        prev_continuation = Some(entry.continuation.clone());
                        // Estimate text right edge — use a larger per-character width so the
                        // extender line starts clearly after wide characters like 'W' or 'y'.
                        prev_right_x = Some(x + text.len() as f64 * 0.45 * sp + lyric_text_gap);
                    }
                }
            }
        }
    }
}
