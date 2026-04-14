use crate::glyph;
use crate::layout;
use crate::pitch;
use crate::types::*;

// ─── Constants ─────────────────────────────────────────────────────────

const STAFF_LINE_THICKNESS: f64 = 0.13;
const STEM_THICKNESS: f64 = 0.12;
const BEAM_THICKNESS: f64 = 0.5;
const BEAM_SPACING: f64 = 0.25;
const BARLINE_THICKNESS: f64 = 0.16;
const THICK_BARLINE: f64 = 0.35;
const LEDGER_LINE_EXTENSION: f64 = 0.4;
const ACCIDENTAL_PADDING: f64 = 0.35;
const INLINE_CLEF_SCALE: f64 = 0.8;
const CLEF_PADDING: f64 = 0.5;
const GRACE_NOTE_SCALE: f64 = 0.68;

// ─── SMuFL codepoint helpers ───────────────────────────────────────────

fn notehead_codepoint(duration: i32) -> u32 {
    match duration {
        1 => 0xE0A2,  // noteheadWhole
        2 => 0xE0A3,  // noteheadHalf
        _ => 0xE0A4,  // noteheadBlack
    }
}

fn notehead_smufl(duration: i32) -> &'static str {
    match duration {
        1 => "noteheadWhole",
        2 => "noteheadHalf",
        _ => "noteheadBlack",
    }
}

fn rest_codepoint(duration: i32) -> u32 {
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

fn rest_smufl(duration: i32) -> &'static str {
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
        "treble" | "treble-8a" | "treble8a" | "treble-8b" | "treble8b"
        | "treble-8" | "treble8" | "treble-15a" | "treble-15b" => 3.0,
        "bass" | "bass-8a" | "bass8a" | "bass-8b" | "bass8b"
        | "bass-15a" | "bass-15b" => 1.0,
        "alto" => 2.0,
        "tenor" => 2.0,
        "percussion" => 2.0,
        _ => 3.0,
    }
}

fn time_digit_codepoint(d: u32) -> u32 {
    0xE080 + d
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
fn emit_glyph(cmds: &mut Vec<DrawCmd>, x: f64, y: f64, smufl_name: &str, codepoint: u32, sp: f64) {
    let fsize = 4.0 * sp;
    let bb = glyph::bbox(smufl_name);
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

/// Place a glyph with an explicit CeTZ anchor and NO bounding-box offset.
/// Use this for articulations and dynamics where the coordinate is the
/// desired glyph edge ("south" = bottom at y, "north" = top at y, etc.).
fn emit_glyph_anchored(cmds: &mut Vec<DrawCmd>, x: f64, y: f64, codepoint: u32, sp: f64, anchor: &str) {
    cmds.push(DrawCmd::Glyph {
        x,
        y,
        c: codepoint,
        s: 4.0 * sp,
        a: anchor.into(),
    });
}

fn emit_glyph_scaled(cmds: &mut Vec<DrawCmd>, x: f64, y: f64, smufl_name: &str, codepoint: u32, sp: f64) {
    emit_glyph(cmds, x, y, smufl_name, codepoint, sp);
}

fn emit_line(cmds: &mut Vec<DrawCmd>, x1: f64, y1: f64, x2: f64, y2: f64, w: f64) {
    cmds.push(DrawCmd::Line { x1, y1, x2, y2, w });
}

// ─── Note stem x computation ──────────────────────────────────────────

fn note_stem_x(x: f64, duration: i32, stem_dir: &str, sp: f64) -> f64 {
    let smufl = notehead_smufl(duration);
    let nh_w = glyph::advance_width(smufl);
    let anchor_key = if stem_dir == "up" { "stemUpSE" } else { "stemDownNW" };
    let anch = glyph::anchor(smufl, anchor_key);
    let (att_x, _att_y) = if let Some(a) = anch {
        (a.x, a.y)
    } else if stem_dir == "up" {
        (nh_w, 0.168)
    } else {
        (0.0, -0.168)
    };
    let sx = x - nh_w / 2.0 * sp + att_x * sp;
    let half_thin = STEM_THICKNESS / 2.0 * sp;
    sx + if stem_dir == "up" { -half_thin } else { half_thin }
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

    let alt_offset = if stem_dir == "down" { -nh_w * lsp } else { nh_w * lsp };
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

// ─── Beam helpers ──────────────────────────────────────────────────────

fn beam_count(duration: i32) -> i32 {
    match duration {
        d if d >= 64 => 4,
        d if d >= 32 => 3,
        d if d >= 16 => 2,
        d if d >= 8 => 1,
        _ => 0,
    }
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
        let has_dynamic   = ev.dynamic_mark().map_or(false, |d| !d.is_empty());
        let has_expression = ev.expression_text().map_or(false, |e| !e.is_empty());
        let has_hairpin   = ev.hairpin().is_some();
        let lyric_count   = ev.lyrics().iter().filter(|l| l.text.is_some()).count();
        // Dynamic glyph sits at y_bottom-1 sp (north anchor), extends ~2.5 sp downward → 3.5 sp.
        // Expression text at y_bottom-3.5 sp (with dynamic) or y_bottom-2.0 sp (alone), ~1.5 sp tall.
        if has_dynamic && has_expression {
            max_sp = max_sp.max(6.0);  // dyn (3.5) + expression below that (~2 sp more)
        } else if has_dynamic {
            max_sp = max_sp.max(3.5);
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
        let has_chord      = ev.chord_symbol().map_or(false, |c| !c.is_empty());
        let has_staff_text = ev.staff_text().map_or(false, |t| !t.is_empty());
        let has_ending     = ev.ending().is_some();
        let has_fingering  = ev.fingering().is_some();
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
) -> SystemOutput {
    let mut cmds = Vec::new();
    let num_staves = laid_out_staves.len();
    let staff_height_mm = 4.0 * sp_unit;
    let use_spanning_barlines = num_staves > 1;

    // Compute shared prefix data
    let (shared_time_x, shared_music_start_x) = compute_shared_prefix(
        laid_out_staves, key, time, sp_unit, show_time,
    );

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
    let mut y_offset = -header_height; // CeTZ y goes up, but staff draws downward

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
            use_spanning_barlines,  // all staves skip barlines when spanning
            fng_pos,
            y_top,
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
                let brace_w = glyph::advance_width("brace") * sp_unit * scale;
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
            emit_line(&mut cmds, bx, last_y_bottom, bx + serif, last_y_bottom, thick);
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
            let scale_x = if s0_total_w > 0.0 { avail_music_w / s0_total_w } else { 1.0 };
            let total_w = compute_total_width(laid_out_staves, sp_unit, avail_width_mm, shared_music_start_x);

            // Opening barline (left edge)
            emit_line(&mut cmds, BARLINE_THICKNESS / 2.0 * sp_unit, first_y_top,
                      BARLINE_THICKNESS / 2.0 * sp_unit, last_y_bottom,
                      BARLINE_THICKNESS * sp_unit);

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
                    if Some(idx) == last_barline_idx { continue; } // handled below as final
                    let bx = shared_music_start_x + item.x * scale_x * sp_unit + 0.5 * sp_unit;
                    render_barline(&mut cmds, bx, first_y_top, last_y_bottom, &b.style, sp_unit);
                }
            }

            // Final barline
            let raw_final_style = if last_item_is_barline {
                items.last()
                    .and_then(|item| if let Event::Barline(b) = &item.event { Some(b.style.as_str()) } else { None })
                    .unwrap_or("final")
            } else {
                "final" // music ends with a note — emit standard final barline
            };
            let final_style = if raw_final_style == "repeat-both" { "repeat-end" } else { raw_final_style };
            let final_x = if matches!(final_style, "final" | "repeat-end") {
                total_w * sp_unit - THICK_BARLINE / 2.0 * sp_unit
            } else {
                total_w * sp_unit - BARLINE_THICKNESS / 2.0 * sp_unit
            };
            render_barline(&mut cmds, final_x, first_y_top, last_y_bottom, final_style, sp_unit);
        }
    }

    // Compute final dimensions
    let total_w_sp = compute_total_width(laid_out_staves, sp_unit, avail_width_mm, shared_music_start_x);
    let width_mm = avail_width_mm.unwrap_or(total_w_sp * sp_unit);

    // Add below-staff content depth
    total_height += 1.75 * sp_unit; // baseline below depth

    SystemOutput {
        width: width_mm,
        height: total_height,
        cmds,
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
        let local_music_start = prefix_x + clef_w + key_w + time_w + 1.0 * sp;
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
    let max_tw = laid_out_staves.iter().map(|s| s.total_width).fold(0.0_f64, f64::max);
    music_start_x / sp + max_tw + 1.0
}

// ─── Single staff rendering ───────────────────────────────────────────

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
) {
    let clef_name = laid_out.clef.as_deref();
    let opening_time = laid_out.time.as_ref().or(time.as_ref());
    let show_opening_time = laid_out.show_time_prefix || show_time;
    let items = &laid_out.items;
    let total_layout_width = laid_out.total_width;

    let y_top = y_top_offset;
    let y_bottom = y_top - 4.0 * sp;

    // Compute prefix
    let mut cx = 0.5 * sp;
    let mut clef_w = 0.0;
    if let Some(c) = clef_name {
        clef_w = layout::clef_advance_sp(c, sp);
    }
    let key_w = layout::key_sig_advance_sp(key, sp);
    let time_w = if show_opening_time {
        if let Some(t) = opening_time {
            layout::time_sig_advance_sp(t.upper, t.lower, t.symbol.as_deref(), sp)
        } else {
            0.0
        }
    } else {
        0.0
    };

    let music_start_x = forced_music_start_x.unwrap_or_else(|| {
        let mut msx = cx + clef_w + key_w + time_w + 1.0 * sp;
        // Extra space for first accidental
        let first_has_acc = items.iter().find(|i| i.event.is_note() || i.event.is_chord()).map_or(false, |i| {
            match &i.event {
                Event::Note(n) => n.accidental.is_some(),
                Event::Chord(c) => c.notes.iter().any(|n| n.accidental.is_some()),
                _ => false,
            }
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
        emit_line(cmds, 0.0, y, total_width * sp, y, STAFF_LINE_THICKNESS * sp);
    }

    // Opening barline — skipped when the group renderer draws a spanning barline
    if !skip_barlines {
        emit_line(cmds, BARLINE_THICKNESS / 2.0 * sp, y_top,
                  BARLINE_THICKNESS / 2.0 * sp, y_bottom, BARLINE_THICKNESS * sp);
    }

    // Draw clef
    cx = 0.5 * sp;
    if let Some(c) = clef_name {
        let origin_offset = clef_origin_offset(c);
        let origin_y = y_top - origin_offset * sp;
        emit_glyph(cmds, cx, origin_y, clef_smufl(c), clef_codepoint(c), sp);
        cx += clef_w;
    }

    // Draw key signature
    render_key_signature(cmds, cx, y_top, key, clef_name, sp);
    cx += key_w;

    // Draw time signature
    if show_opening_time {
        let time_x = forced_time_x.unwrap_or(cx);
        if let Some(t) = opening_time {
            render_time_signature(cmds, time_x, y_top, t.upper, t.lower, t.symbol.as_deref(), sp);
        }
    }

    // Pre-compute item x positions
    let item_xs: Vec<f64> = items.iter().map(|item| music_start_x + item.x * scale_x * sp).collect();

    // Compute notehead bbox
    let black_bb = glyph::bbox("noteheadBlack");
    let black_top = black_bb.map_or(0.82, |b| b.ne_y);
    let black_bottom = black_bb.map_or(-0.82, |b| b.sw_y);

    // ── Auto-beaming ──
    let mut raw_beam_groups: Vec<Vec<usize>> = Vec::new();
    let mut cur_beam: Vec<usize> = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        let beamable = (ev.is_note() || ev.is_chord()) && ev.duration() >= 8;
        let grace = ev.grace();
        if beamable {
            let same_grace = cur_beam.is_empty() || items[*cur_beam.first().unwrap()].event.grace() == grace;
            if !same_grace {
                if cur_beam.len() >= 2 { raw_beam_groups.push(cur_beam.clone()); }
                cur_beam.clear();
            }
            let limit = if grace { 8 } else { 4 };
            if cur_beam.len() == limit {
                raw_beam_groups.push(cur_beam.clone());
                cur_beam.clear();
            }
            cur_beam.push(i);
        } else {
            if cur_beam.len() >= 2 { raw_beam_groups.push(cur_beam.clone()); }
            cur_beam.clear();
        }
    }
    if cur_beam.len() >= 2 { raw_beam_groups.push(cur_beam); }

    // Compute beam geometry
    let mut adj_stem_ends: std::collections::HashMap<usize, f64> = std::collections::HashMap::new();
    let mut adj_stem_dirs: std::collections::HashMap<usize, String> = std::collections::HashMap::new();

    let mut beam_groups_data: Vec<BeamGroupData> = Vec::new();

    for group in &raw_beam_groups {
        let group_is_grace = items[*group.first().unwrap()].event.grace();
        let beam_scale = if group_is_grace { GRACE_NOTE_SCALE } else { 1.0 };

        // Unified stem direction
        let avg_y = group.iter().map(|&idx| items[idx].y).sum::<f64>() / group.len() as f64;
        let avg_staff_pos = -2.0 * avg_y;
        let stem_dir = if avg_staff_pos > 4.0 { "up" } else { "down" };

        let first = &items[*group.first().unwrap()];
        let last = &items[*group.last().unwrap()];
        let mut sy0 = pitch::compute_stem_end_y(first.y, (-2.0 * first.y).round() as i32, stem_dir, beam_scale, 3.5);
        let mut syn = pitch::compute_stem_end_y(last.y, (-2.0 * last.y).round() as i32, stem_dir, beam_scale, 3.5);

        let x0 = item_xs[*group.first().unwrap()];
        let xn = item_xs[*group.last().unwrap()];
        let beam_step_staff = (BEAM_THICKNESS + BEAM_SPACING) * beam_scale;
        let min_clearance = 0.25 * beam_scale;
        let mut required_shift: f64 = 0.0;

        for &idx in group {
            let item = &items[idx];
            let xi = item_xs[idx];
            let t = if xn != x0 { (xi - x0) / (xn - x0) } else { 0.0 };
            let by_staff = sy0 + t * (syn - sy0);
            let beam_levels = beam_count(item.event.duration());
            let nearest_edge = if stem_dir == "up" {
                by_staff - (beam_levels - 1) as f64 * beam_step_staff - BEAM_THICKNESS * beam_scale
            } else {
                by_staff + (beam_levels - 1) as f64 * beam_step_staff + BEAM_THICKNESS * beam_scale
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
            let outward = if stem_dir == "up" { required_shift } else { -required_shift };
            sy0 += outward;
            syn += outward;
        }

        let mut beam_note_data = Vec::new();
        for &idx in group {
            let item = &items[idx];
            let xi = item_xs[idx];
            let t = if xn != x0 { (xi - x0) / (xn - x0) } else { 0.0 };
            let by_staff = sy0 + t * (syn - sy0);
            let sx = note_stem_x(xi, item.event.duration(), stem_dir, sp * beam_scale);
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
                let prev = if i > 0 { Some(&items[i - 1].event) } else { None };
                let next = items.get(i + 1).map(|i| &i.event);
                let offset = inline_clef_draw_offset(prev, next, sp);
                let clef_x = x - offset;
                let origin_y = y_top - clef_origin_offset(&c.clef) * sp;
                emit_glyph_scaled(cmds, clef_x, origin_y, clef_smufl(&c.clef),
                                  clef_codepoint(&c.clef), sp * INLINE_CLEF_SCALE);
            }
            Event::TimeSig(t) => {
                render_time_signature(cmds, x, y_top, t.upper, t.lower, t.symbol.as_deref(), sp);
            }
            Event::Note(n) => {
                let note_scale = if n.grace { GRACE_NOTE_SCALE } else { 1.0 };
                let note_center_y = y_top + y;
                let staff_pos = (-2.0 * item.y).round() as i32;
                let lsp = sp * note_scale;
                let smufl = notehead_smufl(n.duration);
                let cp = notehead_codepoint(n.duration);
                let nh_w = glyph::advance_width(smufl);

                // Ledger lines
                render_ledger_lines(cmds, x, y_top, staff_pos, sp, note_scale);

                // Accidental
                if let Some(ref acc) = n.accidental {
                    if let (Some(acc_cp), Some(acc_sm)) = (accidental_codepoint(acc), accidental_smufl(acc)) {
                        let acc_w = glyph::advance_width(acc_sm);
                        let acc_x = x - nh_w / 2.0 * lsp - ACCIDENTAL_PADDING * lsp - acc_w * lsp;
                        emit_glyph(cmds, acc_x, note_center_y, acc_sm, acc_cp, lsp);
                    }
                }

                // Notehead
                emit_glyph(cmds, x - nh_w / 2.0 * lsp, note_center_y, smufl, cp, lsp);
            }
            Event::Chord(c) => {
                let note_scale = if c.grace { GRACE_NOTE_SCALE } else { 1.0 };
                let lsp = sp * note_scale;
                let smufl = notehead_smufl(c.duration);
                let cp = notehead_codepoint(c.duration);
                let nh_w = glyph::advance_width(smufl);
                let stem_dir = adj_stem_dirs.get(&i).cloned()
                    .or(item.stem_dir.clone())
                    .unwrap_or_else(|| "up".to_string());
                let offsets = chord_notehead_x_offsets(&item.chord_staff_positions, &stem_dir, nh_w, lsp);

                for (ni, cn) in c.notes.iter().enumerate() {
                    let ny = y_top + item.chord_ys[ni] * sp;
                    let nsp = item.chord_staff_positions[ni];
                    let nx = x + offsets[ni];
                    render_ledger_lines(cmds, nx, y_top, nsp, sp, note_scale);
                    if let Some(ref acc) = cn.accidental {
                        if let (Some(acc_cp), Some(acc_sm)) = (accidental_codepoint(acc), accidental_smufl(acc)) {
                            let acc_w = glyph::advance_width(acc_sm);
                            let acc_x = nx - nh_w / 2.0 * lsp - ACCIDENTAL_PADDING * lsp - acc_w * lsp;
                            emit_glyph(cmds, acc_x, ny, acc_sm, acc_cp, lsp);
                        }
                    }
                    emit_glyph(cmds, nx - nh_w / 2.0 * lsp, ny, smufl, cp, lsp);
                }
            }
            Event::Rest(r) => {
                let note_scale = if r.grace { GRACE_NOTE_SCALE } else { 1.0 };
                let lsp = sp * note_scale;
                let rst_smufl = rest_smufl(r.duration);
                let rst_cp = rest_codepoint(r.duration);
                emit_glyph(cmds, x, y_top + y, rst_smufl, rst_cp, lsp);
                // Rest dots
                if r.dots > 0 {
                    let bb = glyph::bbox(rst_smufl);
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
                    render_barline(cmds, x + 0.5 * sp, y_top, y_bottom, &b.style, sp);
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
                let stem_dir = adj_stem_dirs.get(&i).cloned()
                    .or(item.stem_dir.clone())
                    .unwrap_or_else(|| "up".to_string());
                let stem_end = adj_stem_ends.get(&i).copied()
                    .map(|se| y_top + se * sp)
                    .or(item.stem_y_end.map(|se| y_top + se * sp));
                let is_beamed = adj_stem_ends.contains_key(&i);

                // Stem
                if n.duration >= 2 {
                    if let Some(stem_end_y) = stem_end {
                        let smufl_n = notehead_smufl(n.duration);
                        let nh_w = glyph::advance_width(smufl_n);
                        let anchor_key = if stem_dir == "up" { "stemUpSE" } else { "stemDownNW" };
                        let anch = glyph::anchor(smufl_n, anchor_key);
                        let (att_x, att_y) = if let Some(a) = anch { (a.x, a.y) } else if stem_dir == "up" { (nh_w, 0.168) } else { (0.0, -0.168) };
                        let stem_x = x - nh_w / 2.0 * lsp + att_x * lsp;
                        let half_thin = STEM_THICKNESS / 2.0 * lsp;
                        let stem_x = stem_x + if stem_dir == "up" { -half_thin } else { half_thin };
                        let stem_start_y = note_center_y + att_y * lsp;
                        emit_line(cmds, stem_x, stem_start_y, stem_x, stem_end_y, STEM_THICKNESS * lsp);

                        // Flag
                        if n.duration >= 8 && !is_beamed {
                            if let (Some(f_cp), Some(f_sm)) = (flag_codepoint(n.duration, &stem_dir), flag_smufl(n.duration, &stem_dir)) {
                                emit_glyph(cmds, stem_x, stem_end_y, f_sm, f_cp, lsp);
                            }
                        }

                        // Grace slash
                        if is_grace && n.grace_slash && (i == 0 || !items[i - 1].event.grace()) {
                            let thickness = 0.11 * lsp;
                            let x0 = stem_x - 0.65 * lsp;
                            let x1 = stem_x + 0.28 * lsp;
                            let (sl_y0, sl_y1) = if stem_dir == "up" {
                                (note_center_y + 1.95 * lsp, note_center_y + 1.15 * lsp)
                            } else {
                                (note_center_y - 1.15 * lsp, note_center_y - 2.05 * lsp)
                            };
                            emit_line(cmds, x0, sl_y0, x1, sl_y1, thickness);
                        }
                    }
                }

                // Dots
                if n.dots > 0 {
                    let nh_w = glyph::advance_width(notehead_smufl(n.duration));
                    let dot_x_base = x + nh_w / 2.0 * lsp + 0.6 * lsp;
                    for d in 0..n.dots {
                        cmds.push(DrawCmd::Circle {
                            x: dot_x_base + d as f64 * 0.5 * lsp,
                            y: note_center_y,
                            r: 0.2 * lsp,
                        });
                    }
                }

                // Articulations
                render_articulations(cmds, x, note_center_y, &n.articulations, &stem_dir, y_top, sp);

                // Dynamic
                if let Some(ref dyn_mark) = n.dynamic {
                    render_dynamic(cmds, x, y_bottom, dyn_mark, sp, 0.0);
                }
            }
            Event::Chord(c) => {
                let is_grace = c.grace;
                let note_scale = if is_grace { GRACE_NOTE_SCALE } else { 1.0 };
                let lsp = sp * note_scale;
                let chord_ys_abs: Vec<f64> = item.chord_ys.iter().map(|&vy| y_top + vy * sp).collect();
                let stem_dir = adj_stem_dirs.get(&i).cloned()
                    .or(item.stem_dir.clone())
                    .unwrap_or_else(|| "up".to_string());
                let stem_end = adj_stem_ends.get(&i).copied()
                    .map(|se| y_top + se * sp)
                    .or(item.stem_y_end.map(|se| y_top + se * sp));
                let is_beamed = adj_stem_ends.contains_key(&i);

                let top_y = chord_ys_abs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let _bottom_y = chord_ys_abs.iter().copied().fold(f64::INFINITY, f64::min);

                // Dots for each chord note
                let smufl_c = notehead_smufl(c.duration);
                let nh_w = glyph::advance_width(smufl_c);
                let offsets = chord_notehead_x_offsets(&item.chord_staff_positions, &stem_dir, nh_w, lsp);
                if c.dots > 0 {
                    for (ni, &ny) in chord_ys_abs.iter().enumerate() {
                        let nx = x + offsets[ni];
                        let dot_x_base = nx + nh_w / 2.0 * lsp + 0.6 * lsp;
                        for d in 0..c.dots {
                            cmds.push(DrawCmd::Circle {
                                x: dot_x_base + d as f64 * 0.5 * lsp,
                                y: ny,
                                r: 0.2 * lsp,
                            });
                        }
                    }
                }

                // Stem
                if c.duration >= 2 {
                    if let Some(stem_end_y) = stem_end {
                        let anchor_key = if stem_dir == "up" { "stemUpSE" } else { "stemDownNW" };
                        let anch = glyph::anchor(smufl_c, anchor_key);
                        let (att_x, att_y) = if let Some(a) = anch { (a.x, a.y) } else if stem_dir == "up" { (nh_w, 0.168) } else { (0.0, -0.168) };
                        let stem_x = x - nh_w / 2.0 * lsp + att_x * lsp;
                        let half_thin = STEM_THICKNESS / 2.0 * lsp;
                        let stem_x = stem_x + if stem_dir == "up" { -half_thin } else { half_thin };
                        let primary_y_abs = if stem_dir == "up" {
                            chord_ys_abs.iter().copied().fold(f64::INFINITY, f64::min)
                        } else {
                            chord_ys_abs.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                        };
                        let stem_start_y = primary_y_abs + att_y * lsp;
                        emit_line(cmds, stem_x, stem_start_y, stem_x, stem_end_y, STEM_THICKNESS * lsp);

                        // Flag
                        if c.duration >= 8 && !is_beamed {
                            if let (Some(f_cp), Some(f_sm)) = (flag_codepoint(c.duration, &stem_dir), flag_smufl(c.duration, &stem_dir)) {
                                emit_glyph(cmds, stem_x, stem_end_y, f_sm, f_cp, lsp);
                            }
                        }

                        // Grace slash
                        if is_grace && c.grace_slash && (i == 0 || !items[i - 1].event.grace()) {
                            let thickness = 0.11 * lsp;
                            let x0 = stem_x - 0.65 * lsp;
                            let x1 = stem_x + 0.28 * lsp;
                            let (sl_y0, sl_y1) = if stem_dir == "up" {
                                (primary_y_abs + 1.95 * lsp, primary_y_abs + 1.15 * lsp)
                            } else {
                                (primary_y_abs - 1.15 * lsp, primary_y_abs - 2.05 * lsp)
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
                    render_dynamic(cmds, x, y_bottom, dyn_mark, sp, 0.0);
                }
            }
            _ => {}
        }

        // Fingering, chord symbol, staff text, expression text for notes and chords
        match ev {
            Event::Note(n) => {
                let note_center_y = y_top + item.y * sp;
                let stem_dir = adj_stem_dirs.get(&i).cloned()
                    .or(item.stem_dir.clone())
                    .unwrap_or_else(|| "up".to_string());
                let stem_end = adj_stem_ends.get(&i).copied()
                    .map(|se| y_top + se * sp)
                    .or(item.stem_y_end.map(|se| y_top + se * sp));
                let above_anchor = note_top_anchor_y(note_center_y, &stem_dir, stem_end, sp);
                render_inline_text(cmds, x, ev, above_anchor, note_center_y, y_top, y_bottom, sp, fng_pos);

                // Staff markers
                render_staff_markers(cmds, x, &n.staff_markers, y_top, above_anchor, sp);
            }
            Event::Chord(c) => {
                let chord_ys_abs: Vec<f64> = item.chord_ys.iter().map(|&vy| y_top + vy * sp).collect();
                let top_y = chord_ys_abs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let stem_dir = adj_stem_dirs.get(&i).cloned()
                    .or(item.stem_dir.clone())
                    .unwrap_or_else(|| "up".to_string());
                let stem_end = adj_stem_ends.get(&i).copied()
                    .map(|se| y_top + se * sp)
                    .or(item.stem_y_end.map(|se| y_top + se * sp));
                let above_anchor = chord_top_anchor_y(top_y, &stem_dir, stem_end, sp);
                render_inline_text(cmds, x, ev, above_anchor, top_y, y_top, y_bottom, sp, fng_pos);
                render_staff_markers(cmds, x, &c.staff_markers, y_top, above_anchor, sp);
            }
            _ => {}
        }
    }

    cmds.push(DrawCmd::FlushContent);

    // ── Final barline ──
    if !skip_barlines {
        let raw_final_style = items.last()
            .and_then(|item| if let Event::Barline(b) = &item.event { Some(b.style.as_str()) } else { None })
            .unwrap_or("final");
        let final_style = if raw_final_style == "repeat-both" { "repeat-end" } else { raw_final_style };
        let final_x = if matches!(final_style, "final" | "repeat-end" | "repeat-both") {
            total_width * sp - THICK_BARLINE / 2.0 * sp
        } else {
            total_width * sp - BARLINE_THICKNESS / 2.0 * sp
        };
        render_barline(cmds, final_x, y_top, y_bottom, final_style, sp);
    }

    // ── Draw beams ──
    for beam_data in &beam_groups_data {
        render_beam_group(cmds, &beam_data.notes, sp * beam_data.scale);
    }

    // ── Tuplet brackets ──
    render_tuplets(cmds, items, &item_xs, &adj_stem_ends, &adj_stem_dirs, y_top, y_bottom, sp);

    // ── Hairpins ──
    render_hairpins(cmds, items, &item_xs, &adj_stem_ends, &adj_stem_dirs, y_top, y_bottom, sp, music_start_x, total_width);

    // ── Ties and slurs ──
    render_ties_and_slurs(cmds, items, &item_xs, &adj_stem_dirs, y_top, sp);

    // ── Trill lines ──
    render_trills(cmds, items, &item_xs, &adj_stem_ends, y_top, y_bottom, sp, music_start_x, total_width);

    // ── Octave lines ──
    render_octave_lines(cmds, items, &item_xs, &adj_stem_ends, y_top, y_bottom, sp, music_start_x, total_width);

    // ── Ending brackets (voltas) ──
    render_endings(cmds, items, &item_xs, &adj_stem_ends, &adj_stem_dirs, y_top, y_bottom, sp, total_width);

    // ── Lyrics ──
    render_lyrics(cmds, items, &item_xs, &adj_stem_ends, &adj_stem_dirs, y_top, y_bottom, sp,
                  &laid_out.lyric_prefix_states, music_start_x, total_width, fng_pos);
}

// ─── Helper rendering functions ────────────────────────────────────────

fn render_key_signature(cmds: &mut Vec<DrawCmd>, x: f64, y_top: f64, key: &str, clef: Option<&str>, sp: f64) {
    let count = pitch::key_sig_accidental_count(key);
    if count == 0 { return; }
    let n = count.unsigned_abs() as usize;
    let use_clef = clef.unwrap_or("treble");
    let (acc_cp, acc_sm, positions) = if count > 0 {
        (0xE262u32, "accidentalSharp", pitch::key_sig_sharp_positions(use_clef))
    } else {
        (0xE260u32, "accidentalFlat", pitch::key_sig_flat_positions(use_clef))
    };
    let acc_w = glyph::advance_width(acc_sm);
    let acc_spacing = (acc_w + 0.2) * sp;
    for i in 0..n.min(positions.len()) {
        let staff_pos = positions[i];
        let acc_y = y_top - staff_pos as f64 * sp / 2.0;
        let acc_x = x + i as f64 * acc_spacing;
        emit_glyph(cmds, acc_x, acc_y, acc_sm, acc_cp, sp);
    }
}

fn render_time_signature(cmds: &mut Vec<DrawCmd>, x: f64, y_top: f64, upper: i32, lower: i32, symbol: Option<&str>, sp: f64) {
    match symbol {
        Some("common") => {
            emit_glyph(cmds, x, y_top - 2.0 * sp, "timeSigCommon", 0xE08A, sp);
        }
        Some("cut") => {
            emit_glyph(cmds, x, y_top - 2.0 * sp, "timeSigCutCommon", 0xE08B, sp);
        }
        _ => {
            // Upper digits
            let upper_s = upper.to_string();
            let mut dx = 0.0;
            for ch in upper_s.chars() {
                if let Some(d) = ch.to_digit(10) {
                    let name = format!("timeSig{}", d);
                    let cp = time_digit_codepoint(d);
                    emit_glyph(cmds, x + dx, y_top - 1.0 * sp, &name, cp, sp);
                    dx += glyph::advance_width(&name) * sp;
                }
            }
            // Lower digits
            let lower_s = lower.to_string();
            dx = 0.0;
            for ch in lower_s.chars() {
                if let Some(d) = ch.to_digit(10) {
                    let name = format!("timeSig{}", d);
                    let cp = time_digit_codepoint(d);
                    emit_glyph(cmds, x + dx, y_top - 3.0 * sp, &name, cp, sp);
                    dx += glyph::advance_width(&name) * sp;
                }
            }
        }
    }
}

fn render_barline(cmds: &mut Vec<DrawCmd>, x: f64, y_top: f64, y_bottom: f64, style: &str, sp: f64) {
    let thin = BARLINE_THICKNESS * sp;
    let thick = THICK_BARLINE * sp;
    let dot_radius = 0.22 * sp;

    let draw_bar = |cmds: &mut Vec<DrawCmd>, bx: f64, t: f64| {
        emit_line(cmds, bx, y_top, bx, y_bottom, t);
    };
    let draw_dots = |cmds: &mut Vec<DrawCmd>, dx: f64| {
        cmds.push(DrawCmd::Circle { x: dx, y: y_top - 1.5 * sp, r: dot_radius });
        cmds.push(DrawCmd::Circle { x: dx, y: y_top - 2.5 * sp, r: dot_radius });
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
            draw_dots(cmds, x + 1.0 * sp);
        }
        "repeat-end" => {
            draw_dots(cmds, x - 1.0 * sp);
            draw_bar(cmds, x - 0.5 * sp, thin);
            draw_bar(cmds, x, thick);
        }
        "repeat-both" => {
            draw_dots(cmds, x - 1.0 * sp);
            draw_bar(cmds, x - 0.5 * sp, thin);
            draw_bar(cmds, x, thick);
            draw_bar(cmds, x + 0.5 * sp, thin);
            draw_dots(cmds, x + 1.0 * sp);
        }
        _ => draw_bar(cmds, x, thin),
    }
}

fn render_ledger_lines(cmds: &mut Vec<DrawCmd>, x: f64, y_top: f64, staff_pos: i32, sp: f64, note_scale: f64) {
    let info = pitch::ledger_lines_needed(staff_pos);
    if info.0 == 0 { return; }
    let lsp = sp * note_scale;
    let ext = LEDGER_LINE_EXTENSION * lsp;
    let thickness = STAFF_LINE_THICKNESS * lsp;
    let nh_w = glyph::advance_width("noteheadBlack");

    if info.1 == Some("above") {
        for i in 0..info.0 {
            let ledger_pos = -2 - i as i32 * 2;
            let ly = y_top - ledger_pos as f64 * sp / 2.0;
            emit_line(cmds, x - nh_w / 2.0 * lsp - ext, ly, x + nh_w / 2.0 * lsp + ext, ly, thickness);
        }
    } else {
        for i in 0..info.0 {
            let ledger_pos = 10 + i as i32 * 2;
            let ly = y_top - ledger_pos as f64 * sp / 2.0;
            emit_line(cmds, x - nh_w / 2.0 * lsp - ext, ly, x + nh_w / 2.0 * lsp + ext, ly, thickness);
        }
    }
}

fn render_articulations(cmds: &mut Vec<DrawCmd>, x: f64, note_y: f64, articulations: &[String], stem_dir: &str, y_top: f64, sp: f64) {
    if articulations.is_empty() { return; }
    let fermata: Vec<&String> = articulations.iter().filter(|a| a.as_str() == "fermata").collect();
    let non_fermata: Vec<&String> = articulations.iter().filter(|a| a.as_str() != "fermata").collect();
    let art_above = stem_dir == "down";
    // gap_above: positive = above note_center; gap_below: positive = below note_center
    let gap_above = 0.75 * sp;  // first art starts 0.75sp above notehead center ("south" anchor)
    let gap_below = 1.0 * sp;   // first art starts 1sp below notehead center ("north" anchor)
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

fn render_dynamic(cmds: &mut Vec<DrawCmd>, x: f64, y_bottom: f64, dynamic: &str, sp: f64, extra_offset: f64) {
    if dynamic.is_empty() { return; }
    let gap = 1.0 * sp;
    let dyn_y = y_bottom - gap - extra_offset;

    // Check if all chars are SMuFL dynamics
    let all_smufl = dynamic.chars().all(|ch| dynamic_codepoint(ch).is_some());
    if all_smufl {
        // Build a single Unicode string of all SMuFL codepoints and render it as one
        // music-font text element so the font handles kerning/ligatures (e.g. "mf", "mp").
        let dyn_str: String = dynamic.chars()
            .filter_map(|ch| dynamic_codepoint(ch))
            .filter_map(|cp| char::from_u32(cp))
            .collect();
        if !dyn_str.is_empty() {
            cmds.push(DrawCmd::MusicText {
                x, y: dyn_y,
                v: dyn_str,
                s: 4.0 * sp,
                a: "north".into(),
            });
        }
    } else {
        cmds.push(DrawCmd::Text {
            x, y: dyn_y,
            v: dynamic.to_string(),
            s: 8.0,
            w: "bold".into(),
            i: true,
            a: "north".into(),
        });
    }
}

fn render_beam_group(cmds: &mut Vec<DrawCmd>, beam_notes: &[BeamNote], sp: f64) {
    let n = beam_notes.len();
    if n < 2 { return; }
    let stem_dir = &beam_notes[0].stem_dir;
    let sign = if stem_dir == "up" { -1.0 } else { 1.0 };
    let max_beams = beam_notes.iter().map(|bn| beam_count(bn.duration)).max().unwrap_or(0);
    let beam_step = (BEAM_THICKNESS + BEAM_SPACING) * sp;

    for level in 1..=max_beams {
        let y_offset = sign * (level - 1) as f64 * beam_step;
        let threshold = min_dur_for_level(level);
        let mut seg: Vec<&BeamNote> = Vec::new();

        let flush_seg = |cmds: &mut Vec<DrawCmd>, seg: &[&BeamNote], stem_dir: &str, sp: f64, y_offset: f64| {
            if seg.len() >= 2 {
                let t = BEAM_THICKNESS * sp;
                let (x0, y0) = (seg.first().unwrap().stem_x, seg.first().unwrap().beam_y + y_offset);
                let (xn, yn) = (seg.last().unwrap().stem_x, seg.last().unwrap().beam_y + y_offset);
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
                let t = BEAM_THICKNESS * sp;
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
    if !prev_is_music || !next_is_music { return 0.0; }
    let base_shift = 0.5 * CLEF_PADDING * sp;
    let next_has_acc = next.map_or(false, |n| match n {
        Event::Note(note) => note.accidental.is_some(),
        Event::Chord(c) => c.notes.iter().any(|n| n.accidental.is_some()),
        _ => false,
    });
    if next_has_acc { base_shift + 0.1 * sp } else { base_shift }
}

fn render_inline_text(cmds: &mut Vec<DrawCmd>, x: f64, ev: &Event, above_anchor_y: f64, note_y: f64, y_top: f64, y_bottom: f64, sp: f64, fng_pos_default: &str) {
    let fng_stack_step = 1.3 * sp;
    let default_sp_numeric = 1.75; // default-staff-space in mm
    let fng_font_size = 7.25 * (sp / default_sp_numeric);

    // Track the topmost y of items placed above the staff so chord/staff-text
    // can stack above them with a clear gap.
    let mut above_stack_top = above_anchor_y;

    // Fingering
    if let Some(fng) = ev.fingering() {
        let event_fng_pos = ev.fingering_position();
        let fng_pos = if event_fng_pos == "below" { "below" } else { fng_pos_default };
        let values = fng.values();
        if fng_pos == "below" {
            let fng_base_y = (y_bottom - 0.5 * sp).min(note_y - 1.0 * sp);
            let mut cur_y = fng_base_y - fng_stack_step;
            for &v in &values {
                if v != 0 {
                    cmds.push(DrawCmd::Text {
                        x, y: cur_y,
                        v: v.to_string(),
                        s: fng_font_size,
                        w: "regular".into(),
                        i: false,
                        a: "north".into(),
                    });
                    cur_y -= fng_stack_step;
                }
            }
        } else {
            let fng_base_y = (y_top + 1.5 * sp).max(above_anchor_y);
            let mut cur_y = fng_base_y;
            for &v in &values {
                if v != 0 {
                    cmds.push(DrawCmd::Text {
                        x, y: cur_y,
                        v: v.to_string(),
                        s: fng_font_size,
                        w: "regular".into(),
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
                x, y: chord_base_y,
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
                x, y: staff_base_y,
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
            let exp_base_y = if has_dynamic { y_bottom - 3.5 * sp } else { y_bottom - 2.0 * sp };
            cmds.push(DrawCmd::Text {
                x, y: exp_base_y,
                v: et.to_string(),
                s: exp_font_size,
                w: "regular".into(),
                i: true,
                a: "north".into(),
            });
        }
    }
}

fn render_staff_markers(cmds: &mut Vec<DrawCmd>, x: f64, markers: &[String], y_top: f64, above_anchor: f64, sp: f64) {
    let centered: Vec<&String> = markers.iter().filter(|m| m.as_str() != "breath-mark" && m.as_str() != "caesura").collect();
    let right: Vec<&String> = markers.iter().filter(|m| m.as_str() == "breath-mark" || m.as_str() == "caesura").collect();

    // Right-aligned markers (breath mark, caesura)
    for mk in &right {
        let marker_x = x + if mk.as_str() == "caesura" { 1.75 * sp } else { 1.55 * sp };
        // Caesura bbox sw_y ≈ 0, ne_y ≈ 2.13 sp — lower placement by 1 sp so it is
        // visually centred around the top staff line rather than sitting entirely above it.
        let marker_y = if mk.as_str() == "caesura" { y_top - 1.0 * sp } else { y_top + 0.12 * sp };
        if let Some(cp) = staff_marker_codepoint(mk) {
            emit_glyph(cmds, marker_x, marker_y, mk, cp, sp);
        }
    }

    // Centered markers
    let mut cur_y = (y_top + 1.9 * sp).max(above_anchor + 0.3 * sp);
    for mk in &centered {
        if let Some(cp) = staff_marker_codepoint(mk) {
            emit_glyph(cmds, x, cur_y, mk, cp, sp);
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
) {
    // Find tuplet groups
    let mut tuplet_groups: Vec<(Vec<usize>, i32)> = Vec::new();
    let mut cur_indices = Vec::new();
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
        if indices.is_empty() { continue; }

        // Determine stem direction
        let stem_ref: Vec<usize> = indices.iter().copied()
            .filter(|&idx| items[idx].event.is_note() || items[idx].event.is_chord())
            .collect();
        let refs = if stem_ref.is_empty() { indices.clone() } else { stem_ref };
        let stem_dir = refs.iter().find_map(|&idx| {
            adj_stem_dirs.get(&idx).cloned().or(items[idx].stem_dir.clone())
        }).unwrap_or_else(|| "up".to_string());

        let tup_xs: Vec<f64> = indices.iter().map(|&idx| {
            if items[idx].event.is_note() || items[idx].event.is_chord() {
                note_stem_x(item_xs[idx], items[idx].event.duration(), &stem_dir, sp)
            } else {
                item_xs[idx]
            }
        }).collect();

        let tup_stem_ends: Vec<f64> = refs.iter().map(|&idx| {
            if let Some(&se) = adj_stem_ends.get(&idx) {
                y_top + se * sp
            } else if let Some(se) = items[idx].stem_y_end {
                y_top + se * sp
            } else if stem_dir == "up" {
                y_top + 1.6 * sp
            } else {
                y_bottom - 1.6 * sp
            }
        }).collect();

        let pad = 0.26 * sp;
        let x_first = tup_xs.first().unwrap() - pad;
        let x_last = tup_xs.last().unwrap() + pad;

        let bracket_y = if stem_dir == "up" {
            tup_stem_ends.iter().copied().fold(f64::NEG_INFINITY, f64::max) + 0.6 * sp
        } else {
            tup_stem_ends.iter().copied().fold(f64::INFINITY, f64::min) - 0.6 * sp
        };

        let tick_len = 0.4 * sp;
        let tick_dir = if stem_dir == "up" { -1.0 } else { 1.0 };
        let line_w = 0.12 * sp;

        // Bracket lines
        emit_line(cmds, x_first, bracket_y, x_last, bracket_y, line_w);
        emit_line(cmds, x_first, bracket_y, x_first, bracket_y + tick_dir * tick_len, line_w);
        emit_line(cmds, x_last, bracket_y, x_last, bracket_y + tick_dir * tick_len, line_w);

        // Number
        let mid_x = (x_first + x_last) / 2.0;
        let num_offset = 0.25 * sp;
        let (num_y, anchor) = if stem_dir == "up" {
            (bracket_y + num_offset, "south")
        } else {
            (bracket_y - num_offset, "north")
        };
        cmds.push(DrawCmd::Text {
            x: mid_x, y: num_y,
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
    let mut groups: Vec<HairpinGroup> = Vec::new();
    let mut cur_indices = Vec::new();
    let mut cur_kind: Option<String> = None;

    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.is_anchor() { continue; }
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
        if hg.indices.is_empty() { continue; }
        let continuation = !hg.starts_here;
        if continuation {
            // Only draw continued hairpins from the first anchor
            // (simplified - the original Typst code checks first_hairpin_anchor)
        }

        let x_first = item_xs[*hg.indices.first().unwrap()];
        let x_last = item_xs[*hg.indices.last().unwrap()];
        let x0 = if continuation { music_start_x } else { x_first + 0.25 * sp };
        let raw_x1 = if hg.ends_here { x_last + 0.95 * sp } else { total_width * sp - 1.0 * sp };
        let x1 = raw_x1.max(x0 + 1.5 * sp);

        // Compute the lowest note y in this group so the hairpin clears any ledger-line notes.
        let mut min_note_y = y_bottom;
        for &idx in &hg.indices {
            let item = &items[idx];
            match &item.event {
                Event::Note(_) => {
                    let ny = y_top + item.y * sp;
                    if ny < min_note_y { min_note_y = ny; }
                }
                Event::Chord(_) => {
                    for &cy in &item.chord_ys {
                        let ny = y_top + cy * sp;
                        if ny < min_note_y { min_note_y = ny; }
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
        emit_line(cmds, x0, y_center + start_h, x1, y_center + end_h, thickness);
        emit_line(cmds, x0, y_center - start_h, x1, y_center - end_h, thickness);
    }
}

fn render_ties_and_slurs(
    cmds: &mut Vec<DrawCmd>,
    items: &[LaidOutItem],
    item_xs: &[f64],
    adj_stem_dirs: &std::collections::HashMap<usize, String>,
    y_top: f64,
    sp: f64,
) {
    let get_stem_dir = |i: usize| -> String {
        adj_stem_dirs.get(&i).cloned()
            .or(items[i].stem_dir.clone())
            .unwrap_or_else(|| "up".to_string())
    };

    // Ties
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.tie() { continue; }

        let mut j = i + 1;
        while j < items.len() {
            if items[j].event.is_note() || items[j].event.is_chord() { break; }
            j += 1;
        }
        if j >= items.len() { continue; }

        let stem_dir = get_stem_dir(i);
        let direction = if stem_dir == "up" { -1.0 } else { 1.0 };

        let nh_smufl = notehead_smufl(ev.duration());
        let nh_w = glyph::advance_width(nh_smufl) * sp;
        let next_nh_w = glyph::advance_width(notehead_smufl(items[j].event.duration())) * sp;

        let start_x = item_xs[i] + nh_w / 2.0 * 0.8;
        let end_x = item_xs[j] - next_nh_w / 2.0 * 0.8;
        let note_y = y_top + item.y * sp;
        let next_note_y = y_top + items[j].y * sp;
        let y_offset = direction * 0.35 * sp;

        render_arc(cmds, start_x, note_y + y_offset, end_x, next_note_y + y_offset, direction, sp, 0.2, 0.25);
    }

    // Slurs
    let mut slur_starts: Vec<usize> = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.is_note() && !ev.is_chord() { continue; }
        if ev.slur_start() {
            slur_starts.push(i);
        }
        if ev.slur_end() && !slur_starts.is_empty() {
            let start_idx = slur_starts.pop().unwrap();
            let stem_dir = get_stem_dir(start_idx);
            let direction = if stem_dir == "up" { -1.0 } else { 1.0 };

            let nh_w = glyph::advance_width(notehead_smufl(items[start_idx].event.duration())) * sp;
            let next_nh_w = glyph::advance_width(notehead_smufl(ev.duration())) * sp;

            let start_x = item_xs[start_idx] + nh_w / 2.0 * 0.8;
            let end_x = item_xs[i] - next_nh_w / 2.0 * 0.8;
            let start_y = y_top + items[start_idx].y * sp + direction * 0.5 * sp;
            let end_y = y_top + item.y * sp + direction * 0.5 * sp;

            render_arc(cmds, start_x, start_y, end_x, end_y, direction, sp, 0.22, 0.3);
        }
    }
}

fn render_arc(cmds: &mut Vec<DrawCmd>, x1: f64, y1: f64, x2: f64, y2: f64, direction: f64, sp: f64, max_thickness: f64, height_factor: f64) {
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
            x1, y1, outer_cp1_x, outer_cp1_y, outer_cp2_x, outer_cp2_y, x2, y2,
            inner_cp2_x, inner_cp2_y, inner_cp1_x, inner_cp1_y,
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
) {
    let trill_cp = 0xE566u32;
    let wiggle_cp = 0xEAA4u32;
    let tr_width = glyph::advance_width("ornamentTrill") * sp;
    let tr_min_y = y_top + 1.15 * sp;

    // Standalone trills
    for (idx, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.trill() || ev.trill_line() { continue; }
        let visual_top = note_visual_top(item, y_top, sp);
        let trill_y = (visual_top + 0.75 * sp).max(tr_min_y);
        emit_glyph(cmds, item_xs[idx] - 0.55 * tr_width, trill_y, "ornamentTrill", trill_cp, sp);
    }

    // Trill line groups
    struct TrillLineGroup {
        indices: Vec<usize>,
        starts_here: bool,
        ends_here: bool,
    }
    let mut trill_groups: Vec<TrillLineGroup> = Vec::new();
    let mut cur_indices = Vec::new();
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

    let tr_gap = 0.45 * sp;  // enough space so "tr" glyph and wiggle line don't collide
    let wiggle_w = glyph::advance_width("wiggleTrill") * sp;

    for tg in &trill_groups {
        if tg.indices.is_empty() { continue; }
        let line_top = tg.indices.iter().map(|&idx| note_visual_top(&items[idx], y_top, sp))
            .fold(f64::NEG_INFINITY, f64::max);
        let trill_y = (line_top + 0.75 * sp).max(tr_min_y);

        if tg.starts_here {
            let symbol_x = item_xs[*tg.indices.first().unwrap()] - 0.55 * tr_width;
            emit_glyph(cmds, symbol_x, trill_y, "ornamentTrill", trill_cp, sp);
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
        }.max(wiggle_start + 0.4 * sp);

        if wiggle_w > 0.0 {
            let step = wiggle_w * 0.92;
            let mut cx = wiggle_start;
            while cx < wiggle_end {
                emit_glyph(cmds, cx, trill_y + 0.02 * sp, "wiggleTrill", wiggle_cp, sp);
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
    let mut groups: Vec<OctGroup> = Vec::new();
    let mut cur_indices = Vec::new();
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
                direction: items[first].event.octave_line_direction().unwrap_or("above").to_string(),
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
            direction: items[first].event.octave_line_direction().unwrap_or("above").to_string(),
            starts_here: items[first].event.octave_line_start(),
            ends_here: items[last].event.octave_line_end(),
        });
    }

    let tuplet_font_size = 7.75 * (sp / 1.75);
    let line_w = 0.12 * sp;

    for og in &groups {
        if og.indices.is_empty() { continue; }
        let x_first = item_xs[*og.indices.first().unwrap()];
        let x_last = item_xs[*og.indices.last().unwrap()];
        let x0 = if og.starts_here { x_first } else { music_start_x };
        let x1 = if og.ends_here { x_last } else { total_width * sp - 1.0 * sp };

        if og.direction == "above" {
            let elem_ys: Vec<f64> = og.indices.iter().map(|&idx| {
                adj_stem_ends.get(&idx).map(|&se| y_top + se * sp)
                    .or(items[idx].stem_y_end.map(|se| y_top + se * sp))
                    .unwrap_or(y_top)
            }).collect();
            let top_y = elem_ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let bracket_y = top_y + 1.6 * sp;
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
                    x: x0 + 0.3 * sp, y: bracket_y + 0.45 * sp,
                    v: og.number.to_string(),
                    s: tuplet_font_size,
                    w: "bold".into(),
                    i: false,
                    a: "south".into(),
                });
                let offset_x = if og.number.to_string().len() > 1 { 1.3 * sp } else { 0.8 * sp };
                cmds.push(DrawCmd::Text {
                    x: x0 + 0.3 * sp + offset_x, y: bracket_y + 0.45 * sp + 0.40 * sp,
                    v: suffix.to_string(),
                    s: 0.55 * tuplet_font_size,
                    w: "bold".into(),
                    i: false,
                    a: "south".into(),
                });
            }
        } else {
            let elem_ys: Vec<f64> = og.indices.iter().map(|&idx| {
                adj_stem_ends.get(&idx).map(|&se| y_top + se * sp)
                    .or(items[idx].stem_y_end.map(|se| y_top + se * sp))
                    .unwrap_or(y_bottom)
            }).collect();
            let bot_y = elem_ys.iter().copied().fold(f64::INFINITY, f64::min);
            let bracket_y = bot_y - 1.6 * sp;
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
                    x: x0 + 0.3 * sp, y: bracket_y - 0.45 * sp,
                    v: og.number.to_string(),
                    s: tuplet_font_size,
                    w: "bold".into(),
                    i: false,
                    a: "north".into(),
                });
                let offset_x = if og.number.to_string().len() > 1 { 1.3 * sp } else { 0.8 * sp };
                cmds.push(DrawCmd::Text {
                    x: x0 + 0.3 * sp + offset_x, y: bracket_y - 0.45 * sp,
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
) {
    struct EndingGroup {
        indices: Vec<usize>,
        label: String,
        starts_here: bool,
        ends_here: bool,
    }
    let mut groups: Vec<EndingGroup> = Vec::new();
    let mut cur_indices = Vec::new();
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

    let opening_barline_x = BARLINE_THICKNESS / 2.0 * sp;
    let final_barline_x = total_width * sp - THICK_BARLINE / 2.0 * sp;
    let line_w = 0.12 * sp;
    let tuplet_font_size = 7.75 * (sp / 1.75);

    for eg in &groups {
        if eg.indices.is_empty() { continue; }
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
                if pb == items.len() - 1 { final_barline_x } else { item_xs[pb] + 0.5 * sp }
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
                if nb == items.len() - 1 { final_barline_x } else { item_xs[nb] + 0.5 * sp }
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
                x: x0 + 0.45 * sp, y: bracket_y - 0.05 * sp,
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
        Event::Note(_) => {
            let note_y = y_top + item.y * sp;
            (note_y + 0.9 * sp).max(note_y + 1.0 * sp)
        }
        Event::Chord(_) => {
            let top_y = item.chord_ys.iter().map(|&vy| y_top + vy * sp)
                .fold(f64::NEG_INFINITY, f64::max);
            (top_y + 0.9 * sp).max(top_y + 1.0 * sp)
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

    // Count lyric lines
    let lyric_line_count = items.iter().fold(lyric_prefix_states.len(), |count, item| {
        count.max(item.event.lyrics().len())
    });
    if lyric_line_count == 0 { return; }

    let lyric_top_y = (y_bottom - 3.1 * sp).min(y_bottom - 0.85 * sp);

    // Simple lyric rendering
    for (idx, item) in items.iter().enumerate() {
        let ev = &item.event;
        if !ev.is_note() && !ev.is_chord() { continue; }

        let lyrics = ev.lyrics();
        let x = item_xs[idx];

        for (li, entry) in lyrics.iter().enumerate() {
            if entry.carry { continue; }
            let top_y = lyric_top_y - li as f64 * lyric_line_step;

            if let Some(ref text) = entry.text {
                if !text.is_empty() {
                    cmds.push(DrawCmd::Text {
                        x, y: top_y,
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
            if !ev.is_note() && !ev.is_chord() { continue; }

            let lyrics = ev.lyrics();
            let x = item_xs[idx];
            let entry = lyrics.get(li);

            if let Some(entry) = entry {
                if entry.carry { continue; }
                if let Some(ref text) = entry.text {
                    if !text.is_empty() {
                        // Draw continuation from previous
                        if let Some(ref cont) = prev_continuation {
                            if let Some(px) = prev_right_x {
                                if cont == "hyphen" {
                                    let mid_x = (px + x) / 2.0;
                                    cmds.push(DrawCmd::Text {
                                        x: mid_x, y: top_y,
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
                                    let ext_end = x - next_half_w - 0.4 * sp;
                                    if ext_end > px {
                                        let ext_y = top_y - 0.92 * sp - 0.2 * sp;
                                        emit_line(cmds, px, ext_y, ext_end, ext_y, 0.09 * sp);
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
