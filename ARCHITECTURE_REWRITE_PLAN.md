# Scorify Architecture Rewrite Plan

## Goal

Rebuild Scorify so it can deliver a large compile/render speedup, ideally in the 70-90% range and at minimum 50%, while keeping these constraints:

1. Future features must remain easy to add.
2. Placement and engraving logic must stay equivalent to the current behavior.
3. The public Typst import must stay the same, e.g. `#import "@preview/scorify:0.3.0": score, melody`, with no extra user setup.

## Recommendation

The only architecture that has a realistic chance of reaching 50%+ while keeping the Typst-facing API stable is a **split architecture**:

- Keep a tiny Typst package as the public entry point.
- Move parsing, layout, collision solving, and score IR generation into a native core.
- Render through a backend that is not Typst/CeTZ.
- Preserve the existing engraving rules by porting them, not redesigning them.

If Typst cannot directly host that backend in-package, then the package should still expose the same Typst API and delegate to the backend through whatever first-class extension path Typst supports at the time. If no such path exists, then the 50-90% target is not compatible with requirement 3.

## Best Option

### Option A: Typst package front-end + native Rust rendering core

This is the recommended approach if the goal is the biggest possible speedup without changing the user-facing Typst API.

#### What stays the same

- `lib.typ` remains the public entry point.
- `score()` and `melody()` keep the same call shape.
- The layout and placement rules remain the same.
- The output still targets PDF, but the renderer is no longer Typst/CeTZ.

#### What changes

- Parsing moves out of Typst macros and into Rust.
- Layout and spacing become explicit IR transformations.
- The renderer becomes a native PDF/vector backend.
- The Typst package becomes a compatibility layer that converts Typst input into backend input and returns the rendered artifact.

#### Why this is the best path

- It removes Typst evaluation overhead from the hot path.
- It removes CeTZ call overhead from the hot path.
- It gives full control over caching, object reuse, and memory layout.
- It keeps feature growth manageable because new notation features become new IR nodes or passes, not more Typst macro complexity.

## Alternate Options

| Option | Summary | Pros | Cons | Likely Speed Gain |
|---|---|---|---|---|
| A. Typst front-end + native Rust backend | Typst stays as API surface; Rust does parsing, layout, and rendering | Highest realistic ceiling; easiest to keep logic identical; best long-term maintainability for complexity | Requires the largest rewrite; needs a viable way to host or invoke the backend while preserving Typst importability | 50-90% possible |
| B. Typst front-end + Rust/WASM analysis core + native renderer | Same as A, but analysis logic is packaged as WASM for portability | Good portability; clean separation of logic; easier to reuse outside Typst later | WASM itself does not create the speedup; still needs a native render backend; Typst integration path may still be a constraint | 50-90% only if the render backend is native |
| C. Typst-only rewrite and optimization | Keep rendering in Typst/CeTZ but reduce repeated work and simplify passes | Lowest risk; preserves everything exactly; no new runtime requirements | Ceiling is much lower; unlikely to exceed modest double-digit improvements | 10-25% typical, not 50%+ |

## Recommended Target Architecture

### 1. Public Typst package

The package remains the user entry point and exports the same functions:

- `score(...)`
- `melody(...)`

The Typst layer should do only three things:

1. Normalize user input.
2. Serialize score data into a backend-friendly IR.
3. Hand off rendering to the backend and expose the result to Typst.

### 2. Score IR

Define a backend-neutral intermediate representation with stable semantics:

- score metadata
- staves and staff groups
- measures and systems
- notes, rests, chords, clefs, key signatures, time signatures
- beams, slurs, ties, tuplets, hairpins, trills, octave lines
- fingerings, lyrics, staff text, chord symbols, endings, markers
- collision metadata and spacing constraints

This IR is the compatibility contract. Future features should extend it, not fork it.

### 3. Layout engine

Move the current logic into a dedicated layout engine with explicit passes:

1. Parse and normalize events.
2. Build beat and measure structure.
3. Compute horizontal spacing.
4. Compute staff positions and stem directions.
5. Resolve collisions for lyrics, fingerings, chord symbols, dynamics, articulations, trills, endings, and octave lines.
6. Group beamed figures, tuplets, slurs, ties, and spans.
7. Produce a final positioned IR.

Each pass should be pure or nearly pure so it can be tested independently.

### 4. Rendering backend

Use a native renderer that writes PDF directly or writes through a fast vector backend such as Cairo or Skia.

The renderer should:

- consume positioned IR only,
- place glyphs using SMuFL metrics,
- emit lines, paths, text, and barlines,
- avoid recomputing layout decisions,
- keep output stable across runs.

## Migration Plan

### Phase 0: Freeze current behavior

1. Capture golden PDFs for the existing examples and tests.
2. Add a regression suite that compares output visually and, where possible, by hash.
3. Record current compile and render timings on representative scores.

### Phase 1: Define the IR

1. Enumerate all event types currently supported by Scorify.
2. Define stable data structures for each event and span.
3. Define spacing and collision annotations separately from the musical event data.
4. Write a serializer/deserializer for the IR.

### Phase 2: Move parsing out of Typst

1. Port the music-string parser into Rust.
2. Preserve every syntax rule and edge case.
3. Add parser tests using the current examples and parser fixtures.
4. Keep the Typst API unchanged by passing the same user arguments into the new parser.

### Phase 3: Move layout and spacing logic

1. Port duration spacing, measure breaking, and system breaking.
2. Port beam grouping, tuplet grouping, tie/slur span detection, and repeat/endings handling.
3. Port note/chord vertical placement and stem direction rules.
4. Port all collision and stacking logic for lyrics, dynamics, fingerings, chord symbols, staff text, trills, octave lines, and articulations.

### Phase 4: Build the renderer backend

1. Implement glyph lookup against SMuFL metadata in native code.
2. Render the positioned IR into PDF or a vector backend output.
3. Match the current visual output exactly, including spacing, ledger lines, and span placements.
4. Validate against the golden PDFs from Phase 0.

### Phase 5: Keep the Typst import contract intact

1. Leave `score()` and `melody()` as the only public entry points.
2. Keep the package installation and import path unchanged.
3. Hide the backend behind the package so users do not add extra steps.
4. Preserve backwards-compatible defaults and typography.

### Phase 6: Make feature growth easy

1. Add feature modules, not ad hoc branches.
2. Introduce one pass per feature family where practical.
3. Keep IR additions additive and versioned.
4. Require new features to come with layout and output tests before merge.

## Design Rules for Future Features

To keep the system easy to extend:

- Every notation feature should live in a clearly named pass.
- Rendering should depend on a finalized IR, not on the original parser shape.
- Collision rules should be data-driven and testable.
- Glyph metrics should be cached centrally.
- New features should add IR nodes or annotations, not duplicate the rendering pipeline.

## What Not to Do

- Do not keep CeTZ in the hot path if the goal is 50%+ speedup.
- Do not move only parsing to WASM and expect large wins.
- Do not split logic across Typst and a backend in a way that duplicates layout decisions.
- Do not let the Typst layer become a second renderer.

## Feasibility Notes

- A Typst-only rewrite can improve speed, but it is unlikely to reach 50-90%.
- WASM is useful as a packaging and portability layer, not as the main performance lever.
- The speed target becomes realistic only if the render engine moves out of Typst/CeTZ.
- If Typst cannot host or invoke that backend without extra user setup, then requirement 3 and the 50-90% target are in tension.

## Suggested Decision

If you want the highest probability of a massive speedup while keeping the current engraving logic, choose **Option A**:

1. Keep the Typst package surface.
2. Port the musical logic into a backend-neutral IR and native core.
3. Render natively.
4. Keep Typst as the front door, not the engine.

If you want portability or future reuse, add WASM later as a packaging layer around the same core.
