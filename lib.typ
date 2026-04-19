// lib.typ - Main entry point for the scorify package (WASM-backed)
//
// Parsing, layout, and rendering command generation are performed by a Rust
// core compiled to WebAssembly. The Typst frontend serializes user-facing
// parameters to JSON, calls the WASM plugin, and embeds the returned SVG systems
// as vector images.

#let scorify-wasm = plugin("scorify_wasm.wasm")

// Header (standard Typst content, outside the SVG systems)

#let render-header(
  title: none,
  subtitle: none,
  composer: none,
  arranger: none,
  lyricist: none,
) = {
  if title == none and composer == none { return }
  block(width: 100%, {
    if title != none {
      align(center, text(size: 18pt, weight: "bold", title))
    }
    if subtitle != none {
      align(center, text(size: 12pt, style: "italic", subtitle))
    }
    v(2pt)
    if composer != none or arranger != none or lyricist != none {
      grid(
        columns: (1fr, 1fr),
        {
          if lyricist != none {
            align(left, text(size: 10pt, "Text: " + lyricist))
          }
        },
        {
          if composer != none {
            align(right, text(size: 10pt, composer))
          }
          if arranger != none {
            align(right, text(size: 9pt, style: "italic", "arr. " + arranger))
          }
        },
      )
    }
    v(6pt)
  })
}

/// Render a complete music score.
///
/// This is the primary entry point for the scorify library.
///
/// Parameters:
/// - staves: array of staff dictionaries, each with:
///     - clef: "treble", "bass", "alto", "tenor", "treble-8a", etc.
///     - music: music string (see syntax reference)
///     - label: optional staff label
/// - key: key signature string ("C", "G", "D", "Bb", "f#", etc.)
/// - time: time signature string ("4/4", "3/4", "6/8", "C"/"common", "C|"/"cut")
/// - title: piece title
/// - subtitle: subtitle
/// - composer: composer name
/// - arranger: arranger name
/// - lyricist: lyricist name
/// - staff-group: "none", "grand", "choir", "orchestra"
/// - staff-size: staff space distance (default 1.75mm)
/// - system-spacing: vertical space between systems
/// - staff-spacing: vertical space between staves within a system
/// - music-font: SMuFL font family (defaults to Leland)
/// - width: explicit width or auto
/// - measure-numbers: "system", "every", "none"
/// - measures-per-line: if set, force this many measures per system line
#let score(
  staves: (),
  lyrics: (),
  chords: (),
  key: "C",
  time: none,
  tempo: none,
  title: none,
  subtitle: none,
  composer: none,
  arranger: none,
  lyricist: none,
  copyright: none,
  staff-group: "none",
  staff-size: 1.75mm,
  system-spacing: 12mm,
  staff-spacing: 8mm,
  lyric-line-spacing: none,
  music-font: "Leland",
  music-font-metadata: none,
  width: auto,
  measure-numbers: "system",
  relative-octave: false,
  measures-per-line: none,
) = {
  if staves.len() == 0 { return }

  // Alternate fonts are resolved through Typst so missing font-path setup still
  // produces a CLI warning. Default Leland is rendered from the WASM bundle.
  if music-font != "Leland" {
    box(width: 0pt, height: 0pt, hide(text(font: music-font, size: 0.1pt, "\u{E050}")))
  }

  render-header(
    title: title,
    subtitle: subtitle,
    composer: composer,
    arranger: arranger,
    lyricist: lyricist,
  )

  let render-inner(avail-width-mm) = {
    let input = (
      staves: staves.map(s => (
        clef: s.at("clef", default: none),
        music: s.at("music", default: ""),
        label: s.at("label", default: none),
        fingering_position: s.at("fingering-position", default: "above"),
      )),
      key: key,
      time: time,
      title: none,
      subtitle: none,
      composer: none,
      arranger: none,
      lyricist: none,
      staff_group: staff-group,
      staff_size_mm: staff-size / 1mm,
      width_mm: avail-width-mm,
      staff_spacing_mm: staff-spacing / 1mm,
      system_spacing_mm: system-spacing / 1mm,
      measures_per_line: measures-per-line,
      measure_numbers: measure-numbers,
      music_font: music-font,
    )

    let result-bytes = scorify-wasm.render_score(bytes(json.encode(input)))
    let result = json(result-bytes)

    for system in result.systems {
      block(image(bytes(system.svg), format: "svg"))
      v(system-spacing)
    }
  }

  if width == auto {
    layout(size => {
      render-inner(size.width / 1mm)
    })
  } else {
    render-inner(width / 1mm)
  }
}

/// Quick single-staff melody rendering.
///
/// A convenience wrapper around `score()` for simple melodies.
#let melody(
  music: "",
  key: "C",
  time: none,
  clef: none,
  title: none,
  composer: none,
  staff-size: 1.75mm,
  system-spacing: 12mm,
  lyric-line-spacing: none,
  music-font: "Leland",
  music-font-metadata: none,
  width: auto,
  measures-per-line: none,
) = {
  score(
    staves: ((clef: clef, music: music),),
    key: key,
    time: time,
    title: title,
    composer: composer,
    staff-size: staff-size,
    system-spacing: system-spacing,
    lyric-line-spacing: lyric-line-spacing,
    music-font: music-font,
    music-font-metadata: music-font-metadata,
    width: width,
    measures-per-line: measures-per-line,
  )
}

/// Chord chart rendering (not yet implemented).
#let chord-chart(
  chords: "",
  key: "C",
  time: "4/4",
  title: none,
  width: auto,
) = {
  // Stub - not yet implemented
}
