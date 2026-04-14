// lib.typ - Main entry point for the scorify package (WASM-backed)
//
// All parsing, layout, and rendering computation is performed by a native
// Rust core compiled to WebAssembly.  The Typst frontend serialises the
// user-facing parameters to JSON, calls the WASM plugin, and executes the
// returned drawing commands in a CeTZ canvas.

#import "@preview/cetz:0.4.2"

#let scorify-wasm = plugin("scorify_wasm.wasm")

// ─── Header (standard Typst content, outside CeTZ) ────────────────────

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

// ─── Draw-command executor ─────────────────────────────────────────────

#let execute-draw-cmds(cmds, music-font: "Bravura") = {
  import cetz.draw: *
  for cmd in cmds {
    if cmd.t == "L" {
      // Line
      line(
        (cmd.x1, cmd.y1), (cmd.x2, cmd.y2),
        stroke: (paint: black, thickness: cmd.w * 1mm, cap: "butt"),
      )
    } else if cmd.t == "G" {
      // Music glyph
      content(
        (cmd.x, cmd.y),
        text(font: music-font, size: cmd.s * 1pt, str.from-unicode(cmd.c)),
        anchor: cmd.a,
      )
    } else if cmd.t == "T" {
      // Text
      content(
        (cmd.x, cmd.y),
        text(
          size: cmd.s * 1pt,
          weight: cmd.w,
          style: if cmd.i { "italic" } else { "normal" },
          cmd.v,
        ),
        anchor: cmd.a,
      )
    } else if cmd.t == "P" {
      // Filled polygon (beams)
      let pairs = ()
      let j = 0
      while j + 1 < cmd.pts.len() {
        pairs.push((cmd.pts.at(j), cmd.pts.at(j + 1)))
        j += 2
      }
      line(..pairs, close: true, fill: black, stroke: none)
    } else if cmd.t == "B" {
      // Filled bezier (slurs, ties)
      let p = cmd.pts
      merge-path(fill: black, stroke: none, {
        bezier(
          (p.at(0), p.at(1)), (p.at(6), p.at(7)),
          (p.at(2), p.at(3)), (p.at(4), p.at(5)),
        )
        bezier(
          (p.at(6), p.at(7)), (p.at(0), p.at(1)),
          (p.at(8), p.at(9)), (p.at(10), p.at(11)),
        )
      })
    } else if cmd.t == "C" {
      // Filled circle (dots)
      circle((cmd.x, cmd.y), radius: cmd.r, fill: black, stroke: none)
    } else if cmd.t == "M" {
      // Move CeTZ origin for next staff / system
      set-origin((cmd.dx, cmd.dy))
    }
    // "F" (FlushContent) is a no-op on the Typst side
  }
}

// ─── Public API ────────────────────────────────────────────────────────

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
/// - music-font: SMuFL font family (defaults to Bravura)
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
  music-font: "Bravura",
  music-font-metadata: none,
  width: auto,
  measure-numbers: "system",
  relative-octave: false,
  measures-per-line: none,
) = {
  if staves.len() == 0 { return }

  // Header (Typst content, not CeTZ)
  render-header(
    title: title,
    subtitle: subtitle,
    composer: composer,
    arranger: arranger,
    lyricist: lyricist,
  )

  let render-inner(avail-width-mm) = {
    // Build WASM input — all keys use snake_case to match Rust struct fields
    let input = (
      staves: staves.map(s => (
        clef: s.at("clef", default: none),
        music: s.at("music", default: ""),
        label: s.at("label", default: none),
        fingering_position: s.at("fingering-position", default: "above"),
      )),
      key: key,
      time: time,
      // Headers are rendered by Typst above; don't duplicate in WASM
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
      cetz.canvas(
        length: 1mm,
        execute-draw-cmds(system.cmds, music-font: music-font),
      )
      v(system-spacing)
    }
  }

  // Resolve width
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
  music-font: "Bravura",
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
