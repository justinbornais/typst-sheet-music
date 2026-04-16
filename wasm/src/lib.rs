pub mod glyph;
pub mod layout;
pub mod parser;
pub mod pitch;
pub mod renderer;
pub mod types;

use types::*;
use wasm_minimal_protocol::*;

initiate_protocol!();

const MUSIC_START_PADDING_SP: f64 = 2.55;

#[wasm_func]
pub fn render_score(input: &[u8]) -> Result<Vec<u8>, String> {
    let params: ScoreInput =
        serde_json::from_slice(input).map_err(|e| format!("Failed to parse input: {}", e))?;

    let result = process_score(&params);

    serde_json::to_vec(&result).map_err(|e| format!("Failed to serialize output: {}", e))
}

fn process_score(params: &ScoreInput) -> ScoreOutput {
    let ts = parse_time_sig(params.time.as_deref());
    let sp_unit = params.staff_size_mm;

    // Parse music for each staff
    let staves_events: Vec<Vec<Event>> = params
        .staves
        .iter()
        .map(|s| {
            let base_oct = pitch::clef_default_base_octave(s.clef.as_deref().unwrap_or("treble"));
            parser::parse_music(&s.music, base_oct)
        })
        .collect();

    // Build systems
    let first_events = if staves_events.is_empty() {
        &[][..]
    } else {
        &staves_events[0]
    };
    let first_clef = params.staves.first().and_then(|s| s.clef.as_deref());

    let show_time = ts.is_some();
    let prefix_first = prefix_width_sp(first_clef, &params.key, show_time, &ts);
    let prefix_cont = prefix_width_sp(first_clef, &params.key, false, &ts);

    let avail_width_mm = params.width_mm;
    let first_avail = avail_width_mm.map(|w| w / sp_unit - prefix_first - 1.0);
    let cont_avail = avail_width_mm.map(|w| w / sp_unit - prefix_cont - 1.0);

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
            laid_out_staves.push(layout::layout_staff(
                &sys_info.events,
                sys_info.clef.as_deref(),
                sys_info.time.as_ref(),
                sys_info.show_time_prefix,
                &sys_info.lyric_prefix_states,
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
                .map(|s| s.fingering_position.as_deref().unwrap_or("above"))
                .collect::<Vec<_>>(),
            &params.music_font,
        );
        output_systems.push(sys_output);
    }

    ScoreOutput {
        systems: output_systems,
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

fn prefix_width_sp(clef: Option<&str>, key: &str, show_time: bool, ts: &Option<TimeInfo>) -> f64 {
    let mut pf = 0.5; // left margin
    if let Some(c) = clef {
        pf += layout::clef_advance_sp(c, 1.0);
    }
    pf += layout::key_sig_advance_sp(key, 1.0);
    if show_time {
        if let Some(t) = ts {
            pf += layout::time_sig_advance_sp(t.upper, t.lower, t.symbol.as_deref(), 1.0);
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
        // Advance lyric states
        for ev in &cleaned {
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
