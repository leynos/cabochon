# Define the minimum Burin output contract (roadmap 1.0.4)

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
`Tolerances`, `Risks`, `Progress`, `Surprises & discoveries`, `Decision log`,
`Outcomes & retrospective`, `Conformance basis`, and `Verification plan` must
be kept up to date as work proceeds.

Status: DRAFT

Implementation must not begin until a human reviewer explicitly approves this
plan and the 1.0.3 plan (`docs/execplans/1-0-3-lapidary-scene-contract.md`) has
been implemented and merged. Silence is not approval. Implement on a branch
named `1-0-4-burin-output-contract`, in its own pull request.

## Purpose / big picture

Cabochon models its application environment on the Graphics Environment Manager
(GEM): Clerestory is the application environment, Escutcheon the resource
system, Lapidary the device-independent scene model (GEM's Virtual Device
Interface (VDI) analogue), and Burin the renderer. ADR 0007 makes all four
production contracts in hosted mode. Roadmap step 1.0 delivers them as four
task plans: 1.0.3 Lapidary (done before this one), 1.0.4 Burin (this plan),
1.0.2 Escutcheon (independent of this plan), and 1.0.1 Clerestory (which needs
all three).

Burin turns one resolved Lapidary scene into concrete output for every target,
which `docs/context.md` calls a workstation: interactive display, PDF,
print-oriented PDF, raster PNG, thumbnail, accessibility geometry, clipboard
payloads, and a hit-testing map. After this plan, a developer can hand Burin
any Lapidary scene and receive each output, together with proof that they agree
with each other and with the scene: geometry, text placement, clipping, and
metadata match within declared tolerances; redrawing after an edit touches only
the damaged pixels and leaves none stale; and a rendering failure yields a
bounded diagnostic without altering or flattening the scene.

Observable success:

1. `cargo run -p cabochon-burin --example render_all -- --out /tmp/burin`
   writes `content.pdf`, `content-print.pdf`, `content.png`, `thumbnail.png`,
   `accessibility.json`, `clipboard/`, and `hit-map.json` for a sample scene
   containing clipped, rotated, bidirectional text, an image, and hit regions.
2. `make test` passes, including the cross-target parity suite and
   `crates/cabochon-burin/tests/features/output.feature`.
3. Roadmap tasks 1.0.3 and 1.0.4 are marked done, because the parity suite
   proves both success criteria (decision T1).

How to read identifiers: `RM-` roadmap task, `TDD-` technical-design
requirement, `EP-` milestone, `OB-` verification obligation, `A-` axiom, `C`
constraint, `D` decision inherited from the reviewed step-level plan, `T`
decision made for the task-level plans.

## Constraints

- C1, GEM-derived invariants. Burin renders only Lapidary scenes; no provider
  supplies a private renderer (ADR 0008). Burin names no Enfilade type.
- C2, dependency direction, enforced by allow-lists (decision T13).
  `OB-ARCH-DEPS` rows: `cabochon-burin` may depend on `cabochon-lapidary`,
  `vello_cpu`, `krilla`, `parley`, `fontique`, `kurbo`, `png`, `serde`,
  `serde_json`, `thiserror`, `tracing`, and `metrics`; `cabochon-fonts` may
  depend on nothing; the testkit's row gains `cabochon-burin`, `hayro`, and
  `hayro-interpret`. Normal and build edges count.
- Module containment (`OB-ARCH-IMPORTS` rows). Burin is organized so that each
  third-party crate has one home: `vello_cpu` only under `backend::vello` (used
  by the raster, thumbnail, interactive, and hit-map workstations), `krilla`
  only under `backend::krilla` (PDF, print, and clipboard PDF), `parley` and
  `fontique` only under `text`, `png` only under `image`, `kurbo` only under
  `backend` and `float_boundary`, and `serde` and `serde_json` only under
  `wire`, whose private data-transfer types serialize the accessibility,
  clipboard, and hit-map JSON outputs (decision T15). Workstation modules do
  not import one another's backends.
- C3, no third-party types in Burin's public signatures, which expose only
  Lapidary types and Burin-owned types. `OB-ARCH-PUBAPI` row:
  `cabochon-burin`'s public API may name only `std`, `core`, `alloc`, and
  `cabochon_lapidary`.
- C4, integer geometry. Floating-point arithmetic and lossy casts appear only
  in `cabochon_burin::float_boundary`, which `OB-ARCH-IMPORTS` checks: a float
  or cast lint expectation anywhere else fails. Each function there carries a
  reasoned `#[expect(clippy::float_arithmetic, reason = "...")]` (or the
  matching cast lint), as the lint baseline in `docs/developers-guide.md`
  allows. Coordinate casts are exact because Lapidary's `to_backend_units` and
  `to_page_units` bound them below 2^24 (`OB-LAP-BACKEND`).
- C5, lint, safety, and documentation policy as in the repository: `forbid`
  unsafe code, no `#[allow]`, no `.expect()` or `.unwrap()` outside tests, no
  file over 400 lines, and the style guide's doc-comment rules.
- C6, existing contracts, including the `msrv` CI job, keep passing; member
  crates declare no features.
- C12, tolerance integrity: no declared tolerance is loosened silently;
  loosening one is a recorded decision that needs approval.
- C10, library instrumentation only: Burin emits `tracing` and `metrics` but
  installs no subscriber or recorder.

## Tolerances (exception triggers)

- Scope: a milestone exceeds 3,000 net added lines of Rust (excluding fixtures
  and snapshots) or 60 files.
- Interface: a Lapidary contract must change, or a Burin contract changes in a
  way that alters the 1.0.1 plan or roadmap tasks 2.1.2 and 2.1.3.
- Dependencies: a dependency not listed here is needed, or a listed crate must
  move to a new major or pre-1.0 minor series.
- Parity: a measured disagreement exceeds a declared tolerance after three
  focused fix attempts. Loosening a tolerance needs approval.
- Spikes: S-VELLO, S-INVAL, or S-PDF rejects its hypothesis.
- Upstream conflict: evidence contradicts `docs/cabochon-design.md`, an ADR,
  or the roadmap; set `Status: BLOCKED` and record the proposed deviation.
- Iterations: a red test cannot be turned green after five attempts with
  distinct hypotheses.

## Risks

- Risk: the rendering crates are pre-1.0 (`vello_cpu` 0.3, `krilla` 0.8,
  `parley` 0.12, `hayro` 0.8) and churn roughly quarterly. Severity: medium.
  Likelihood: high. Mitigation: one Burin adapter module per crate behind a
  Burin-owned trait; parity tests gate upgrades.
- Risk: `vello_cpu` selects single instruction, multiple data (SIMD) code
  paths at run time, so tests could diverge between hosts. While the Cranelift
  development backend remains (the 1.0.3 plan's EP-M0b expunges it if Kani
  cannot run with it present), Cranelift also implements only part of
  `core::arch`, so tests could crash or diverge between Cranelift and the LLVM
  coverage build. Severity: high. Likelihood: medium. Mitigation: S-VELLO;
  every test render goes through one testkit constructor pinning the scalar
  level, single-threaded rendering, and one render mode; if Cranelift is still
  the development backend and still fails, a per-package profile override
  compiles `vello_cpu` with LLVM (needs approval). The production SIMD path is
  a documented untested residual.
- Risk: font-stack skew (`krilla` pins `skrifa` 0.42; `parley` and
  `vello_cpu` use newer). Severity: low. Likelihood: high. Mitigation: fonts
  cross stacks only as bytes plus glyph identifiers through Lapidary's
  `FontSet`.
- Risk: damage-clipped re-rendering may not be bit-identical to a full
  re-render. Severity: medium. Likelihood: medium. Mitigation: S-INVAL first;
  escalate with the measured difference before adopting any tolerance.
- Risk: `krilla` may round glyph adjustments per glyph and converts quadratic
  curves to cubics; `hayro` may not return right-to-left or ligated text in
  logical order. Severity: medium. Likelihood: medium. Mitigation: S-PDF
  measures both before the vector tolerance is fixed; parity compares on-curve
  points and normalizes text to logical order and NFC.
- Risk: anti-aliasing makes pixel comparison flaky. Severity: medium.
  Likelihood: medium. Mitigation: compare geometry wherever possible; pixel
  checks require exact agreement outside a one-pixel edge band.
- Risk: CI time grows with the heavy dependency set and rendering property
  tests. Severity: medium. Likelihood: high. Mitigation: render suites run 32
  cases with `max_shrink_time` of 30 seconds in the nextest `render` group;
  measure build times in EP-M1 and propose an optimization-level override only
  if justified.
- Risk: `make audit` surfaces advisories in the new tree. Severity: low.
  Likelihood: high. Mitigation: each dependency commit runs `make audit`; any
  ignore is a reviewed decision.
- Risk: bundled fonts add weight and licence obligations. Severity: low.
  Likelihood: certain. Mitigation: static subsets of Open Font Licence (OFL)
  fonts with licence texts and checksums.

## Progress

- [x] (2026-10-18) Step-level plan drafted, reviewed by four
  community-of-experts panels, and revised (commit `616a8fa`).
- [x] (2026-10-22) Reviewer accepted decisions D1, D6, D7, D8, D10, D11, D12,
  D13, and D14 on the step-level plan.
- [x] (2026-10-23) Decomposed into four task-level plans; this is the second.
- [ ] Reviewer approval of this plan; the 1.0.3 plan implemented and merged.
- [ ] EP-M1: fonts, text layout, float boundary, raster, and thumbnail.
- [ ] EP-M2: interactive surface and invalidation.
- [ ] EP-M3: hit map, accessibility, clipboard geometry, failure handling,
  and metrics.
- [ ] EP-M4: PDF read-back device and precision spike.
- [ ] EP-M5: PDF and print workstations, full parity suite, and roadmap
  ticks.

## Surprises & discoveries

- Observation: `krilla` supports PDF/A and PDF/UA but not PDF/X. Impact:
  print-oriented output is PDF/A-2b with page boxes, an sRGB output intent, and
  a print ticket (decision D6).
- Observation: the lint baseline denies float arithmetic and lossy casts, and
  the developers' guide permits a reasoned `#[expect]` for genuine
  floating-point arithmetic. Impact: one float-boundary module (C4).

## Decision log

Inherited from the reviewed step-level plan (accepted ones say so):

- Decision D3: Burin uses `vello_cpu` for raster output, `krilla` for PDF,
  `parley` and `fontique` for text layout, `png` for image decoding, and
  `hayro` (development only) to read PDF output back. `tiny-skia` was
  considered (mature, but no text and a separate stack). This is the hosted
  proving ground's software renderer; the deferred graphics processing unit
  (GPU) renderer decision is unaffected. Recorded as ADR 0011.
- Decision D6 (accepted 2026-10-22): print-oriented output is a paginated
  PDF/A-2b file with media, trim, and bleed boxes, an sRGB output intent, and a
  `PrintTicket` (media, copies, page range). Submission to the Common UNIX
  Printing System (CUPS) or the print portal stays with roadmap 5.1.2; PDF/X is
  deferred.
- Decision D7 (accepted 2026-10-22): no SVG workstation in step 1.0.
- Decision D9: every workstation consumes the same Lapidary `ResolvedScene`,
  and Burin implements Lapidary's `TextLayout` port.
- Decision D15: font policy. `ParleyTextLayout` takes an explicit font
  collection; bundled fonts live in `cabochon-fonts`; tests never use system
  fonts; a script no registered font covers is `NoFontCovers`, never
  missing-glyph boxes. (How the application chooses system fallback is the
  1.0.1 plan's concern.)
- Decision D16: images arrive encoded (PNG) with a declared size and digest;
  Burin decodes them, so decoding failure is a real render failure.
- Decision D20: assertion style (`googletest` matchers, `pretty_assertions`
  for whole values, `insta` for stable summaries; `#[gtest]` before
  `#[rstest]`).

Task-level decisions:

- Decisions T13, T14, and T15 (reviewer direction, 2026-10-23, recorded in
  the 1.0.3 plan): the architecture is enforced by tested guards with
  allow-lists at crate level (`OB-ARCH-DEPS`), lint inheritance
  (`OB-ARCH-LINTS`), module level (`OB-ARCH-IMPORTS`, a `syn` import lint), and
  public-API level (`OB-ARCH-PUBAPI`, using the `public-api` crate), and
  `serde` is confined to private data-transfer types. The 1.0.3 plan builds the
  guards and their rule tables in `tests/architecture/rules.rs`; this plan adds
  its crates' rows, each with a seeded violation that must fail.
- Decision T1: implementation order 1.0.3, then 1.0.4 and 1.0.2, then 1.0.1.
  This plan marks roadmap tasks 1.0.3 and 1.0.4 done, because its parity suite
  proves both success criteria. Date: 2026-10-23.
- Decision T3: Lapidary's Kani-proved `to_backend_units` and `to_page_units`
  produce coordinates bounded below 2^24 in 1/256 units, so Burin's float
  boundary casts them to `f32` exactly and applies the constant scale
  0.003_906_25 (exactly 1/256) in the backend transform. Items wholly outside
  the surface or page are culled before conversion; a visible item whose
  coordinates still exceed the bound yields `RenderError::GeometryOutOfRange`.
  This deliberately narrows the step plan's Kani coverage of the float
  boundary: the reverse direction, rounding `parley`'s `f32` results back to
  EMU, is checked by a property test over every class of finite `f32` (zero,
  subnormal, normal, the domain limits) plus named non-finite cases, rather
  than by Kani, because a Kani harness in Burin would have to build the whole
  rendering stack. Date: 2026-10-23.
- Decision T8: one pull request per task plan, replacing the step plan's
  decision D18, following the reviewer's direction on 2026-10-23. Date:
  2026-10-23.
- Decision T9: ADR numbers are reserved by this decomposition: 0009 for the
  1.0.3 plan, 0010 for the 1.0.2 plan, 0011 for this plan, and 0012 for the
  1.0.1 plan. 0013 is reserved for the 1.0.3 plan's Cranelift decision, if its
  EP-M0b expunges Cranelift (decision T10 there). Any other pull request
  needing an ADR takes 0014 or later. Date: 2026-10-23.
- Decision T4: add a `render_all` example to `cabochon-burin` as this task's
  observable demonstration, since the hello-window application belongs to the
  1.0.1 plan. Date: 2026-10-23.

## Outcomes & retrospective

Not started.

## Context and orientation

When this plan starts, the repository is a Cargo workspace (from the 1.0.3
plan) with `crates/cabochon-lapidary`, `crates/cabochon-substrate-testkit`,
prover tooling, `tests/substrate_dependency_contract.rs`, and
`.config/nextest.toml`. Lapidary provides `Scene`, `resolve`, `ResolvedScene`,
`DeviceMapping`, `PageMapping`, `damage`, `hit_test`, `accessibility_nodes`, the
`TextLayout` port, and `FontSet`. Read the 1.0.3 plan's
`Interfaces and dependencies` and the Lapidary rustdoc before starting.

Governing documents: `docs/cabochon-design.md` §3, §7.1, §8.3, §8.4 (Burin),
§8.9, §11 (failure behaviour), §13.1 (VDI conformance tests), and §14
(observability);
`docs/adrs/adr-0007-make-the-gem-substrate-a-production-boundary.md`;
`docs/adrs/adr-0008-keep-enfilade-out-of-rendering-authority.md`;
`docs/context.md`; and `docs/roadmap.md` tasks 1.0.3 and 1.0.4.

Gates: `make check-fmt`, `make lint`, and `make test` (which runs every
workspace member's tests and doctests). Compilation queues for admission; a
waiting build is not hung.

Terms: workstation (a Burin output target); resolved scene (Lapidary's ordered
display list); damage (rectangles to redraw); device mapping (scene to device
pixels at a 1/120 scale); backend units (1/256 device pixel or 1/256 point,
bounded below 2^24); edge band (pixels within one pixel of any drawn or clip
edge, where anti-aliasing differs).

Skills: `execplans`, `rust-router` (then `rust-types-and-apis`, `rust-errors`,
`rust-performance-and-layout` for render costs), `hexagonal-architecture`,
`rust-unit-testing`, `proptest`, `rust-verification`, `arch-decision-records`,
`nextest`, `en-gb-oxendict-style`, `codegraph-mcp`, and `firecrawl-mcp` for
upstream crate documentation. References as listed in the 1.0.3 plan.

## Conformance basis

Upstream at `main` commit `0de2b57`: `docs/cabochon-design.md` version 0.2,
ADRs 0007 and 0008, and `docs/roadmap.md`. Traced items: `RM-1.0.3`, `RM-1.0.4`,
`TDD-INV-3` (design §3), `TDD-BUR` (§8.4), `TDD-PRES` (§8.9),
`TDD-FAIL-RENDER` (§11, Table 2), `TDD-OBS` (§14), and `TDD-VDI-PARITY`,
`TDD-VDI-PROP`, `TDD-VDI-INVAL`, `TDD-VDI-TEXT`, and `TDD-VDI-FAIL` (§13.1).

```plaintext
RM-1.0.3, RM-1.0.4 -> TDD-BUR, TDD-PRES, TDD-VDI-PARITY, TDD-VDI-PROP -> EP-M1..EP-M5
    -> OB-BUR-PARITY-GEOM, OB-BUR-PARITY-TEXT, OB-BUR-NODROP
RM-1.0.3 -> TDD-VDI-INVAL (pixel half) -> EP-M2 -> OB-BUR-INVAL, OB-BUR-LOCALITY
RM-1.0.4 -> TDD-FAIL-RENDER, TDD-VDI-FAIL -> EP-M3 -> OB-BUR-FAIL, output.feature
RM-1.0.4 -> TDD-VDI-TEXT -> EP-M1, EP-M5 -> OB-TEXT-CARET, OB-TEXT-BIDI, OB-TEXT-FALLBACK
RM-1.0.4 -> print preparation (D6) -> EP-M5 -> OB-BUR-PRINT
TDD-OBS -> EP-M3 -> metrics and span assertions
```

## Verification plan

Burin's obligations depend on real third-party rendering behaviour, so the
right methods are property tests, named and parameterized cases, behavioural
scenarios, and focused snapshots, not formal proof. Burin introduces no
algebraic kernel of its own: its coordinate casts rely on Lapidary's
Kani-proved bounds (`OB-LAP-BACKEND`), and its redraw logic relies on
Lapidary's Verus-proved damage and snapping (`OB-LAP-DAMAGE`, `OB-LAP-SNAP`).
This plan supplies the link those proofs cannot: `OB-BUR-LOCALITY` tests that
the real rasterizer changes pixels only inside an item's bounds, which is the
assumption under which damage soundness implies "no stale pixels".

Non-vacuity mechanics follow the 1.0.3 plan: a manual `TestRunner` with a fixed
seed, per-class counters asserted after the run, and generators that construct
important classes. Render suites run 32 cases with `max_shrink_time` of 30
seconds in the nextest `render` group; regression files are committed.

Axioms:

- `A-RASTER-LOCALITY`: `vello_cpu` changes pixels only within an item's
  conservative bounds (including stroke outsets) dilated by one device pixel,
  and a pixel-aligned rectangular clip stops all writes outside it.
- `A-RASTER-DET`: `vello_cpu`, pinned to its scalar path and single-threaded,
  is deterministic for identical input across machines.
- `A-PDF-PRECISION`: `krilla` writes geometry with the precision S-PDF
  measures, and `hayro` interprets `krilla`'s content streams faithfully.
- `A-TEXT-DET`: `parley` with a fixed font collection and system fonts
  disabled produces identical glyphs and positions for identical input.
- `A-PNG`: the `png` crate decodes conforming PNG files correctly.

Declared tolerances, defined in `crates/cabochon-burin/src/parity/tolerance.rs`
and documented in `docs/cabochon-design.md` §8.4:

- Vector targets (PDF, print): on-curve path points, clip vertices, image
  placements, and glyph origins within the tolerance S-PDF measures; the
  working target is 127 English Metric Units (EMU), 0.01 point, and fixing the
  final value is a recorded decision.
- Raster targets (interactive, raster, thumbnail): item bounds within one
  device pixel; exact pixel agreement with the reference render outside the
  edge band; glyph origins, checked through the raster workstation's recording
  seam (positions passed to the rasterizer after the float boundary), within
  one device pixel.
- Geometry-only targets (accessibility, clipboard geometry, hit map): exact
  EMU equality with the resolved scene.
- Text content: exact equality after normalizing to logical order and NFC
  between the scene's runs, the accessibility extraction, and PDF text read
  back through `hayro`.
- Invalidation: bit-exact equality between an incremental frame and a full
  re-render, and zero writes outside snapped damage, subject to S-INVAL.
- Hit map: agreement with Lapidary's `hit_test` at every pixel centre more
  than one pixel (plus the quarter-pixel curve-flattening tolerance) from any
  region or clip edge.

### Obligations

`OB-TEXT-CARET`: every caret stop `ParleyTextLayout` reports lies on a
grapheme-cluster boundary (oracle: `unicode-segmentation`), ordered visually
within each run. Named cases: a zero-width-joiner emoji sequence, combining
marks, and the Arabic lam-alef ligature; plus a property test.

`OB-TEXT-BIDI`: visual run order for mixed-direction text matches a
`unicode-bidi` oracle, including mixed numerals. Named cases plus a property
test.

`OB-TEXT-FALLBACK`: text whose script the primary font lacks uses the next
registered font; a script no registered font covers yields
`TextLayoutError::NoFontCovers` naming the run. Named cases.

`OB-BUR-PARITY-GEOM` and `OB-BUR-PARITY-TEXT`: every workstation's output
agrees with the shared `ResolvedScene` within the declared tolerances. PDF and
print output are read back through a `hayro-interpret` `Device` in the testkit
that records paths, clips, images, and glyph origins with their transforms.
Method: a property test over generated scenes (the 1.0.3 plan's generator,
bounded to 12 nodes and depth 3), a fixed corpus of named scenes (`rstest` with
`#[files]`), and `insta` snapshots of a normalized, target-independent summary
per workstation (item kinds, rounded bounds, text, metadata), never raw bytes.
Non-vacuity: counters for clipping, rotated transforms, right-to-left text,
images, layers, embedded scenes, and each output intent; seeded faults:
offsetting PDF text by one point and shifting a raster shape by two pixels must
each fail.

`OB-BUR-NODROP`: for every workstation that can be read back, the count of
items of each kind (paths, glyph runs, images) equals the resolved scene's.
Property test.

`OB-BUR-HITMAP`: the hit map lists each region's path, scene-space polygon, and
effective clip (exact), and renders an identifier raster agreeing with
`hit_test` away from edges. Property test plus named cases.

`OB-BUR-A11Y`: the accessibility workstation serializes
`ResolvedScene::accessibility_nodes()` unaltered, and the PDF workstation's
structure tree follows the same nodes. Named cases.

`OB-BUR-CLIP`: the clipboard workstation produces a geometry descriptor with
exact scene bounds, a PNG whose size equals the bounds at the requested
resolution (within one pixel), and a PDF whose media box equals the bounds
within the vector tolerance. Named cases.

`OB-BUR-INVAL`: for any generated scene and edit sequence of length up to 8,
`InteractiveSurface`'s incremental frame equals a full re-render bit for bit,
and a sentinel-filled frame shows no writes outside snapped damage. Property
test.

`OB-BUR-LOCALITY` (exercises `A-RASTER-LOCALITY`): for any generated scene `S`
and item `i`, rendering `S` and `S` with `i` removed or recoloured changes no
pixel outside `i`'s snapped bounds dilated by one pixel. Property test.
Non-vacuity: thick strokes with sharp mitre joins, square caps, combining
marks, and colours distinct from the background; seeded fault: bounds ignoring
the mitre extension must fail.

`OB-BUR-FLOAT`: float-boundary conversions cast Lapidary backend units to `f32`
exactly and round `parley`'s `f32` results back to EMU to nearest, rejecting
non-finite values and results outside the EMU domain. The cast's exactness
rests on `OB-LAP-BACKEND`. The reverse conversion is checked by a property test
whose generator covers zero, subnormal, and normal `f32` values up to and
beyond the domain limits, plus named cases for NaN and infinities (decision
T3). Non-vacuity: counters per class; seeded fault: truncating instead of
rounding must fail.

`OB-BUR-FAIL`: each failure class (unsupported feature, undecodable image,
image size mismatch, resource limit exceeded, backend error) yields a bounded
`RenderError` with a low-cardinality class, leaves the input scene's digest
unchanged, and never substitutes a rasterized copy for text;
`render_with_fallback` returns a diagnostic scene naming the failed item's
path. Named cases with a fault-injecting workstation supplied by dependency
injection, plus the behavioural feature.

`OB-BUR-PRINT`: print output carries the scene's media, trim, and bleed boxes
and an sRGB output intent, excludes non-printable layers, and passes `krilla`'s
PDF/A-2b validator. Named cases read back through `hayro`.

`OB-BUR-DET`: with creation date and document identifier pinned, rendering the
same scene twice produces byte-identical PNG and PDF. Named cases; if `krilla`
is not byte-deterministic, record it and compare normalized content.

`OB-BUR-OBS`: rendering emits one `tracing` span per workstation call and
increments the metrics below with fixed-enumeration labels only. Named cases
with a test recorder installed in the test, never by the library.

## Plan of work

Red-Green-Refactor throughout. Each milestone ends with `make check-fmt`,
`make lint`, and `make test` passing (sequentially, through the `scrutineer`
agent) and a commit; each dependency addition is its own commit with
`make audit`.

### EP-M1: fonts, text layout, float boundary, raster, and thumbnail

First run S-VELLO: `vello_cpu`, pinned to its scalar path, renders a rectangle
with identical pixels under `make test` and `make coverage`. If the 1.0.3 plan
kept Cranelift, this compares Cranelift with LLVM; if it expunged Cranelift,
both builds use LLVM and the spike checks only that the pinned scalar path is
deterministic across the two profiles. Create `crates/cabochon-fonts` (data
only: static subsets of Noto Sans, Noto Sans Arabic, and Noto Sans Hebrew, each
with `OFL.txt` and a `SHA256SUMS` entry) and `crates/cabochon-burin`, and add
both rows to the dependency contract. Record ownership and reuse policies in
`docs/cabochon-design.md` and `docs/repository-layout.md`. Implement
`Workstation` (with associated `Request` and `Output` types), `RenderError`,
`float_boundary`, `ParleyTextLayout` (with `OB-TEXT-*`), PNG decoding,
`RasterWorkstation`, and `ThumbnailWorkstation`, and the testkit render
constructor that pins determinism. Measure cold and warm build times and record
them. Add Burin's and the fonts crate's rows to the architecture guards first,
and run them red with a seeded violation for each. Draft ADR 0011 as
`docs/adrs/adr-0011-<title-slug>.md`.

### EP-M2: interactive surface and invalidation

First run S-INVAL: a damage-clipped re-render equals a full re-render for a
recolour edit. Implement `InteractiveSurface`, a stateful surface whose
`redraw(&mut self, scene, damage, mapping)` redraws only snapped damage and
records written spans through a `FrameSink`. Write `OB-BUR-INVAL` and
`OB-BUR-LOCALITY`.

### EP-M3: hit map, accessibility, clipboard geometry, failure, and metrics

Implement `HitMapWorkstation`, `AccessibilityWorkstation`, the PNG and geometry
parts of `ClipboardWorkstation`, `render_with_fallback`, and the
fault-injection seam. Instrument with spans and the metrics below. Write
`OB-BUR-HITMAP`, `OB-BUR-A11Y`, `OB-BUR-CLIP`, `OB-BUR-FAIL`, `OB-BUR-OBS`, the
raster halves of `OB-BUR-PARITY-*` and `OB-BUR-NODROP`, and the failure
scenario of `output.feature`.

### EP-M4: PDF read-back device and precision spike

Implement the `hayro-interpret` recording `Device` in the testkit and validate
it against handwritten PDFs whose geometry and text are known exactly. Run
S-PDF: render the corpus through `krilla`, read it back, measure the maximum
on-curve and glyph-origin error, and check whether `hayro` returns
right-to-left and ligated text in logical order. Record the measurements and
propose the final vector tolerance for approval.

### EP-M5: PDF and print workstations, parity suite, and roadmap ticks

Implement `PdfWorkstation` (tagged PDF whose structure follows the
accessibility nodes, metadata, real text through `krilla`'s positioned-glyph
API from the shared layout, original PNG bytes, clips, non-printable layers
kept but marked), `PrintWorkstation` (`PrintIntent` page boxes, the PDF/A-2b
validator, sRGB output intent, `PrintTicket`, non-printable layers excluded),
and the PDF part of `ClipboardWorkstation`. Write `OB-BUR-PARITY-GEOM`,
`OB-BUR-PARITY-TEXT`, the vector half of `OB-BUR-NODROP`, `OB-BUR-PRINT`,
`OB-BUR-DET`, and the remaining scenario. Add the `render_all` example. Accept
ADR 0011. Record tolerances in `docs/cabochon-design.md` §8.4 and the SVG
deferral (decision D7) in §15. Mark roadmap tasks 1.0.3 and 1.0.4 done.

## Milestones and plateaus

Conformance check at each boundary: requirements met, design followed,
assumptions valid, no unapproved interface, dependency, trust boundary, or
persisted format, trace links current. Recovery: revert the milestone's
commits. Compatibility decision: none (pre-1.0, internal).

- EP-M1. Outcome: one scene renders to raster and thumbnail with real text.
  Requirements: `RM-1.0.4` (raster, thumbnail), `TDD-VDI-TEXT`. Evidence:
  S-VELLO result; `OB-TEXT-*`, `OB-BUR-FLOAT`, and raster parity cases pass.
  Check: floats only in `float_boundary`.
- EP-M2. Outcome: damage-aware interactive redraw. Requirements: `RM-1.0.3`
  (invalidation), `RM-1.0.4` (interactive), `TDD-VDI-INVAL`. Evidence: S-INVAL
  result, `OB-BUR-INVAL`, `OB-BUR-LOCALITY`.
- EP-M3. Outcome: hit-map, accessibility, and clipboard-geometry outputs,
  bounded failure, and metrics. Requirements: `RM-1.0.4`, `TDD-VDI-FAIL`,
  `TDD-FAIL-RENDER`, `TDD-OBS`. Evidence: the listed obligations and the
  failure scenario.
- EP-M4. Outcome: a validated read-back device and an approved vector
  tolerance. Evidence: handwritten-PDF cases pass; S-PDF results recorded.
- EP-M5. Outcome: PDF and print outputs and the full parity suite.
  Requirements: completes `RM-1.0.3` and `RM-1.0.4`, `TDD-VDI-PARITY`,
  `TDD-VDI-PROP`. Evidence: all `OB-BUR-*` obligations and `output.feature`
  pass; `render_all` writes every output; roadmap 1.0.3 and 1.0.4 marked done.
  Check: decisions D6 and D7 respected.

## Behavioural specification

`crates/cabochon-burin/tests/features/output.feature`:

```gherkin
Feature: Burin renders one scene to every workstation

  Scenario: One scene, every output
    Given a sample scene with clipped, rotated, bidirectional text, an image, and hit regions
    When it is rendered to every workstation
    Then each output's geometry matches the scene within its declared tolerance
    And the text read back from the PDF equals the scene's text runs

  Scenario: Redrawing after an edit leaves no stale pixels
    Given the sample scene rendered to the interactive surface
    When one shape is recoloured and the surface redraws its damage
    Then the frame equals a full re-render
    And no pixel outside the snapped damage was written

  Scenario: A renderer failure is bounded and harmless
    Given a scene whose PNG image bytes are corrupt
    When it is rendered to the raster workstation
    Then the result is an "undecodable-image" render error naming the image node
    And the scene is unchanged
    And the fallback output is a diagnostic scene, not a flattened copy
```

## Concrete steps

```bash
BRANCH=$(git branch --show-current)
make check-fmt 2>&1 | tee /tmp/check-fmt-cabochon-${BRANCH}.out
make lint 2>&1 | tee /tmp/lint-cabochon-${BRANCH}.out
make test 2>&1 | tee /tmp/test-cabochon-${BRANCH}.out
make audit 2>&1 | tee /tmp/audit-cabochon-${BRANCH}.out
cd /tmp && cargo +1.85.0 check --lib --all-features --locked \
  --manifest-path "$OLDPWD/Cargo.toml"
```

The last command reproduces the `msrv` CI job from outside the checkout
(constraint C6).

Focused loops and the demonstration:

```bash
cargo nextest run -p cabochon-burin --test parity
cargo run -p cabochon-burin --example render_all -- --out /tmp/burin
ls /tmp/burin
```

```plaintext
accessibility.json  clipboard  content-print.pdf  content.pdf  content.png  hit-map.json  thumbnail.png
```

## Validation and acceptance

- Tests: `make test` passes with every obligation's tests and
  `output.feature`.
- Lint and formatting: `make check-fmt` and `make lint` pass with no
  `#[expect]` outside `float_boundary`.
- Architecture: the Burin and fonts rows of `OB-ARCH-DEPS`,
  `OB-ARCH-IMPORTS`, and `OB-ARCH-PUBAPI` pass, and each row's seeded violation
  (an unlisted dependency, `vello_cpu` imported outside `backend::vello`, and a
  public function returning a `krilla` type) fails.
- Security: no `unsafe`, no process-environment access, `make audit` passes
  with any ignore recorded as a decision.
- Documentation: `make markdownlint`, `make nixie`, and `make spelling` pass.

Record spike results and red, green, and refactor evidence in
`Artefacts and notes`.

## Idempotence and recovery

Every step is additive and rerunnable. `render_all` writes only inside the
requested directory and overwrites its own files. Accept `insta` snapshot
changes only after confirming an intended contract change.

## Artefacts and notes

Record S-VELLO, S-INVAL, and S-PDF results, build-time measurements, the final
vector tolerance, and red, green, and refactor transcripts here.

## Interfaces and dependencies

```rust,no_run
pub trait Workstation {
    type Request;
    type Output;
    fn render(&self, scene: &ResolvedScene, request: &Self::Request) -> Result<Self::Output, RenderError>;
}

pub struct InteractiveSurface { /* frame buffer and previous scene */ }
impl InteractiveSurface {
    pub fn redraw(
        &mut self,
        scene: &ResolvedScene,
        damage: &Damage,
        mapping: &DeviceMapping,
    ) -> Result<FrameReport, RenderError>;
    pub fn pixels(&self) -> &[u32];
}

pub enum RenderError {
    UnsupportedFeature { workstation: WorkstationKind, feature: FeatureKind },
    UndecodableImage { node: NodePath },
    ImageSizeMismatch { node: NodePath },
    ResourceLimit { workstation: WorkstationKind, limit: LimitKind },
    GeometryOutOfRange { node: NodePath },
    Backend { workstation: WorkstationKind, class: BackendFailureClass },
}

pub struct ParleyTextLayout { /* font collection injected at construction */ }
impl ParleyTextLayout {
    pub fn new(fonts: FontCollectionSpec) -> Result<Self, FontLoadError>;
}

pub fn render_with_fallback<W: Workstation>(
    workstation: &W,
    scene: &ResolvedScene,
    request: &W::Request,
) -> RenderOutcome<W::Output>;
```

Workstations: `RasterWorkstation`, `ThumbnailWorkstation`, `PdfWorkstation`,
`PrintWorkstation`, `AccessibilityWorkstation`, `ClipboardWorkstation`, and
`HitMapWorkstation`, plus the stateful `InteractiveSurface`. `ParleyTextLayout`
implements Lapidary's `TextLayout`. Metrics, described with `describe_counter!`
and `describe_histogram!`: `burin_render_total{workstation, outcome}`,
`burin_render_failures_total{workstation, class}`, and
`burin_render_seconds{workstation}`.

Dependencies (caret requirements; newest compatible release when each milestone
starts):

- `cabochon-burin`: `cabochon-lapidary`, `vello_cpu` 0.3, `krilla` 0.8,
  `parley` 0.12, `fontique` 0.12, `kurbo` 0.13, `png` 0.18, `serde` 1,
  `serde_json` 1, `thiserror` 2, `tracing` 0.1, `metrics` 0.24.
- `cabochon-fonts`: no dependencies.
- Development: `hayro` 0.8, `hayro-interpret` 0.8, `unicode-segmentation` 1,
  `unicode-bidi` 0.3, `metrics-util` (the release matching `metrics` 0.24, for
  its debugging recorder), and `tracing-test` 0.2, plus the testing crates from
  the 1.0.3 plan. The testkit gains normal dependencies on `cabochon-burin`,
  `hayro`, and `hayro-interpret`; update its row in the dependency contract.

## Documentation obligations

- ADR 0011 (software renderer and workstation parity) under `docs/adrs/`,
  referenced from `docs/cabochon-design.md` §15.
- `docs/cabochon-design.md` §8.4: the workstation contract, declared
  tolerances, and failure classes; §13.1: the delivered VDI conformance suite
  (parity, invalidation, text, and failure tests) and its tolerances; §15: the
  SVG deferral (D7) and print boundary (D6).
- `docs/developers-guide.md`: the float-boundary policy, render-test budgets
  and the `render` test group, the determinism constructor, and the untested
  production SIMD path. Ownership of `cabochon-fonts` (data only) and the
  testkit read-back device.
- `docs/repository-layout.md` and `docs/contents.md`: the new crates.
- `docs/roadmap.md`: 1.0.3 and 1.0.4 marked done.

## Revision notes

- 2026-10-23: created by decomposing the reviewed step-level plan (commit
  `616a8fa`). This plan carries the step plan's EP-M3a to EP-M4b, its Burin and
  text obligations, and the roadmap ticks for 1.0.3 and 1.0.4. New here: the
  `render_all` demonstration (decision T4) and Burin's reliance on Lapidary's
  Kani-proved backend units instead of its own harness (decision T3).
- 2026-10-23: applied the cross-plan consistency review: renamed the
  tolerance constraint C12 (C8 means "no proof shortcuts" elsewhere), made
  decision T3's narrowing of float-boundary coverage explicit with a property
  test for the reverse direction, added culling and a `GeometryOutOfRange`
  render error, added test-recorder dependencies, gave this plan the §13.1
  design update, added the `msrv` command, and recorded one pull request per
  plan (T8) and the ADR number reservation (T9).
- 2026-10-23: made S-VELLO and the SIMD risk conditional on whether the 1.0.3
  plan keeps Cranelift (Kani takes precedence there; its EP-M0b may expunge
  Cranelift), and reserved ADR 0013 for that decision (T9).
- 2026-10-23: added Burin's rows to the architecture guards (decisions T13
  to T15 from the 1.0.3 plan): a crate allow-list, one module home per
  third-party crate (`backend::vello`, `backend::krilla`, `text`, `image`,
  `wire`), float expectations confined to `float_boundary`, a public-API
  allow-list, and private data-transfer types for JSON outputs.
