use std::collections::HashMap;

/// Diatonic index for each note name (C=0..B=6).
pub fn note_to_diatonic(name: &str) -> i32 {
    match name {
        "c" => 0,
        "d" => 1,
        "e" => 2,
        "f" => 3,
        "g" => 4,
        "a" => 5,
        "b" => 6,
        _ => 0,
    }
}

/// Top-line diatonic number for each clef.
pub fn clef_top_line_diatonic(clef: &str) -> i32 {
    match clef {
        "treble" | "treble-8a" | "treble8a" | "treble-8b" | "treble8b" | "treble-8" | "treble8"
        | "treble-15a" | "treble-15b" | "percussion" => 38, // F5
        "bass" | "bass-8a" | "bass8a" | "bass-8b" | "bass8b" | "bass-15a" | "bass-15b" => 26, // A3
        "alto" => 32,                                                                         // G4
        "tenor" => 30,                                                                        // E4
        _ => 38,
    }
}

/// Map variant clef names to their base clef for staff position calculation.
fn base_clef(clef: &str) -> &str {
    match clef {
        "treble-8a" | "treble8a" | "treble-8b" | "treble8b" | "treble-8" | "treble8"
        | "treble-15a" | "treble-15b" => "treble",
        "bass-8a" | "bass8a" | "bass-8b" | "bass8b" | "bass-15a" | "bass-15b" => "bass",
        _ => clef,
    }
}

/// Compute the diatonic number of a pitch.
pub fn pitch_to_diatonic(name: &str, octave: i32) -> i32 {
    note_to_diatonic(name) + octave * 7
}

/// Compute staff position for a note given a clef.
/// 0 = top staff line, each +1 = one half-space downward.
pub fn staff_position(name: &str, octave: i32, clef: &str) -> i32 {
    let used = base_clef(clef);
    let diatonic = pitch_to_diatonic(name, octave);
    let top = clef_top_line_diatonic(used);
    top - diatonic
}

/// Determine stem direction based on staff position.
pub fn auto_stem_direction(staff_pos: i32) -> &'static str {
    if staff_pos <= 4 {
        "down"
    } else {
        "up"
    }
}

/// Compute stem end Y position (in staff-space units, y-up convention).
pub fn compute_stem_end_y(
    note_y: f64,
    staff_pos: i32,
    stem_dir: &str,
    staff_space: f64,
    min_length: f64,
) -> f64 {
    let mut length = min_length * staff_space;
    if staff_pos < -2 && stem_dir == "down" {
        length = length.max((-staff_pos as f64) * staff_space / 2.0);
    }
    if staff_pos > 10 && stem_dir == "up" {
        length = length.max((staff_pos - 8) as f64 * staff_space / 2.0);
    }
    if stem_dir == "up" {
        note_y + length
    } else {
        note_y - length
    }
}

/// Number and direction of ledger lines needed.
pub fn ledger_lines_needed(staff_pos: i32) -> (i32, Option<&'static str>) {
    if staff_pos <= -2 {
        let count = staff_pos.unsigned_abs() as i32 / 2;
        (count, Some("above"))
    } else if staff_pos >= 10 {
        let count = (staff_pos - 8) / 2;
        (count, Some("below"))
    } else {
        (0, None)
    }
}

/// Default base octave for a clef.
pub fn clef_default_base_octave(clef: &str) -> i32 {
    match clef {
        "bass" | "bass-8a" | "bass8a" | "bass-8b" | "bass8b" | "bass-15a" | "bass-15b" => 3,
        _ => 4,
    }
}

/// Key signature accidental count (positive = sharps, negative = flats).
pub fn key_sig_accidental_count(key: &str) -> i32 {
    match key {
        "C" => 0,
        "G" => 1,
        "D" => 2,
        "A" => 3,
        "E" => 4,
        "B" => 5,
        "F#" => 6,
        "C#" => 7,
        "F" => -1,
        "Bb" => -2,
        "Eb" => -3,
        "Ab" => -4,
        "Db" => -5,
        "Gb" => -6,
        "Cb" => -7,
        "a" => 0,
        "e" => 1,
        "b" => 2,
        "f#" => 3,
        "c#" => 4,
        "g#" => 5,
        "d#" => 6,
        "a#" => 7,
        "d" => -1,
        "g" => -2,
        "c" => -3,
        "f" => -4,
        "bb" => -5,
        "eb" => -6,
        "ab" => -7,
        _ => 0,
    }
}

/// Key signature sharp positions by clef (staff positions for each accidental).
pub fn key_sig_sharp_positions(clef: &str) -> [i32; 7] {
    match clef {
        "bass" => [2, 5, 1, 4, 7, 3, 6],
        "alto" => [1, 4, 0, 3, 6, 2, 5],
        "tenor" => [3, 6, 2, 5, 1, 4, 7],
        _ => [0, 3, -1, 2, 5, 1, 4], // treble and variants
    }
}

/// Key signature flat positions by clef.
pub fn key_sig_flat_positions(clef: &str) -> [i32; 7] {
    match clef {
        "bass" => [6, 3, 7, 4, 8, 5, 2],
        "alto" => [5, 2, 6, 3, 7, 4, 1],
        "tenor" => [7, 4, 1, 5, 2, 6, 3],
        _ => [4, 1, 5, 2, 6, 3, 7], // treble and variants
    }
}

/// Get the notes affected by a key signature.
pub fn key_sig_accidentals(key: &str) -> HashMap<String, String> {
    let count = key_sig_accidental_count(key);
    let sharp_order = ["f", "c", "g", "d", "a", "e", "b"];
    let flat_order = ["b", "e", "a", "d", "g", "c", "f"];
    let mut result = HashMap::new();
    if count > 0 {
        for i in 0..count.min(7) as usize {
            result.insert(sharp_order[i].to_string(), "sharp".to_string());
        }
    } else if count < 0 {
        for i in 0..(-count).min(7) as usize {
            result.insert(flat_order[i].to_string(), "flat".to_string());
        }
    }
    result
}

/// Check if a clef name is supported.
pub fn is_supported_clef(clef: &str) -> bool {
    matches!(
        clef,
        "treble"
            | "bass"
            | "alto"
            | "tenor"
            | "treble-8a"
            | "treble8a"
            | "treble-8b"
            | "treble8b"
            | "treble-8"
            | "treble8"
            | "treble-15a"
            | "treble-15b"
            | "bass-8a"
            | "bass8a"
            | "bass-8b"
            | "bass8b"
            | "bass-15a"
            | "bass-15b"
            | "percussion"
    )
}
