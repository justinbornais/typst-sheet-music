use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub const DURATION_MAXIMA: i32 = -8;
pub const DURATION_LONGA: i32 = -4;
pub const DURATION_BREVE: i32 = -2;

// ─── Event types (mirrors model.typ) ───────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub name: String,
    pub accidental: Option<String>,
    pub octave: i32,
    pub duration: i32,
    pub dots: i32,
    pub tie: bool,
    pub slur_start: bool,
    pub slur_end: bool,
    pub beam_start: bool,
    pub beam_end: bool,
    pub articulations: Vec<String>,
    pub dynamic: Option<String>,
    pub hairpin: Option<String>,
    pub hairpin_start: bool,
    pub hairpin_end: bool,
    pub trill: bool,
    pub trill_line: bool,
    pub trill_start: bool,
    pub trill_end: bool,
    pub grace: bool,
    pub grace_slash: bool,
    pub ending: Option<String>,
    pub ending_start: bool,
    pub ending_end: bool,
    pub fingering: Option<Fingering>,
    pub fingering_position: String,
    pub chord_symbol: Option<String>,
    pub staff_markers: Vec<String>,
    pub staff_text: Option<String>,
    pub expression_text: Option<String>,
    pub lyrics: Vec<LyricEntry>,
    pub tuplet_beats: f64,
    pub tuplet_number: i32,
    pub tuplet_count: i32,
    pub tuplet_start: bool,
    pub tuplet_end: bool,
    pub octave_line_number: i32,
    pub octave_line_direction: Option<String>,
    pub octave_line_start: bool,
    pub octave_line_end: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Fingering {
    Single(i32),
    Multiple(Vec<i32>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LyricEntry {
    pub text: Option<String>,
    pub carry: bool,
    pub continuation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChordNote {
    pub name: String,
    pub accidental: Option<String>,
    pub octave: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rest {
    pub duration: i32,
    pub dots: i32,
    pub tuplet_beats: f64,
    pub tuplet_number: i32,
    pub tuplet_count: i32,
    pub tuplet_start: bool,
    pub tuplet_end: bool,
    pub octave_line_number: i32,
    pub octave_line_direction: Option<String>,
    pub octave_line_start: bool,
    pub octave_line_end: bool,
    pub hairpin: Option<String>,
    pub hairpin_start: bool,
    pub hairpin_end: bool,
    pub trill: bool,
    pub trill_line: bool,
    pub trill_start: bool,
    pub trill_end: bool,
    pub grace: bool,
    pub grace_slash: bool,
    pub ending: Option<String>,
    pub ending_start: bool,
    pub ending_end: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chord {
    pub notes: Vec<ChordNote>,
    pub duration: i32,
    pub dots: i32,
    pub tie: bool,
    pub slur_start: bool,
    pub slur_end: bool,
    pub beam_start: bool,
    pub beam_end: bool,
    pub articulations: Vec<String>,
    pub dynamic: Option<String>,
    pub hairpin: Option<String>,
    pub hairpin_start: bool,
    pub hairpin_end: bool,
    pub trill: bool,
    pub trill_line: bool,
    pub trill_start: bool,
    pub trill_end: bool,
    pub grace: bool,
    pub grace_slash: bool,
    pub ending: Option<String>,
    pub ending_start: bool,
    pub ending_end: bool,
    pub fingering: Option<Fingering>,
    pub fingering_position: String,
    pub chord_symbol: Option<String>,
    pub staff_markers: Vec<String>,
    pub staff_text: Option<String>,
    pub expression_text: Option<String>,
    pub lyrics: Vec<LyricEntry>,
    pub tuplet_beats: f64,
    pub tuplet_number: i32,
    pub tuplet_count: i32,
    pub tuplet_start: bool,
    pub tuplet_end: bool,
    pub octave_line_number: i32,
    pub octave_line_direction: Option<String>,
    pub octave_line_start: bool,
    pub octave_line_end: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barline {
    pub style: String,
    pub ending: Option<String>,
    pub ending_start: bool,
    pub ending_end: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClefChange {
    pub clef: String,
    pub ending: Option<String>,
    pub ending_start: bool,
    pub ending_end: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSig {
    pub upper: i32,
    pub lower: i32,
    pub symbol: Option<String>,
    pub ending: Option<String>,
    pub ending_start: bool,
    pub ending_end: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeySig {
    pub key: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gap {
    pub amount: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacer {
    pub duration: i32,
    pub dots: i32,
    pub ending: Option<String>,
    pub ending_start: bool,
    pub ending_end: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "note")]
    Note(Note),
    #[serde(rename = "rest")]
    Rest(Rest),
    #[serde(rename = "chord")]
    Chord(Chord),
    #[serde(rename = "barline")]
    Barline(Barline),
    #[serde(rename = "clef")]
    Clef(ClefChange),
    #[serde(rename = "time-sig")]
    TimeSig(TimeSig),
    #[serde(rename = "key-sig")]
    KeySig(KeySig),
    #[serde(rename = "gap")]
    Gap(Gap),
    #[serde(rename = "spacer")]
    Spacer(Spacer),
    #[serde(rename = "line-break")]
    LineBreak,
}

impl Event {
    pub fn is_note(&self) -> bool {
        matches!(self, Event::Note(_))
    }
    pub fn is_rest(&self) -> bool {
        matches!(self, Event::Rest(_))
    }
    pub fn is_chord(&self) -> bool {
        matches!(self, Event::Chord(_))
    }
    pub fn is_barline(&self) -> bool {
        matches!(self, Event::Barline(_))
    }
    pub fn is_anchor(&self) -> bool {
        matches!(self, Event::Note(_) | Event::Rest(_) | Event::Chord(_))
    }

    pub fn duration(&self) -> i32 {
        match self {
            Event::Note(n) => n.duration,
            Event::Rest(r) => r.duration,
            Event::Chord(c) => c.duration,
            Event::Spacer(s) => s.duration,
            _ => 0,
        }
    }
    pub fn dots(&self) -> i32 {
        match self {
            Event::Note(n) => n.dots,
            Event::Rest(r) => r.dots,
            Event::Chord(c) => c.dots,
            Event::Spacer(s) => s.dots,
            _ => 0,
        }
    }
    pub fn grace(&self) -> bool {
        match self {
            Event::Note(n) => n.grace,
            Event::Rest(r) => r.grace,
            Event::Chord(c) => c.grace,
            _ => false,
        }
    }
    pub fn tuplet_beats(&self) -> f64 {
        match self {
            Event::Note(n) => n.tuplet_beats,
            Event::Rest(r) => r.tuplet_beats,
            Event::Chord(c) => c.tuplet_beats,
            _ => 0.0,
        }
    }
    pub fn tuplet_count(&self) -> i32 {
        match self {
            Event::Note(n) => n.tuplet_count,
            Event::Rest(r) => r.tuplet_count,
            Event::Chord(c) => c.tuplet_count,
            _ => 0,
        }
    }
    pub fn tuplet_number(&self) -> i32 {
        match self {
            Event::Note(n) => n.tuplet_number,
            Event::Rest(r) => r.tuplet_number,
            Event::Chord(c) => c.tuplet_number,
            _ => 0,
        }
    }
    pub fn tuplet_start(&self) -> bool {
        match self {
            Event::Note(n) => n.tuplet_start,
            Event::Rest(r) => r.tuplet_start,
            Event::Chord(c) => c.tuplet_start,
            _ => false,
        }
    }
    pub fn tuplet_end(&self) -> bool {
        match self {
            Event::Note(n) => n.tuplet_end,
            Event::Rest(r) => r.tuplet_end,
            Event::Chord(c) => c.tuplet_end,
            _ => false,
        }
    }
    pub fn tie(&self) -> bool {
        match self {
            Event::Note(n) => n.tie,
            Event::Chord(c) => c.tie,
            _ => false,
        }
    }
    pub fn slur_start(&self) -> bool {
        match self {
            Event::Note(n) => n.slur_start,
            Event::Chord(c) => c.slur_start,
            _ => false,
        }
    }
    pub fn slur_end(&self) -> bool {
        match self {
            Event::Note(n) => n.slur_end,
            Event::Chord(c) => c.slur_end,
            _ => false,
        }
    }
    pub fn articulations(&self) -> &[String] {
        match self {
            Event::Note(n) => &n.articulations,
            Event::Chord(c) => &c.articulations,
            _ => &[],
        }
    }
    pub fn dynamic_mark(&self) -> Option<&str> {
        match self {
            Event::Note(n) => n.dynamic.as_deref(),
            Event::Chord(c) => c.dynamic.as_deref(),
            _ => None,
        }
    }
    pub fn hairpin(&self) -> Option<&str> {
        match self {
            Event::Note(n) => n.hairpin.as_deref(),
            Event::Rest(r) => r.hairpin.as_deref(),
            Event::Chord(c) => c.hairpin.as_deref(),
            _ => None,
        }
    }
    pub fn hairpin_start(&self) -> bool {
        match self {
            Event::Note(n) => n.hairpin_start,
            Event::Rest(r) => r.hairpin_start,
            Event::Chord(c) => c.hairpin_start,
            _ => false,
        }
    }
    pub fn hairpin_end(&self) -> bool {
        match self {
            Event::Note(n) => n.hairpin_end,
            Event::Rest(r) => r.hairpin_end,
            Event::Chord(c) => c.hairpin_end,
            _ => false,
        }
    }
    pub fn trill(&self) -> bool {
        match self {
            Event::Note(n) => n.trill,
            Event::Rest(r) => r.trill,
            Event::Chord(c) => c.trill,
            _ => false,
        }
    }
    pub fn trill_line(&self) -> bool {
        match self {
            Event::Note(n) => n.trill_line,
            Event::Rest(r) => r.trill_line,
            Event::Chord(c) => c.trill_line,
            _ => false,
        }
    }
    pub fn trill_start(&self) -> bool {
        match self {
            Event::Note(n) => n.trill_start,
            Event::Rest(r) => r.trill_start,
            Event::Chord(c) => c.trill_start,
            _ => false,
        }
    }
    pub fn trill_end(&self) -> bool {
        match self {
            Event::Note(n) => n.trill_end,
            Event::Rest(r) => r.trill_end,
            Event::Chord(c) => c.trill_end,
            _ => false,
        }
    }
    pub fn ending(&self) -> Option<&str> {
        match self {
            Event::Note(n) => n.ending.as_deref(),
            Event::Rest(r) => r.ending.as_deref(),
            Event::Chord(c) => c.ending.as_deref(),
            Event::Barline(b) => b.ending.as_deref(),
            Event::Clef(cl) => cl.ending.as_deref(),
            Event::TimeSig(t) => t.ending.as_deref(),
            Event::Spacer(s) => s.ending.as_deref(),
            _ => None,
        }
    }
    pub fn ending_start(&self) -> bool {
        match self {
            Event::Note(n) => n.ending_start,
            Event::Rest(r) => r.ending_start,
            Event::Chord(c) => c.ending_start,
            Event::Barline(b) => b.ending_start,
            Event::Clef(cl) => cl.ending_start,
            Event::TimeSig(t) => t.ending_start,
            Event::Spacer(s) => s.ending_start,
            _ => false,
        }
    }
    pub fn ending_end(&self) -> bool {
        match self {
            Event::Note(n) => n.ending_end,
            Event::Rest(r) => r.ending_end,
            Event::Chord(c) => c.ending_end,
            Event::Barline(b) => b.ending_end,
            Event::Clef(cl) => cl.ending_end,
            Event::TimeSig(t) => t.ending_end,
            Event::Spacer(s) => s.ending_end,
            _ => false,
        }
    }
    pub fn octave_line_number(&self) -> i32 {
        match self {
            Event::Note(n) => n.octave_line_number,
            Event::Rest(r) => r.octave_line_number,
            Event::Chord(c) => c.octave_line_number,
            _ => 0,
        }
    }
    pub fn octave_line_direction(&self) -> Option<&str> {
        match self {
            Event::Note(n) => n.octave_line_direction.as_deref(),
            Event::Rest(r) => r.octave_line_direction.as_deref(),
            Event::Chord(c) => c.octave_line_direction.as_deref(),
            _ => None,
        }
    }
    pub fn octave_line_start(&self) -> bool {
        match self {
            Event::Note(n) => n.octave_line_start,
            Event::Rest(r) => r.octave_line_start,
            Event::Chord(c) => c.octave_line_start,
            _ => false,
        }
    }
    pub fn octave_line_end(&self) -> bool {
        match self {
            Event::Note(n) => n.octave_line_end,
            Event::Rest(r) => r.octave_line_end,
            Event::Chord(c) => c.octave_line_end,
            _ => false,
        }
    }
    pub fn fingering(&self) -> Option<&Fingering> {
        match self {
            Event::Note(n) => n.fingering.as_ref(),
            Event::Chord(c) => c.fingering.as_ref(),
            _ => None,
        }
    }
    pub fn fingering_position(&self) -> &str {
        match self {
            Event::Note(n) => &n.fingering_position,
            Event::Chord(c) => &c.fingering_position,
            _ => "above",
        }
    }
    pub fn chord_symbol(&self) -> Option<&str> {
        match self {
            Event::Note(n) => n.chord_symbol.as_deref(),
            Event::Chord(c) => c.chord_symbol.as_deref(),
            _ => None,
        }
    }
    pub fn staff_markers(&self) -> &[String] {
        match self {
            Event::Note(n) => &n.staff_markers,
            Event::Chord(c) => &c.staff_markers,
            _ => &[],
        }
    }
    pub fn staff_text(&self) -> Option<&str> {
        match self {
            Event::Note(n) => n.staff_text.as_deref(),
            Event::Chord(c) => c.staff_text.as_deref(),
            _ => None,
        }
    }
    pub fn expression_text(&self) -> Option<&str> {
        match self {
            Event::Note(n) => n.expression_text.as_deref(),
            Event::Chord(c) => c.expression_text.as_deref(),
            _ => None,
        }
    }
    pub fn lyrics(&self) -> &[LyricEntry] {
        match self {
            Event::Note(n) => &n.lyrics,
            Event::Chord(c) => &c.lyrics,
            _ => &[],
        }
    }
}

// ─── Constructors ──────────────────────────────────────────────────────

impl Note {
    pub fn new(name: &str, octave: i32) -> Self {
        Note {
            name: name.to_string(),
            accidental: None,
            octave,
            duration: 4,
            dots: 0,
            tie: false,
            slur_start: false,
            slur_end: false,
            beam_start: false,
            beam_end: false,
            articulations: vec![],
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
            staff_markers: vec![],
            staff_text: None,
            expression_text: None,
            lyrics: vec![],
            tuplet_beats: 0.0,
            tuplet_number: 0,
            tuplet_count: 0,
            tuplet_start: false,
            tuplet_end: false,
            octave_line_number: 0,
            octave_line_direction: None,
            octave_line_start: false,
            octave_line_end: false,
        }
    }
}

impl Rest {
    pub fn new(duration: i32) -> Self {
        Rest {
            duration,
            dots: 0,
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
        }
    }
}

impl Barline {
    pub fn new(style: &str) -> Self {
        Barline {
            style: style.to_string(),
            ending: None,
            ending_start: false,
            ending_end: false,
        }
    }
}

// ─── WASM input/output types ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreInput {
    pub staves: Vec<StaffInput>,
    pub key: String,
    pub time: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub composer: Option<String>,
    pub arranger: Option<String>,
    pub lyricist: Option<String>,
    pub staff_group: String,
    pub staff_size_mm: f64,
    pub width_mm: Option<f64>,
    pub staff_spacing_mm: f64,
    pub system_spacing_mm: f64,
    pub measures_per_line: Option<i32>,
    pub measure_numbers: String,
    pub music_font: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffInput {
    pub clef: Option<String>,
    pub music: String,
    pub label: Option<String>,
    pub fingering_position: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreOutput {
    pub systems: Vec<SystemOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOutput {
    pub width: f64,
    pub height: f64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub svg: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cmds: Vec<DrawCmd>,
}

/// Internal drawing commands converted to SVG before returning to Typst.
/// All coordinates use the original renderer convention: x-right, y-up, in mm.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum DrawCmd {
    /// Line: from (x1,y1) to (x2,y2) with thickness w (mm)
    #[serde(rename = "L")]
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        w: f64,
    },

    /// Music glyph: place at (x,y), codepoint c, font size s (mm), anchor a
    #[serde(rename = "G")]
    Glyph {
        x: f64,
        y: f64,
        c: u32,
        s: f64,
        a: Cow<'static, str>,
    },

    /// Multi-codepoint music text: render string v with the music font at size s (mm)
    /// Used for composites like dynamics "mf" where kerning/ligatures must be font-handled.
    #[serde(rename = "GM")]
    MusicText {
        x: f64,
        y: f64,
        v: String,
        s: f64,
        a: Cow<'static, str>,
    },

    /// Text: place at (x,y), string v, size s (pt), weight w, italic i, anchor a
    #[serde(rename = "T")]
    Text {
        x: f64,
        y: f64,
        v: String,
        s: f64,
        w: Cow<'static, str>,
        i: bool,
        a: Cow<'static, str>,
    },

    /// Filled polygon (beams): flat array of x,y pairs
    #[serde(rename = "P")]
    Polygon { pts: Vec<f64> },

    /// Filled bezier shape (slurs, ties): two cubic beziers forming a closed region
    /// [x1,y1, c1x,c1y, c2x,c2y, x2,y2, c3x,c3y, c4x,c4y]
    #[serde(rename = "B")]
    BezierFill { pts: Vec<f64> },

    /// Filled circle: center (x,y), radius r
    #[serde(rename = "C")]
    Circle { x: f64, y: f64, r: f64 },

    /// Content batch separator — tells the Typst frontend to flush accumulated
    /// glyph/text items using the batch placement system for performance.
    #[serde(rename = "F")]
    FlushContent,

    /// Move the drawing origin for the next system/staff. dx, dy are offsets in mm.
    #[serde(rename = "M")]
    MoveOrigin { dx: f64, dy: f64 },
}

// ─── Laid-out item (after layout pass) ─────────────────────────────────

#[derive(Debug, Clone)]
pub struct LaidOutItem {
    pub event: Event,
    pub x: f64,
    pub y: f64,
    pub stem_dir: Option<String>,
    pub stem_y_end: Option<f64>,
    pub width: f64,
    pub chord_ys: Vec<f64>,
    pub chord_staff_positions: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct LaidOutStaff {
    pub items: Vec<LaidOutItem>,
    pub total_width: f64,
    pub clef: Option<String>,
    pub time: Option<TimeInfo>,
    pub show_time_prefix: bool,
    pub lyric_prefix_states: Vec<Option<String>>,
}

#[derive(Debug, Clone)]
pub struct TimeInfo {
    pub upper: i32,
    pub lower: i32,
    pub symbol: Option<String>,
}

impl Fingering {
    pub fn values(&self) -> Vec<i32> {
        match self {
            Fingering::Single(v) => vec![*v],
            Fingering::Multiple(vs) => vs.clone(),
        }
    }
}
