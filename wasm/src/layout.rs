use crate::glyph;
use crate::pitch;
use crate::types::*;
use std::collections::BTreeMap;
use std::collections::HashMap;

// ─── Constants (mirrors constants.typ) ─────────────────────────────────

const DEFAULT_NOTE_SPACING_BASE: f64 = 2.5;
const DEFAULT_CLEF_PADDING: f64 = 0.5;
const DEFAULT_KEY_SIG_PADDING: f64 = 1.0;
const DEFAULT_TIME_SIG_PADDING: f64 = 1.25;
const DEFAULT_ACCIDENTAL_PADDING: f64 = 0.35;
const DEFAULT_INLINE_CLEF_SCALE: f64 = 0.8;

// ─── Utility functions (mirrors utils.typ) ─────────────────────────────

pub fn duration_to_beats(duration: i32, dots: i32) -> f64 {
    let base = 1.0 / duration as f64;
    let mut total = base;
    let mut dot_value = base;
    for _ in 0..dots {
        dot_value /= 2.0;
        total += dot_value;
    }
    total
}

pub fn duration_spacing_factor(duration: f64, dots: i32) -> f64 {
    let base_factor = (4.0_f64 / duration).log2() + 1.0;
    let mut factor = base_factor.max(0.75);
    if dots >= 1 { factor *= 1.15; }
    if dots >= 2 { factor *= 1.1; }
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
    let smufl = clef_smufl_name(clef);
    glyph::advance_width(smufl) * sp + DEFAULT_CLEF_PADDING * sp
}

pub fn key_sig_advance_sp(key: &str, sp: f64) -> f64 {
    let count = pitch::key_sig_accidental_count(key);
    let n = count.unsigned_abs() as usize;
    if n == 0 {
        return 0.0;
    }
    let acc_smufl = if count > 0 { "accidentalSharp" } else { "accidentalFlat" };
    let acc_w = glyph::advance_width(acc_smufl);
    n as f64 * (acc_w + 0.2) * sp + DEFAULT_KEY_SIG_PADDING * sp
}

pub fn time_sig_advance_sp(upper: i32, lower: i32, symbol: Option<&str>, sp: f64) -> f64 {
    match symbol {
        Some("common") => glyph::advance_width("timeSigCommon") * sp + DEFAULT_TIME_SIG_PADDING * sp,
        Some("cut") => glyph::advance_width("timeSigCutCommon") * sp + DEFAULT_TIME_SIG_PADDING * sp,
        _ => {
            let upper_s = upper.to_string();
            let lower_s = lower.to_string();
            let upper_w: f64 = upper_s.chars().filter(|c| c.is_ascii_digit()).map(|c| {
                let name = format!("timeSig{}", c);
                glyph::advance_width(&name) * sp
            }).sum();
            let lower_w: f64 = lower_s.chars().filter(|c| c.is_ascii_digit()).map(|c| {
                let name = format!("timeSig{}", c);
                glyph::advance_width(&name) * sp
            }).sum();
            upper_w.max(lower_w) + DEFAULT_TIME_SIG_PADDING * sp
        }
    }
}

fn inline_time_sig_width(event: &Event, prev: Option<&Event>, next: Option<&Event>) -> f64 {
    if let Event::TimeSig(t) = event {
        let glyph_w = (time_sig_advance_sp(t.upper, t.lower, t.symbol.as_deref(), 1.0) - DEFAULT_TIME_SIG_PADDING).max(0.0);
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

fn notehead_width(duration: i32) -> f64 {
    let smufl = match duration {
        1 => "noteheadWhole",
        2 => "noteheadHalf",
        _ => "noteheadBlack",
    };
    glyph::advance_width(smufl)
}

fn accidental_width(acc: Option<&str>) -> f64 {
    let smufl = match acc {
        Some("sharp") => "accidentalSharp",
        Some("flat") => "accidentalFlat",
        Some("natural") => "accidentalNatural",
        Some("double-sharp") => "accidentalDoubleSharp",
        Some("double-flat") => "accidentalDoubleFlat",
        _ => return 0.0,
    };
    glyph::advance_width(smufl)
}

fn required_leading_accidental_space(next: Option<&Event>) -> f64 {
    let next = match next {
        Some(n) => n,
        None => return 0.0,
    };
    let next_is_grace = next.grace();
    let scale = if next_is_grace { 0.68 } else { 1.0 };
    let cluster_factor = if next_is_grace && (next.is_note() || next.is_chord()) { 0.55 } else { 1.0 };
    match next {
        Event::Note(n) => {
            if n.accidental.is_some() {
                (accidental_width(n.accidental.as_deref()) + DEFAULT_ACCIDENTAL_PADDING) * scale * cluster_factor
            } else {
                0.0
            }
        }
        Event::Chord(c) => {
            let max_w = c.notes.iter()
                .map(|n| accidental_width(n.accidental.as_deref()))
                .fold(0.0_f64, f64::max);
            if max_w > 0.0 {
                (max_w + DEFAULT_ACCIDENTAL_PADDING) * scale * cluster_factor
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

fn leading_accidental_extra(available_space: f64, next: Option<&Event>) -> f64 {
    let required = required_leading_accidental_space(next);
    if required <= 0.0 { return 0.0; }
    let comfort = 0.08;
    (required + comfort - available_space).max(0.0)
}

fn grace_body_width(event: &Event, prev: Option<&Event>, next: Option<&Event>) -> f64 {
    let grace_scale = 0.68;
    let duration = event.duration();
    let head_w = notehead_width(duration);
    let rest_w = if event.is_rest() {
        let smufl = match duration {
            1 => "restWhole",
            2 => "restHalf",
            4 => "restQuarter",
            8 => "rest8th",
            16 => "rest16th",
            32 => "rest32nd",
            64 => "rest64th",
            _ => "restQuarter",
        };
        glyph::advance_width(smufl)
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
    head_w.max(rest_w) * grace_scale + inter_note_gap
}

pub fn event_width(event: &Event, prev: Option<&Event>, next: Option<&Event>) -> f64 {
    match event {
        Event::Barline(_) => {
            let touches_inline_boundary = prev.map_or(false, |p| matches!(p, Event::Clef(_) | Event::TimeSig(_)))
                || next.map_or(false, |n| matches!(n, Event::Clef(_) | Event::TimeSig(_)));
            if touches_inline_boundary { 0.6 } else { 2.5 }
        }
        Event::Clef(c) => {
            let smufl = clef_smufl_name(&c.clef);
            glyph::advance_width(smufl) * DEFAULT_INLINE_CLEF_SCALE + DEFAULT_CLEF_PADDING
        }
        Event::TimeSig(_) => inline_time_sig_width(event, prev, next),
        Event::KeySig(_) => 2.0,
        Event::Gap(g) => 0.7 * g.amount as f64,
        Event::LineBreak => 0.0,
        _ => {
            // Notes, rests, spacers, chords
            if event.grace() {
                let body = grace_body_width(event, prev, next);
                return body + leading_accidental_extra(body, next);
            }
            let dur = event.duration();
            let dots = event.dots();
            let factor = duration_spacing_factor(dur as f64, dots);
            let mut w = DEFAULT_NOTE_SPACING_BASE * factor;

            let tb = event.tuplet_beats();
            let tc = event.tuplet_count();
            if tb > 0.0 && tc > 0 {
                let equiv_dur = 4.0 / tb;
                let total_w = DEFAULT_NOTE_SPACING_BASE * duration_spacing_factor(equiv_dur, 0);
                w = total_w / tc as f64;
            }
            w + leading_accidental_extra(w, next)
        }
    }
}

// ─── Event positions ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PosInfo {
    pub x: f64,
    pub width: f64,
}

pub fn compute_event_positions(events: &[Event]) -> Vec<PosInfo> {
    let mut positions = Vec::with_capacity(events.len());
    let mut x = 0.0;
    for (i, event) in events.iter().enumerate() {
        let prev = if i > 0 { Some(&events[i - 1]) } else { None };
        let next = events.get(i + 1);
        let w = event_width(event, prev, next);
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
        let is_rhythmic = ev.is_note() || ev.is_rest() || matches!(ev, Event::Spacer(_)) || ev.is_chord() || ev.is_barline();
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
        if mc == 0 && !is_last && !remaining.is_empty() && matches!(remaining[0], Event::LineBreak) {
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

pub fn layout_staff(
    events: &[Event],
    clef: Option<&str>,
    time: Option<&TimeInfo>,
    show_time_prefix: bool,
    lyric_prefix_states: &[Option<String>],
) -> LaidOutStaff {
    let positions = compute_event_positions(events);
    let mut items = Vec::with_capacity(events.len());
    let layout_clef = clef.unwrap_or("treble");
    let mut current_clef = layout_clef.to_string();
    // Cache key: (note_name_byte, octave, clef_id) — avoids format!/String allocation.
    // clef_id is a compact hash of the clef string (first 4 bytes packed into u32).
    let clef_id = |c: &str| -> u32 {
        let b = c.as_bytes();
        let mut v = 0u32;
        for (i, &byte) in b.iter().take(4).enumerate() { v |= (byte as u32) << (i * 8); }
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

        match event {
            Event::Note(n) => {
                let cache_key = (n.name.as_bytes()[0], n.octave, cur_clef_id);
                let sp = *sp_cache.entry(cache_key).or_insert_with(|| {
                    pitch::staff_position(&n.name, n.octave, &current_clef)
                });
                y = -sp as f64 / 2.0;
                stem_dir = Some(pitch::auto_stem_direction(sp).to_string());
                stem_y_end = Some(pitch::compute_stem_end_y(y, sp, stem_dir.as_deref().unwrap(), 1.0, 3.5));
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
                let sd = pitch::auto_stem_direction(avg_sp as i32);
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
                stem_y_end = Some(pitch::compute_stem_end_y(tip_y, tip_sp, &sd, 1.0, 3.5));
                stem_dir = Some(sd.to_string());
                chord_ys = y_list;
                chord_staff_positions = sp_list;
            }
            Event::Rest(r) => {
                y = match r.duration {
                    1 => -1.0,
                    _ => -2.0,
                };
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
            width: pos_info.width,
            chord_ys,
            chord_staff_positions,
        });
    }

    let tw = if !positions.is_empty() {
        positions.last().unwrap().x + positions.last().unwrap().width
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
    (ev.is_note() || ev.is_rest() || matches!(ev, Event::Spacer(_)) || ev.is_chord()) && !is_grace_event(ev)
}

fn is_boundary_event(ev: &Event) -> bool {
    matches!(ev, Event::Barline(_) | Event::Clef(_) | Event::KeySig(_) | Event::TimeSig(_) | Event::Gap(_))
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

pub fn align_staves_by_beat(laid_out_staves: &[LaidOutStaff]) -> Vec<LaidOutStaff> {
    if laid_out_staves.len() <= 1 {
        return laid_out_staves.to_vec();
    }

    let barline_epsilon = 0.000001;

    // 1. For each beat boundary, compute the maximum number of non-rhythmic
    //    columns that occur before the next rhythmic event on any staff.
    let mut beat_boundary_widths: HashMap<i64, usize> = HashMap::new();
    for laid_out in laid_out_staves {
        let mut beat = 0.0;
        let mut boundary_count = 0usize;
        let items = &laid_out.items;
        for (ii, item) in items.iter().enumerate() {
            let ev = &item.event;
            let key = beat_ikey(beat);
            if is_pre_barline_boundary(items, ii) {
                continue;
            } else if is_grace_event(ev) || is_boundary_event(ev) {
                boundary_count += 1;
                let current = *beat_boundary_widths.get(&key).unwrap_or(&0);
                if boundary_count > current {
                    beat_boundary_widths.insert(key, boundary_count);
                }
            } else if is_rhythmic_event(ev) {
                let current = *beat_boundary_widths.get(&key).unwrap_or(&0);
                if boundary_count > current {
                    beat_boundary_widths.insert(key, boundary_count);
                }
                boundary_count = 0;

                let dur = ev.duration();
                let dots = ev.dots();
                let mut dur_beats = duration_to_beats(dur, dots);
                let tb = ev.tuplet_beats();
                let tc = ev.tuplet_count();
                if tb > 0.0 && tc > 0 {
                    dur_beats = tb / tc as f64;
                }
                beat += dur_beats;
            }
        }
        let final_key = beat_ikey(beat);
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
        let mut beat = 0.0;
        let mut boundary_phase = 0usize;
        for (ii, item) in items.iter().enumerate() {
            let ev = &item.event;
            let rb = (beat * 1_000_000.0_f64).round() / 1_000_000.0;
            let boundary_width = *beat_boundary_widths.get(&beat_ikey(beat)).unwrap_or(&0);

            if is_pre_barline_boundary(items, ii) {
                keys.push(beat_ikey(rb - barline_epsilon));
            } else if is_grace_event(ev) || is_boundary_event(ev) {
                keys.push(beat_ikey(rb + boundary_phase as f64 * barline_epsilon));
                boundary_phase += 1;
            } else if is_rhythmic_event(ev) {
                keys.push(beat_ikey(rb + boundary_width as f64 * barline_epsilon));

                let dur = ev.duration();
                let dots = ev.dots();
                let mut dur_beats = duration_to_beats(dur, dots);
                let tb = ev.tuplet_beats();
                let tc = ev.tuplet_count();
                if tb > 0.0 && tc > 0 {
                    dur_beats = tb / tc as f64;
                }
                beat += dur_beats;
                boundary_phase = 0;
            } else {
                keys.push(beat_ikey(rb + boundary_width as f64 * barline_epsilon));
            }
        }
        let terminal_bw = *beat_boundary_widths.get(&beat_ikey(beat)).unwrap_or(&0);
        let rb = (beat * 1_000_000.0).round() / 1_000_000.0;
        staff_terminal_keys.push(beat_ikey(rb + terminal_bw as f64 * barline_epsilon));
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
            let prev = if ii > 0 { Some(&items[ii - 1].event) } else { None };
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
    let mut x = 0.0;
    for &w in &col_widths {
        col_xs.push(x);
        x += w;
    }
    let total_w = x;

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
                width: item.width,
                chord_ys: item.chord_ys.clone(),
                chord_staff_positions: item.chord_staff_positions.clone(),
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
