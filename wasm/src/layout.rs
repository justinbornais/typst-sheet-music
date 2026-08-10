use crate::glyph;
use crate::pitch;
use crate::types::*;
use std::collections::BTreeMap;
use std::collections::HashMap;

// ─── Constants (mirrors constants.typ) ─────────────────────────────────

const DEFAULT_NOTE_SPACING_BASE: f64 = 2.5;
const PLAIN_NOTE_SPACING_MULTIPLIER: f64 = 0.86;
const DEFAULT_CLEF_PADDING: f64 = 0.5;
const DEFAULT_KEY_SIG_PADDING: f64 = 1.0;
const DEFAULT_TIME_SIG_PADDING: f64 = 1.25;
const DEFAULT_ACCIDENTAL_PADDING: f64 = 0.35;
const DEFAULT_CHORD_ACCIDENTAL_STACK_PADDING: f64 = 0.22;
const DEFAULT_ACCIDENTAL_CLEARANCE: f64 = 0.16;
const ACCIDENTAL_STACK_VERTICAL_GAP: f64 = 0.04;
const BARLINE_TO_ACCIDENTAL_CLEARANCE: f64 = 0.75;
const TIED_GRACE_TO_ACCIDENTAL_CLEARANCE: f64 = 0.75;
const SHORT_NOTE_ACCIDENTAL_CLEARANCE: f64 = 0.55;
const FLAGGED_NOTE_TO_ACCIDENTAL_CLEARANCE: f64 = 0.24;
const MIN_NOTEHEAD_PAIR_CLEARANCE: f64 = 1.1;
const EMPTY_MEASURE_REST_WIDTH: f64 = 1.8;
const SYSTEM_START_CONTENT_PADDING: f64 = 0.55;
const GRACE_NOTE_SCALE: f64 = 0.68;
const GRACE_STEM_MIN_LENGTH: f64 = 3.0;
const DEFAULT_INLINE_CLEF_SCALE: f64 = 0.8;
const BREATH_MARK_X_OFFSET: f64 = 1.55;
const CAESURA_X_OFFSET: f64 = 1.75;
const RIGHT_STAFF_MARKER_PADDING: f64 = 0.14;

// ─── Utility functions (mirrors utils.typ) ─────────────────────────────

pub fn duration_to_beats(duration: i32, dots: i32) -> f64 {
    let base = match duration {
        DURATION_MAXIMA => 8.0,
        DURATION_LONGA => 4.0,
        DURATION_BREVE => 2.0,
        d if d > 0 => 1.0 / d as f64,
        _ => 1.0 / 4.0,
    };
    let mut total = base;
    let mut dot_value = base;
    for _ in 0..dots {
        dot_value /= 2.0;
        total += dot_value;
    }
    total
}

pub fn duration_spacing_factor(duration: f64, dots: i32) -> f64 {
    let duration = if duration == DURATION_MAXIMA as f64 {
        0.125
    } else if duration == DURATION_LONGA as f64 {
        0.25
    } else if duration == DURATION_BREVE as f64 {
        0.5
    } else if duration > 0.0 {
        duration
    } else {
        4.0
    };
    let base_factor = (4.0_f64 / duration).log2() + 1.0;
    let mut factor = base_factor.max(0.75);
    if dots >= 1 {
        factor *= 1.15;
    }
    if dots >= 2 {
        factor *= 1.1;
    }
    factor
}

// ─── SMuFL name mappings ───────────────────────────────────────────────

fn clef_smufl_name(clef: &str) -> &'static str {
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

// ─── Width calculation functions ───────────────────────────────────────

pub fn clef_advance_sp(clef: &str, sp: f64) -> f64 {
    clef_advance_sp_font(clef, sp, glyph::FontId::Bravura)
}

pub fn clef_advance_sp_font(clef: &str, sp: f64, font: glyph::FontId) -> f64 {
    let smufl = clef_smufl_name(clef);
    glyph::advance_width_for(font, smufl) * sp + DEFAULT_CLEF_PADDING * sp
}

pub fn key_sig_advance_sp(key: &str, sp: f64) -> f64 {
    key_sig_advance_sp_font(key, sp, glyph::FontId::Bravura)
}

pub fn key_sig_advance_sp_font(key: &str, sp: f64, font: glyph::FontId) -> f64 {
    let count = pitch::key_sig_accidental_count(key);
    let n = count.unsigned_abs() as usize;
    if n == 0 {
        return 0.0;
    }
    let acc_smufl = if count > 0 {
        "accidentalSharp"
    } else {
        "accidentalFlat"
    };
    let acc_w = glyph::advance_width_for(font, acc_smufl);
    n as f64 * (acc_w + 0.2) * sp + DEFAULT_KEY_SIG_PADDING * sp
}

pub fn time_sig_advance_sp(upper: i32, lower: i32, symbol: Option<&str>, sp: f64) -> f64 {
    time_sig_advance_sp_font(upper, lower, symbol, sp, glyph::FontId::Bravura)
}

pub fn time_sig_advance_sp_font(
    upper: i32,
    lower: i32,
    symbol: Option<&str>,
    sp: f64,
    font: glyph::FontId,
) -> f64 {
    match symbol {
        Some("common") => {
            glyph::advance_width_for(font, "timeSigCommon") * sp + DEFAULT_TIME_SIG_PADDING * sp
        }
        Some("cut") => {
            glyph::advance_width_for(font, "timeSigCutCommon") * sp + DEFAULT_TIME_SIG_PADDING * sp
        }
        _ => {
            let upper_s = upper.to_string();
            let lower_s = lower.to_string();
            let upper_w: f64 = upper_s
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| {
                    let name = format!("timeSig{}", c);
                    glyph::advance_width_for(font, &name) * sp
                })
                .sum();
            let lower_w: f64 = lower_s
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| {
                    let name = format!("timeSig{}", c);
                    glyph::advance_width_for(font, &name) * sp
                })
                .sum();
            upper_w.max(lower_w) + DEFAULT_TIME_SIG_PADDING * sp
        }
    }
}

fn inline_time_sig_width(
    event: &Event,
    prev: Option<&Event>,
    next: Option<&Event>,
    font: glyph::FontId,
) -> f64 {
    if let Event::TimeSig(t) = event {
        let glyph_w = (time_sig_advance_sp_font(t.upper, t.lower, t.symbol.as_deref(), 1.0, font)
            - DEFAULT_TIME_SIG_PADDING)
            .max(0.0);
        let extra = if prev.map_or(false, |p| p.is_barline()) {
            0.18
        } else if next.map_or(false, |n| n.is_barline()) {
            0.0
        } else {
            0.12
        };
        glyph_w + extra
    } else {
        0.0
    }
}

fn notehead_width(duration: i32, font: glyph::FontId) -> f64 {
    let smufl = match duration {
        DURATION_MAXIMA => "mensuralWhiteMaxima",
        DURATION_LONGA => "mensuralWhiteLonga",
        DURATION_BREVE => "noteheadDoubleWhole",
        1 => "noteheadWhole",
        2 => "noteheadHalf",
        _ => "noteheadBlack",
    };
    glyph::advance_width_for(font, smufl)
}

fn rest_smufl_name(duration: i32) -> &'static str {
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

fn accidental_smufl(acc: Option<&str>) -> Option<&'static str> {
    match acc {
        Some("sharp") => Some("accidentalSharp"),
        Some("flat") => Some("accidentalFlat"),
        Some("natural") => Some("accidentalNatural"),
        Some("double-sharp") => Some("accidentalDoubleSharp"),
        Some("double-flat") => Some("accidentalDoubleFlat"),
        _ => None,
    }
}

fn accidental_width(acc: Option<&str>, font: glyph::FontId) -> f64 {
    accidental_smufl(acc).map_or(0.0, |name| glyph::advance_width_for(font, name))
}

fn accidental_vertical_span(acc: Option<&str>, font: glyph::FontId) -> Option<(f64, f64)> {
    let name = accidental_smufl(acc)?;
    let (mut bottom, mut top) = glyph::bbox_for(font, name)
        .map(|bbox| (bbox.sw_y, bbox.ne_y))
        .unwrap_or((-1.3, 1.3));

    match acc {
        Some("flat") => {
            bottom += 0.18;
            top -= 0.32;
        }
        Some("double-flat") => {
            bottom += 0.14;
            top -= 0.24;
        }
        _ => {}
    }

    Some((bottom, top))
}

fn alternating_accidental_order(sorted_indices: &[usize]) -> Vec<usize> {
    let mut order = Vec::with_capacity(sorted_indices.len());
    if sorted_indices.is_empty() {
        return order;
    }

    let mut top = 0usize;
    let mut bottom = sorted_indices.len() - 1;
    while top <= bottom {
        order.push(sorted_indices[top]);
        if top == bottom {
            break;
        }
        order.push(sorted_indices[bottom]);
        top += 1;
        bottom = bottom.saturating_sub(1);
    }

    order
}

fn accidental_ranges_overlap(a_bottom: f64, a_top: f64, b_bottom: f64, b_top: f64) -> bool {
    a_bottom < b_top + ACCIDENTAL_STACK_VERTICAL_GAP
        && b_bottom < a_top + ACCIDENTAL_STACK_VERTICAL_GAP
}

pub fn chord_accidental_lanes(
    positions: &[i32],
    accidentals: &[Option<&str>],
    font: glyph::FontId,
) -> (Vec<Option<usize>>, Vec<f64>) {
    let mut note_lanes = vec![None; accidentals.len()];
    let mut lane_widths: Vec<f64> = Vec::new();
    let mut lane_spans: Vec<Vec<(f64, f64)>> = Vec::new();

    let mut accidental_indices: Vec<usize> = accidentals
        .iter()
        .enumerate()
        .filter_map(|(idx, acc)| acc.map(|_| idx))
        .collect();
    accidental_indices.sort_by_key(|&idx| positions.get(idx).copied().unwrap_or_default());

    for note_idx in alternating_accidental_order(&accidental_indices) {
        let Some(accidental) = accidentals.get(note_idx).copied().flatten() else {
            continue;
        };
        let Some((span_bottom, span_top)) = accidental_vertical_span(Some(accidental), font) else {
            continue;
        };
        let center_y = -(positions.get(note_idx).copied().unwrap_or_default() as f64) / 2.0;
        let note_bottom = center_y + span_bottom;
        let note_top = center_y + span_top;
        let width = accidental_width(Some(accidental), font);

        let lane = lane_spans
            .iter()
            .position(|lane| {
                lane.iter().all(|&(existing_bottom, existing_top)| {
                    !accidental_ranges_overlap(note_bottom, note_top, existing_bottom, existing_top)
                })
            })
            .unwrap_or_else(|| {
                lane_spans.push(Vec::new());
                lane_widths.push(0.0);
                lane_spans.len() - 1
            });

        lane_spans[lane].push((note_bottom, note_top));
        lane_widths[lane] = lane_widths[lane].max(width);
        note_lanes[note_idx] = Some(lane);
    }

    (note_lanes, lane_widths)
}

fn event_has_accidental(event: &Event) -> bool {
    match event {
        Event::Note(n) => n.accidental.is_some(),
        Event::Chord(c) => c.notes.iter().any(|n| n.accidental.is_some()),
        _ => false,
    }
}

fn event_is_note_cluster(event: &Event) -> bool {
    matches!(event, Event::Note(_) | Event::Chord(_))
}

fn event_note_diatonics(event: &Event) -> Vec<i32> {
    match event {
        Event::Note(n) => vec![pitch::pitch_to_diatonic(&n.name, n.octave)],
        Event::Chord(c) => c
            .notes
            .iter()
            .map(|n| pitch::pitch_to_diatonic(&n.name, n.octave))
            .collect(),
        _ => Vec::new(),
    }
}

fn event_accidental_diatonics(event: &Event) -> Vec<i32> {
    match event {
        Event::Note(n) if n.accidental.is_some() => {
            vec![pitch::pitch_to_diatonic(&n.name, n.octave)]
        }
        Event::Chord(c) => c
            .notes
            .iter()
            .filter(|n| n.accidental.is_some())
            .map(|n| pitch::pitch_to_diatonic(&n.name, n.octave))
            .collect(),
        _ => Vec::new(),
    }
}

fn note_cluster_needs_accidental_space(event: &Event, next: &Event) -> bool {
    let event_diatonics = event_note_diatonics(event);
    let next_accidental_diatonics = event_accidental_diatonics(next);
    event_diatonics
        .iter()
        .any(|d| next_accidental_diatonics.contains(d))
        || note_cluster_stem_needs_accidental_space(event, next)
}

fn note_cluster_stem_needs_accidental_space(event: &Event, next: &Event) -> bool {
    if event.duration() < 2 {
        return false;
    }
    if next.duration() <= event.duration() {
        return false;
    }
    let event_diatonics = event_note_diatonics(event);
    let next_accidental_diatonics = event_accidental_diatonics(next);
    event_diatonics.iter().any(|event_d| {
        next_accidental_diatonics.iter().any(|next_d| {
            let interval = next_d - event_d;
            (2..=7).contains(&interval)
        })
    })
}

fn flagged_note_before_longer_accidental(event: &Event, next: &Event) -> bool {
    event.duration() >= 8
        && next.duration() > 0
        && event.duration() > next.duration()
        && !event.grace()
        && !next.grace()
        && event_is_note_cluster(event)
        && event_is_note_cluster(next)
        && event_has_accidental(next)
}

fn is_short_gap_neighbor(event: &Event) -> bool {
    event_is_note_cluster(event) && !event.grace() && event.duration() >= 8
}

fn gap_extra_space_units(gap: &Gap, prev: Option<&Event>, next: Option<&Event>) -> i32 {
    let mut amount = gap.amount;
    if amount > 0
        && prev.map_or(false, is_short_gap_neighbor)
        && next.map_or(false, is_short_gap_neighbor)
    {
        amount -= 1;
    }
    amount.max(0)
}

fn needs_leading_accidental_space(event: &Event, next: &Event) -> bool {
    if !event_has_accidental(next) {
        return false;
    }
    match event {
        Event::Barline(_) | Event::Rest(_) => true,
        _ if event.grace() && event.tie() => true,
        _ if event_is_note_cluster(event) => note_cluster_needs_accidental_space(event, next),
        _ => false,
    }
}

fn plain_note_pair(event: &Event, next: Option<&Event>) -> bool {
    let next = match next {
        Some(n) => n,
        None => return false,
    };
    event_is_note_cluster(event)
        && event_is_note_cluster(next)
        && !event.grace()
        && !next.grace()
        && event.dots() == 0
        && next.dots() == 0
        && !needs_leading_accidental_space(event, next)
}

fn notehead_half_width(event: &Event, font: glyph::FontId) -> f64 {
    match event {
        Event::Note(n) => notehead_width(n.duration, font) / 2.0,
        Event::Chord(c) => notehead_width(c.duration, font) / 2.0,
        _ => 0.0,
    }
}

fn event_right_collision_extent(event: &Event, font: glyph::FontId) -> f64 {
    match event {
        Event::Note(_) | Event::Chord(_) => notehead_half_width(event, font),
        Event::Rest(r) => {
            let smufl = rest_smufl_name(r.duration);
            glyph::bbox_for(font, smufl).map_or(0.45, |b| b.ne_x.max(0.45))
        }
        Event::Barline(_) => 0.5 + BARLINE_TO_ACCIDENTAL_CLEARANCE,
        _ => 0.0,
    }
}

fn minimum_note_pair_spacing(event: &Event, next: Option<&Event>, font: glyph::FontId) -> f64 {
    let next = match next {
        Some(next) => next,
        None => return 0.0,
    };
    if !event_is_note_cluster(event) || !event_is_note_cluster(next) {
        return 0.0;
    }
    if event.duration() < 32 && next.duration() < 32 {
        return 0.0;
    }

    let event_scale = if event.grace() { GRACE_NOTE_SCALE } else { 1.0 };
    let next_scale = if next.grace() { GRACE_NOTE_SCALE } else { 1.0 };

    event_right_collision_extent(event, font) * event_scale
        + notehead_half_width(next, font) * next_scale
        + MIN_NOTEHEAD_PAIR_CLEARANCE * event_scale.max(next_scale)
}

fn pre_accidental_clearance(event: &Event) -> f64 {
    let mut clearance = 0.0;
    if event.grace() && event.tie() {
        clearance += TIED_GRACE_TO_ACCIDENTAL_CLEARANCE;
    }
    clearance
}

fn flagged_note_accidental_extra(event: &Event, next: Option<&Event>) -> f64 {
    if next.map_or(false, |next| {
        flagged_note_before_longer_accidental(event, next)
    }) {
        FLAGGED_NOTE_TO_ACCIDENTAL_CLEARANCE
    } else {
        0.0
    }
}

fn accidental_readability_clearance(event: &Event, next: &Event) -> f64 {
    if next.grace() {
        return 0.0;
    }
    let dense_pair = event_is_note_cluster(event)
        && event_is_note_cluster(next)
        && (event.duration() >= 8 || next.duration() >= 8);
    if dense_pair {
        SHORT_NOTE_ACCIDENTAL_CLEARANCE
    } else {
        0.0
    }
}

fn is_empty_measure_whole_rest(event: &Event, prev: Option<&Event>, next: Option<&Event>) -> bool {
    matches!(event, Event::Rest(r) if r.duration == 1 && r.dots == 0)
        && prev.map_or(true, |p| p.is_barline())
        && next.map_or(true, |n| n.is_barline())
}

fn required_leading_accidental_space(
    event: &Event,
    next: Option<&Event>,
    font: glyph::FontId,
) -> f64 {
    let next = match next {
        Some(n) => n,
        None => return 0.0,
    };
    if !needs_leading_accidental_space(event, next) {
        return 0.0;
    }
    let next_is_grace = next.grace();
    let scale = if next_is_grace { 0.68 } else { 1.0 };
    let event_scale = if event.grace() { 0.68 } else { 1.0 };
    let event_right_extent =
        event_right_collision_extent(event, font) * event_scale + pre_accidental_clearance(event);
    let cluster_factor = if next_is_grace && (next.is_note() || next.is_chord()) {
        0.55
    } else {
        1.0
    };
    match next {
        Event::Note(n) => {
            if n.accidental.is_some() {
                event_right_extent
                    + (accidental_width(n.accidental.as_deref(), font)
                        + DEFAULT_ACCIDENTAL_PADDING
                        + notehead_half_width(next, font)
                        + DEFAULT_ACCIDENTAL_CLEARANCE
                        + accidental_readability_clearance(event, next))
                        * scale
                        * cluster_factor
            } else {
                0.0
            }
        }
        Event::Chord(c) => {
            let diatonic_positions: Vec<i32> = c
                .notes
                .iter()
                .map(|n| pitch::pitch_to_diatonic(&n.name, n.octave))
                .collect();
            let accidental_specs: Vec<Option<&str>> =
                c.notes.iter().map(|n| n.accidental.as_deref()).collect();
            let (_, lane_widths) =
                chord_accidental_lanes(&diatonic_positions, &accidental_specs, font);
            let stack_width = if lane_widths.is_empty() {
                0.0
            } else {
                lane_widths.iter().sum::<f64>()
                    + DEFAULT_ACCIDENTAL_PADDING
                    + DEFAULT_CHORD_ACCIDENTAL_STACK_PADDING
                        * (lane_widths.len().saturating_sub(1) as f64)
            };
            if stack_width > 0.0 {
                event_right_extent
                    + (stack_width
                        + notehead_half_width(next, font)
                        + DEFAULT_ACCIDENTAL_CLEARANCE
                        + accidental_readability_clearance(event, next))
                        * scale
                        * cluster_factor
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

fn leading_accidental_extra(
    event: &Event,
    available_space: f64,
    next: Option<&Event>,
    font: glyph::FontId,
) -> f64 {
    let required = required_leading_accidental_space(event, next, font);
    if required <= 0.0 {
        return 0.0;
    }
    (required - available_space).max(0.0)
}

fn grace_body_width(
    event: &Event,
    prev: Option<&Event>,
    next: Option<&Event>,
    font: glyph::FontId,
) -> f64 {
    let duration = event.duration();
    let head_w = notehead_width(duration, font);
    let rest_w = if event.is_rest() {
        let smufl = rest_smufl_name(duration);
        glyph::advance_width_for(font, smufl)
    } else {
        0.0
    };
    let inter_note_gap = if next.map_or(false, |n| !n.grace()) {
        0.04
    } else if prev.map_or(false, |p| p.grace()) {
        0.08
    } else {
        0.12
    };
    head_w.max(rest_w) * GRACE_NOTE_SCALE + inter_note_gap
}

fn right_staff_marker_trailing_extra(
    event: &Event,
    available_space: f64,
    font: glyph::FontId,
) -> f64 {
    let required = event
        .staff_markers()
        .iter()
        .filter_map(|marker| match marker.as_str() {
            "breath-mark" => Some(
                BREATH_MARK_X_OFFSET
                    + glyph::advance_width_for(font, "breathMarkComma")
                    + RIGHT_STAFF_MARKER_PADDING,
            ),
            "caesura" => Some(
                CAESURA_X_OFFSET
                    + glyph::advance_width_for(font, "caesura")
                    + RIGHT_STAFF_MARKER_PADDING,
            ),
            _ => None,
        })
        .fold(0.0_f64, f64::max);
    (required - available_space).max(0.0)
}

pub fn event_width(event: &Event, prev: Option<&Event>, next: Option<&Event>) -> f64 {
    event_width_font(event, prev, next, glyph::FontId::Bravura)
}

pub fn event_width_font(
    event: &Event,
    prev: Option<&Event>,
    next: Option<&Event>,
    font: glyph::FontId,
) -> f64 {
    match event {
        Event::Barline(_) => {
            let touches_inline_boundary = prev
                .map_or(false, |p| matches!(p, Event::Clef(_) | Event::TimeSig(_)))
                || next.map_or(false, |n| matches!(n, Event::Clef(_) | Event::TimeSig(_)));
            let w = if touches_inline_boundary { 0.6 } else { 2.5 };
            w + leading_accidental_extra(event, w, next, font)
        }
        Event::Clef(c) => {
            let smufl = clef_smufl_name(&c.clef);
            glyph::advance_width_for(font, smufl) * DEFAULT_INLINE_CLEF_SCALE + DEFAULT_CLEF_PADDING
        }
        Event::TimeSig(_) => inline_time_sig_width(event, prev, next, font),
        Event::KeySig(_) => 2.0,
        Event::Gap(g) => 0.7 * gap_extra_space_units(g, prev, next) as f64,
        Event::LineBreak => 0.0,
        Event::VoiceGroup(vg) => voice_group_width_font(vg, font),
        Event::Rest(_) if is_empty_measure_whole_rest(event, prev, next) => {
            EMPTY_MEASURE_REST_WIDTH
        }
        _ => {
            // Notes, rests, spacers, chords
            if event.grace() {
                let body = grace_body_width(event, prev, next, font);
                return body + leading_accidental_extra(event, body, next, font);
            }
            let dur = event.duration();
            let dots = event.dots();
            let factor = duration_spacing_factor(dur as f64, dots);
            let mut w = DEFAULT_NOTE_SPACING_BASE * factor;

            if let Some(scale) = tuplet_duration_scale(event) {
                w *= scale;
            }
            if plain_note_pair(event, next) {
                w *= PLAIN_NOTE_SPACING_MULTIPLIER;
            }
            w = w.max(minimum_note_pair_spacing(event, next, font));
            w + leading_accidental_extra(event, w, next, font)
                + flagged_note_accidental_extra(event, next)
                + right_staff_marker_trailing_extra(event, w, font)
        }
    }
}

fn tuplet_duration_scale(event: &Event) -> Option<f64> {
    let in_time_of = event.tuplet_beats();
    if in_time_of <= 0.0 {
        return None;
    }
    let written_count = if event.tuplet_number() > 0 {
        event.tuplet_number()
    } else {
        event.tuplet_count()
    };
    if written_count > 0 {
        Some(in_time_of / written_count as f64)
    } else {
        None
    }
}

fn event_advance_beats(event: &Event) -> f64 {
    match event {
        Event::VoiceGroup(vg) => voice_group_duration_beats(vg),
        _ if is_rhythmic_event(event) => {
            let mut dur_beats = duration_to_beats(event.duration(), event.dots());
            if let Some(scale) = tuplet_duration_scale(event) {
                dur_beats *= scale;
            }
            dur_beats
        }
        _ => 0.0,
    }
}

fn voice_sequence_duration_beats(events: &[Event]) -> f64 {
    events.iter().map(event_advance_beats).sum()
}

fn voice_group_duration_beats(vg: &VoiceGroup) -> f64 {
    voice_sequence_duration_beats(&vg.upper).max(voice_sequence_duration_beats(&vg.lower))
}

fn placeholder_staff_for_voice(events: &[Event], font: glyph::FontId) -> LaidOutStaff {
    let positions = compute_event_positions_font(events, font);
    let items = events
        .iter()
        .zip(positions.iter())
        .map(|(event, pos)| LaidOutItem {
            event: event.clone(),
            x: pos.x,
            y: 0.0,
            stem_dir: None,
            stem_y_end: None,
            stem_forced: false,
            voice: None,
            width: pos.width,
            chord_ys: Vec::new(),
            chord_staff_positions: Vec::new(),
            voice_items: Vec::new(),
        })
        .collect::<Vec<_>>();
    let total_width = positions
        .last()
        .map(|pos| pos.x + pos.width)
        .unwrap_or(SYSTEM_START_CONTENT_PADDING);
    LaidOutStaff {
        items,
        total_width,
        clef: None,
        time: None,
        show_time_prefix: false,
        lyric_prefix_states: Vec::new(),
    }
}

fn voice_group_width_font(vg: &VoiceGroup, font: glyph::FontId) -> f64 {
    let upper = placeholder_staff_for_voice(&vg.upper, font);
    let lower = placeholder_staff_for_voice(&vg.lower, font);
    let aligned = align_staves_by_beat(&[upper, lower]);
    aligned
        .first()
        .map(|staff| staff.total_width)
        .unwrap_or(SYSTEM_START_CONTENT_PADDING)
}

// ─── Event positions ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PosInfo {
    pub x: f64,
    pub width: f64,
}

pub fn compute_event_positions(events: &[Event]) -> Vec<PosInfo> {
    compute_event_positions_font(events, glyph::FontId::Bravura)
}

pub fn compute_event_positions_font(events: &[Event], font: glyph::FontId) -> Vec<PosInfo> {
    let mut positions = Vec::with_capacity(events.len());
    let mut x = SYSTEM_START_CONTENT_PADDING;
    for (i, event) in events.iter().enumerate() {
        let prev = if i > 0 { Some(&events[i - 1]) } else { None };
        let next = events.get(i + 1);
        let w = event_width_font(event, prev, next, font);
        positions.push(PosInfo { x, width: w });
        x += w;
    }
    positions
}

// ─── System breaks ─────────────────────────────────────────────────────

pub fn has_line_breaks(events: &[Event]) -> bool {
    events.iter().any(|e| matches!(e, Event::LineBreak))
}

pub fn split_at_line_breaks(events: &[Event]) -> Vec<Vec<Event>> {
    let mut systems = Vec::new();
    let mut current = Vec::new();
    for event in events {
        if matches!(event, Event::LineBreak) {
            if !current.is_empty() {
                systems.push(current);
                current = Vec::new();
            }
        } else {
            current.push(event.clone());
        }
    }
    if !current.is_empty() {
        systems.push(current);
    }
    systems
}

fn split_into_measures(events: &[Event]) -> Vec<Vec<Event>> {
    let mut measures = Vec::new();
    let mut current = Vec::new();
    for event in events {
        current.push(event.clone());
        if event.is_barline() {
            measures.push(current);
            current = Vec::new();
        }
    }
    if !current.is_empty() {
        measures.push(current);
    }
    measures
}

fn measure_width(events: &[Event]) -> f64 {
    let mut w = 0.0;
    for (i, ev) in events.iter().enumerate() {
        let is_rhythmic = ev.is_note()
            || ev.is_rest()
            || matches!(ev, Event::Spacer(_))
            || ev.is_chord()
            || ev.is_voice_group()
            || ev.is_barline();
        if is_rhythmic {
            let prev = if i > 0 { Some(&events[i - 1]) } else { None };
            let next = events.get(i + 1);
            w += event_width(ev, prev, next);
        }
    }
    w
}

pub fn compute_system_breaks(
    events: &[Event],
    available_width: Option<f64>,
    measures_per_line: Option<i32>,
) -> Vec<Vec<Event>> {
    let measures = split_into_measures(events);
    if measures.is_empty() {
        return vec![vec![]];
    }

    // Fixed measures-per-line mode
    if let Some(mpl) = measures_per_line {
        if mpl > 0 {
            let mut systems = Vec::new();
            let mut current_events = Vec::new();
            let mut measure_count = 0;
            for measure in &measures {
                current_events.extend(measure.iter().cloned());
                measure_count += 1;
                if measure_count >= mpl {
                    systems.push(current_events);
                    current_events = Vec::new();
                    measure_count = 0;
                }
            }
            if !current_events.is_empty() {
                systems.push(current_events);
            }
            return systems;
        }
    }

    // Width-based breaking
    let aw = match available_width {
        Some(w) if w > 0.0 => w,
        _ => return vec![events.to_vec()],
    };

    let mut systems = Vec::new();
    let mut current_events = Vec::new();
    let mut current_width = 0.0;

    for measure in &measures {
        let mw = measure_width(measure);
        if !current_events.is_empty() && current_width + mw > aw {
            systems.push(current_events);
            current_events = Vec::new();
            current_width = 0.0;
        }
        current_events.extend(measure.iter().cloned());
        current_width += mw;
    }
    if !current_events.is_empty() {
        systems.push(current_events);
    }
    systems
}

pub fn mirror_breaks(events: &[Event], measure_counts: &[usize]) -> Vec<Vec<Event>> {
    let mut mirrored = Vec::new();
    let mut remaining = events.to_vec();
    for (mc_idx, &mc) in measure_counts.iter().enumerate() {
        let is_last = mc_idx == measure_counts.len() - 1;
        let mut seg = Vec::new();
        let mut bars_seen = 0;
        let mut j = 0;
        while j < remaining.len() && (is_last || bars_seen < mc) {
            seg.push(remaining[j].clone());
            if remaining[j].is_barline() {
                bars_seen += 1;
            }
            j += 1;
        }
        if mc == 0 && !is_last && !remaining.is_empty() && matches!(remaining[0], Event::LineBreak)
        {
            seg.push(remaining[0].clone());
            j = 1;
        } else if is_last {
            j = remaining.len();
        }
        mirrored.push(seg);
        remaining = remaining[j..].to_vec();
    }
    if !remaining.is_empty() {
        mirrored.push(remaining);
    }
    mirrored
}

// ─── Staff layout ──────────────────────────────────────────────────────

fn clef_id(clef: &str) -> u32 {
    let b = clef.as_bytes();
    let mut v = 0u32;
    for (i, &byte) in b.iter().take(4).enumerate() {
        v |= (byte as u32) << (i * 8);
    }
    v
}

fn layout_note_geometry(
    n: &Note,
    current_clef: &str,
    cur_clef_id: u32,
    sp_cache: &mut HashMap<(u8, i32, u32), i32>,
    forced_stem_dir: Option<&str>,
) -> (f64, Option<String>, Option<f64>) {
    let cache_key = (n.name.as_bytes()[0], n.octave, cur_clef_id);
    let sp = *sp_cache
        .entry(cache_key)
        .or_insert_with(|| pitch::staff_position(&n.name, n.octave, current_clef));
    let y = -sp as f64 / 2.0;
    let sd = forced_stem_dir.unwrap_or_else(|| {
        if n.grace {
            "up"
        } else {
            pitch::auto_stem_direction(sp)
        }
    });
    let stem_scale = if n.grace { GRACE_NOTE_SCALE } else { 1.0 };
    let stem_min = if n.grace { GRACE_STEM_MIN_LENGTH } else { 3.5 };
    let stem_y_end = pitch::compute_stem_end_y(y, sp, sd, stem_scale, stem_min);
    (y, Some(sd.to_string()), Some(stem_y_end))
}

fn layout_chord_geometry(
    c: &Chord,
    current_clef: &str,
    cur_clef_id: u32,
    sp_cache: &mut HashMap<(u8, i32, u32), i32>,
    forced_stem_dir: Option<&str>,
) -> (f64, Option<String>, Option<f64>, Vec<f64>, Vec<i32>) {
    let mut sp_list = Vec::with_capacity(c.notes.len());
    for cn in &c.notes {
        let cache_key = (cn.name.as_bytes()[0], cn.octave, cur_clef_id);
        let spos = *sp_cache
            .entry(cache_key)
            .or_insert_with(|| pitch::staff_position(&cn.name, cn.octave, current_clef));
        sp_list.push(spos);
    }
    let y_list: Vec<f64> = sp_list.iter().map(|&spos| -spos as f64 / 2.0).collect();
    let avg_sp = sp_list.iter().sum::<i32>() as f64 / sp_list.len() as f64;
    let sd = forced_stem_dir.unwrap_or_else(|| {
        if c.grace {
            "up"
        } else {
            pitch::auto_stem_direction(avg_sp as i32)
        }
    });
    let primary_sp = if sd == "up" {
        *sp_list.iter().max().unwrap()
    } else {
        *sp_list.iter().min().unwrap()
    };
    let y = -primary_sp as f64 / 2.0;
    let tip_sp = if sd == "up" {
        *sp_list.iter().min().unwrap()
    } else {
        *sp_list.iter().max().unwrap()
    };
    let tip_y = -tip_sp as f64 / 2.0;
    let stem_scale = if c.grace { GRACE_NOTE_SCALE } else { 1.0 };
    let stem_min = if c.grace { GRACE_STEM_MIN_LENGTH } else { 3.5 };
    let stem_y_end = pitch::compute_stem_end_y(tip_y, tip_sp, sd, stem_scale, stem_min);
    (y, Some(sd.to_string()), Some(stem_y_end), y_list, sp_list)
}

fn layout_voice_group_items(
    vg: &VoiceGroup,
    current_clef: &str,
    font: glyph::FontId,
) -> Vec<LaidOutItem> {
    let (mut upper_items, upper_width) =
        layout_event_sequence_font(&vg.upper, current_clef, font, Some("up"), Some(1));
    let (mut lower_items, lower_width) =
        layout_event_sequence_font(&vg.lower, current_clef, font, Some("down"), Some(2));
    adjust_voice_rests(&mut upper_items, &vg.upper, &vg.lower, 2.6);
    adjust_voice_rests(&mut lower_items, &vg.lower, &vg.upper, -4.0);

    let upper = LaidOutStaff {
        items: upper_items,
        total_width: upper_width,
        clef: Some(current_clef.to_string()),
        time: None,
        show_time_prefix: false,
        lyric_prefix_states: Vec::new(),
    };
    let lower = LaidOutStaff {
        items: lower_items,
        total_width: lower_width,
        clef: Some(current_clef.to_string()),
        time: None,
        show_time_prefix: false,
        lyric_prefix_states: Vec::new(),
    };

    let aligned = align_staves_by_beat(&[upper, lower]);
    let mut voice_items = Vec::new();
    for staff in aligned {
        for mut item in staff.items {
            item.x -= SYSTEM_START_CONTENT_PADDING;
            voice_items.push(item);
        }
    }
    voice_items
}

fn rest_y_position(duration: i32) -> f64 {
    match duration {
        1 => -1.0,
        DURATION_MAXIMA | DURATION_LONGA | DURATION_BREVE => -2.0,
        _ => -2.0,
    }
}

fn rhythmic_spans(events: &[Event]) -> Vec<Option<(f64, f64)>> {
    let mut beat = 0.0;
    let mut spans = Vec::with_capacity(events.len());
    for event in events {
        if is_rhythmic_event(event) {
            let start = beat;
            beat += event_advance_beats(event);
            spans.push(Some((start, beat)));
        } else {
            spans.push(None);
        }
    }
    spans
}

fn has_opposing_voice_activity(span: (f64, f64), opposing_spans: &[Option<(f64, f64)>]) -> bool {
    const TOUCH_EPSILON: f64 = 0.000001;
    let (start, end) = span;
    opposing_spans.iter().any(|opposing| {
        let Some((other_start, other_end)) = opposing else {
            return false;
        };
        *other_end >= start - TOUCH_EPSILON && *other_start <= end + TOUCH_EPSILON
    })
}

fn adjust_voice_rests(
    items: &mut [LaidOutItem],
    voice_events: &[Event],
    opposing_events: &[Event],
    separated_y: f64,
) {
    let voice_spans = rhythmic_spans(voice_events);
    let opposing_spans = rhythmic_spans(opposing_events);
    for (idx, item) in items.iter_mut().enumerate() {
        if !matches!(item.event, Event::Rest(_)) {
            continue;
        }
        if let Some(span) = voice_spans.get(idx).and_then(|span| *span) {
            if has_opposing_voice_activity(span, &opposing_spans) {
                item.y = separated_y;
            }
        }
    }
}

fn layout_event_sequence_font(
    events: &[Event],
    clef: &str,
    font: glyph::FontId,
    forced_stem_dir: Option<&str>,
    voice: Option<i32>,
) -> (Vec<LaidOutItem>, f64) {
    let positions = compute_event_positions_font(events, font);
    let mut items = Vec::with_capacity(events.len());
    let mut current_clef = clef.to_string();
    let mut sp_cache: HashMap<(u8, i32, u32), i32> = HashMap::new();
    let mut cur_clef_id = clef_id(&current_clef);

    for (i, event) in events.iter().enumerate() {
        let pos_info = &positions[i];
        let x = pos_info.x;
        let mut y = 0.0;
        let mut stem_dir = None;
        let mut stem_y_end = None;
        let mut chord_ys = Vec::new();
        let mut chord_staff_positions = Vec::new();
        let mut voice_items = Vec::new();

        match event {
            Event::Note(n) => {
                let geo = layout_note_geometry(
                    n,
                    &current_clef,
                    cur_clef_id,
                    &mut sp_cache,
                    forced_stem_dir,
                );
                y = geo.0;
                stem_dir = geo.1;
                stem_y_end = geo.2;
            }
            Event::Chord(c) => {
                let geo = layout_chord_geometry(
                    c,
                    &current_clef,
                    cur_clef_id,
                    &mut sp_cache,
                    forced_stem_dir,
                );
                y = geo.0;
                stem_dir = geo.1;
                stem_y_end = geo.2;
                chord_ys = geo.3;
                chord_staff_positions = geo.4;
            }
            Event::VoiceGroup(vg) => {
                voice_items = layout_voice_group_items(vg, &current_clef, font);
            }
            Event::Rest(r) => {
                y = rest_y_position(r.duration);
            }
            Event::Clef(c) => {
                current_clef = c.clef.clone();
                cur_clef_id = clef_id(&current_clef);
            }
            _ => {}
        }

        items.push(LaidOutItem {
            event: event.clone(),
            x,
            y,
            stem_dir,
            stem_y_end,
            stem_forced: forced_stem_dir.is_some() && event.is_anchor(),
            voice,
            width: pos_info.width,
            chord_ys,
            chord_staff_positions,
            voice_items,
        });
    }

    let total_width = positions.last().map(|pos| pos.x + pos.width).unwrap_or(0.0);
    (items, total_width)
}

pub fn layout_staff(
    events: &[Event],
    clef: Option<&str>,
    time: Option<&TimeInfo>,
    show_time_prefix: bool,
    lyric_prefix_states: &[Option<String>],
) -> LaidOutStaff {
    layout_staff_font(
        events,
        clef,
        time,
        show_time_prefix,
        lyric_prefix_states,
        glyph::FontId::Bravura,
    )
}

pub fn layout_staff_font(
    events: &[Event],
    clef: Option<&str>,
    time: Option<&TimeInfo>,
    show_time_prefix: bool,
    lyric_prefix_states: &[Option<String>],
    font: glyph::FontId,
) -> LaidOutStaff {
    let positions = compute_event_positions_font(events, font);
    let mut items = Vec::with_capacity(events.len());
    let layout_clef = clef.unwrap_or("treble");
    let mut current_clef = layout_clef.to_string();
    // Cache key: (note_name_byte, octave, clef_id) — avoids format!/String allocation.
    // clef_id is a compact hash of the clef string (first 4 bytes packed into u32).
    let clef_id = |c: &str| -> u32 {
        let b = c.as_bytes();
        let mut v = 0u32;
        for (i, &byte) in b.iter().take(4).enumerate() {
            v |= (byte as u32) << (i * 8);
        }
        v
    };
    let mut sp_cache: HashMap<(u8, i32, u32), i32> = HashMap::new();
    let mut cur_clef_id = clef_id(&current_clef);

    for (i, event) in events.iter().enumerate() {
        let pos_info = &positions[i];
        let x = pos_info.x;
        let mut y = 0.0;
        let mut stem_dir = None;
        let mut stem_y_end = None;
        let mut chord_ys = Vec::new();
        let mut chord_staff_positions = Vec::new();
        let mut voice_items = Vec::new();

        match event {
            Event::Note(n) => {
                let cache_key = (n.name.as_bytes()[0], n.octave, cur_clef_id);
                let sp = *sp_cache
                    .entry(cache_key)
                    .or_insert_with(|| pitch::staff_position(&n.name, n.octave, &current_clef));
                y = -sp as f64 / 2.0;
                let sd = if n.grace {
                    "up"
                } else {
                    pitch::auto_stem_direction(sp)
                };
                stem_dir = Some(sd.to_string());
                let stem_scale = if n.grace { GRACE_NOTE_SCALE } else { 1.0 };
                let stem_min = if n.grace { GRACE_STEM_MIN_LENGTH } else { 3.5 };
                stem_y_end = Some(pitch::compute_stem_end_y(y, sp, sd, stem_scale, stem_min));
            }
            Event::Chord(c) => {
                let mut sp_list = Vec::with_capacity(c.notes.len());
                for cn in &c.notes {
                    let cache_key = (cn.name.as_bytes()[0], cn.octave, cur_clef_id);
                    let spos = *sp_cache.entry(cache_key).or_insert_with(|| {
                        pitch::staff_position(&cn.name, cn.octave, &current_clef)
                    });
                    sp_list.push(spos);
                }
                let y_list: Vec<f64> = sp_list.iter().map(|&spos| -spos as f64 / 2.0).collect();
                let avg_sp = sp_list.iter().sum::<i32>() as f64 / sp_list.len() as f64;
                let sd = if c.grace {
                    "up"
                } else {
                    pitch::auto_stem_direction(avg_sp as i32)
                };
                let primary_sp = if sd == "up" {
                    *sp_list.iter().max().unwrap()
                } else {
                    *sp_list.iter().min().unwrap()
                };
                y = -primary_sp as f64 / 2.0;
                let tip_sp = if sd == "up" {
                    *sp_list.iter().min().unwrap()
                } else {
                    *sp_list.iter().max().unwrap()
                };
                let tip_y = -tip_sp as f64 / 2.0;
                let stem_scale = if c.grace { GRACE_NOTE_SCALE } else { 1.0 };
                let stem_min = if c.grace { GRACE_STEM_MIN_LENGTH } else { 3.5 };
                stem_y_end = Some(pitch::compute_stem_end_y(
                    tip_y, tip_sp, &sd, stem_scale, stem_min,
                ));
                stem_dir = Some(sd.to_string());
                chord_ys = y_list;
                chord_staff_positions = sp_list;
            }
            Event::Rest(r) => {
                y = match r.duration {
                    1 => -1.0,
                    DURATION_MAXIMA | DURATION_LONGA | DURATION_BREVE => -2.0,
                    _ => -2.0,
                };
            }
            Event::VoiceGroup(vg) => {
                voice_items = layout_voice_group_items(vg, &current_clef, font);
            }
            Event::Clef(c) => {
                current_clef = c.clef.clone();
                cur_clef_id = clef_id(&current_clef);
            }
            _ => {}
        }

        items.push(LaidOutItem {
            event: event.clone(),
            x,
            y,
            stem_dir,
            stem_y_end,
            stem_forced: false,
            voice: None,
            width: pos_info.width,
            chord_ys,
            chord_staff_positions,
            voice_items,
        });
    }

    let tw = if let Some(last_item) = items.last() {
        if last_item.event.is_barline() {
            positions.last().unwrap().x
        } else {
            positions.last().unwrap().x + positions.last().unwrap().width
        }
    } else {
        0.0
    };

    LaidOutStaff {
        items,
        total_width: tw,
        clef: clef.map(|s| s.to_string()),
        time: time.cloned(),
        show_time_prefix,
        lyric_prefix_states: lyric_prefix_states.to_vec(),
    }
}

// ─── Multi-staff beat alignment ────────────────────────────────────────

fn is_grace_event(ev: &Event) -> bool {
    ev.grace()
}

fn is_rhythmic_event(ev: &Event) -> bool {
    (ev.is_note()
        || ev.is_rest()
        || matches!(ev, Event::Spacer(_))
        || ev.is_chord()
        || ev.is_voice_group())
        && !is_grace_event(ev)
}

fn is_boundary_event(ev: &Event) -> bool {
    matches!(
        ev,
        Event::Barline(_) | Event::Clef(_) | Event::KeySig(_) | Event::TimeSig(_) | Event::Gap(_)
    )
}

fn is_pre_barline_boundary(items: &[LaidOutItem], idx: usize) -> bool {
    idx + 1 < items.len()
        && matches!(items[idx].event, Event::Clef(_) | Event::TimeSig(_))
        && items[idx + 1].event.is_barline()
}

/// Convert a beat position to a fixed‑point integer key (micro-beats).
/// This replaces the previous `format!("{:.6}", ...)` String keys, eliminating
/// thousands of heap allocations during beat alignment.
#[inline]
fn beat_ikey(beat: f64) -> i64 {
    (beat * 1_000_000.0).round() as i64
}

#[inline]
fn measure_beat_ikey(measure_idx: i64, beat: f64) -> i64 {
    const MEASURE_KEY_STRIDE: i64 = 1_000_000_000;
    measure_idx * MEASURE_KEY_STRIDE + beat_ikey(beat)
}

pub fn align_staves_by_beat(laid_out_staves: &[LaidOutStaff]) -> Vec<LaidOutStaff> {
    if laid_out_staves.len() <= 1 {
        return laid_out_staves.to_vec();
    }

    let barline_epsilon = 0.000001;

    // 1. For each beat boundary, compute the maximum number of non-rhythmic
    //    columns that occur before the next rhythmic event on any staff.
    let mut beat_boundary_widths: HashMap<i64, usize> = HashMap::new();
    for laid_out in laid_out_staves {
        let mut measure_idx = 0_i64;
        let mut beat = 0.0;
        let mut boundary_count = 0usize;
        let mut has_measure_content = false;
        let items = &laid_out.items;
        for (ii, item) in items.iter().enumerate() {
            let ev = &item.event;
            let boundary_measure_idx = if ev.is_barline() && has_measure_content {
                measure_idx + 1
            } else {
                measure_idx
            };
            let key = if ev.is_barline() {
                measure_beat_ikey(boundary_measure_idx, 0.0)
            } else {
                measure_beat_ikey(measure_idx, beat)
            };
            if is_pre_barline_boundary(items, ii) {
                continue;
            } else if is_grace_event(ev) || is_boundary_event(ev) {
                boundary_count += 1;
                let current = *beat_boundary_widths.get(&key).unwrap_or(&0);
                if boundary_count > current {
                    beat_boundary_widths.insert(key, boundary_count);
                }
                if ev.is_barline() && has_measure_content {
                    measure_idx += 1;
                    beat = 0.0;
                    has_measure_content = false;
                }
            } else if is_rhythmic_event(ev) {
                let current = *beat_boundary_widths.get(&key).unwrap_or(&0);
                if boundary_count > current {
                    beat_boundary_widths.insert(key, boundary_count);
                }
                boundary_count = 0;

                beat += event_advance_beats(ev);
                has_measure_content = true;
            }
        }
        let final_key = measure_beat_ikey(measure_idx, beat);
        let current = *beat_boundary_widths.get(&final_key).unwrap_or(&0);
        if boundary_count > current {
            beat_boundary_widths.insert(final_key, boundary_count);
        }
    }

    // 2. Compute cumulative beat offsets for every item in every staff.
    let num_staves = laid_out_staves.len();
    let mut staves_beat_keys: Vec<Vec<i64>> = Vec::with_capacity(num_staves);
    let mut staff_terminal_keys: Vec<i64> = Vec::with_capacity(num_staves);
    for laid_out in laid_out_staves {
        let items = &laid_out.items;
        let mut keys = Vec::with_capacity(items.len());
        let mut measure_idx = 0_i64;
        let mut beat = 0.0;
        let mut boundary_phase = 0usize;
        let mut has_measure_content = false;
        for (ii, item) in items.iter().enumerate() {
            let ev = &item.event;
            let rb = (beat * 1_000_000.0_f64).round() / 1_000_000.0;
            let boundary_measure_idx = if ev.is_barline() && has_measure_content {
                measure_idx + 1
            } else {
                measure_idx
            };
            let boundary_key = if ev.is_barline() {
                measure_beat_ikey(boundary_measure_idx, 0.0)
            } else {
                measure_beat_ikey(measure_idx, beat)
            };
            let boundary_width = *beat_boundary_widths.get(&boundary_key).unwrap_or(&0);

            if is_pre_barline_boundary(items, ii) {
                let pre_barline_measure_idx = if has_measure_content {
                    measure_idx + 1
                } else {
                    measure_idx
                };
                keys.push(measure_beat_ikey(pre_barline_measure_idx, -barline_epsilon));
            } else if is_grace_event(ev) || is_boundary_event(ev) {
                keys.push(boundary_key + boundary_phase as i64);
                boundary_phase += 1;
                if ev.is_barline() && has_measure_content {
                    measure_idx += 1;
                    beat = 0.0;
                    has_measure_content = false;
                }
            } else if is_rhythmic_event(ev) {
                keys.push(measure_beat_ikey(
                    measure_idx,
                    rb + boundary_width as f64 * barline_epsilon,
                ));

                beat += event_advance_beats(ev);
                has_measure_content = true;
                boundary_phase = 0;
            } else {
                keys.push(measure_beat_ikey(
                    measure_idx,
                    rb + boundary_width as f64 * barline_epsilon,
                ));
            }
        }
        let terminal_key = measure_beat_ikey(measure_idx, beat);
        let terminal_bw = *beat_boundary_widths.get(&terminal_key).unwrap_or(&0);
        let rb = (beat * 1_000_000.0).round() / 1_000_000.0;
        staff_terminal_keys.push(measure_beat_ikey(
            measure_idx,
            rb + terminal_bw as f64 * barline_epsilon,
        ));
        staves_beat_keys.push(keys);
    }

    // 3. Sorted unique beat positions using a BTreeMap<i64, ()>.
    let mut beat_set: BTreeMap<i64, ()> = BTreeMap::new();
    for staff_keys in &staves_beat_keys {
        for &k in staff_keys {
            beat_set.entry(k).or_insert(());
        }
    }
    for &k in &staff_terminal_keys {
        beat_set.entry(k).or_insert(());
    }
    let all_keys: Vec<i64> = beat_set.keys().copied().collect();
    let n_cols = all_keys.len();

    // 4. Beat -> column index map.
    let mut key_to_col: HashMap<i64, usize> = HashMap::with_capacity(n_cols);
    for (ci, &k) in all_keys.iter().enumerate() {
        key_to_col.insert(k, ci);
    }

    // 5. Compute column widths using the distributed-width approach.
    let mut col_widths = vec![0.0_f64; n_cols];

    for (si, laid_out) in laid_out_staves.iter().enumerate() {
        let staff_keys = &staves_beat_keys[si];
        let terminal_col = *key_to_col.get(&staff_terminal_keys[si]).unwrap_or(&0);
        let items = &laid_out.items;
        for (ii, item) in items.iter().enumerate() {
            let start_col = *key_to_col.get(&staff_keys[ii]).unwrap_or(&0);
            let end_col = if ii + 1 < items.len() {
                *key_to_col.get(&staff_keys[ii + 1]).unwrap_or(&0)
            } else {
                terminal_col
            };
            let span = (end_col.saturating_sub(start_col)).max(1);
            let prev = if ii > 0 {
                Some(&items[ii - 1].event)
            } else {
                None
            };
            let next = items.get(ii + 1).map(|i| &i.event);
            let w = event_width(&item.event, prev, next);
            let distributed = w / span as f64;
            for c in start_col..end_col.min(n_cols) {
                if distributed > col_widths[c] {
                    col_widths[c] = distributed;
                }
            }
        }
    }

    // 6. Cumulative x positions per column.
    let mut col_xs = Vec::with_capacity(n_cols);
    let mut x = SYSTEM_START_CONTENT_PADDING;
    for &w in &col_widths {
        col_xs.push(x);
        x += w;
    }
    let total_w = if laid_out_staves.iter().all(|s| s.items.last().map_or(false, |it| it.event.is_barline())) {
        col_xs.last().copied().unwrap_or(x)
    } else {
        x
    };

    // 7. Reassign x to each item based on its column.
    let mut result = Vec::with_capacity(num_staves);
    for (si, laid_out) in laid_out_staves.iter().enumerate() {
        let staff_keys = &staves_beat_keys[si];
        let mut new_items = Vec::with_capacity(laid_out.items.len());
        for (ii, item) in laid_out.items.iter().enumerate() {
            let ci = *key_to_col.get(&staff_keys[ii]).unwrap_or(&0);
            new_items.push(LaidOutItem {
                event: item.event.clone(),
                x: col_xs[ci],
                y: item.y,
                stem_dir: item.stem_dir.clone(),
                stem_y_end: item.stem_y_end,
                stem_forced: item.stem_forced,
                voice: item.voice,
                width: item.width,
                chord_ys: item.chord_ys.clone(),
                chord_staff_positions: item.chord_staff_positions.clone(),
                voice_items: item.voice_items.clone(),
            });
        }
        result.push(LaidOutStaff {
            items: new_items,
            total_width: total_w,
            clef: laid_out.clef.clone(),
            time: laid_out.time.clone(),
            show_time_prefix: laid_out.show_time_prefix,
            lyric_prefix_states: laid_out.lyric_prefix_states.clone(),
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(name: &str, accidental: Option<&str>, duration: i32) -> Event {
        Event::Note(Note {
            name: name.to_string(),
            accidental: accidental.map(str::to_string),
            octave: 4,
            duration,
            dots: 0,
            tie: false,
            slur_start: false,
            slur_end: false,
            beam_start: false,
            beam_end: false,
            articulations: Vec::new(),
            dynamic: None,
            hairpin: None,
            hairpin_start: false,
            hairpin_end: false,
            trill: false,
            trill_line: false,
            trill_start: false,
            trill_end: false,
            grace: false,
            grace_slash: false,
            ending: None,
            ending_start: false,
            ending_end: false,
            fingering: None,
            fingering_position: "above".to_string(),
            chord_symbol: None,
            staff_markers: Vec::new(),
            staff_text: None,
            expression_text: None,
            lyrics: Vec::new(),
            tuplet_beats: 0.0,
            tuplet_number: 0,
            tuplet_count: 0,
            tuplet_start: false,
            tuplet_end: false,
            octave_line_number: 0,
            octave_line_direction: None,
            octave_line_start: false,
            octave_line_end: false,
            colors: ElementColors::default(),
        })
    }

    fn rest(duration: i32) -> Event {
        Event::Rest(Rest {
            duration,
            dots: 0,
            dynamic: None,
            chord_symbol: None,
            staff_markers: Vec::new(),
            staff_text: None,
            expression_text: None,
            lyrics: Vec::new(),
            tuplet_beats: 0.0,
            tuplet_number: 0,
            tuplet_count: 0,
            tuplet_start: false,
            tuplet_end: false,
            octave_line_number: 0,
            octave_line_direction: None,
            octave_line_start: false,
            octave_line_end: false,
            hairpin: None,
            hairpin_start: false,
            hairpin_end: false,
            trill: false,
            trill_line: false,
            trill_start: false,
            trill_end: false,
            grace: false,
            grace_slash: false,
            ending: None,
            ending_start: false,
            ending_end: false,
            colors: ElementColors::default(),
        })
    }

    fn barline() -> Event {
        Event::Barline(Barline {
            style: "single".to_string(),
            ending: None,
            ending_start: false,
            ending_end: false,
            color: None,
        })
    }

    fn gap(amount: i32) -> Event {
        Event::Gap(Gap::new(amount))
    }

    fn mark_tuplet(event: &mut Event, in_time_of: f64, number: i32, count: i32) {
        match event {
            Event::Note(n) => {
                n.tuplet_beats = in_time_of;
                n.tuplet_number = number;
                n.tuplet_count = count;
            }
            Event::Rest(r) => {
                r.tuplet_beats = in_time_of;
                r.tuplet_number = number;
                r.tuplet_count = count;
            }
            Event::Chord(c) => {
                c.tuplet_beats = in_time_of;
                c.tuplet_number = number;
                c.tuplet_count = count;
            }
            _ => {}
        }
    }

    fn chord(notes: &[(&str, Option<&str>, i32)], duration: i32) -> Event {
        Event::Chord(Chord {
            notes: notes
                .iter()
                .map(|(name, accidental, octave)| ChordNote {
                    name: (*name).to_string(),
                    accidental: accidental.map(str::to_string),
                    octave: *octave,
                    color: None,
                })
                .collect(),
            duration,
            dots: 0,
            tie: false,
            slur_start: false,
            slur_end: false,
            beam_start: false,
            beam_end: false,
            articulations: Vec::new(),
            dynamic: None,
            hairpin: None,
            hairpin_start: false,
            hairpin_end: false,
            trill: false,
            trill_line: false,
            trill_start: false,
            trill_end: false,
            grace: false,
            grace_slash: false,
            ending: None,
            ending_start: false,
            ending_end: false,
            fingering: None,
            fingering_position: "above".to_string(),
            chord_symbol: None,
            staff_markers: Vec::new(),
            staff_text: None,
            expression_text: None,
            lyrics: Vec::new(),
            tuplet_beats: 0.0,
            tuplet_number: 0,
            tuplet_count: 0,
            tuplet_start: false,
            tuplet_end: false,
            octave_line_number: 0,
            octave_line_direction: None,
            octave_line_start: false,
            octave_line_end: false,
            colors: ElementColors::default(),
        })
    }

    #[test]
    fn first_gap_space_is_free_between_short_notes() {
        let prev = note("d", None, 8);
        let next = note("e", None, 8);
        let width = event_width(&gap(1), Some(&prev), Some(&next));

        assert_eq!(width, 0.0);
    }

    #[test]
    fn additional_gap_space_remains_between_short_notes() {
        let prev = note("d", None, 8);
        let next = note("e", None, 16);
        let width = event_width(&gap(2), Some(&prev), Some(&next));

        assert_eq!(width, 0.7);
    }

    #[test]
    fn gap_space_is_kept_when_either_side_is_not_short() {
        let short_prev = note("d", None, 8);
        let quarter_next = note("e", None, 4);
        let quarter_prev = note("d", None, 4);
        let short_next = note("e", None, 8);

        assert_eq!(
            event_width(&gap(1), Some(&short_prev), Some(&quarter_next)),
            0.7
        );
        assert_eq!(
            event_width(&gap(1), Some(&quarter_prev), Some(&short_next)),
            0.7
        );
    }

    #[test]
    fn plain_adjacent_notes_use_tighter_spacing() {
        let c = note("c", None, 4);
        let d = note("d", None, 4);
        let plain_pair_width = event_width(&c, None, Some(&d));
        let isolated_width = event_width(&c, None, None);

        assert!(plain_pair_width < isolated_width);
        assert_eq!(
            plain_pair_width,
            DEFAULT_NOTE_SPACING_BASE
                * duration_spacing_factor(4.0, 0)
                * PLAIN_NOTE_SPACING_MULTIPLIER
        );
    }

    #[test]
    fn tuplets_scale_written_duration_for_spacing_and_alignment() {
        let mut triplet_eighths = vec![note("c", None, 8), note("d", None, 8), note("e", None, 8)];
        for event in &mut triplet_eighths {
            mark_tuplet(event, 2.0, 3, 3);
        }

        let regular_eighth = note("f", None, 8);
        let triplet_width = event_width(&triplet_eighths[0], None, None);
        let regular_width = event_width(&regular_eighth, None, None);

        assert_eq!(
            event_advance_beats(&triplet_eighths[0]),
            duration_to_beats(8, 0) * 2.0 / 3.0
        );
        assert_eq!(
            voice_sequence_duration_beats(&triplet_eighths),
            duration_to_beats(8, 0) * 2.0
        );
        assert!(triplet_width < regular_width);
    }

    #[test]
    fn multi_voice_rests_move_away_from_opposing_voice() {
        let default_rest_y = layout_staff(&[rest(8)], Some("treble"), None, false, &[]).items[0].y;
        let voice_items = layout_voice_group_items(
            &VoiceGroup {
                upper: vec![rest(8), note("c", None, 8)],
                lower: vec![note("e", None, 8), rest(8)],
            },
            "treble",
            glyph::FontId::Bravura,
        );
        let upper_rest = voice_items
            .iter()
            .find(|item| item.voice == Some(1) && matches!(item.event, Event::Rest(_)))
            .unwrap();
        let lower_rest = voice_items
            .iter()
            .find(|item| item.voice == Some(2) && matches!(item.event, Event::Rest(_)))
            .unwrap();

        assert!(upper_rest.y > default_rest_y);
        assert!(lower_rest.y < default_rest_y);
    }

    #[test]
    fn multi_voice_rests_stay_normal_without_opposing_activity() {
        let default_rest_y = layout_staff(&[rest(8)], Some("treble"), None, false, &[]).items[0].y;
        let voice_items = layout_voice_group_items(
            &VoiceGroup {
                upper: vec![rest(8)],
                lower: Vec::new(),
            },
            "treble",
            glyph::FontId::Bravura,
        );
        let upper_rest = voice_items
            .iter()
            .find(|item| item.voice == Some(1) && matches!(item.event, Event::Rest(_)))
            .unwrap();

        assert_eq!(upper_rest.y, default_rest_y);
    }

    #[test]
    fn first_event_starts_after_system_padding() {
        let events = [note("e", None, 4), note("f", None, 4)];
        let positions = compute_event_positions(&events);

        assert_eq!(positions[0].x, SYSTEM_START_CONTENT_PADDING);
    }

    #[test]
    fn grace_notes_default_to_stems_up() {
        let mut high_note = note("f", None, 8);
        if let Event::Note(n) = &mut high_note {
            n.octave = 5;
            n.grace = true;
        }

        let staff = layout_staff(&[high_note], Some("treble"), None, false, &[]);

        assert_eq!(staff.items[0].stem_dir.as_deref(), Some("up"));
    }

    #[test]
    fn grace_note_stems_are_scaled_shorter() {
        let mut grace = note("a", None, 8);
        if let Event::Note(n) = &mut grace {
            n.grace = true;
        }
        let normal = note("a", None, 8);

        let grace_staff = layout_staff(&[grace], Some("treble"), None, false, &[]);
        let normal_staff = layout_staff(&[normal], Some("treble"), None, false, &[]);
        let grace_item = &grace_staff.items[0];
        let normal_item = &normal_staff.items[0];
        let grace_len = grace_item.stem_y_end.unwrap() - grace_item.y;
        let normal_len = normal_item.stem_y_end.unwrap() - normal_item.y;

        assert!(grace_len < normal_len);
        assert_eq!(grace_len, GRACE_STEM_MIN_LENGTH * GRACE_NOTE_SCALE);
    }

    #[test]
    fn accidental_notes_reserve_left_side_space() {
        let d = note("d", None, 4);
        let d_sharp = note("d", Some("sharp"), 4);
        let e = note("e", None, 4);

        let plain_pair_width = event_width(&d, None, Some(&e));
        let accidental_pair_width = event_width(&d, None, Some(&d_sharp));

        assert!(accidental_pair_width > plain_pair_width);
        assert!(accidental_pair_width > DEFAULT_NOTE_SPACING_BASE);
    }

    #[test]
    fn scalar_accidentals_do_not_reserve_left_side_space() {
        let e = note("e", None, 8);
        let f_sharp = note("f", Some("sharp"), 8);
        let scalar_width = event_width(&e, None, Some(&f_sharp));
        let compact_width = DEFAULT_NOTE_SPACING_BASE
            * duration_spacing_factor(8.0, 0)
            * PLAIN_NOTE_SPACING_MULTIPLIER;

        assert_eq!(scalar_width, compact_width);
    }

    #[test]
    fn ascending_stems_leave_room_before_accidentals() {
        let f_eighth = note("f", None, 8);
        let b_flat_sixteenth = note("b", Some("flat"), 16);
        let stem_lane_width = event_width(&f_eighth, None, Some(&b_flat_sixteenth));
        let scalar_eighth_width = DEFAULT_NOTE_SPACING_BASE * duration_spacing_factor(8.0, 0);

        assert!(stem_lane_width > scalar_eighth_width);
    }

    #[test]
    fn flagged_short_notes_leave_room_before_longer_accidentals() {
        let e_eighth = note("e", None, 8);
        let f_sharp_quarter = note("f", Some("sharp"), 4);
        let flagged_width = event_width(&e_eighth, None, Some(&f_sharp_quarter));
        let scalar_eighth_width = DEFAULT_NOTE_SPACING_BASE * duration_spacing_factor(8.0, 0);
        let compact_eighth_width = scalar_eighth_width * PLAIN_NOTE_SPACING_MULTIPLIER;

        assert!(flagged_width > compact_eighth_width);
        assert!(flagged_width < scalar_eighth_width);
    }

    #[test]
    fn same_duration_arpeggio_accidentals_do_not_add_stem_lane_space() {
        let g_eighth = note("g", None, 8);
        let b_flat_eighth = note("b", Some("flat"), 8);
        let arpeggio_width = event_width(&g_eighth, None, Some(&b_flat_eighth));
        let scalar_eighth_width = DEFAULT_NOTE_SPACING_BASE
            * duration_spacing_factor(8.0, 0)
            * PLAIN_NOTE_SPACING_MULTIPLIER;

        assert_eq!(arpeggio_width, scalar_eighth_width);
    }

    #[test]
    fn barlines_leave_clearance_before_accidentals() {
        let barline = barline();
        let b_flat = note("b", Some("flat"), 4);
        let plain_width = event_width(&barline, None, None);
        let before_flat_width = event_width(&barline, None, Some(&b_flat));

        assert!(before_flat_width > plain_width);
    }

    #[test]
    fn tied_grace_notes_leave_clearance_before_accidentals() {
        let mut grace = note("a", None, 8);
        if let Event::Note(n) = &mut grace {
            n.grace = true;
            n.tie = true;
        }
        let g_flat = note("g", Some("flat"), 8);
        let untied_grace_width = {
            let mut untied = grace.clone();
            if let Event::Note(n) = &mut untied {
                n.tie = false;
            }
            event_width(&untied, None, Some(&g_flat))
        };
        let tied_grace_width = event_width(&grace, None, Some(&g_flat));

        assert!(tied_grace_width > untied_grace_width);
    }

    #[test]
    fn short_notes_get_extra_room_before_accidentals() {
        let b_flat_sixteenth = note("b", Some("flat"), 16);
        let b_natural_sixteenth = note("b", Some("natural"), 16);
        let c_quarter = note("c", None, 4);
        let c_sharp_quarter = note("c", Some("sharp"), 4);

        let dense_accidental_width =
            event_width(&b_flat_sixteenth, None, Some(&b_natural_sixteenth));
        let quarter_accidental_width = event_width(&c_quarter, None, Some(&c_sharp_quarter));

        assert!(dense_accidental_width > quarter_accidental_width);
    }

    #[test]
    fn very_short_notes_keep_minimum_head_clearance() {
        let e64 = note("e", None, 64);
        let d64 = note("d", None, 64);

        let width = event_width(&e64, None, Some(&d64));
        let minimum = notehead_half_width(&e64, glyph::FontId::Bravura)
            + notehead_half_width(&d64, glyph::FontId::Bravura)
            + MIN_NOTEHEAD_PAIR_CLEARANCE;
        let compact_duration_width = DEFAULT_NOTE_SPACING_BASE
            * duration_spacing_factor(64.0, 0)
            * PLAIN_NOTE_SPACING_MULTIPLIER;

        assert_eq!(width, minimum);
        assert!(width > compact_duration_width);
    }

    #[test]
    fn very_short_notes_with_accidentals_expand_past_minimum_spacing() {
        let e64 = note("e", None, 64);
        let e_sharp64 = note("e", Some("sharp"), 64);
        let f64 = note("f", None, 64);

        let plain_width = event_width(&e64, None, Some(&f64));
        let accidental_width = event_width(&e64, None, Some(&e_sharp64));

        assert!(accidental_width > plain_width);
    }

    #[test]
    fn dense_chord_accidentals_alternate_outer_notes_first() {
        let positions = [0, 1, 2];
        let accidentals = [Some("flat"), Some("flat"), Some("flat")];

        let (lanes, lane_widths) =
            chord_accidental_lanes(&positions, &accidentals, glyph::FontId::Bravura);

        assert_eq!(lanes, vec![Some(0), Some(2), Some(1)]);
        assert_eq!(lane_widths.len(), 3);
    }

    #[test]
    fn spaced_chord_accidentals_share_a_single_lane() {
        let positions = [0, 8, 16];
        let accidentals = [Some("flat"), Some("flat"), Some("flat")];

        let (lanes, lane_widths) =
            chord_accidental_lanes(&positions, &accidentals, glyph::FontId::Bravura);

        assert_eq!(lanes, vec![Some(0), Some(0), Some(0)]);
        assert_eq!(lane_widths.len(), 1);
    }

    #[test]
    fn dense_chord_accidentals_reserve_more_leading_space() {
        let bar = barline();
        let single = chord(&[("b", Some("flat"), 4)], 4);
        let dense = chord(
            &[
                ("b", Some("flat"), 4),
                ("d", Some("flat"), 5),
                ("f", Some("flat"), 5),
            ],
            4,
        );

        let single_width = event_width(&bar, None, Some(&single));
        let dense_width = event_width(&bar, None, Some(&dense));

        assert!(dense_width > single_width);
    }

    #[test]
    fn flat_accidentals_can_share_more_lanes_than_naturals() {
        let positions = [6, 8, 10];
        let flats = [Some("flat"), Some("flat"), Some("flat")];
        let naturals = [Some("natural"), Some("natural"), Some("natural")];

        let (_, flat_lane_widths) =
            chord_accidental_lanes(&positions, &flats, glyph::FontId::Bravura);
        let (_, natural_lane_widths) =
            chord_accidental_lanes(&positions, &naturals, glyph::FontId::Bravura);

        assert!(flat_lane_widths.len() < natural_lane_widths.len());
    }

    #[test]
    fn whole_rest_only_measures_use_compact_width() {
        let bar = barline();
        let whole_rest = rest(1);
        let compact_width = event_width(&whole_rest, Some(&bar), Some(&bar));
        let regular_width = DEFAULT_NOTE_SPACING_BASE * duration_spacing_factor(1.0, 0);

        assert_eq!(compact_width, EMPTY_MEASURE_REST_WIDTH);
        assert!(compact_width < regular_width);
    }

    #[test]
    fn caesura_reserves_room_after_right_side_marker() {
        let mut with_caesura = note("c", None, 4);
        if let Event::Note(n) = &mut with_caesura {
            n.staff_markers.push("caesura".into());
        }
        let next = note("d", None, 8);
        let plain_width = event_width(&note("c", None, 4), None, Some(&next));
        let caesura_width = event_width(&with_caesura, None, Some(&next));

        assert!(caesura_width > plain_width);
        assert!(
            caesura_width
                >= CAESURA_X_OFFSET + glyph::advance_width_for(glyph::FontId::Bravura, "caesura")
        );
    }

    #[test]
    fn longer_than_whole_durations_get_longer_beats_and_spacing() {
        assert_eq!(duration_to_beats(DURATION_BREVE, 0), 2.0);
        assert_eq!(duration_to_beats(DURATION_LONGA, 0), 4.0);
        assert_eq!(duration_to_beats(DURATION_MAXIMA, 0), 8.0);

        let whole_width = event_width(&note("c", None, 1), None, None);
        let breve_width = event_width(&note("c", None, DURATION_BREVE), None, None);
        let longa_width = event_width(&note("c", None, DURATION_LONGA), None, None);
        let maxima_width = event_width(&note("c", None, DURATION_MAXIMA), None, None);

        assert!(breve_width > whole_width);
        assert!(longa_width > breve_width);
        assert!(maxima_width > longa_width);
    }

    #[test]
    fn compact_plain_measures_fit_before_breaking() {
        let mut events = Vec::new();
        for _ in 0..2 {
            events.extend([
                note("c", None, 8),
                note("d", None, 8),
                note("e", None, 8),
                note("f", None, 8),
                barline(),
            ]);
        }

        let first_measure_width = measure_width(&events[..5]);
        let systems = compute_system_breaks(&events, Some(first_measure_width * 2.0), None);

        assert_eq!(systems.len(), 1);
    }

    #[test]
    fn multi_staff_alignment_resets_at_measure_boundaries() {
        let upper = layout_staff(
            &[
                note("c", None, 2),
                barline(),
                note("d", None, DURATION_BREVE),
            ],
            Some("treble"),
            None,
            false,
            &[],
        );
        let lower = layout_staff(
            &[
                note("c", None, 1),
                barline(),
                note("d", None, DURATION_BREVE),
            ],
            Some("bass"),
            None,
            false,
            &[],
        );

        let aligned = align_staves_by_beat(&[upper, lower]);
        let upper_barline_x = aligned[0].items[1].x;
        let lower_barline_x = aligned[1].items[1].x;
        let upper_second_measure_x = aligned[0].items[2].x;
        let lower_second_measure_x = aligned[1].items[2].x;

        assert!((upper_barline_x - lower_barline_x).abs() < 0.000001);
        assert!((upper_second_measure_x - lower_second_measure_x).abs() < 0.000001);
    }
}
