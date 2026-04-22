use crate::pitch::{clef_default_base_octave, is_supported_clef};
use crate::types::*;

fn is_digit(ch: u8) -> bool {
    ch >= b'0' && ch <= b'9'
}
fn is_lower(ch: u8) -> bool {
    ch >= b'a' && ch <= b'z'
}
fn is_whitespace_char(ch: u8) -> bool {
    ch == b' ' || ch == b'\t' || ch == b'\r'
}
fn is_word_char(ch: u8) -> bool {
    is_lower(ch) || is_digit(ch) || ch == b'-'
}

#[derive(Clone, Copy)]
enum EventColorTarget {
    Overall,
    Tie,
    Slur,
    Beam,
    Articulations,
    Dynamic,
    ChordSymbol,
    StaffText,
    ExpressionText,
    Fingering,
    Lyrics,
    Trill,
    StaffMarkers,
}

#[derive(Clone, Copy)]
enum LastColorTarget {
    Event(EventColorTarget),
    Barline,
    Clef,
    TimeSig,
    Spacer,
}

struct ColorSpan {
    start_idx: usize,
    color: String,
    open_order: i32,
}

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
    last_duration: i32,
    current_base_octave: i32,
    events: Vec<Event>,
    curly_open_serial: i32,
    // Tuplet state
    tuplet_start_idx: Option<usize>,
    tuplet_n: i32,
    tuplet_m: i32,
    tuplet_open_order: Option<i32>,
    // Octave-line state
    octline_start_idx: Option<usize>,
    octline_number: i32,
    octline_dir: Option<String>,
    octline_open_order: Option<i32>,
    // Hairpin state
    hairpin_start_idx: Option<usize>,
    hairpin_kind: Option<String>,
    hairpin_open_order: Option<i32>,
    // Trill state
    trill_start_idx: Option<usize>,
    trill_open_order: Option<i32>,
    // Grace-note state
    grace_start_idx: Option<usize>,
    grace_slash: bool,
    grace_open_order: Option<i32>,
    // Ending state
    ending_start_idx: Option<usize>,
    ending_label: Option<String>,
    ending_open_order: Option<i32>,
    // Selection color state
    color_spans: Vec<ColorSpan>,
    last_color_target: Option<LastColorTarget>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str, base_octave: i32) -> Self {
        Parser {
            input: input.as_bytes(),
            pos: 0,
            last_duration: 4,
            current_base_octave: base_octave,
            events: Vec::with_capacity(input.len() / 3), // rough estimate: ~3 chars per event
            curly_open_serial: 0,
            tuplet_start_idx: None,
            tuplet_n: 0,
            tuplet_m: 0,
            tuplet_open_order: None,
            octline_start_idx: None,
            octline_number: 0,
            octline_dir: None,
            octline_open_order: None,
            hairpin_start_idx: None,
            hairpin_kind: None,
            hairpin_open_order: None,
            trill_start_idx: None,
            trill_open_order: None,
            grace_start_idx: None,
            grace_slash: false,
            grace_open_order: None,
            ending_start_idx: None,
            ending_label: None,
            ending_open_order: None,
            color_spans: Vec::new(),
            last_color_target: None,
        }
    }

    fn len(&self) -> usize {
        self.input.len()
    }
    fn peek(&self, p: usize) -> Option<u8> {
        if p < self.len() {
            Some(self.input[p])
        } else {
            None
        }
    }
    fn _peek_char(&self, p: usize) -> Option<char> {
        self.peek(p).map(|b| b as char)
    }
    fn ch(&self) -> u8 {
        self.input[self.pos]
    }

    fn next_nonspace_char(&self, mut p: usize) -> Option<u8> {
        while p < self.len() && is_whitespace_char(self.input[p]) {
            p += 1;
        }
        self.peek(p)
    }

    fn parse_accidental(&self, mut p: usize) -> (Option<String>, usize) {
        match self.peek(p) {
            Some(b'#') => {
                p += 1;
                if self.peek(p) == Some(b'#') {
                    p += 1;
                    (Some("double-sharp".into()), p)
                } else {
                    (Some("sharp".into()), p)
                }
            }
            Some(b'&') => {
                p += 1;
                if self.peek(p) == Some(b'&') {
                    p += 1;
                    (Some("double-flat".into()), p)
                } else {
                    (Some("flat".into()), p)
                }
            }
            Some(b'=') => (Some("natural".into()), p + 1),
            _ => (None, p),
        }
    }

    fn parse_octave_markers(&self, mut p: usize, mut octave: i32) -> (i32, usize) {
        while self.peek(p) == Some(b'\'') {
            octave += 1;
            p += 1;
        }
        while self.peek(p) == Some(b',') {
            octave -= 1;
            p += 1;
        }
        (octave, p)
    }

    fn parse_duration_dots(&self, mut p: usize, sticky: i32) -> (i32, i32, usize) {
        let mut dur_str = String::new();
        while let Some(ch) = self.peek(p) {
            if is_digit(ch) {
                dur_str.push(ch as char);
                p += 1;
            } else {
                break;
            }
        }
        let duration = if dur_str.is_empty() {
            if self.input[p..].starts_with(b"maxima") {
                p += 6;
                DURATION_MAXIMA
            } else if self.input[p..].starts_with(b"breve") {
                p += 5;
                DURATION_BREVE
            } else if self.input[p..].starts_with(b"longa") {
                p += 5;
                DURATION_LONGA
            } else {
                sticky
            }
        } else {
            dur_str
                .parse::<i32>()
                .ok()
                .filter(|duration| *duration > 0)
                .unwrap_or(sticky)
        };
        let mut dots = 0;
        while self.peek(p) == Some(b'.') {
            dots += 1;
            p += 1;
        }
        (duration, dots, p)
    }

    fn read_bracketed_text(&self, mut p: usize) -> (String, usize) {
        let mut value = String::new();
        while p < self.len() && self.input[p] != b']' {
            value.push(self.input[p] as char);
            p += 1;
        }
        if p < self.len() {
            p += 1;
        } // consume ']'
        (value, p)
    }

    fn read_voice_group(&self, mut p: usize) -> Option<(String, String, usize)> {
        if self.peek(p) != Some(b'{') {
            return None;
        }
        p += 1;
        let body_start = p;
        let mut depth = 0i32;
        let mut split = None;
        while p < self.len() {
            match self.input[p] {
                b'{' => depth += 1,
                b'}' if depth == 0 => {
                    let body_end = p;
                    let split = split?;
                    let upper = String::from_utf8_lossy(&self.input[body_start..split]).to_string();
                    let lower =
                        String::from_utf8_lossy(&self.input[split + 1..body_end]).to_string();
                    return Some((upper, lower, p + 1));
                }
                b'}' => depth -= 1,
                b';' if depth == 0 => split = Some(p),
                _ => {}
            }
            p += 1;
        }
        None
    }

    fn parse_color_call(&self, p: usize) -> Option<(String, Option<usize>, usize)> {
        if self.peek(p) != Some(b'{') {
            return None;
        }

        let mut cursor = p + 1;
        let mut color = String::new();
        while cursor < self.len() {
            match self.input[cursor] {
                b':' => {
                    let value = color.trim().to_string();
                    return Some((value, Some(cursor + 1), cursor + 1));
                }
                b'}' => {
                    let value = color.trim().to_string();
                    return Some((value, None, cursor + 1));
                }
                ch => {
                    color.push(ch as char);
                    cursor += 1;
                }
            }
        }

        None
    }

    fn set_if_missing(slot: &mut Option<String>, color: &str) {
        if slot.is_none() {
            *slot = Some(color.to_string());
        }
    }

    fn apply_color_to_colors(
        colors: &mut ElementColors,
        target: EventColorTarget,
        color: &str,
    ) {
        match target {
            EventColorTarget::Overall => Self::set_if_missing(&mut colors.overall, color),
            EventColorTarget::Tie => Self::set_if_missing(&mut colors.tie, color),
            EventColorTarget::Slur => Self::set_if_missing(&mut colors.slur, color),
            EventColorTarget::Beam => Self::set_if_missing(&mut colors.beam, color),
            EventColorTarget::Articulations => {
                Self::set_if_missing(&mut colors.articulations, color)
            }
            EventColorTarget::Dynamic => Self::set_if_missing(&mut colors.dynamic, color),
            EventColorTarget::ChordSymbol => {
                Self::set_if_missing(&mut colors.chord_symbol, color)
            }
            EventColorTarget::StaffText => Self::set_if_missing(&mut colors.staff_text, color),
            EventColorTarget::ExpressionText => {
                Self::set_if_missing(&mut colors.expression_text, color)
            }
            EventColorTarget::Fingering => Self::set_if_missing(&mut colors.fingering, color),
            EventColorTarget::Lyrics => Self::set_if_missing(&mut colors.lyrics, color),
            EventColorTarget::Trill => Self::set_if_missing(&mut colors.trill, color),
            EventColorTarget::StaffMarkers => {
                Self::set_if_missing(&mut colors.staff_markers, color)
            }
        }
    }

    fn apply_color_to_last_target(&mut self, color: &str) {
        let Some(target) = self.last_color_target else {
            return;
        };

        let Some(last_event) = self.events.last_mut() else {
            return;
        };

        match (target, last_event) {
            (LastColorTarget::Event(kind), Event::Note(n)) => {
                Self::apply_color_to_colors(&mut n.colors, kind, color);
            }
            (LastColorTarget::Event(kind), Event::Rest(r)) => {
                Self::apply_color_to_colors(&mut r.colors, kind, color);
            }
            (LastColorTarget::Event(kind), Event::Chord(c)) => {
                Self::apply_color_to_colors(&mut c.colors, kind, color);
            }
            (LastColorTarget::Barline, Event::Barline(b)) => {
                Self::set_if_missing(&mut b.color, color);
            }
            (LastColorTarget::Clef, Event::Clef(clef)) => {
                Self::set_if_missing(&mut clef.color, color);
            }
            (LastColorTarget::TimeSig, Event::TimeSig(time_sig)) => {
                Self::set_if_missing(&mut time_sig.color, color);
            }
            (LastColorTarget::Spacer, Event::Spacer(spacer)) => {
                Self::set_if_missing(&mut spacer.color, color);
            }
            _ => {}
        }
    }

    fn apply_selection_color(&mut self, color: &str, start: usize) {
        for event in self.events.iter_mut().skip(start) {
            match event {
                Event::Note(n) => {
                    Self::set_if_missing(&mut n.colors.overall, color);
                    if n.tie {
                        Self::set_if_missing(&mut n.colors.tie, color);
                    }
                    if n.slur_start || n.slur_end {
                        Self::set_if_missing(&mut n.colors.slur, color);
                    }
                    if n.beam_start || n.beam_end {
                        Self::set_if_missing(&mut n.colors.beam, color);
                    }
                    if !n.articulations.is_empty() {
                        Self::set_if_missing(&mut n.colors.articulations, color);
                    }
                    if n.dynamic.is_some() {
                        Self::set_if_missing(&mut n.colors.dynamic, color);
                    }
                    if n.chord_symbol.is_some() {
                        Self::set_if_missing(&mut n.colors.chord_symbol, color);
                    }
                    if n.staff_text.is_some() {
                        Self::set_if_missing(&mut n.colors.staff_text, color);
                    }
                    if n.expression_text.is_some() {
                        Self::set_if_missing(&mut n.colors.expression_text, color);
                    }
                    if n.fingering.is_some() {
                        Self::set_if_missing(&mut n.colors.fingering, color);
                    }
                    if !n.lyrics.is_empty() {
                        Self::set_if_missing(&mut n.colors.lyrics, color);
                    }
                    if n.trill || n.trill_line {
                        Self::set_if_missing(&mut n.colors.trill, color);
                    }
                    if !n.staff_markers.is_empty() {
                        Self::set_if_missing(&mut n.colors.staff_markers, color);
                    }
                    if n.octave_line_number != 0 {
                        Self::set_if_missing(&mut n.colors.octave_line, color);
                    }
                }
                Event::Rest(r) => {
                    Self::set_if_missing(&mut r.colors.overall, color);
                    if r.dynamic.is_some() {
                        Self::set_if_missing(&mut r.colors.dynamic, color);
                    }
                    if r.chord_symbol.is_some() {
                        Self::set_if_missing(&mut r.colors.chord_symbol, color);
                    }
                    if r.staff_text.is_some() {
                        Self::set_if_missing(&mut r.colors.staff_text, color);
                    }
                    if r.expression_text.is_some() {
                        Self::set_if_missing(&mut r.colors.expression_text, color);
                    }
                    if !r.lyrics.is_empty() {
                        Self::set_if_missing(&mut r.colors.lyrics, color);
                    }
                    if r.trill || r.trill_line {
                        Self::set_if_missing(&mut r.colors.trill, color);
                    }
                    if !r.staff_markers.is_empty() {
                        Self::set_if_missing(&mut r.colors.staff_markers, color);
                    }
                    if r.octave_line_number != 0 {
                        Self::set_if_missing(&mut r.colors.octave_line, color);
                    }
                }
                Event::Chord(c) => {
                    Self::set_if_missing(&mut c.colors.overall, color);
                    if c.tie {
                        Self::set_if_missing(&mut c.colors.tie, color);
                    }
                    if c.slur_start || c.slur_end {
                        Self::set_if_missing(&mut c.colors.slur, color);
                    }
                    if c.beam_start || c.beam_end {
                        Self::set_if_missing(&mut c.colors.beam, color);
                    }
                    if !c.articulations.is_empty() {
                        Self::set_if_missing(&mut c.colors.articulations, color);
                    }
                    if c.dynamic.is_some() {
                        Self::set_if_missing(&mut c.colors.dynamic, color);
                    }
                    if c.chord_symbol.is_some() {
                        Self::set_if_missing(&mut c.colors.chord_symbol, color);
                    }
                    if c.staff_text.is_some() {
                        Self::set_if_missing(&mut c.colors.staff_text, color);
                    }
                    if c.expression_text.is_some() {
                        Self::set_if_missing(&mut c.colors.expression_text, color);
                    }
                    if c.fingering.is_some() {
                        Self::set_if_missing(&mut c.colors.fingering, color);
                    }
                    if !c.lyrics.is_empty() {
                        Self::set_if_missing(&mut c.colors.lyrics, color);
                    }
                    if c.trill || c.trill_line {
                        Self::set_if_missing(&mut c.colors.trill, color);
                    }
                    if !c.staff_markers.is_empty() {
                        Self::set_if_missing(&mut c.colors.staff_markers, color);
                    }
                    if c.octave_line_number != 0 {
                        Self::set_if_missing(&mut c.colors.octave_line, color);
                    }
                }
                Event::Clef(clef) => Self::set_if_missing(&mut clef.color, color),
                Event::TimeSig(time_sig) => Self::set_if_missing(&mut time_sig.color, color),
                Event::Spacer(spacer) => Self::set_if_missing(&mut spacer.color, color),
                Event::Barline(_) | Event::Gap(_) | Event::KeySig(_) | Event::VoiceGroup(_) | Event::LineBreak => {}
            }
        }
    }

    fn close_color_span(&mut self) {
        if let Some(span) = self.color_spans.pop() {
            self.apply_selection_color(&span.color, span.start_idx);
        }
    }

    fn is_fingering_start(&self, p: usize) -> bool {
        self.peek(p) == Some(b'n')
            && p + 1 < self.len()
            && (self.input[p + 1] == b'['
                || (self.input[p + 1] == b'_' && p + 2 < self.len() && self.input[p + 2] == b'['))
    }

    fn parse_fingering_part(part: &str) -> Option<FingeringMark> {
        let bold = part.len() >= 3 && part.starts_with('*') && part.ends_with('*');
        let value_text = if bold {
            &part[1..part.len() - 1]
        } else {
            part
        };
        value_text
            .parse::<i32>()
            .ok()
            .map(|value| FingeringMark {
                value,
                bold,
                color: None,
            })
    }

    fn split_fingering_parts(value: &str) -> Vec<&str> {
        let mut parts = Vec::new();
        let mut start = None;
        let mut depth = 0i32;

        for (idx, ch) in value.char_indices() {
            match ch {
                '{' => {
                    if start.is_none() {
                        start = Some(idx);
                    }
                    depth += 1;
                }
                '}' => {
                    depth -= 1;
                }
                c if c.is_whitespace() && depth == 0 => {
                    if let Some(s) = start.take() {
                        if s < idx {
                            parts.push(&value[s..idx]);
                        }
                    }
                }
                _ => {
                    if start.is_none() {
                        start = Some(idx);
                    }
                }
            }
        }

        if let Some(s) = start {
            if s < value.len() {
                parts.push(&value[s..]);
            }
        }

        parts
    }

    fn parse_colored_fingering_part(part: &str) -> Option<FingeringMark> {
        if !part.starts_with("color{") || !part.ends_with('}') {
            return None;
        }

        let inner = &part[6..part.len() - 1];
        let colon = inner.find(':')?;
        let color = &inner[..colon];
        let value = &inner[colon + 1..];
        let mut mark = Self::parse_fingering_part(value)?;
        mark.color = Some(color.to_string());
        Some(mark)
    }

    fn parse_fingering_mark(part: &str) -> Option<FingeringMark> {
        Self::parse_colored_fingering_part(part).or_else(|| Self::parse_fingering_part(part))
    }

    fn parse_fingering(&self, p: usize) -> (Option<Fingering>, String, usize) {
        let below = p + 1 < self.len() && self.input[p + 1] == b'_';
        let start = p + if below { 3 } else { 2 };
        let (value, next_pos) = self.read_bracketed_text(start);
        let parts = Self::split_fingering_parts(&value);
        let fingering = if !parts.is_empty() {
            let marks: Vec<FingeringMark> = parts
                .iter()
                .filter_map(|part| Self::parse_fingering_mark(part))
                .collect();
            let has_custom_marks = marks
                .iter()
                .any(|mark| mark.bold || mark.color.is_some());
            if marks.is_empty() {
                None
            } else if has_custom_marks {
                Some(Fingering::Marked(marks))
            } else if marks.len() == 1 {
                Some(Fingering::Single(marks[0].value))
            } else {
                Some(Fingering::Multiple(
                    marks.iter().map(|mark| mark.value).collect(),
                ))
            }
        } else {
            None
        };
        let pos = if below { "below" } else { "above" };
        (fingering, pos.to_string(), next_pos)
    }

    fn parse_lyric(&self, p: usize) -> (Option<LyricEntry>, usize) {
        if self.peek(p) != Some(b'l') {
            return (None, p);
        }
        if p + 1 < self.len() && self.input[p + 1] == b'[' {
            let (raw, next_pos) = self.read_bracketed_text(p + 2);
            let (text, continuation) = if raw.ends_with('-') {
                (raw[..raw.len() - 1].to_string(), "hyphen".to_string())
            } else if raw.ends_with('_') {
                (raw[..raw.len() - 1].to_string(), "extender".to_string())
            } else {
                (raw, "none".to_string())
            };
            (
                Some(LyricEntry {
                    text: Some(text),
                    carry: false,
                    continuation,
                }),
                next_pos,
            )
        } else {
            (
                Some(LyricEntry {
                    text: None,
                    carry: true,
                    continuation: "none".to_string(),
                }),
                p + 1,
            )
        }
    }

    fn parse_tagged_text(&self, p: usize, tag: &str) -> (Option<String>, usize) {
        let tag_bytes = tag.as_bytes();
        if p + tag_bytes.len() >= self.len() {
            return (None, p);
        }
        if &self.input[p..p + tag_bytes.len()] != tag_bytes {
            return (None, p);
        }
        if self.input[p + tag_bytes.len()] != b'[' {
            return (None, p);
        }
        let (value, next_pos) = self.read_bracketed_text(p + tag_bytes.len() + 1);
        let v = if value.is_empty() { None } else { Some(value) };
        (v, next_pos)
    }

    fn parse_staff_marker(&self, p: usize) -> (Option<String>, usize) {
        if self.peek(p) == Some(b'c')
            && self.peek(p + 1) == Some(b'o')
            && self.peek(p + 2) == Some(b'd')
            && self.peek(p + 3) == Some(b'a')
        {
            return (Some("coda".into()), p + 4);
        }
        if self.peek(p) == Some(b'b') && self.peek(p + 1) == Some(b'm') {
            return (Some("breath-mark".into()), p + 2);
        }
        if self.peek(p) == Some(b'd') && self.peek(p + 1) == Some(b's') {
            return (Some("dal-segno".into()), p + 2);
        }
        if self.peek(p) == Some(b'/') && self.peek(p + 1) == Some(b'/') {
            return (Some("caesura".into()), p + 2);
        }
        (None, p)
    }

    fn attach_staff_marker_to_previous(&mut self, marker: String) -> bool {
        match self.events.last_mut() {
            Some(Event::Note(n)) => n.staff_markers.push(marker),
            Some(Event::Rest(r)) => r.staff_markers.push(marker),
            Some(Event::Chord(c)) => c.staff_markers.push(marker),
            _ => return false,
        }
        self.last_color_target = Some(LastColorTarget::Event(EventColorTarget::StaffMarkers));
        true
    }

    fn parse_note_attachments(&self, mut p: usize) -> (NoteAttachments, usize) {
        let mut att = NoteAttachments::default();

        // Tie before articulations
        if self.peek(p) == Some(b'~') {
            att.tie = true;
            att.last_color_target = EventColorTarget::Tie;
            p += 1;
        }

        // Articulations
        loop {
            match self.peek(p) {
                Some(b'>') => {
                    att.articulations.push("accent".into());
                    att.last_color_target = EventColorTarget::Articulations;
                    p += 1;
                }
                Some(b'*') => {
                    att.articulations.push("staccato".into());
                    att.last_color_target = EventColorTarget::Articulations;
                    p += 1;
                }
                Some(b'-') => {
                    att.articulations.push("tenuto".into());
                    att.last_color_target = EventColorTarget::Articulations;
                    p += 1;
                }
                Some(b'_') => {
                    att.articulations.push("fermata".into());
                    att.last_color_target = EventColorTarget::Articulations;
                    p += 1;
                }
                _ => break,
            }
        }

        // Dynamic
        if self.peek(p) == Some(b'v') && p + 1 < self.len() && self.input[p + 1] == b'[' {
            let (value, next_pos) = self.read_bracketed_text(p + 2);
            if !value.is_empty() {
                att.dynamic = Some(value);
                att.last_color_target = EventColorTarget::Dynamic;
            }
            p = next_pos;
        }

        // Tie after articulations/dynamic
        if !att.tie && self.peek(p) == Some(b'~') {
            att.tie = true;
            att.last_color_target = EventColorTarget::Tie;
            p += 1;
        }

        // Trill
        if self.peek(p) == Some(b't') && p + 1 < self.len() && self.input[p + 1] == b'r' {
            att.trill = true;
            att.last_color_target = EventColorTarget::Trill;
            p += 2;
        }

        // Staff markers
        loop {
            let (marker, next_p) = self.parse_staff_marker(p);
            if let Some(m) = marker {
                att.staff_markers.push(m);
                att.last_color_target = EventColorTarget::StaffMarkers;
                p = next_p;
            } else {
                break;
            }
        }

        // Slur start/end
        if self.peek(p) == Some(b'(') {
            att.slur_start = true;
            att.last_color_target = EventColorTarget::Slur;
            p += 1;
        }
        if self.peek(p) == Some(b')') {
            att.slur_end = true;
            att.last_color_target = EventColorTarget::Slur;
            p += 1;
        }

        // Beam start/end
        if self.peek(p) == Some(b'[') {
            let nxt = if p + 1 < self.len() {
                Some(self.input[p + 1])
            } else {
                None
            };
            let is_chord_bracket = nxt.map_or(false, |c| c >= b'A' && c <= b'G');
            if !is_chord_bracket {
                att.beam_start = true;
                att.last_color_target = EventColorTarget::Beam;
                p += 1;
            }
        }
        if self.peek(p) == Some(b']') {
            att.beam_end = true;
            att.last_color_target = EventColorTarget::Beam;
            p += 1;
        }

        // Chord symbol, staff text, expression text, fingering, lyrics
        loop {
            while p < self.len() && is_whitespace_char(self.input[p]) {
                p += 1;
            }

            if self.peek(p) == Some(b'l') {
                let (entry, next_p) = self.parse_lyric(p);
                if let Some(e) = entry {
                    att.lyrics.push(e);
                    att.last_color_target = EventColorTarget::Lyrics;
                }
                p = next_p;
            } else if p + 5 <= self.len() && &self.input[p..p + 5] == b"text[" {
                let (v, next_p) = self.parse_tagged_text(p, "text");
                att.staff_text = v;
                att.last_color_target = EventColorTarget::StaffText;
                p = next_p;
            } else if p + 4 <= self.len() && &self.input[p..p + 4] == b"exp[" {
                let (v, next_p) = self.parse_tagged_text(p, "exp");
                att.expression_text = v;
                att.last_color_target = EventColorTarget::ExpressionText;
                p = next_p;
            } else if self.is_fingering_start(p) {
                let (fng, fng_pos, next_p) = self.parse_fingering(p);
                att.fingering = fng;
                att.fingering_position = fng_pos;
                att.last_color_target = EventColorTarget::Fingering;
                p = next_p;
            } else if self.peek(p) == Some(b'[') {
                let (value, next_p) = self.read_bracketed_text(p + 1);
                if !value.is_empty() {
                    att.chord_symbol = Some(value);
                    att.last_color_target = EventColorTarget::ChordSymbol;
                }
                p = next_p;
            } else {
                break;
            }
        }

        (att, p)
    }

    fn parse_note_pitch(&self, p: usize) -> (Option<String>, i32, usize) {
        let (accidental, p2) = self.parse_accidental(p);
        let (octave, p3) = self.parse_octave_markers(p2, self.current_base_octave);
        (accidental, octave, p3)
    }

    fn parse_note_event_data(&self, p: usize) -> (NoteEventData, usize) {
        let (accidental, octave, p2) = self.parse_note_pitch(p);
        let (duration, dots, p3) = self.parse_duration_dots(p2, self.last_duration);
        let (att, p4) = self.parse_note_attachments(p3);
        (
            NoteEventData {
                accidental,
                octave,
                duration,
                dots,
                att,
            },
            p4,
        )
    }

    fn parse_time_token(&self, token: &str) -> Option<Event> {
        match token {
            "common" | "C" => Some(Event::TimeSig(TimeSig {
                upper: 4,
                lower: 4,
                symbol: Some("common".into()),
                ending: None,
                ending_start: false,
                ending_end: false,
                color: None,
            })),
            "cut" | "C|" => Some(Event::TimeSig(TimeSig {
                upper: 2,
                lower: 2,
                symbol: Some("cut".into()),
                ending: None,
                ending_start: false,
                ending_end: false,
                color: None,
            })),
            _ if token.contains('/') => {
                let parts: Vec<&str> = token.split('/').collect();
                if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
                    if let (Ok(upper), Ok(lower)) = (parts[0].parse(), parts[1].parse()) {
                        Some(Event::TimeSig(TimeSig {
                            upper,
                            lower,
                            symbol: None,
                            ending: None,
                            ending_start: false,
                            ending_end: false,
                            color: None,
                        }))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse(&mut self) {
        while self.pos < self.len() {
            let ch = self.ch();

            // Skip whitespace
            if is_whitespace_char(ch) {
                let start = self.pos;
                while self.pos < self.len() && is_whitespace_char(self.input[self.pos]) {
                    self.pos += 1;
                }
                let run_len = self.pos - start;
                let next = self.next_nonspace_char(self.pos);
                let prev_event = self.events.last();
                let prev_is_barline = prev_event.map_or(false, |e| e.is_barline());
                let prev_is_linebreak = prev_event.map_or(false, |e| matches!(e, Event::LineBreak));
                if run_len > 1
                    && prev_event.is_some()
                    && !prev_is_barline
                    && !prev_is_linebreak
                    && next.is_some()
                    && next != Some(b'\n')
                    && next != Some(b'|')
                {
                    self.events.push(Event::Gap(Gap {
                        amount: (run_len - 1) as i32,
                    }));
                }
                continue;
            }

            // Newlines → line breaks
            if ch == b'\n' {
                self.pos += 1;
                while self.pos < self.len() {
                    let nc = self.input[self.pos];
                    if nc == b' ' || nc == b'\t' || nc == b'\r' || nc == b'\n' {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                if self.pos < self.len() && !self.events.is_empty() {
                    self.events.push(Event::LineBreak);
                }
                continue;
            }

            let (standalone_marker, marker_end) = self.parse_staff_marker(self.pos);
            if let Some(marker) = standalone_marker {
                self.attach_staff_marker_to_previous(marker);
                self.pos = marker_end;
                continue;
            }

            // Barlines
            if ch == b'|' {
                let next = self.peek(self.pos + 1);
                if next == Some(b'|') {
                    if self.peek(self.pos + 2) == Some(b':') {
                        self.events
                            .push(Event::Barline(Barline::new("repeat-start")));
                        self.last_color_target = Some(LastColorTarget::Barline);
                        self.pos += 3;
                    } else {
                        self.events.push(Event::Barline(Barline::new("double")));
                        self.last_color_target = Some(LastColorTarget::Barline);
                        self.pos += 2;
                    }
                } else if next == Some(b':') {
                    self.events
                        .push(Event::Barline(Barline::new("repeat-start")));
                    self.last_color_target = Some(LastColorTarget::Barline);
                    self.pos += 2;
                } else if next == Some(b'.') {
                    self.events.push(Event::Barline(Barline::new("final")));
                    self.last_color_target = Some(LastColorTarget::Barline);
                    self.pos += 2;
                } else {
                    self.events.push(Event::Barline(Barline::new("single")));
                    self.last_color_target = Some(LastColorTarget::Barline);
                    self.pos += 1;
                }
                continue;
            }

            // Repeat-end barlines starting with ":"
            if ch == b':' {
                let next = self.peek(self.pos + 1);
                if next == Some(b'|') {
                    if self.peek(self.pos + 2) == Some(b'|')
                        && self.peek(self.pos + 3) == Some(b':')
                    {
                        self.events
                            .push(Event::Barline(Barline::new("repeat-both")));
                        self.last_color_target = Some(LastColorTarget::Barline);
                        self.pos += 4;
                    } else if self.peek(self.pos + 2) == Some(b':') {
                        self.events
                            .push(Event::Barline(Barline::new("repeat-both")));
                        self.last_color_target = Some(LastColorTarget::Barline);
                        self.pos += 3;
                    } else if self.peek(self.pos + 2) == Some(b'|') {
                        self.events.push(Event::Barline(Barline::new("repeat-end")));
                        self.last_color_target = Some(LastColorTarget::Barline);
                        self.pos += 3;
                    } else {
                        self.events.push(Event::Barline(Barline::new("repeat-end")));
                        self.last_color_target = Some(LastColorTarget::Barline);
                        self.pos += 2;
                    }
                } else {
                    self.pos += 1;
                }
                continue;
            }

            // Chords: <note1 note2 ...>duration
            if ch == b'<' {
                self.pos += 1;
                let mut chord_notes = Vec::new();
                while self.pos < self.len() && self.input[self.pos] != b'>' {
                    let c = self.input[self.pos];
                    if is_whitespace_char(c) {
                        self.pos += 1;
                        continue;
                    }
                    if c >= b'a' && c <= b'g' {
                        let cname = (c as char).to_string();
                        self.pos += 1;
                        let (accidental, octave, p2) = self.parse_note_pitch(self.pos);
                        self.pos = p2;
                        let mut color = None;
                        if self.pos + 5 < self.len()
                            && &self.input[self.pos..self.pos + 5] == b"color"
                        {
                            if let Some((value, None, next_pos)) = self.parse_color_call(self.pos + 5)
                            {
                                if !value.is_empty() {
                                    color = Some(value);
                                }
                                self.pos = next_pos;
                            }
                        }
                        chord_notes.push(ChordNote {
                            name: cname,
                            accidental,
                            octave,
                            color,
                        });
                    } else {
                        self.pos += 1;
                    }
                }
                if self.pos < self.len() && self.input[self.pos] == b'>' {
                    self.pos += 1;
                }
                let (duration, dots, p3) = self.parse_duration_dots(self.pos, self.last_duration);
                let (att, p4) = self.parse_note_attachments(p3);
                self.pos = p4;
                self.last_duration = duration;

                if !chord_notes.is_empty() {
                    self.events.push(Event::Chord(Chord {
                        notes: chord_notes,
                        duration,
                        dots,
                        tie: att.tie,
                        slur_start: att.slur_start,
                        slur_end: att.slur_end,
                        beam_start: att.beam_start,
                        beam_end: att.beam_end,
                        articulations: att.articulations,
                        dynamic: att.dynamic,
                        hairpin: None,
                        hairpin_start: false,
                        hairpin_end: false,
                        trill: att.trill,
                        trill_line: false,
                        trill_start: false,
                        trill_end: false,
                        grace: false,
                        grace_slash: false,
                        ending: None,
                        ending_start: false,
                        ending_end: false,
                        fingering: att.fingering,
                        fingering_position: att.fingering_position,
                        chord_symbol: att.chord_symbol,
                        staff_markers: att.staff_markers,
                        staff_text: att.staff_text,
                        expression_text: att.expression_text,
                        lyrics: att.lyrics,
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
                    }));
                    self.last_color_target = Some(LastColorTarget::Event(att.last_color_target));
                }
                continue;
            }

            // Inline clef changes and keywords
            if is_lower(ch) {
                let mut word_end = self.pos;
                while word_end < self.len() && is_word_char(self.input[word_end]) {
                    word_end += 1;
                }
                let token = std::str::from_utf8(&self.input[self.pos..word_end]).unwrap_or("");

                // Two voices on one staff: v{upper voice;lower voice}
                if token == "v" {
                    let mut group_pos = word_end;
                    while group_pos < self.len() && is_whitespace_char(self.input[group_pos]) {
                        group_pos += 1;
                    }
                    if let Some((upper_src, lower_src, next_pos)) = self.read_voice_group(group_pos)
                    {
                        let mut upper_parser =
                            Parser::new(upper_src.trim(), self.current_base_octave);
                        upper_parser.last_duration = self.last_duration;
                        upper_parser.parse();
                        let mut lower_parser =
                            Parser::new(lower_src.trim(), self.current_base_octave);
                        lower_parser.last_duration = self.last_duration;
                        lower_parser.parse();
                        self.events.push(Event::VoiceGroup(VoiceGroup {
                            upper: upper_parser.events,
                            lower: lower_parser.events,
                        }));
                        self.pos = next_pos;
                        continue;
                    }
                }

                // Ending: end{label: ...}
                if token == "end" && self.peek(word_end) == Some(b'{') {
                    let mut label_pos = word_end + 1;
                    let mut label = String::new();
                    while label_pos < self.len()
                        && self.input[label_pos] != b':'
                        && self.input[label_pos] != b'}'
                    {
                        label.push(self.input[label_pos] as char);
                        label_pos += 1;
                    }
                    if label_pos < self.len() && self.input[label_pos] == b':' {
                        // Close previous ending if any
                        self.close_ending();
                        self.ending_start_idx = Some(self.events.len());
                        self.ending_label = Some(label.trim().to_string());
                        self.curly_open_serial += 1;
                        self.ending_open_order = Some(self.curly_open_serial);
                        self.pos = label_pos + 1;
                        continue;
                    }
                }

                // Hairpin: cresc{ or decresc{
                if (token == "cresc" || token == "decresc") && self.peek(word_end) == Some(b'{') {
                    self.close_hairpin();
                    self.hairpin_start_idx = Some(self.events.len());
                    self.hairpin_kind = Some(token.to_string());
                    self.curly_open_serial += 1;
                    self.hairpin_open_order = Some(self.curly_open_serial);
                    self.pos = word_end + 1;
                    continue;
                }

                // Trill: tr{
                if token == "tr" && self.peek(word_end) == Some(b'{') {
                    self.close_trill();
                    self.trill_start_idx = Some(self.events.len());
                    self.curly_open_serial += 1;
                    self.trill_open_order = Some(self.curly_open_serial);
                    self.pos = word_end + 1;
                    continue;
                }

                // Grace: grace{
                if token == "grace" && self.peek(word_end) == Some(b'{') {
                    self.grace_start_idx = Some(self.events.len());
                    self.grace_slash = false;
                    self.curly_open_serial += 1;
                    self.grace_open_order = Some(self.curly_open_serial);
                    self.pos = word_end + 1;
                    continue;
                }

                if token == "color" {
                    if let Some((value, span_body_start, next_pos)) = self.parse_color_call(word_end)
                    {
                        if let Some(body_start) = span_body_start {
                            self.curly_open_serial += 1;
                            self.color_spans.push(ColorSpan {
                                start_idx: self.events.len(),
                                color: value,
                                open_order: self.curly_open_serial,
                            });
                            self.pos = body_start;
                        } else {
                            if !value.is_empty() {
                                self.apply_color_to_last_target(&value);
                            }
                            self.pos = next_pos;
                        }
                        continue;
                    }
                }

                // Time signature token
                if let Some(ts) = self.parse_time_token(token) {
                    self.events.push(ts);
                    self.last_color_target = Some(LastColorTarget::TimeSig);
                    self.pos = word_end;
                    continue;
                }

                // Clef change
                if is_supported_clef(token) {
                    self.events.push(Event::Clef(ClefChange {
                        clef: token.to_string(),
                        ending: None,
                        ending_start: false,
                        ending_end: false,
                        color: None,
                    }));
                    self.last_color_target = Some(LastColorTarget::Clef);
                    self.current_base_octave = clef_default_base_octave(token);
                    self.pos = word_end;
                    continue;
                }
            }

            // Inline time signatures with uppercase shorthand
            if ch == b'C' {
                let token = if self.peek(self.pos + 1) == Some(b'|') {
                    "C|"
                } else {
                    "C"
                };
                let end_pos = if token == "C|" {
                    self.pos + 2
                } else {
                    self.pos + 1
                };
                let next = self.peek(end_pos);
                if let Some(ts) = self.parse_time_token(token) {
                    if next.is_none()
                        || next == Some(b' ')
                        || next == Some(b'\t')
                        || next == Some(b'|')
                        || next == Some(b'\n')
                        || next == Some(b'\r')
                    {
                        self.events.push(ts);
                        self.last_color_target = Some(LastColorTarget::TimeSig);
                        self.pos = end_pos;
                        continue;
                    }
                }
            }

            // Notes (a-g)
            if ch >= b'a' && ch <= b'g' {
                let name = (ch as char).to_string();
                self.pos += 1;
                let (data, p) = self.parse_note_event_data(self.pos);
                self.pos = p;
                self.last_duration = data.duration;
                let mut note = Note::new(&name, data.octave);
                note.accidental = data.accidental;
                note.duration = data.duration;
                note.dots = data.dots;
                note.tie = data.att.tie;
                note.slur_start = data.att.slur_start;
                note.slur_end = data.att.slur_end;
                note.beam_start = data.att.beam_start;
                note.beam_end = data.att.beam_end;
                note.articulations = data.att.articulations;
                note.dynamic = data.att.dynamic;
                note.trill = data.att.trill;
                note.fingering = data.att.fingering;
                note.fingering_position = data.att.fingering_position;
                note.chord_symbol = data.att.chord_symbol;
                note.staff_markers = data.att.staff_markers;
                note.staff_text = data.att.staff_text;
                note.expression_text = data.att.expression_text;
                note.lyrics = data.att.lyrics;
                self.events.push(Event::Note(note));
                self.last_color_target = Some(LastColorTarget::Event(data.att.last_color_target));
                continue;
            }

            // Rests
            if ch == b'r' {
                self.pos += 1;
                let (duration, dots, p) = self.parse_duration_dots(self.pos, self.last_duration);
                let (att, p) = self.parse_note_attachments(p);
                self.pos = p;
                self.last_duration = duration;
                let mut rest = Rest::new(duration);
                rest.dots = dots;
                rest.dynamic = att.dynamic;
                rest.chord_symbol = att.chord_symbol;
                rest.staff_markers = att.staff_markers;
                rest.staff_text = att.staff_text;
                rest.expression_text = att.expression_text;
                rest.lyrics = att.lyrics;
                self.events.push(Event::Rest(rest));
                self.last_color_target = Some(LastColorTarget::Event(att.last_color_target));
                continue;
            }

            // Spacers
            if ch == b's' {
                self.pos += 1;
                let (duration, dots, p) = self.parse_duration_dots(self.pos, self.last_duration);
                self.pos = p;
                self.last_duration = duration;
                self.events.push(Event::Spacer(Spacer {
                    duration,
                    dots,
                    ending: None,
                    ending_start: false,
                    ending_end: false,
                    color: None,
                }));
                self.last_color_target = Some(LastColorTarget::Spacer);
                continue;
            }

            // Slur start/end (not after note)
            if ch == b'(' {
                if let Some(Event::Note(n)) = self.events.last_mut() {
                    n.slur_start = true;
                    self.last_color_target = Some(LastColorTarget::Event(EventColorTarget::Slur));
                }
                self.pos += 1;
                continue;
            }
            if ch == b')' {
                if let Some(Event::Note(n)) = self.events.last_mut() {
                    n.slur_end = true;
                    self.last_color_target = Some(LastColorTarget::Event(EventColorTarget::Slur));
                }
                self.pos += 1;
                continue;
            }

            // Tuplet start: {n,m:
            if ch == b'{' {
                self.pos += 1;
                while self.pos < self.len() && is_whitespace_char(self.input[self.pos]) {
                    self.pos += 1;
                }
                let mut n_str = String::new();
                while self.pos < self.len() && is_digit(self.input[self.pos]) {
                    n_str.push(self.input[self.pos] as char);
                    self.pos += 1;
                }
                if !n_str.is_empty() {
                    let tb: i32 = n_str.parse().unwrap_or(0);
                    let mut tn = tb;
                    if self.peek(self.pos) == Some(b',') {
                        self.pos += 1;
                        let mut m_str = String::new();
                        while self.pos < self.len() && is_digit(self.input[self.pos]) {
                            m_str.push(self.input[self.pos] as char);
                            self.pos += 1;
                        }
                        if !m_str.is_empty() {
                            tn = m_str.parse().unwrap_or(tn);
                        }
                    }
                    if self.peek(self.pos) == Some(b':') {
                        self.pos += 1;
                    }
                    while self.pos < self.len() && is_whitespace_char(self.input[self.pos]) {
                        self.pos += 1;
                    }
                    self.tuplet_start_idx = Some(self.events.len());
                    self.tuplet_n = tn;
                    self.tuplet_m = tb;
                    self.curly_open_serial += 1;
                    self.tuplet_open_order = Some(self.curly_open_serial);
                }
                continue;
            }

            // Octave-line start: <number>a{ or <number>b{
            if is_digit(ch) {
                let mut p = self.pos;
                let mut nstr = String::new();
                while p < self.len() && is_digit(self.input[p]) {
                    nstr.push(self.input[p] as char);
                    p += 1;
                }
                // Check for time signature: n/d
                if !nstr.is_empty() && p < self.len() && self.input[p] == b'/' {
                    let q_start = p + 1;
                    let mut q = q_start;
                    let mut dstr = String::new();
                    while q < self.len() && is_digit(self.input[q]) {
                        dstr.push(self.input[q] as char);
                        q += 1;
                    }
                    if !dstr.is_empty() {
                        let next = self.peek(q);
                        if next.is_none()
                            || next == Some(b' ')
                            || next == Some(b'\t')
                            || next == Some(b'|')
                            || next == Some(b'\n')
                            || next == Some(b'\r')
                        {
                            let upper: i32 = nstr.parse().unwrap_or(0);
                            let lower: i32 = dstr.parse().unwrap_or(0);
                            self.events.push(Event::TimeSig(TimeSig {
                                upper,
                                lower,
                                symbol: None,
                                ending: None,
                                ending_start: false,
                                ending_end: false,
                                color: None,
                            }));
                            self.last_color_target = Some(LastColorTarget::TimeSig);
                            self.pos = q;
                            continue;
                        }
                    }
                }
                // Check for octave line: na{ or nb{
                if !nstr.is_empty() && p < self.len() {
                    let suf = self.input[p];
                    if suf == b'a' || suf == b'b' {
                        let mut q = p + 1;
                        while q < self.len() && is_whitespace_char(self.input[q]) {
                            q += 1;
                        }
                        if q < self.len() && self.input[q] == b'{' {
                            self.octline_start_idx = Some(self.events.len());
                            self.octline_number = nstr.parse().unwrap_or(0);
                            self.octline_dir =
                                Some(if suf == b'a' { "above" } else { "below" }.to_string());
                            self.curly_open_serial += 1;
                            self.octline_open_order = Some(self.curly_open_serial);
                            self.pos = q + 1;
                            continue;
                        }
                    }
                }
            }

            // Grace slash: /}
            if ch == b'/'
                && self.peek(self.pos + 1) == Some(b'}')
                && self.grace_open_order.is_some()
            {
                self.grace_slash = true;
                self.pos += 1;
                continue;
            }

            // Close curly brace
            if ch == b'}' {
                let mut latest_order = -1;
                let mut close_kind = "";

                if let Some(o) = self.tuplet_open_order {
                    if o > latest_order {
                        latest_order = o;
                        close_kind = "tuplet";
                    }
                }
                if let Some(o) = self.octline_open_order {
                    if o > latest_order {
                        latest_order = o;
                        close_kind = "octline";
                    }
                }
                if let Some(o) = self.ending_open_order {
                    if o > latest_order {
                        latest_order = o;
                        close_kind = "ending";
                    }
                }
                if let Some(o) = self.hairpin_open_order {
                    if o > latest_order {
                        latest_order = o;
                        close_kind = "hairpin";
                    }
                }
                if let Some(o) = self.trill_open_order {
                    if o > latest_order {
                        latest_order = o;
                        close_kind = "trill";
                    }
                }
                if let Some(o) = self.grace_open_order {
                    if o > latest_order {
                        let _ = o;
                        close_kind = "grace";
                    }
                }
                if let Some(span) = self.color_spans.last() {
                    if span.open_order > latest_order {
                        close_kind = "color";
                    }
                }

                match close_kind {
                    "tuplet" => self.close_tuplet(),
                    "octline" => self.close_octline(),
                    "ending" => self.close_ending(),
                    "hairpin" => self.close_hairpin(),
                    "trill" => self.close_trill(),
                    "grace" => self.close_grace(),
                    "color" => self.close_color_span(),
                    _ => {}
                }
                self.pos += 1;
                continue;
            }

            // Unknown character: skip
            self.pos += 1;
        }

        // Close any unclosed spans at end of input
        self.close_hairpin();
        self.close_trill();
        self.close_ending();
        while !self.color_spans.is_empty() {
            self.close_color_span();
        }
    }

    fn close_tuplet(&mut self) {
        if let Some(start) = self.tuplet_start_idx {
            let end = self.events.len();
            let count = (end - start) as i32;
            for i in start..end {
                match &mut self.events[i] {
                    Event::Note(n) => {
                        n.tuplet_beats = self.tuplet_m as f64;
                        n.tuplet_number = self.tuplet_n;
                        n.tuplet_count = count;
                        if i == start {
                            n.tuplet_start = true;
                        }
                        if i == end - 1 {
                            n.tuplet_end = true;
                        }
                    }
                    Event::Rest(r) => {
                        r.tuplet_beats = self.tuplet_m as f64;
                        r.tuplet_number = self.tuplet_n;
                        r.tuplet_count = count;
                        if i == start {
                            r.tuplet_start = true;
                        }
                        if i == end - 1 {
                            r.tuplet_end = true;
                        }
                    }
                    Event::Chord(c) => {
                        c.tuplet_beats = self.tuplet_m as f64;
                        c.tuplet_number = self.tuplet_n;
                        c.tuplet_count = count;
                        if i == start {
                            c.tuplet_start = true;
                        }
                        if i == end - 1 {
                            c.tuplet_end = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        self.tuplet_start_idx = None;
        self.tuplet_open_order = None;
    }

    fn close_octline(&mut self) {
        if let Some(start) = self.octline_start_idx {
            let end = self.events.len();
            for i in start..end {
                let set_fields =
                    |number: i32, dir: &Option<String>, is_start: bool, is_end: bool| {
                        (number, dir.clone(), is_start, is_end)
                    };
                let (num, dir, is_start, is_end) = set_fields(
                    self.octline_number,
                    &self.octline_dir,
                    i == start,
                    i == end - 1,
                );
                match &mut self.events[i] {
                    Event::Note(n) => {
                        n.octave_line_number = num;
                        n.octave_line_direction = dir;
                        n.octave_line_start = is_start;
                        n.octave_line_end = is_end;
                    }
                    Event::Rest(r) => {
                        r.octave_line_number = num;
                        r.octave_line_direction = dir;
                        r.octave_line_start = is_start;
                        r.octave_line_end = is_end;
                    }
                    Event::Chord(c) => {
                        c.octave_line_number = num;
                        c.octave_line_direction = dir;
                        c.octave_line_start = is_start;
                        c.octave_line_end = is_end;
                    }
                    _ => {}
                }
            }
        }
        self.octline_start_idx = None;
        self.octline_dir = None;
        self.octline_open_order = None;
    }

    fn close_ending(&mut self) {
        if let Some(start) = self.ending_start_idx {
            let label = self.ending_label.clone();
            let mut members: Vec<usize> = Vec::new();
            for i in start..self.events.len() {
                if !matches!(self.events[i], Event::LineBreak) {
                    members.push(i);
                }
            }
            if !members.is_empty() {
                let first = members[0];
                let last = *members.last().unwrap();
                for &i in &members {
                    let is_first = i == first;
                    let is_last = i == last;
                    let l = label.clone();
                    match &mut self.events[i] {
                        Event::Note(n) => {
                            n.ending = l;
                            n.ending_start = is_first;
                            n.ending_end = is_last;
                        }
                        Event::Rest(r) => {
                            r.ending = l;
                            r.ending_start = is_first;
                            r.ending_end = is_last;
                        }
                        Event::Chord(c) => {
                            c.ending = l;
                            c.ending_start = is_first;
                            c.ending_end = is_last;
                        }
                        Event::Barline(b) => {
                            b.ending = l;
                            b.ending_start = is_first;
                            b.ending_end = is_last;
                        }
                        Event::Clef(cl) => {
                            cl.ending = l;
                            cl.ending_start = is_first;
                            cl.ending_end = is_last;
                        }
                        Event::TimeSig(t) => {
                            t.ending = l;
                            t.ending_start = is_first;
                            t.ending_end = is_last;
                        }
                        Event::Spacer(s) => {
                            s.ending = l;
                            s.ending_start = is_first;
                            s.ending_end = is_last;
                        }
                        _ => {}
                    }
                }
            }
        }
        self.ending_start_idx = None;
        self.ending_label = None;
        self.ending_open_order = None;
    }

    fn close_hairpin(&mut self) {
        if let Some(start) = self.hairpin_start_idx {
            let kind = self.hairpin_kind.clone();
            let mut anchors: Vec<usize> = Vec::new();
            for i in start..self.events.len() {
                if self.events[i].is_anchor() {
                    anchors.push(i);
                }
            }
            if !anchors.is_empty() {
                let first = anchors[0];
                let last = *anchors.last().unwrap();
                for &i in &anchors {
                    let k = kind.clone();
                    let is_first = i == first;
                    let is_last = i == last;
                    match &mut self.events[i] {
                        Event::Note(n) => {
                            n.hairpin = k;
                            n.hairpin_start = is_first;
                            n.hairpin_end = is_last;
                        }
                        Event::Rest(r) => {
                            r.hairpin = k;
                            r.hairpin_start = is_first;
                            r.hairpin_end = is_last;
                        }
                        Event::Chord(c) => {
                            c.hairpin = k;
                            c.hairpin_start = is_first;
                            c.hairpin_end = is_last;
                        }
                        _ => {}
                    }
                }
            }
        }
        self.hairpin_start_idx = None;
        self.hairpin_kind = None;
        self.hairpin_open_order = None;
    }

    fn close_trill(&mut self) {
        if let Some(start) = self.trill_start_idx {
            let mut anchors: Vec<usize> = Vec::new();
            for i in start..self.events.len() {
                if self.events[i].is_anchor() {
                    anchors.push(i);
                }
            }
            if !anchors.is_empty() {
                let first = anchors[0];
                let last = *anchors.last().unwrap();
                for &i in &anchors {
                    let (is_first, is_last) = (i == first, i == last);
                    match &mut self.events[i] {
                        Event::Note(n) => {
                            n.trill = true;
                            n.trill_line = true;
                            n.trill_start = is_first;
                            n.trill_end = is_last;
                        }
                        Event::Rest(r) => {
                            r.trill = true;
                            r.trill_line = true;
                            r.trill_start = is_first;
                            r.trill_end = is_last;
                        }
                        Event::Chord(c) => {
                            c.trill = true;
                            c.trill_line = true;
                            c.trill_start = is_first;
                            c.trill_end = is_last;
                        }
                        _ => {}
                    }
                }
            }
        }
        self.trill_start_idx = None;
        self.trill_open_order = None;
    }

    fn close_grace(&mut self) {
        if let Some(start) = self.grace_start_idx {
            let slash = self.grace_slash;
            for i in start..self.events.len() {
                if self.events[i].is_anchor() {
                    match &mut self.events[i] {
                        Event::Note(n) => {
                            n.grace = true;
                            n.grace_slash = slash;
                        }
                        Event::Rest(r) => {
                            r.grace = true;
                            r.grace_slash = slash;
                        }
                        Event::Chord(c) => {
                            c.grace = true;
                            c.grace_slash = slash;
                        }
                        _ => {}
                    }
                }
            }
        }
        self.grace_start_idx = None;
        self.grace_slash = false;
        self.grace_open_order = None;
    }
}

struct NoteAttachments {
    tie: bool,
    articulations: Vec<String>,
    dynamic: Option<String>,
    trill: bool,
    slur_start: bool,
    slur_end: bool,
    beam_start: bool,
    beam_end: bool,
    chord_symbol: Option<String>,
    staff_markers: Vec<String>,
    staff_text: Option<String>,
    expression_text: Option<String>,
    fingering: Option<Fingering>,
    fingering_position: String,
    lyrics: Vec<LyricEntry>,
    last_color_target: EventColorTarget,
}

impl Default for NoteAttachments {
    fn default() -> Self {
        NoteAttachments {
            tie: false,
            articulations: Vec::new(),
            dynamic: None,
            trill: false,
            slur_start: false,
            slur_end: false,
            beam_start: false,
            beam_end: false,
            chord_symbol: None,
            staff_markers: Vec::new(),
            staff_text: None,
            expression_text: None,
            fingering: None,
            fingering_position: "above".to_string(),
            lyrics: Vec::new(),
            last_color_target: EventColorTarget::Overall,
        }
    }
}

struct NoteEventData {
    accidental: Option<String>,
    octave: i32,
    duration: i32,
    dots: i32,
    att: NoteAttachments,
}

/// Parse a music string into an array of events.
pub fn parse_music(input: &str, base_octave: i32) -> Vec<Event> {
    let mut parser = Parser::new(input, base_octave);
    parser.parse();
    parser.events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_two_voice_groups() {
        let events = parse_music("v{c2 g,;c4 e g c}", 4);

        match &events[0] {
            Event::VoiceGroup(vg) => {
                assert_eq!(vg.upper.len(), 2);
                assert_eq!(vg.lower.len(), 4);
                match &vg.upper[0] {
                    Event::Note(n) => assert_eq!(n.duration, 2),
                    other => panic!("expected upper voice note, got {other:?}"),
                }
                match &vg.lower[0] {
                    Event::Note(n) => assert_eq!(n.duration, 4),
                    other => panic!("expected lower voice note, got {other:?}"),
                }
            }
            other => panic!("expected voice group, got {other:?}"),
        }
    }

    #[test]
    fn parses_named_longer_than_whole_durations() {
        let events = parse_music(
            "cbreve. d | rlonga <e g>breve slonga cmaxima rmaxima smaxima",
            4,
        );

        match &events[0] {
            Event::Note(n) => {
                assert_eq!(n.duration, DURATION_BREVE);
                assert_eq!(n.dots, 1);
            }
            other => panic!("expected breve note, got {other:?}"),
        }

        match &events[1] {
            Event::Note(n) => assert_eq!(n.duration, DURATION_BREVE),
            other => panic!("expected sticky breve note, got {other:?}"),
        }

        match &events[3] {
            Event::Rest(r) => assert_eq!(r.duration, DURATION_LONGA),
            other => panic!("expected longa rest, got {other:?}"),
        }

        match &events[4] {
            Event::Chord(c) => assert_eq!(c.duration, DURATION_BREVE),
            other => panic!("expected breve chord, got {other:?}"),
        }

        match &events[5] {
            Event::Spacer(s) => assert_eq!(s.duration, DURATION_LONGA),
            other => panic!("expected longa spacer, got {other:?}"),
        }

        match &events[6] {
            Event::Note(n) => assert_eq!(n.duration, DURATION_MAXIMA),
            other => panic!("expected maxima note, got {other:?}"),
        }

        match &events[7] {
            Event::Rest(r) => assert_eq!(r.duration, DURATION_MAXIMA),
            other => panic!("expected maxima rest, got {other:?}"),
        }

        match &events[8] {
            Event::Spacer(s) => assert_eq!(s.duration, DURATION_MAXIMA),
            other => panic!("expected maxima spacer, got {other:?}"),
        }
    }

    #[test]
    fn parses_bold_fingering_marks() {
        let events = parse_music("<c e g>4n[1 *3* 5]", 4);

        match &events[0] {
            Event::Chord(c) => {
                let marks = c.fingering.as_ref().expect("fingering").marks();
                assert_eq!(marks.len(), 3);
                assert_eq!(marks[0].value, 1);
                assert!(!marks[0].bold);
                assert_eq!(marks[1].value, 3);
                assert!(marks[1].bold);
                assert_eq!(marks[2].value, 5);
                assert!(!marks[2].bold);
            }
            other => panic!("expected chord, got {other:?}"),
        }
    }

    #[test]
    fn parses_spaced_text_attachments_without_leaking_music_tokens() {
        let chord_events = parse_music("<e g# b>breve text[breve spacer stress] sbreve", 4);

        assert_eq!(chord_events.len(), 2);
        match &chord_events[0] {
            Event::Chord(c) => {
                assert_eq!(c.duration, DURATION_BREVE);
                assert_eq!(c.staff_text.as_deref(), Some("breve spacer stress"));
            }
            other => panic!("expected chord, got {other:?}"),
        }
        match &chord_events[1] {
            Event::Spacer(s) => assert_eq!(s.duration, DURATION_BREVE),
            other => panic!("expected spacer, got {other:?}"),
        }

        let rest_events = parse_music("rmaxima exp[huge silent duration]", 4);
        assert_eq!(rest_events.len(), 1);
        match &rest_events[0] {
            Event::Rest(r) => {
                assert_eq!(r.duration, DURATION_MAXIMA);
                assert_eq!(r.expression_text.as_deref(), Some("huge silent duration"));
            }
            other => panic!("expected rest, got {other:?}"),
        }
    }

    #[test]
    fn parses_standalone_staff_markers_on_previous_event() {
        let events = parse_music("e'2tr ds e'4 // r4", 4);

        assert_eq!(events.len(), 3);
        match &events[0] {
            Event::Note(n) => {
                assert_eq!(n.name, "e");
                assert_eq!(n.duration, 2);
                assert_eq!(n.staff_markers, vec!["dal-segno"]);
            }
            other => panic!("expected first note, got {other:?}"),
        }
        match &events[1] {
            Event::Note(n) => {
                assert_eq!(n.name, "e");
                assert_eq!(n.duration, 4);
                assert_eq!(n.staff_markers, vec!["caesura"]);
            }
            other => panic!("expected second note, got {other:?}"),
        }
    }
}
