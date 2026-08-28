pub mod glyph;
pub mod layout;
pub mod parser;
pub mod pitch;
pub mod renderer;
pub mod types;

use glyph::FontId;
use types::*;
#[cfg(not(test))]
use wasm_minimal_protocol::*;

#[cfg(not(test))]
initiate_protocol!();

const MUSIC_START_PADDING_SP: f64 = 2.55;

#[cfg_attr(not(test), wasm_func)]
pub fn render_score(input: &[u8]) -> Result<Vec<u8>, String> {
    let params: ScoreInput =
        serde_json::from_slice(input).map_err(|e| format!("Failed to parse input: {}", e))?;

    let result = process_score(&params);

    serde_json::to_vec(&result).map_err(|e| format!("Failed to serialize output: {}", e))
}

fn process_score(params: &ScoreInput) -> ScoreOutput {
    let ts = parse_time_sig(params.time.as_deref());
    let sp_unit = params.staff_size_mm;
    let font = FontId::from_name(&params.music_font);

    // Parse music for each staff
    let staves_events: Vec<Vec<Event>> = params
        .staves
        .iter()
        .map(|s| {
            let base_oct = pitch::clef_default_base_octave(s.clef.as_deref().unwrap_or("treble"));
            parser::parse_music_with_note_colors(
                &s.music,
                base_oct,
                params.note_colors.as_ref(),
                s.note_colors.as_ref(),
            )
        })
        .collect();
    let staff_group_ranges = build_staff_group_ranges(params);

    // Build systems
    let first_events = if staves_events.is_empty() {
        &[][..]
    } else {
        &staves_events[0]
    };
    let first_clef = params.staves.first().and_then(|s| s.clef.as_deref());

    let show_time = ts.is_some();
    let prefix_first = prefix_width_sp(first_clef, &params.key, show_time, &ts, font);
    let prefix_cont = prefix_width_sp(first_clef, &params.key, false, &ts, font);
    let instrument_indent_first = instrument_indent_sp(params, true, &staff_group_ranges);
    let instrument_indent_cont = instrument_indent_sp(params, false, &staff_group_ranges);

    let avail_width_mm = params.width_mm;
    let first_avail =
        avail_width_mm.map(|w| w / sp_unit - prefix_first - instrument_indent_first - 1.0);
    let cont_avail =
        avail_width_mm.map(|w| w / sp_unit - prefix_cont - instrument_indent_cont - 1.0);

    // Compute system breaks for staff 0
    let staff0_systems = if layout::has_line_breaks(first_events) {
        layout::split_at_line_breaks(first_events)
    } else if let Some(mpl) = params.measures_per_line {
        layout::compute_system_breaks(first_events, None, Some(mpl))
    } else if let Some(fa) = first_avail {
        let mut all_systems = Vec::new();
        let first_batch = layout::compute_system_breaks(first_events, Some(fa), None);
        if !first_batch.is_empty() {
            all_systems.push(first_batch[0].clone());
            let rest: Vec<Event> = first_batch[1..]
                .iter()
                .flat_map(|s| s.iter().cloned())
                .collect();
            if !rest.is_empty() {
                all_systems.extend(layout::compute_system_breaks(&rest, cont_avail, None));
            }
        }
        all_systems
    } else {
        vec![first_events.to_vec()]
    };

    // Count measures per system for mirroring to other staves
    let measure_counts: Vec<usize> = staff0_systems
        .iter()
        .map(|sys| sys.iter().filter(|e| e.is_barline()).count())
        .collect();

    let num_systems = staff0_systems.len();
    let num_staves = params.staves.len();

    // Build systems for each staff
    let mut systems_per_staff: Vec<Vec<PreparedSystem>> = Vec::new();
    for (si, staff_events) in staves_events.iter().enumerate() {
        let initial_clef = params.staves[si].clef.clone();
        let initial_time = ts.clone();
        if si == 0 {
            let split = add_repeat_both_continuations(&staff0_systems);
            systems_per_staff.push(prepare_staff_systems(
                &split,
                initial_clef.as_deref(),
                initial_time.as_ref(),
                show_time,
            ));
        } else {
            let split =
                if layout::has_line_breaks(first_events) && layout::has_line_breaks(staff_events) {
                    layout::split_at_line_breaks(staff_events)
                } else {
                    layout::mirror_breaks(staff_events, &measure_counts)
                };
            let split = add_repeat_both_continuations(&split);
            systems_per_staff.push(prepare_staff_systems(
                &split,
                initial_clef.as_deref(),
                initial_time.as_ref(),
                show_time,
            ));
        }
    }

    // Render each system
    let mut output_systems = Vec::new();
    for sys_idx in 0..num_systems {
        let is_first = sys_idx == 0;
        let mut laid_out_staves = Vec::new();
        for si in 0..num_staves {
            let sys_info = if sys_idx < systems_per_staff[si].len() {
                &systems_per_staff[si][sys_idx]
            } else {
                continue;
            };
            laid_out_staves.push(layout::layout_staff_font(
                &sys_info.events,
                sys_info.clef.as_deref(),
                sys_info.time.as_ref(),
                sys_info.show_time_prefix,
                &sys_info.lyric_prefix_states,
                font,
            ));
        }

        // Beat-align across staves
        if laid_out_staves.len() > 1 {
            laid_out_staves = layout::align_staves_by_beat(&laid_out_staves);
        }

        let sys_output = renderer::render_system_group(
            &laid_out_staves,
            &params.key,
            &ts,
            sp_unit,
            avail_width_mm,
            params.staff_spacing_mm,
            &params.staff_group,
            &staff_group_ranges,
            if is_first {
                params.title.as_deref()
            } else {
                None
            },
            if is_first {
                params.subtitle.as_deref()
            } else {
                None
            },
            if is_first {
                params.composer.as_deref()
            } else {
                None
            },
            if is_first {
                params.arranger.as_deref()
            } else {
                None
            },
            if is_first {
                params.lyricist.as_deref()
            } else {
                None
            },
            is_first && show_time,
            &params
                .staves
                .iter()
                .map(|s| {
                    if is_first {
                        s.instrument_name.as_deref()
                    } else {
                        s.instrument_name_cont.as_deref()
                    }
                })
                .collect::<Vec<_>>(),
            &params
                .staves
                .iter()
                .map(|s| s.instrument_name_shared)
                .collect::<Vec<_>>(),
            &params
                .staves
                .iter()
                .map(|s| s.fingering_position.as_deref().unwrap_or("above"))
                .collect::<Vec<_>>(),
            params.color.as_deref(),
            &params
                .staves
                .iter()
                .map(|s| s.color.as_deref())
                .collect::<Vec<_>>(),
            &params.music_font,
            &params.tuplet_style,
            params.vertical_spacing.as_deref(),
        );
        output_systems.push(sys_output);
    }

    ScoreOutput {
        systems: output_systems,
    }
}

fn normalize_instrument_name(name: &str) -> String {
    name.replace('&', "\u{266d}")
        .replace('#', "\u{266f}")
        .replace('=', "\u{266e}")
}

fn instrument_name_lines(name: &str) -> Vec<String> {
    let normalized = normalize_instrument_name(name);
    let trimmed = normalized.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    if trimmed.chars().count() <= 12 {
        return vec![trimmed.to_string()];
    }
    if let Some(idx) = trimmed.find(" in ") {
        return vec![
            trimmed[..idx].trim().to_string(),
            trimmed[idx + 1..].trim().to_string(),
        ];
    }
    let words: Vec<&str> = trimmed.split_whitespace().collect();
    if words.len() <= 1 {
        return vec![trimmed.to_string()];
    }
    let half = (trimmed.chars().count() + 1) / 2;
    let mut first = String::new();
    let mut second = String::new();
    for word in words {
        if first.chars().count() < half {
            if !first.is_empty() {
                first.push(' ');
            }
            first.push_str(word);
        } else {
            if !second.is_empty() {
                second.push(' ');
            }
            second.push_str(word);
        }
    }
    if second.is_empty() {
        vec![first]
    } else {
        vec![first, second]
    }
}

fn instrument_name_width_sp(name: &str) -> f64 {
    instrument_name_lines(name)
        .iter()
        .map(|line| line.chars().count() as f64 * 0.56 + 0.4)
        .fold(0.0_f64, f64::max)
}

fn legacy_staff_group_kind(staff_group: &str) -> Option<StaffGroupKind> {
    match staff_group {
        "grand" => Some(StaffGroupKind::Brace),
        "bracket" => Some(StaffGroupKind::Bracket),
        "barline" | "barlines" | "connected" => Some(StaffGroupKind::Barline),
        _ => None,
    }
}

fn has_per_staff_grouping(params: &ScoreInput) -> bool {
    params.staves.iter().any(|staff| {
        staff.barline_group_start
            || staff.barline_group_end
            || staff.bracket_start
            || staff.bracket_end
            || staff.brace_start
            || staff.brace_end
    })
}

fn collect_staff_group_ranges<FStart, FEnd>(
    staves: &[StaffInput],
    kind: StaffGroupKind,
    starts: FStart,
    ends: FEnd,
) -> Vec<StaffGroupRange>
where
    FStart: Fn(&StaffInput) -> bool,
    FEnd: Fn(&StaffInput) -> bool,
{
    let mut ranges = Vec::new();
    let mut active_start: Option<usize> = None;
    for (idx, staff) in staves.iter().enumerate() {
        if starts(staff) {
            active_start = Some(idx);
        }
        if ends(staff) {
            if let Some(start) = active_start.take() {
                if idx > start {
                    ranges.push(StaffGroupRange {
                        start,
                        end: idx,
                        kind: kind.clone(),
                    });
                }
            }
        }
    }
    ranges
}

fn build_staff_group_ranges(params: &ScoreInput) -> Vec<StaffGroupRange> {
    let staff_count = params.staves.len();
    if staff_count < 2 {
        return Vec::new();
    }

    if has_per_staff_grouping(params) {
        let mut ranges = Vec::new();
        ranges.extend(collect_staff_group_ranges(
            &params.staves,
            StaffGroupKind::Barline,
            |s| s.barline_group_start,
            |s| s.barline_group_end,
        ));
        ranges.extend(collect_staff_group_ranges(
            &params.staves,
            StaffGroupKind::Bracket,
            |s| s.bracket_start,
            |s| s.bracket_end,
        ));
        ranges.extend(collect_staff_group_ranges(
            &params.staves,
            StaffGroupKind::Brace,
            |s| s.brace_start,
            |s| s.brace_end,
        ));
        return ranges;
    }

    legacy_staff_group_kind(&params.staff_group)
        .map(|kind| {
            vec![StaffGroupRange {
                start: 0,
                end: staff_count - 1,
                kind,
            }]
        })
        .unwrap_or_default()
}

fn instrument_group_symbol_sp_for_ranges(ranges: &[StaffGroupRange]) -> f64 {
    if has_overlapping_brace_and_bracket(ranges) {
        return 3.0;
    }

    ranges
        .iter()
        .map(|range| match range.kind {
            StaffGroupKind::Brace => 2.1,
            StaffGroupKind::Bracket => 1.4,
            StaffGroupKind::Barline => 0.0,
        })
        .fold(0.0_f64, f64::max)
}

fn group_ranges_overlap(a: &StaffGroupRange, b: &StaffGroupRange) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn overlaps_group_kind(
    range: &StaffGroupRange,
    ranges: &[StaffGroupRange],
    kind: StaffGroupKind,
) -> bool {
    ranges
        .iter()
        .any(|other| other.kind == kind && group_ranges_overlap(range, other))
}

fn has_overlapping_brace_and_bracket(ranges: &[StaffGroupRange]) -> bool {
    ranges.iter().any(|range| {
        range.kind == StaffGroupKind::Brace
            && overlaps_group_kind(range, ranges, StaffGroupKind::Bracket)
    })
}

fn instrument_indent_sp(
    params: &ScoreInput,
    first_system: bool,
    group_ranges: &[StaffGroupRange],
) -> f64 {
    let max_name_width = params
        .staves
        .iter()
        .filter_map(|s| {
            if first_system {
                s.instrument_name.as_deref()
            } else {
                s.instrument_name_cont.as_deref()
            }
        })
        .map(instrument_name_width_sp)
        .fold(0.0_f64, f64::max);
    let group_symbol_width = instrument_group_symbol_sp_for_ranges(group_ranges);
    if max_name_width > 0.0 {
        max_name_width + group_symbol_width + 1.4
    } else if group_symbol_width > 0.0 {
        group_symbol_width
    } else {
        0.0
    }
}

fn parse_time_sig(ts: Option<&str>) -> Option<TimeInfo> {
    let ts = ts?;
    match ts {
        "C" | "c" | "common" => Some(TimeInfo {
            upper: 4,
            lower: 4,
            symbol: Some("common".into()),
        }),
        "C|" | "c|" | "cut" => Some(TimeInfo {
            upper: 2,
            lower: 2,
            symbol: Some("cut".into()),
        }),
        _ => {
            let parts: Vec<&str> = ts.split('/').collect();
            if parts.len() == 2 {
                let upper = parts[0].trim().parse().ok()?;
                let lower = parts[1].trim().parse().ok()?;
                Some(TimeInfo {
                    upper,
                    lower,
                    symbol: None,
                })
            } else {
                None
            }
        }
    }
}

fn prefix_width_sp(
    clef: Option<&str>,
    key: &str,
    show_time: bool,
    ts: &Option<TimeInfo>,
    font: FontId,
) -> f64 {
    let mut pf = 0.5; // left margin
    if let Some(c) = clef {
        pf += layout::clef_advance_sp_font(c, 1.0, font);
    }
    pf += layout::key_sig_advance_sp_font(key, 1.0, font);
    if show_time {
        if let Some(t) = ts {
            pf +=
                layout::time_sig_advance_sp_font(t.upper, t.lower, t.symbol.as_deref(), 1.0, font);
        }
    }
    pf += MUSIC_START_PADDING_SP; // music-start padding
    pf
}

struct PreparedSystem {
    events: Vec<Event>,
    clef: Option<String>,
    time: Option<TimeInfo>,
    show_time_prefix: bool,
    lyric_prefix_states: Vec<Option<String>>,
}

fn prepare_staff_systems(
    systems: &[Vec<Event>],
    initial_clef: Option<&str>,
    initial_time: Option<&TimeInfo>,
    show_initial_time: bool,
) -> Vec<PreparedSystem> {
    let mut prepared = Vec::new();
    let mut current_clef = initial_clef.map(|s| s.to_string());
    let mut current_time = initial_time.cloned();
    let mut lyric_states: Vec<Option<String>> = Vec::new();

    for (idx, sys) in systems.iter().enumerate() {
        let mut system_clef = current_clef.clone();
        let mut system_time = current_time.clone();
        let lyric_prefix_states = lyric_states.clone();
        let mut show_time = idx == 0 && show_initial_time && system_time.is_some();

        // Skip leading line breaks, clef and time sig changes at start of system
        let mut start = 0;
        while start < sys.len() && matches!(sys[start], Event::LineBreak) {
            start += 1;
        }
        while start < sys.len() {
            match &sys[start] {
                Event::Clef(c) => {
                    system_clef = Some(c.clef.clone());
                    start += 1;
                }
                Event::TimeSig(t) => {
                    system_time = Some(TimeInfo {
                        upper: t.upper,
                        lower: t.lower,
                        symbol: t.symbol.clone(),
                    });
                    show_time = true;
                    start += 1;
                }
                _ => break,
            }
        }

        let cleaned = sys[start..].to_vec();
        prepared.push(PreparedSystem {
            events: cleaned.clone(),
            clef: system_clef.clone(),
            time: system_time.clone(),
            show_time_prefix: show_time,
            lyric_prefix_states,
        });

        current_clef = system_clef;
        current_time = system_time;
        for ev in &cleaned {
            match ev {
                Event::Clef(c) => current_clef = Some(c.clef.clone()),
                Event::TimeSig(t) => {
                    current_time = Some(TimeInfo {
                        upper: t.upper,
                        lower: t.lower,
                        symbol: t.symbol.clone(),
                    });
                }
                _ => {}
            }
            lyric_states = advance_lyric_states(&lyric_states, ev);
        }
    }
    prepared
}

fn add_repeat_both_continuations(systems: &[Vec<Event>]) -> Vec<Vec<Event>> {
    let mut result = Vec::with_capacity(systems.len());

    for (idx, system) in systems.iter().enumerate() {
        let mut events = system.clone();
        let previous_ended_repeat_both = idx > 0
            && systems[idx - 1].last().is_some_and(
                |event| matches!(event, Event::Barline(b) if b.style == "repeat-both"),
            );

        if previous_ended_repeat_both && !starts_with_repeat_start(&events) {
            let insert_at = leading_prefix_event_count(&events);
            events.insert(insert_at, Event::Barline(Barline::new("repeat-start")));
        }

        result.push(events);
    }

    result
}

fn starts_with_repeat_start(events: &[Event]) -> bool {
    events
        .get(leading_prefix_event_count(events))
        .is_some_and(|event| matches!(event, Event::Barline(b) if b.style == "repeat-start"))
}

fn leading_prefix_event_count(events: &[Event]) -> usize {
    events
        .iter()
        .take_while(|event| matches!(event, Event::LineBreak | Event::Clef(_) | Event::TimeSig(_)))
        .count()
}

fn advance_lyric_states(states: &[Option<String>], event: &Event) -> Vec<Option<String>> {
    if !event.is_anchor() {
        return states.to_vec();
    }
    let lyrics = event.lyrics();
    let line_count = states.len().max(lyrics.len());
    let mut next_states = Vec::new();
    for li in 0..line_count {
        let entry = if li < lyrics.len() {
            Some(&lyrics[li])
        } else {
            None
        };
        let current = if li < states.len() {
            states[li].clone()
        } else {
            None
        };
        if let Some(e) = entry {
            if e.carry {
                next_states.push(current);
            } else {
                match e.continuation.as_str() {
                    "hyphen" | "extender" => next_states.push(Some(e.continuation.clone())),
                    _ => next_states.push(None),
                }
            }
        } else {
            next_states.push(None);
        }
    }
    // Trim trailing Nones
    while next_states.last() == Some(&None) {
        next_states.pop();
    }
    next_states
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clef(name: &str) -> Event {
        Event::Clef(ClefChange {
            clef: name.to_string(),
            ending: None,
            ending_start: false,
            ending_end: false,
            color: None,
        })
    }

    fn time_sig(upper: i32, lower: i32) -> Event {
        Event::TimeSig(TimeSig {
            upper,
            lower,
            symbol: None,
            ending: None,
            ending_start: false,
            ending_end: false,
            color: None,
        })
    }

    fn staff() -> StaffInput {
        StaffInput {
            clef: Some("treble".into()),
            music: "c4 | d4".into(),
            label: None,
            instrument_name: None,
            instrument_name_cont: None,
            instrument_name_shared: false,
            fingering_position: None,
            barline_group_start: false,
            barline_group_end: false,
            bracket_start: false,
            bracket_end: false,
            brace_start: false,
            brace_end: false,
            color: None,
            note_colors: None,
        }
    }

    fn score_input(staves: Vec<StaffInput>, staff_group: &str) -> ScoreInput {
        ScoreInput {
            staves,
            key: "C".into(),
            time: Some("4/4".into()),
            title: None,
            subtitle: None,
            composer: None,
            arranger: None,
            lyricist: None,
            staff_group: staff_group.into(),
            staff_size_mm: 1.75,
            width_mm: Some(120.0),
            staff_spacing_mm: 8.0,
            system_spacing_mm: 12.0,
            measures_per_line: None,
            measure_numbers: "none".into(),
            music_font: "Leland".into(),
            color: None,
            note_colors: None,
            tuplet_style: "bracket".into(),
            vertical_spacing: None,
        }
    }

    #[test]
    fn inline_clef_changes_carry_to_following_system() {
        let systems = vec![
            vec![Event::Barline(Barline::new("single")), clef("treble")],
            vec![Event::Barline(Barline::new("single"))],
        ];

        let prepared = prepare_staff_systems(&systems, Some("bass"), None, false);

        assert_eq!(prepared[0].clef.as_deref(), Some("bass"));
        assert_eq!(prepared[1].clef.as_deref(), Some("treble"));
    }

    #[test]
    fn inline_time_signature_changes_carry_to_following_system() {
        let systems = vec![
            vec![Event::Barline(Barline::new("single")), time_sig(3, 4)],
            vec![Event::Barline(Barline::new("single"))],
        ];
        let initial_time = TimeInfo {
            upper: 4,
            lower: 4,
            symbol: None,
        };

        let prepared = prepare_staff_systems(&systems, Some("treble"), Some(&initial_time), true);

        assert_eq!(
            prepared[0].time.as_ref().map(|t| (t.upper, t.lower)),
            Some((4, 4))
        );
        assert_eq!(
            prepared[1].time.as_ref().map(|t| (t.upper, t.lower)),
            Some((3, 4))
        );
        assert!(!prepared[1].show_time_prefix);
    }

    #[test]
    fn legacy_staff_group_is_used_only_for_explicit_group_values() {
        let separate = score_input(vec![staff(), staff()], "none");
        assert!(build_staff_group_ranges(&separate).is_empty());

        let grand = score_input(vec![staff(), staff()], "grand");
        assert_eq!(
            build_staff_group_ranges(&grand),
            vec![StaffGroupRange {
                start: 0,
                end: 1,
                kind: StaffGroupKind::Brace,
            }]
        );
    }

    #[test]
    fn group_symbols_reserve_indent_without_instrument_names() {
        let grand = score_input(vec![staff(), staff()], "grand");
        let grand_ranges = build_staff_group_ranges(&grand);

        assert!(instrument_indent_sp(&grand, true, &grand_ranges) > 0.0);
    }

    #[test]
    fn per_staff_grouping_overrides_legacy_staff_group() {
        let mut staves = vec![staff(), staff(), staff(), staff()];
        staves[1].bracket_start = true;
        staves[3].bracket_end = true;
        let input = score_input(staves, "grand");

        assert_eq!(
            build_staff_group_ranges(&input),
            vec![StaffGroupRange {
                start: 1,
                end: 3,
                kind: StaffGroupKind::Bracket,
            }]
        );
    }


}
