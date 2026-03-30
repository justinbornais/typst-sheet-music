# typst-sheet-music
Render sheet music directly inside Typst documents using SMuFL-aware glyph placement and Typst drawing primitives.

Features
- SMuFL/Bravura-aware glyph placement (precise bbox & anchors)
- Metadata-driven rendering for clefs, key/time signatures, noteheads, stems, flags, augmentation dots
- Accurate ledger-line computation for edge pitches
- Accidental-aware spacing and extra padding for first-note collisions
- Full-line stretching with improved per-note spacing heuristics
- System/line breaks via literal newlines, measures-per-line, or automatic width-based breaking
- Fingering numbers above notes
- Tunable spacing parameters (note spacing base, duration factors, accidental padding, dot size)

Quick start
1. Add this repository (or copy `lib.typ` + `src/` files) into your Typst project directory.
2. Import the library from your Typst document:

```
import "lib.typ"
```

Sample input
Here is a minimal example showing an inline use of the library. Adjust the string format and function names to match your desired melody.

```
import "lib.typ"

// A small melody; the library provides `score` and `melody` helpers
score(
	melody("C4 q D4 q E4 h | C4 q D4 q E4 h", fingerings=[1,2,3,1,2,3,1,2]),
	measures_per_line=2
)
```

How to incorporate into your Typst documents
- Place the library files (for example `lib.typ` and the `src/` folder) in your project root or a subfolder.
- Import the library with `import "lib.typ"` (or `import "src/lib.typ"` if you keep sources under `src/`).
- Use the provided API functions such as `melody()` and `score()` to construct music objects, then place them in your Typst layout.

Compiling and exporting
You can compile examples or your Typst documents with the `typst` CLI. Example (used in this repository):

```
typst compile examples/ode-to-joy.typ --font-path fonts/ --root .
```

Notes and tuning
- This project uses Bravura SMuFL metadata for accurate glyph placement; ensure the fonts referenced in `--font-path` include the SMuFL font (e.g., Bravura).
- Spacing parameters (note spacing base, minimum duration factor, accidental padding, dot radius) are tunable in the library constants if you want to tweak engraving for different fonts or PPIs.
- System breaks are primarily controlled by literal newlines in the music string; `measures_per_line` and automatic width-based breaking are available as fallbacks.

If you want a quick visual check, compile `examples/ode-to-joy.typ` (this repository includes that example).

Contributing
- Bug reports, feature requests, and pull requests are welcome.

License
- See `REQUIREMENTS.md` for environment and dependency notes.
