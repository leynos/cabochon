# Define the minimum Lapidary scene contract (roadmap 1.0.3)

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
`Tolerances`, `Risks`, `Progress`, `Surprises & discoveries`, `Decision log`,
`Outcomes & retrospective`, `Conformance basis`, and `Verification plan` must
be kept up to date as work proceeds.

Status: DRAFT

Implementation must not begin until a human reviewer explicitly approves this
plan. Silence is not approval. Implement on a branch named
`1-0-3-lapidary-scene-contract`, in its own pull request.

## Purpose / big picture

Cabochon is a document-centred application environment modelled on the Graphics
Environment Manager (GEM). GEM split the desktop into an Application
Environment Services (AES) layer for windows, menus, and dialogs, and a Virtual
Device Interface (VDI) for device-independent drawing. Cabochon keeps that
split under four names: Clerestory (the application environment), Escutcheon
(resources), Lapidary (the device-independent scene model, the VDI analogue),
and Burin (the renderer). ADR 0007 makes all four production contracts in
hosted mode, before any Enfilade (object graph) behaviour exists. Roadmap step
1.0 delivers them as four tasks, each with its own ExecPlan:

1. 1.0.3, Lapidary (this plan), which also converts the repository into a
   Cargo workspace and installs the prover tooling the other plans reuse.
2. 1.0.4, Burin (`docs/execplans/1-0-4-burin-output-contract.md`), which
   renders Lapidary scenes. It requires this plan.
3. 1.0.2, Escutcheon (`docs/execplans/1-0-2-escutcheon-resource-format.md`),
   which compiles resources. It requires this plan and can proceed in parallel
   with 1.0.4.
4. 1.0.1, Clerestory
   (`docs/execplans/1-0-1-clerestory-application-contract.md`), which runs
   resource-defined applications. It requires all three.

After this plan, a developer can describe a drawing as a Lapidary scene in
Rust: rectangles, paths, text runs, images, clipping, transforms, layers, hit
regions, embedded scenes, page geometry, and output intent, all measured in
exact physical units. They can save the scene as a version 1 scene document and
load it back unchanged, resolve it into an ordered display list, ask which hit
region lies under a point, compute exactly which areas an edit damages, extract
accessibility geometry, and map scene coordinates to device pixels at any 1/120
display scale. Verus proves the hit-testing, damage, transform-bounds, winding,
and snapping kernels for all inputs; Kani proves the unit and fixed-point
arithmetic safe.

Observable success:

1. `make test` passes, including the new `cabochon-lapidary` property suites
   and `crates/cabochon-lapidary/tests/features/scene.feature`.
2. `make kani` reports every harness `VERIFICATION:- SUCCESSFUL`.
3. `make verus` reports `verification results:: <N> verified, 0 errors`.
4. A doctest on `SceneDocument` builds a scene, encodes it, decodes it, and
   asserts equality; a doctest on `ResolvedScene::hit_test` shows the topmost
   region winning.

Roadmap task 1.0.3's success criterion ("one scene renders consistently to
display, PDF-oriented output, …") needs Burin, so the 1.0.4 plan, not this one,
marks 1.0.3 done (decision T1).

How to read identifiers: `RM-` names a roadmap task, `TDD-` a technical-design
requirement, `TOR-` a terms-of-reference measure, `EP-` a milestone of this
plan, `OB-` a verification obligation, `A-` a trusted external assumption (an
axiom), `C` a constraint, `D` a decision inherited from the reviewed step-level
plan, and `T` a decision made for this task-level plan.

## Constraints

- C1, GEM-derived invariants. `docs/cabochon-design.md` §3 holds. Lapidary is
  the drawing model; no Lapidary type names an Enfilade type, a renderer, or a
  host (ADR 0007, ADR 0008). Later Enfilade integration lives in bridge crates
  (decision D12).
- C2, dependency direction. `cabochon-lapidary` depends on no other Cabochon
  crate, counting normal (non-development) dependency edges. `OB-ARCH-DEPS`
  enforces this and is extended by each later plan.
- C3, no third-party types in contracts. Lapidary's public signatures never
  mention `kurbo`, `parley`, `krilla`, `vello_cpu`, `winit`, AccessKit, `kdl`,
  Fluent, or `miette` types, and Lapidary does not depend on them.
- C4, integer geometry. Lapidary uses integer English Metric Units (EMU) and
  fixed-point transforms. Lapidary contains no floating-point arithmetic at
  all; float conversion happens only in later plans' named float-boundary
  modules.
- C5, repository lint, safety, and documentation policy. `unsafe_code` stays
  `forbid`; `#[allow]` is never used; lint expectations are tightly scoped and
  reasoned; no `.expect()` or `.unwrap()` outside tests; no source file exceeds
  400 lines. Public items follow the doc-comment rules in
  `docs/documentation-style-guide.md` (`# Parameters`, `# Returns`, `# Errors`,
  and `# Examples` that run as doctests unless they genuinely cannot execute).
- C6, existing contracts. The root package `cabochon`, its binary, and
  `tests/cli_greeting.rs` keep working; every existing contract test under
  `tests/` keeps passing; the `test`, `lint-clippy`, `typecheck`, and `build`
  recipes gain no Cargo invocations or prerequisites; `default-members` is not
  introduced; member crates declare no Cargo features (every gate runs with
  `--all-features`). The root package keeps `rust-version = "1.85"`, and the
  `msrv` CI job (`cargo +1.85.0 check --lib --all-features --locked`) keeps
  passing after the workspace conversion. The one sanctioned exception is
  EP-M0b's removal of Cranelift under C13, which rewrites the build-standard
  contracts that assert Cranelift.
- C7, no persisted user-document format. The scene document defined here is a
  Lapidary interchange and test format, not a user-document format, and must
  not be presented as one.
- C8, no proof shortcuts. No `assume`, `admit`, `assume_specification`,
  `#[verifier::external_body]`, `#[verifier(external_body)]`,
  `#[verifier::external]`, or trusted proof bodies. Kani harnesses may use
  `kani::assume` only to state an input domain that the production code also
  checks, naming that check in a comment.
- C9, tests do not mutate the process environment. `clippy.toml` already
  forbids direct `std::env` access.
- C10, library instrumentation only. Lapidary emits no global subscriber or
  recorder.
- C13, Kani takes precedence over Cranelift (reviewer direction,
  2026-10-23). Kani runs through `prover-tools kani exec`. If Kani cannot run
  while the Cranelift development backend is configured, Cranelift is expunged
  from the development standard, not worked around, and a GitHub issue is
  raised to revisit the matter six months later (decision T10). No handwritten
  Kani invocation that sidesteps the repository's Cargo configuration is
  introduced.

## Tolerances (exception triggers)

- Scope: a milestone needs more than 3,000 net added lines of Rust (excluding
  fixtures and snapshots) or touches more than 60 files.
- Interface: a contract in `Interfaces and dependencies` must change in a way
  that alters the 1.0.4, 1.0.2, or 1.0.1 plans or roadmap tasks 2.1.2 and
  2.1.3. Record the impact on each affected plan before continuing.
- Dependencies: a dependency not listed here is needed, or a listed crate
  must move to a new major or pre-1.0 minor series.
- Proof: a Verus obligation stays unproved after two working days, or appears
  to need a construct banned by C8. Use the named fallback only after approval.
- Spike: S-VERUS rejects its hypothesis, or S-KANI fails for a reason other
  than the Cranelift configuration. A Cranelift refusal is not an escalation:
  C13 resolves it by expunging Cranelift in EP-M0b.
- Cranelift removal: Kani still cannot run after Cranelift is expunged, or
  removing Cranelift would change a contract unrelated to the development
  backend (for example release routing, coverage, or the `mold` linker).
- MSRV: the `msrv` CI job cannot stay green after the workspace conversion.
- Upstream conflict: evidence contradicts `docs/cabochon-design.md`, an ADR,
  or the roadmap. Set `Status: BLOCKED` and record the proposed deviation.
- Iterations: a red test cannot be turned green after five attempts with
  distinct hypotheses.

## Risks

- Risk: the Verus proofs prove harder than sketched, especially transform
  bounds and winding, which need non-linear arithmetic. Severity: medium.
  Likelihood: medium. Mitigation: small, total, integer-only kernels; each
  obligation names a fallback that needs approval.
- Risk: the proved kernel drifts from the production code. Severity: high.
  Likelihood: medium. Mitigation: prefer a single-source kernel (decision D11),
  decided by S-VERUS; otherwise a spec mirror with a parser-based drift guard
  (`OB-PROOF-MIRROR`).
- Risk: depending on `vstd` in production code ties Lapidary to Verus's dated
  pre-release crates. Severity: medium. Likelihood: high if the single-source
  route is taken. Mitigation: pin the exact release matching
  `tools/verus/VERSION`, bump both together, and keep `vstd` usage inside the
  kernel module.
- Risk: the workspace conversion breaks the Rust 1.85 `msrv` CI job, or the
  Rust-version-aware resolver (resolver 3) prefers older dependency releases
  because the root declares `rust-version = "1.85"`. Severity: medium.
  Likelihood: medium. Mitigation: each member declares its own `rust-version`,
  no lower than the highest among its dependencies; M0 runs the `msrv` command
  locally and checks that `Cargo.lock` selects the newest releases the caret
  requirements allow.
- Risk: Kani 0.67.0 builds Lapidary with its own bundled toolchain
  (`nightly-2025-11-21`), which is older than the repository's pinned nightly.
  Lapidary's code, its `rust-version`, and (on the single-source route) the
  October 2026 `vstd` and `verus_builtin_macros` releases must all compile on
  that toolchain. Severity: medium. Likelihood: medium. Mitigation: S-VERUS and
  S-KANI check exactly this; if `vstd` cannot build under Kani's toolchain,
  fall back to spec mirrors (decision D11) rather than dropping Kani coverage.
- Risk: expunging Cranelift slows development builds (LLVM replaces it for
  `make build`, `make test`, `make lint`, and `make typecheck`) and touches the
  build standard: `.cargo/config.toml`, `rust-toolchain.toml`, the Makefile,
  both build-tools scripts, three CI workflows, about ten contract tests, and
  the developer, user, layout, and agent documentation. Severity: medium.
  Likelihood: high, because `kani exec` refuses the Cranelift configuration at
  the pinned commit. Mitigation: EP-M0b is its own milestone with its own
  commits; measure cold and warm build times before and after and record them;
  ADR 0013 records the decision; the revisit issue reopens it in six months.
- Risk: backend units bound coordinates below 2^24 / 256 = 65,536 device
  pixels (or points). A surface or page larger than that, or an item far
  outside the visible area, returns `GeometryOutOfRange`. Severity: low.
  Likelihood: low for step 1.0, higher for large documents in roadmap 2.1.
  Mitigation: Burin culls items wholly outside the surface before conversion
  (the 1.0.4 plan); revisit the unit if 2.1 needs larger surfaces.
- Risk: the workspace conversion collides with roadmap task 1.1.3, which owns
  "the workspace crate structure". Severity: medium. Likelihood: medium.
  Mitigation: create only the crates this plan needs and document their
  ownership and reuse policy before code (decision D1).

## Progress

- [x] (2026-10-18) Step-level plan drafted, reviewed by four
  community-of-experts panels, and revised (step plan at commit `616a8fa`).
- [x] (2026-10-22) Reviewer accepted decisions D1, D6, D7, D8, D10, D11, D12,
  D13, and D14 on the step-level plan.
- [x] (2026-10-23) Decomposed the step-level plan into four task-level plans;
  this is the first.
- [x] (2026-10-23) Raised
      [leynos/rust-prover-tools#206](https://github.com/leynos/rust-prover-tools/issues/206)
      so that
  `kani exec` can run with a configured development backend; the reviewer
  accepted the loss of Cranelift until it is resolved.
- [ ] Reviewer approval of this plan.
- [ ] EP-M0a: workspace, prover tooling, guards, and spikes.
- [ ] EP-M0b: Kani route, expunging Cranelift if Kani requires it, with the
  revisit issue raised.
- [ ] EP-M1: scene model and scene documents.
- [ ] EP-M2: resolution, device mapping, hit testing, damage, and
  accessibility nodes.
- [ ] EP-M3: Verus proofs, formal CI workflow, and ADR 0009 accepted.

## Surprises & discoveries

- Observation: the lint baseline denies `float_arithmetic` and the lossy cast
  lints. Impact: integer geometry is the natural representation and makes the
  Verus obligations tractable (decision D2).
- Observation: `docs/developers-guide.md` says Cabochon is a single crate.
  Impact: EP-M0a rewrites that paragraph and moves the lint tables to
  `[workspace.lints]`.
- Observation: `vstd` and `verus_builtin_macros` are published on crates.io
  with dated pre-release versions (for example `0.0.0-2026-10-18-0147`).
  Evidence: crates.io API, 2026-10-18. Impact: a single-source kernel may be
  feasible (decision D11).
- Observation: `leynos/rust-prover-tools` provides `prover-tools kani install`,
  `kani check-version`, `kani exec`, `verus install`, and `verus run`.
  `kani exec` runs an installed Kani 0.67.0 from the proof package's directory,
  keeps `RUSTC_WRAPPER` (so compile admission still applies), strips the
  Cranelift backend, `mold`, and frontend-thread flags, and rejects
  `--manifest-path`. Kani 0.67.0 is installed on the development host at
  `~/.kani/kani-0.67.0`. Evidence: the tool's `docs/users-guide.md` at commit
  `9c4a27a`, and `cargo kani --version`. Impact: `make kani` delegates to
  `kani exec` (decision T2).
- Observation: at commit `9c4a27a`, `prover-tools kani exec` refuses any
  `codegen-backend` key in the Cargo configuration of its working directory or
  any parent (`_check_backend` in `rust_prover_tools/kani_release_config.py`:
  "Kani exec cannot prove isolation of Cargo backend configuration"). This
  repository's `.cargo/config.toml` sets `[unstable] codegen-backend = true` and
  `[profile.dev] codegen-backend = "cranelift"`, and `kani exec` also rejects
  `--manifest-path`, so it cannot run here as things stand. Evidence: source
  review on 2026-10-23. Impact: unless the pinned tool has changed by EP-M0a,
  Kani cannot run here while Cranelift is configured, so EP-M0b expunges
  Cranelift (constraint C13, decision T10). Raised upstream as
  [leynos/rust-prover-tools#206](https://github.com/leynos/rust-prover-tools/issues/206).
- Observation: the build-standard contract
  (`tests/build_standard_support/make.rs`) reads only command lines containing
  `cargo` or `whitaker`, and fails a held-out target that runs no Cargo
  command. Impact: the prover targets (`install-kani`, `kani`, `install-verus`,
  and `verus`) all run through `prover-tools`, contain no `cargo` command line,
  and join neither list; `OB-PROVER-TOOLING` covers them instead.
- Observation: pull request #51 gave the root package
  `rust-version = "1.85"` and an `msrv` CI job; pull request #65 moved ADRs to
  `docs/adrs/adr-nnnn-title-slug.md`. Impact: constraint C6 protects the MSRV
  job, and this plan's ADR is ADR 0009 under `docs/adrs/`.

## Decision log

Inherited decisions from the reviewed step-level plan. Those the reviewer
accepted on 2026-10-22 say so; the rest were proposed in the step plan and are
approved with this plan.

- Decision D1 (accepted 2026-10-22): create the substrate crates as Cargo
  workspace members under `crates/` now, keeping the root package `cabochon`.
  Roadmap 1.1.3 keeps the Enfilade, runtime, storage, and portal crates. Each
  task plan creates its own crate; this one converts the workspace and creates
  `cabochon-lapidary` and `cabochon-substrate-testkit`.
- Decision D2: Lapidary measures length in integer EMU (`i64`, 914,400 per
  inch, 12,700 per point, 36,000 per millimetre, 9,525 per 96-dpi reference
  pixel), bounded to ±2^47, and transforms with a 16.16 fixed-point linear part
  plus an EMU translation. Transform application is checked; a result outside
  the EMU domain is a `GeometryOutOfRange` error. Decimal lengths such as
  `10.5pt` parse exactly or are rejected. Prior art: Office Open XML DrawingML
  (EMU) and FreeType's `FT_Matrix` (16.16). Recorded as ADR 0009.
- Decision D9: Lapidary owns resolution, in scene space, into a
  `ResolvedScene`; it also owns hit testing, damage, accessibility-node
  extraction, and the `DeviceMapping`. Text shaping enters through a
  Lapidary-owned `TextLayout` port that Burin implements; Lapidary's own tests
  use a deterministic fake.
- Decision D10 (accepted 2026-10-22): screen output uses a 96-dpi reference
  pixel scaled by the device scale in 1/120 units, as in Wayland's
  `wp_fractional_scale_v1`. Snapping snaps edges, not sizes.
- Decision D11 (accepted 2026-10-22, including the production dependency on
  `vstd` if the spike succeeds): prove the kernels from a single source if
  S-VERUS succeeds; otherwise use spec mirrors with a drift guard. Kernels are
  total functions with no preconditions.
- Decision D12 (accepted 2026-10-22): substrate crates never depend on
  Enfilade; integration lives in bridge crates.
- Decision D16: image resources are stored encoded (PNG only in step 1.0) with
  a declared pixel size and a digest; Burin decodes them.
- Decision D17: a scene may embed another scene through `Node::Subscene`;
  resolved items, hit results, and accessibility nodes carry a `NodePath`.
  `OpaqueOrigin` is a bounded, opaque label, not an Enfilade type.
- Decision D20: assertion style. `googletest` matchers for structural and
  semantic assertions, `pretty_assertions::assert_eq!` for whole-value
  equality, and `insta` only for stable, reviewer-useful summaries. `#[gtest]`
  is placed before `#[rstest]`.

Decisions made for this task-level plan:

- Decision T1: decompose step 1.0 into four task plans, implemented in the
  order 1.0.3, then 1.0.4 and 1.0.2 (in either order or in parallel), then
  1.0.1. Roadmap ticks follow proof of each success criterion: the 1.0.4 plan
  marks 1.0.3 and 1.0.4 done; the 1.0.1 plan marks 1.0.2 and 1.0.1 done.
  Rationale: 1.0.3's criterion is cross-target rendering, which only Burin can
  show, and 1.0.2's is the hello-window application, which needs Clerestory.
  Date: 2026-10-23.
- Decision T2: use `rust-prover-tools` for both provers. `make install-kani`
  runs `prover-tools kani install` (reading `tools/kani/VERSION`, pinned to
  0.67.0, the release `kani exec` is qualified for); `make install-verus` and
  `make verus` run `prover-tools verus install` and `prover-tools verus run`.
  `make kani` runs `prover-tools kani check-version` and then
  `prover-tools kani exec --installation "$(KANI_INSTALLATION)" --cwd
  crates/cabochon-lapidary`.
  Rationale: the tool owns version pinning, checksums, toolchain selection,
  compile admission, and isolation from the development flags. A handwritten
  `cargo kani` route run from outside the checkout was considered and rejected
  under constraint C13. Date: 2026-10-23.
- Decision T10 (reviewer direction, 2026-10-23): Kani is more important than
  Cranelift. If S-KANI shows that `prover-tools kani exec` cannot run while
  `.cargo/config.toml` selects Cranelift (as at the pinned commit, which
  rejects any `codegen-backend` key), EP-M0b expunges Cranelift from the
  development standard: development builds, tests, lint, and type checks use
  the default LLVM backend. EP-M0b then raises a GitHub issue on
  `leynos/cabochon` to revisit Cranelift six months after the removal date,
  records the issue number here and in `docs/developers-guide.md`, and records
  the decision as ADR 0013. If S-KANI shows `kani exec` running with Cranelift
  present (because the tool has changed), Cranelift stays and EP-M0b only adds
  the `kani` target. On 2026-10-23 the reviewer accepted the loss of Cranelift
  until
  [leynos/rust-prover-tools#206](https://github.com/leynos/rust-prover-tools/issues/206),
  which asks `kani exec` to support a configured development backend, is
  resolved. The Cabochon revisit issue links #206 and falls due when #206 is
  resolved or six months after removal, whichever comes first. Date: 2026-10-23.
- Decision T8: one pull request per task plan, replacing the step plan's
  decision D18 (one stacked pull request per milestone), following the
  reviewer's direction on 2026-10-23 to review and merge these plans first and
  implement each in its own pull request. Milestones remain commit-level
  plateaus within that pull request. Date: 2026-10-23.
- Decision T9: ADR numbers are reserved by this decomposition: 0009 for this
  plan, 0010 for the 1.0.2 plan, 0011 for the 1.0.4 plan, 0012 for the 1.0.1
  plan, and 0013 for this plan's Cranelift decision if EP-M0b expunges it. Any
  other pull request needing an ADR takes 0014 or later. Date: 2026-10-23.
- Decision T3: the checked integer conversion from EMU to backend units
  (1/256 device pixel, or 1/256 point for PDF, each bounded below 2^24 so a
  later `f32` cast is exact) lives in Lapidary as
  `DeviceMapping::to_backend_units` and `PageMapping::to_page_units`, proved by
  Kani (`OB-LAP-BACKEND`). Burin keeps only the cast. Rationale: Kani then
  checks a small, dependency-free crate rather than Burin's rendering stack.
  Date: 2026-10-23.

## Outcomes & retrospective

Not started.

## Context and orientation

The repository today holds one Rust package, `cabochon`, with `src/lib.rs` (a
`greet()` function) and `src/main.rs`. The tests under `tests/` check the build
standard: the Makefile, CI workflows, linker provisioning, and release routing.
There is no workspace, no substrate code, and no prover tooling.

Governing documents: `docs/cabochon-design.md` §3 (GEM-derived invariants),
§7.1, §8.3 (Lapidary), §8.9, and §13.1 (VDI conformance tests);
`docs/adrs/adr-0007-make-the-gem-substrate-a-production-boundary.md`;
`docs/adrs/adr-0008-keep-enfilade-out-of-rendering-authority.md`;
`docs/adrs/adr-0002-keep-the-hosted-runtime-independent-of-the-shell.md`;
`docs/adrs/adr-0004-require-interoperable-authoring-paths.md`;
`docs/context.md` (Lapidary, Surface, Workstation); and `docs/roadmap.md` task
1.0.3.

Build and quality gates live in `Makefile`. `make check-fmt` checks Rust and
Markdown formatting, `make lint` runs rustdoc, Clippy, and Whitaker, and
`make test` runs `cargo nextest` with
`--workspace --all-targets --all-features` and then the doctests, so new member
crates join every gate automatically. `.cargo/config.toml` selects the
Cranelift backend, the parallel frontend, and the pinned `mold` linker; EP-M0b
removes the Cranelift selection if Kani cannot run with it present (constraint
C13). Compilation passes through a shared admission queue
(`build-limits-rustc`); a waiting build is not hung.

Terms:

- EMU (English Metric Unit): an integer length; 914,400 EMU make one inch.
- Fixed16: a signed 16.16 fixed-point number in an `i32`; 65,536 is 1.0.
- Scene: a Lapidary tree of groups, shapes, text runs, images, hit regions,
  and embedded scenes, with page geometry, layers, and an output intent.
- Scene space: the scene's coordinate system, in EMU.
- Device mapping: the translation and 1/120 scale relating scene space to
  device pixels.
- Resolved scene: the ordered, flattened display list produced from a scene;
  each item carries its path, composed transform, effective clip, and
  conservative integer bounds.
- Damage: the scene-space rectangles that must be redrawn after a change.
- Spec mirror: a Verus copy of a production function, kept identical to it by
  a drift-guard test.

Skills to load: `execplans`, `rust-router` (then `rust-types-and-apis`,
`rust-errors`, and `arch-crate-design`), `hexagonal-architecture`,
`rust-unit-testing`, `proptest`, `kani`, `verus`, `rust-verification`,
`arch-decision-records`, `nextest`, `en-gb-oxendict-style`, and
`codegraph-mcp`. References: `docs/rust-testing-with-rstest-fixtures.md`,
`docs/reliable-testing-in-rust-via-dependency-injection.md`,
`docs/rust-doctest-dry-guide.md`,
`docs/complexity-antipatterns-and-refactoring-strategies.md`, and
`docs/documentation-style-guide.md`. The prover tool's own guide is
`docs/users-guide.md` in `leynos/rust-prover-tools`; the Verus on-ramp is in the
`verus` skill's `references/project-on-ramp.md`.

## Conformance basis

Upstream artefacts at `main` commit `0de2b57`: `docs/terms-of-reference.md`,
`docs/cabochon-design.md` version 0.2, ADRs 0002, 0004, 0007, and 0008, and
`docs/roadmap.md`. Traced items: `RM-1.0.3`; `TDD-INV-2` and `TDD-INV-5`
(design §3); `TDD-LAP` (§8.3); `TDD-PRES` (§8.9); `TDD-VDI-RT` and
`TDD-VDI-INVAL` (§13.1).

```plaintext
RM-1.0.3 -> TDD-LAP, TDD-VDI-RT -> EP-M1 -> tests/round_trip.rs, OB-LAP-ROUNDTRIP
RM-1.0.3 -> TDD-LAP (hit regions) -> EP-M2, EP-M3 -> OB-LAP-WINDING, OB-LAP-HIT, OB-LAP-RESOLVE
RM-1.0.3 -> TDD-VDI-INVAL (semantic half) -> EP-M2, EP-M3 -> OB-LAP-DAMAGE, OB-LAP-SNAP
RM-1.0.3 -> TDD-PRES -> EP-M2 -> OB-LAP-A11Y, OB-LAP-BACKEND (consumed by the 1.0.4 plan)
TDD-INV-5, ADR-0007, ADR-0008 -> EP-M0a -> tests/substrate_dependency_contract.rs, OB-ARCH-DEPS
```

The pixel half of `TDD-VDI-INVAL`, cross-target parity, and the roadmap tick
for 1.0.3 belong to the 1.0.4 plan.

## Verification plan

Verus is used for five small integer kernels whose correctness must hold for
scenes of any size, and on which Clerestory (pointer routing) and Burin
(redraw) rely: transform bounds, polygon winding with its bounding-box
prefilter, topmost hit scanning, damage soundness, and device snapping.
Rectangle algebra appears only as supporting lemmas, valued mainly for overflow
freedom. Kani covers bounded arithmetic safety: unit conversion, decimal
parsing, fixed-point overflow and floor identity, and backend-unit conversion.
Everything else uses example, parameterized, property, behavioural, and
snapshot tests.

Proof and test relate explicitly. Verus establishes the kernels' properties for
all inputs. Single-source compilation (or the drift guard) ties the proved code
to the production code. Property tests then check through the public API that
resolution produces the inputs the theorems are about (`OB-LAP-RESOLVE`) and
that the production kernels behave as proved (an executable twin of each
theorem). The link from damage, which is about bounds, to real pixels is the
1.0.4 plan's `OB-BUR-LOCALITY`.

Non-vacuity mechanics: proptest has no built-in classification, so suites whose
non-vacuity depends on case classes run through a manual
`proptest::test_runner::TestRunner` with a fixed seed, accumulate per-class
counters, and assert minimum counts after the run. Generators construct
important classes directly. Budgets: 256 cases per suite; regression files under
`proptest-regressions/` are committed; `PROPTEST_CASES` may raise budgets for
deep runs.

Axioms: `A-VERUS` (the pinned Verus and its Z3 solver are sound, and, on the
single-source route, `verus_builtin_macros` erases only ghost code) and
`A-KANI` (Kani 0.67.0 is sound for the harnessed code).

### Obligations

`OB-LAP-UNITS`, exact and checked units. For every `i64` input, each `Emu`
constructor (`from_points`, `from_millimetres`, `from_inches`,
`from_reference_pixels`) returns the exact product when it lies within
±`Emu::LIMIT` (2^47) and `LengthOutOfRange` otherwise; `Emu::parse` accepts a
decimal with suffix `pt`, `mm`, `in`, or `px` exactly when the value is a whole
number of EMU in range, and otherwise returns `InexactLength` or
`LengthOutOfRange`. No panic or overflow. Method: Kani (exhaustive for the
constructors; inputs up to 24 characters for `parse`), named boundary cases,
and a property test that `parse(format(e)) == e`. Artefacts:
`crates/cabochon-lapidary/src/units/kani.rs` (behind `#[cfg(kani)]`) and
`src/units/tests.rs`. Red step: the harness runs first against an unchecked
multiply, and Kani reports the overflow. Non-vacuity: `kani::cover!` both
outcomes and the boundaries; seeded fault `wrapping_mul` must fail.

`OB-LAP-FIXED`, fixed-point application. `Fixed16::apply(c, e)` computes
`floor(c × e / 2^16)` with an `i128` intermediate and an arithmetic right shift
(never `/`, which truncates towards zero), never overflows for any `c` and any
in-domain `e`, and satisfies `r × 2^16 ≤ c × e < (r + 1) × 2^16`. Method: Kani
with the CaDiCaL solver and a per-harness timeout; measured times recorded in
`Artefacts and notes`. Monotonicity is left to Verus. Non-vacuity: covers for
negative, zero, and positive coefficients; seeded fault `/` must break the
floor identity for negative products.

`OB-LAP-BACKEND`, backend-unit conversion (decision T3). For every in-domain
point and mapping, `DeviceMapping::to_backend_units` computes 1/256 device
pixels in `i128` and returns them as `i32` exactly when their magnitude is
below 2^24, and `GeometryOutOfRange` otherwise; `PageMapping::to_page_units`
does the same for 1/256 points. Every returned value is therefore exactly
representable as `f32`. Method: Kani. Non-vacuity: covers for both outcomes at
100%, 125%, and 200% scale; seeded fault: a bound of 2^25 must fail.

`OB-LAP-XFORM-BOUNDS`, transformed bounds are sound for the exact image. For
every rectangle `r` and transform `t` whose result is in range,
`transform_bounds(t, r)` returns `[min, max)` such that for every point `p` in
`r`, `2^16 × min.x ≤ xx × p.x + xy × p.y + 2^16 × tx < 2^16 × max.x` (and
likewise for `y`); containment of the floored `apply(t, p)` follows. The
function evaluates the four inclusive corners, floors the minimum exact image,
takes one more than the floor of the maximum, and returns `GeometryOutOfRange`
when a value leaves the domain. Lemma sketch: each output coordinate is a sum
of two terms, each monotone in one input with direction set by the
coefficient's sign, so extremes over a rectangle occur at corners; flooring the
minimum and adding one to the floored maximum yields a half-open interval
containing every exact image. Method: Verus; fallback (needs approval) Kani
bounded to ±2^20 plus a full-domain property test. Non-vacuity: witnesses with
mixed-sign coefficients; seeded fault: evaluating two corners only must fail.

`OB-LAP-WINDING`, polygon coverage and prefilter soundness.
`polygon_covers(poly, p)` computes the non-zero winding number with half-open
crossings (`a.y ≤ p.y < b.y` or `b.y ≤ p.y < a.y`) and a strict left-of-edge
test using exact `i128` cross products. If `p` lies outside the vertex bounds
(half-open), the winding is zero, and a polygon built from an axis-aligned
rectangle covers `p` exactly when `rect_contains` does. Lemma sketch, with
`s(v) = [v.y ≤ p.y]`: (1) cancellation, each edge contributes `s(a) − s(b)` and
these sum to zero round a closed polygon; (2) right side, edges with both
endpoints at `x ≤ p.x` are never counted; (3) left side, if both endpoints
satisfy `x > p.x` every crossing is counted with its sign, so the winding
equals the cancelling sum; (4) scanline, no edge crosses when `p.y` lies outside
`[min.y, max.y)`; (5) rectangle equivalence. Coordinates below 2^62 keep cross
products exact; EMU's 2^47 bound satisfies this. Method: Verus
(`nonlinear_arith` on `(b.x − a.x)(p.y − a.y) − (p.x − a.x)(b.y − a.y)`), plus
a property test against an independent oracle using exact rational crossing
x-coordinates. Non-vacuity: constructed points on edges and vertices, scanlines
through vertices, horizontal edges, a bow tie, and a pentagram (winding 2);
seeded fault: the even-odd rule must fail on the pentagram.

`OB-LAP-HIT`, topmost hit testing. For a resolved hit list `h` in painting
order and scene point `p`, `hit_test` returns `Some(i)` exactly when `i` is the
greatest index with `covers(h[i], p)`, and `None` when none covers `p`.
`covers` requires `p` in the effective clip (a rectangle intersected with a
sequence of polygons, all of which must cover `p`) and in the entry's polygon.
Lemma sketch: the reverse scan's invariant is that no later entry covers `p`;
skipping on bounds is sound by `OB-LAP-WINDING`; clip intersection is
conjunction. Method: Verus; a property test against a scan-everything oracle
using the independent winding oracle; named cases for nested clips, rotated
transforms, overlaps, embedded scenes (path-valued results), and edges.
Non-vacuity: at least 20% of cases with two or more covering regions and 10%
with clip-excluded hits; seeded fault: dropping the clip check must fail both
the proof and the property. Red step: the oracle property fails against a
forward scan.

`OB-LAP-DAMAGE`, damage soundness. For resolved scenes `old` and `new` and
`D = damage(old, new)`, every point outside `D` is covered by the same paint
items, in the same order, in both scenes. Algorithm: intern each item to a
`u32` key by full equality (never a digest) over its path, transform, clip, and
paint; find the longest common prefix `pre`, then the longest common suffix
`suf` with `pre + suf ≤ min(old.len, new.len)`; add the bounds of every item in
both middles to `D`. Lemma sketch: filtering distributes over concatenation; for
`q` outside `D` no middle item covers `q`; prefix and suffix items are equal.
Method: Verus; property tests over generated edits (insert, remove, move,
recolour, retransform, layer visibility) checking identical scenes give empty
damage, a single insertion damages exactly its bounds, interning agrees with
equality, and an executable twin of the theorem for random points outside `D`
biased to its edges. Non-vacuity: counters for each edit kind; seeded faults:
omitting removed items must fail; overlapping prefix and suffix must fail on
`old = [A, A]`, `new = [A]`.

`OB-LAP-SNAP`, snapping covers damage. For every rectangle `d`, mapping `m`,
and device pixel intersecting `d` dilated by one pixel, that pixel lies in
`m.snap_out(d)`, which floors minimum edges and ceils maximum edges. Method:
Verus. Non-vacuity: witnesses at 100%, 125%, 150%, and 200%; seeded fault:
rounding both edges to nearest must fail.

`OB-LAP-RESOLVE`, resolution produces the theorems' inputs. Items are in
painting order; transforms compose ancestors'; effective clips intersect
ancestors'; bounds contain every flattened vertex, the stroke outset (half the
width times the larger of the mitre limit and 1.5, which exceeds the square-cap
factor √2), glyph ink bounds, and image footprints; hidden layers are absent;
embedded scenes contribute namespaced paths; resolution is deterministic.
Method: a property test against a naive recursive oracle, plus named cases.
Non-vacuity: thick strokes with sharp mitre joins, rotated nested clips,
embedded scenes reusing identifiers; seeded fault: ignoring the mitre extension
must fail.

`OB-LAP-A11Y`, accessibility nodes. `ResolvedScene::accessibility_nodes()`
returns one node per text run and per annotated group, with bounds equal to the
resolved bounds, a role from the annotation (or `Text`), the annotation's label
or the run's text as its name, and children nested by the group-span index in
painting order. Property test plus an `insta` snapshot of a fixed sample scene.

`OB-LAP-ROUNDTRIP`, scene documents. For every valid scene,
`SceneDocument::decode(encode(s)) == s`; an unknown major version is rejected
with `UnsupportedVersion`. Method: property test over generated scenes (depth
up to 4, up to 24 nodes, every node kind, layers, nested clips and transforms,
left-to-right and right-to-left runs, PNG images up to 16 by 16 pixels, every
output and colour intent), an `insta` snapshot of one canonical document, and
named unhappy cases. Non-vacuity: counters for every node kind and intent;
seeded fault: skipping the hit-region tag must fail.

`OB-LAP-VALID`, validation rejects duplicate node identifiers within one scene,
dangling image or layer references, coordinates outside the domain, degenerate
transforms (zero determinant), oversized image bytes, and opaque origins longer
than 64 bytes. Parameterized unhappy cases plus a property test that generated
valid scenes pass.

`OB-ARCH-DEPS`, dependency direction. `tests/substrate_dependency_contract.rs`
runs `cargo metadata --offline --locked`, considers only normal edges, and
asserts that `cabochon-lapidary` depends on no Cabochon crate and on none of
`winit`, `softbuffer`, `accesskit*`, `kurbo`, `parley`, `krilla`, `vello*`,
`kdl`, `fluent*`, `unic-langid`, or `miette`, and that no workspace crate
depends on a crate whose name contains `enfilade`, `runtime`, or `portal`. Its
rules are table-driven so each later plan adds its crate's row. The testkit's
row allows it to depend on any Cabochon crate and on test-only libraries such as
`hayro`, and the contract asserts that no production crate depends on the
testkit through a normal edge. Non-vacuity: it also asserts each expected edge
exists.

`OB-ARCH-FEATURES`, the existing `the_crate_declares_no_features` test in
`tests/workflow_suite_contract.rs` is extended to every workspace member.

`OB-PROOF-MIRROR`, proof hygiene. `tests/verus_mirror_contract.rs` asserts that
proof sources contain no construct banned by C8, that each kernel region sits
inside `verus!` with an `ensures` clause, and (from EP-M3) that at least five
kernel regions exist. On the spec-mirror fallback it also parses both copies
with `proc-macro2`, erases ghost syntax by a documented rule (remove `requires`,
`ensures`, `decreases`, and loop `invariant` clauses and `proof { ... }`
blocks), and asserts token identity.

`OB-PROVER-TOOLING`, prover tooling contract.
`tests/prover_tooling_contract.rs` checks that `tools/kani/VERSION`,
`tools/verus/VERSION`, `tools/verus/SHA256SUMS`, and
`tools/rust-prover-tools/REF` are well formed; that the `install-kani`, `kani`,
`install-verus`, and `verus` recipes delegate only through `$(PROVER_TOOLS)`
and contain no `curl`, `unzip`, `sha256sum`, or `cargo kani`; and, with a
recording fake passed as `PROVER_TOOLS_CONTRACT_TEST=1 PROVER_TOOLS=<fake>` on
the `make` command line, that `make kani` issues exactly
`kani check-version --repo-root .` and then
`kani exec --installation <path> --cwd crates/cabochon-lapidary`, and that
`make verus` issues exactly
`verus run --repo-root . --proof-file verus/lapidary_proofs.rs`.

`OB-KANI-COMPAT`, Kani compatibility guard (only if EP-M0b expunges Cranelift).
`tests/kani_compatibility_contract.rs` asserts that neither
`.cargo/config.toml` nor any workspace manifest contains a `codegen-backend`
key, and that `rust-toolchain.toml` and the build-tools scripts no longer
request `rustc-codegen-cranelift-preview`, so Cranelift cannot return unnoticed
before the revisit issue is resolved. Its failure message names the revisit
issue. Non-vacuity: a seeded fault that restores
`[profile.dev] codegen-backend = "cranelift"` in a temporary copy of the
configuration must fail the parser-level check the test uses.

## Plan of work

Each milestone follows Red-Green-Refactor: write the smallest failing test,
scenario, Kani harness, or Verus lemma statement; watch it fail for the
expected reason; make it pass minimally; refactor and rerun. Each milestone
ends with `make check-fmt`, `make lint`, and `make test` passing (run
sequentially through the `scrutineer` agent), plus `make kani` and `make verus`
where stated, and a commit.

### EP-M0a: workspace, prover tooling, guards, and spikes

Workspace. Convert `Cargo.toml` to a workspace whose root package stays
`cabochon` with `rust-version = "1.85"`. Move the `[lints.*]` tables unchanged
to `[workspace.lints.*]`, add the `kani` check-cfg entry, and declare
`[lints] workspace = true` in the root and every member. Create
`crates/cabochon-lapidary` and `crates/cabochon-substrate-testkit` with `//!`
documentation, `publish = false`, their own `rust-version`, and empty public
surfaces. The testkit is test support, consumed only from integration tests
under each crate's `tests/`, never from unit tests (a development-dependency
cycle would otherwise yield two copies of Lapidary's types). Add
`[workspace.dependencies]` entries with caret requirements in the commit that
first uses each; each dependency addition is its own commit. Run the `msrv`
job's command locally afterwards.

Prover tooling (decision T2). Add `tools/kani/VERSION` (`0.67.0`),
`tools/verus/VERSION` (the latest weekly release when EP-M0a starts),
`tools/verus/SHA256SUMS`, `tools/verus/README.md`, and
`tools/rust-prover-tools/REF` (`ref: <40-hex>`, the then-current commit of
`leynos/rust-prover-tools`; `9c4a27a86c678ceac1155404f45d95ee4a14a172` at
planning time). Add `.verus/` to `.gitignore` before the first install so
`mdtablefix --git --include-untracked` never rewrites Verus's bundled Markdown.
Add the `install-kani`, `install-verus`, and `verus` targets from
`Interfaces and dependencies`; the `kani` target arrives in EP-M0b, once Kani
can run. None of these targets joins `make all`, `make test`, or `make lint`,
and none joins the build-standard target lists, which read only `cargo` and
`whitaker` commands. Until EP-M3, `verus/lapidary_proofs.rs` holds S-VERUS's
verified toy kernel, so `make verus` always verifies a real file
(`prover-tools verus run` has no skip mode). Add the new targets to `.PHONY`.

Test infrastructure. Add `.config/nextest.toml` (a `render` test group for
later plans, and slow-test timeouts), one `rstest-bdd` smoke scenario in the
testkit, and one `#[gtest]` plus `#[rstest]` test proving the harnesses compose.

Guards first: `tests/substrate_dependency_contract.rs`,
`tests/prover_tooling_contract.rs`, `tests/verus_mirror_contract.rs`, and the
`OB-ARCH-FEATURES` extension. Each fails before the workspace and tooling exist.

Spikes, each recording hypothesis, command, and result in `Artefacts and notes`:

1. S-VERUS: a toy kernel with `i128` multiplication, an arithmetic shift,
   slices, and a `while` loop verifies under the pinned Verus through
   `make verus` (it becomes the EP-M0a content of `verus/lapidary_proofs.rs`);
   and, for decision D11, the same kernel written inside `verus!` with the
   matching crates.io `vstd` and `verus_builtin_macros` compiles and passes
   `make lint` and `make test`. Record the Rust toolchain the pinned Verus
   requires (`prover-tools verus run` installs it) and whether
   `rust-toolchain.toml` interferes. The Kani half of this spike (building the
   single-source kernel under Kani's own `nightly-2025-11-21` toolchain, and
   finding the highest `rust-version` Lapidary can declare for it) runs at the
   end of EP-M0b, once Kani can run.
2. S-KANI: after `make install-kani`, run
   `prover-tools kani exec --installation "$HOME/.kani/kani-0.67.0" --cwd
   crates/cabochon-lapidary`
   on one trivial harness with the repository's configuration unchanged.
   Hypothesis: it refuses with "Kani exec cannot prove isolation of Cargo
   backend configuration". Record the exact outcome; it selects EP-M0b's branch
   (decision T10).

Documentation. Draft ADR 0009 (integer EMU geometry and fixed-point transforms)
as `docs/adrs/adr-0009-<title-slug>.md`, status `Proposed`. Record the
ownership, permitted callers, and reuse policy of Lapidary and the testkit in a
new crate-ownership subsection of `docs/cabochon-design.md` §8 and in
`docs/repository-layout.md`. Update `docs/developers-guide.md`: the "Lint
baseline" now describes `[workspace.lints]`; new sections cover the workspace,
the dependency rule and its contract test, test conventions and budgets, and
the prover tooling. Update `docs/contents.md`.

### EP-M0b: Kani route, expunging Cranelift if Kani requires it

Constraint C13 and decision T10 govern this milestone.

If S-KANI showed `kani exec` running with Cranelift present, add the `kani`
target and its `OB-PROVER-TOOLING` expectations, record the outcome, and finish
the milestone.

Otherwise, expunge Cranelift, as its own sequence of commits:

1. Measure cold and warm `make test` and `make lint` times with Cranelift and
   record them in `Artefacts and notes`.
2. Write `tests/kani_compatibility_contract.rs` (`OB-KANI-COMPAT`) and run it
   red against the current configuration.
3. Remove the `[unstable] codegen-backend` and `[profile.dev] codegen-backend`
   keys from `.cargo/config.toml`, keeping the parallel frontend and the `mold`
   linker; remove `rustc-codegen-cranelift-preview` from `rust-toolchain.toml`,
   `scripts/install-build-tools.sh`, and `scripts/check-build-tools.sh`
   (including the `CARGO_PROFILE_*_CODEGEN_BACKEND` override checks); and
   remove Cranelift-specific Makefile lines and comments, such as the coverage
   recipe's `CARGO_PROFILE_*_CODEGEN_BACKEND=llvm` overrides. Isolation that
   stays in the release and `lint-whitaker` recipes must be justified without
   reference to Cranelift.
4. Update every contract test that asserts Cranelift
   (`tests/build_backend_contract.rs` and its `ci_installs.rs` and `release.rs`
   modules, `tests/build_standard_contract.rs`,
   `tests/build_tools_installer_contract.rs`, `tests/makefile_contract.rs`,
   `tests/build_target_routing.rs`, `tests/release_workflow/routing.rs`, and
   `tests/stable_cargo_config.rs`) so it asserts the LLVM development standard
   instead, and update the CI workflows (`ci.yml`, `coverage-main.yml`, and
   `release.yml`) that install or mention the Cranelift component.
5. Update `AGENTS.md` ("Development builds"), `docs/developers-guide.md` (the
   build standard and its Figure 1), `docs/users-guide.md`, and
   `docs/repository-layout.md` (the `.cargo/config.toml` description).
6. Write ADR 0013 (Kani compatibility over the Cranelift development backend)
   under `docs/adrs/`, status `Accepted`, citing this plan, the `prover-tools`
   check, and the measured build times.
7. Raise the revisit issue with the standard GitHub client, for example:

   ```bash
   gh issue create -R leynos/cabochon \
     --title "Revisit the Cranelift development backend (after rust-prover-tools#206, or <removal date + 6 months>)" \
     --body-file /tmp/cranelift-revisit-issue.md
   ```

   The body states the removal date and commit, why Cranelift was removed
   (`prover-tools kani exec` rejects `codegen-backend` configuration; tracked
   upstream as leynos/rust-prover-tools#206), the measured build-time cost, ADR
   0013, and what to check on revisiting: whether `rust-prover-tools` or Kani
   now tolerate a configured development backend, and whether Cranelift's
   build-time gain still justifies it. Record the issue number in decision T10,
   ADR 0013, and `docs/developers-guide.md`.
8. Measure the build times again with LLVM and record them.

Then add the `kani` target, run S-KANI again (`make kani` on the trivial
harness, with `build-limits-status` showing its compiler invocations admitted
through the queue), and run S-VERUS's Kani half.

### EP-M1: scene model and scene documents

Implement, each behind red tests: `units` (`Emu`, constructors, `parse`);
`geometry` (`Point`, `Size`, `Rect`, `Fixed16`, checked `Transform`, `Path` of
move, line, quadratic, cubic, and close segments, `FillRule`); `paint`
(`Colour` as sRGB with 16-bit channels and alpha, `Paint`, `Stroke` with width,
joins, caps, and a Fixed16 mitre limit); `text` (`TextRun`, `FontRequest`,
`TextDirection`, `LanguageTag`); `image` (`ImageResource` with PNG bytes,
declared pixel size, intrinsic size, and digest); `layer` (`LayerSpec` with
name, visible, and printable flags); `scene` (`Scene`, `Node`, `NodeId`,
`Group`, `Shape`, `TextNode`, `ImageNode`, `HitRegion`, `HitRegionId`,
`Subscene`, `OpaqueOrigin`, `Annotation`, `SemanticRole`, `PageGeometry`,
`OutputIntent`, `ColourIntent`, `PrintIntent`); `validate`; and `document`
(`SceneDocument` version 1, JSON through `serde_json`, image bytes in base64).
Write the Kani harnesses for `OB-LAP-UNITS` and `OB-LAP-FIXED` alongside. Put
the scene generator in the testkit. Write a prose reference for scene document
version 1 for authors working in other languages (ADR 0004), linked from
`docs/contents.md`.

### EP-M2: resolution, device mapping, hit testing, damage, and accessibility nodes

Define the `TextLayout` port, `ShapedRun`, `FontKey`, `FontData`, and
`FontSet`, and a deterministic `MonospaceTextLayout` fake in the testkit (each
grapheme advances by a fixed amount, with caret stops at grapheme boundaries).
Implement `resolve`, producing a `ResolvedScene` in scene space: ordered
`ResolvedItem`s (path, composed transform, effective clip as a rectangle plus a
sequence of polygons, conservative bounds, paint payload), the group-span
index, the used `FontSet`, and the hit list (curves flattened by integer de
Casteljau subdivision to a declared tolerance). Implement `DeviceMapping`
(`scene_to_device`, `device_to_scene`, `snap_out`, `to_backend_units`) and
`PageMapping` (`to_page_units`), with the `OB-LAP-BACKEND` harnesses. Implement
`hit_test`, `hit_test_all`, `damage`, and `accessibility_nodes`, with the five
kernels in `crates/cabochon-lapidary/src/kernel/` as total, integer-only Rust
shaped for proof (slices, `while` loops, no preconditions). Write the property
suites and named cases for `OB-LAP-WINDING`, `OB-LAP-HIT`, `OB-LAP-DAMAGE`,
`OB-LAP-RESOLVE`, and `OB-LAP-A11Y`, and `scene.feature`.

### EP-M3: Verus proofs, formal CI workflow, and ADR 0009

Following S-VERUS's outcome, write the lemma statements first and run
`make verus` to see the expected open goals; then prove `OB-LAP-XFORM-BOUNDS`,
`OB-LAP-WINDING`, `OB-LAP-HIT`, `OB-LAP-DAMAGE`, and `OB-LAP-SNAP`, with
rectangle algebra as supporting lemmas. Proof files live under `verus/`
(`lapidary_proofs.rs` includes `lapidary_rect.rs`, `lapidary_transform.rs`,
`lapidary_winding.rs`, `lapidary_hit.rs`, `lapidary_damage.rs`, and
`lapidary_snap.rs`) or, on the single-source route, inside the kernel modules
with `verus/lapidary_proofs.rs` as the entry point. Enable `OB-PROOF-MIRROR`'s
minimum-region assertion. Add `.github/workflows/formal.yml`, triggered by
`workflow_dispatch` and a weekly schedule, that sets up Rust through
`setup-rust` with `install-mold: 'true'` (build scripts and procedural macros
still link through `scripts/clang-pinned-mold.sh`) and runs `make install-kani`,
`make kani`, `make install-verus`, and `make verus`; it must satisfy the
existing workflow contract tests. Accept ADR 0009 and record the proof
inventory, and what each proof does not cover, in `docs/developers-guide.md`.

## Milestones and plateaus

Every milestone's conformance check asks whether its requirements are met, the
design is still followed, upstream assumptions hold, no unapproved interface,
dependency, trust boundary, or persisted format appeared, and trace links are
current. Recovery is to revert the milestone's commits (dependency additions
are separate commits). The compatibility decision is "none": every interface is
pre-1.0 and internal.

- EP-M0a. Outcome: a workspace with two member crates, Verus tooling,
  `install-kani`, guards, and the S-VERUS and S-KANI results. Evidence: all
  gates pass; the `msrv` command passes locally; `make verus` verifies
  S-VERUS's toy kernel; `cargo tree -p cabochon` is unchanged.
- EP-M0b. Outcome: Kani runs through `prover-tools kani exec`. Evidence:
  `make kani` runs the trivial harness under compile admission; all gates pass.
  If Cranelift was expunged: `OB-KANI-COMPAT` passes, the rewritten
  build-standard contracts pass, ADR 0013 is accepted, the revisit issue exists
  and is linked from decision T10 and the developers' guide, and the
  before-and-after build times are recorded. Recovery: revert the milestone's
  commits, which restores Cranelift and removes the `kani` target together.
- EP-M1. Outcome: Lapidary models, validates, and round-trips scenes.
  Requirements: `RM-1.0.3` (data model), `TDD-VDI-RT`. Evidence: `OB-LAP-UNITS`,
  `OB-LAP-FIXED`, `OB-LAP-VALID`, and `OB-LAP-ROUNDTRIP` pass under
  `make test` and `make kani`.
- EP-M2. Outcome: Lapidary resolves, maps, hit-tests, damages, and extracts
  accessibility nodes. Requirements: `RM-1.0.3` (hit regions, invalidation,
  physical units), `TDD-LAP`, `TDD-PRES`. Evidence: `OB-LAP-BACKEND` (Kani),
  `OB-LAP-WINDING`, `OB-LAP-HIT`, `OB-LAP-DAMAGE`, `OB-LAP-RESOLVE`, and
  `OB-LAP-A11Y` property suites, and `scene.feature`.
- EP-M3. Outcome: the five kernels are proved. Evidence:
  `make verus` (whose recipe passes no extra Verus arguments) reports zero
  errors, and the verified-item count recorded at completion becomes the
  minimum; `OB-PROOF-MIRROR` finds five or more regions; `make kani` and
  `make test` pass; ADR 0009 accepted.

## Behavioural specification

`crates/cabochon-lapidary/tests/features/scene.feature`, bound with
`rstest-bdd` and run red first:

```gherkin
Feature: Lapidary scenes describe geometry independently of devices

  Scenario: The topmost hit region wins
    Given a scene with two overlapping hit regions "back" and "front"
    When the point inside both regions is hit-tested
    Then the result is "front"

  Scenario: Clipping hides hit regions
    Given a hit region inside a group clipped to its left half
    When a point in the right half of the region is hit-tested
    Then no hit region is reported

  Scenario: Embedded scenes keep their identity
    Given a scene that embeds another scene containing hit region "price"
    When a point inside "price" is hit-tested
    Then the result path names the embedded scene and then "price"

  Scenario: Editing one shape damages only that shape
    Given a resolved scene with three separated rectangles
    When the middle rectangle is recoloured
    Then the damage equals the middle rectangle's bounds

  Scenario: Scene documents round-trip
    Given a scene with a clipped, rotated group containing text and an image
    When it is encoded as a scene document and decoded
    Then the decoded scene equals the original
```

## Concrete steps

Run from the worktree root. Gates, sequentially, through `tee`:

```bash
BRANCH=$(git branch --show-current)
make check-fmt 2>&1 | tee /tmp/check-fmt-cabochon-${BRANCH}.out
make lint 2>&1 | tee /tmp/lint-cabochon-${BRANCH}.out
make test 2>&1 | tee /tmp/test-cabochon-${BRANCH}.out
```

Formal gates:

```bash
make install-kani
make kani 2>&1 | tee /tmp/kani-cabochon-${BRANCH}.out
make install-verus
make verus VERUS_PROOF_FILE=verus/lapidary_proofs.rs 2>&1 | tee /tmp/verus-cabochon-${BRANCH}.out
```

MSRV check, from outside the checkout:

```bash
cd /tmp && cargo +1.85.0 check --lib --all-features --locked \
  --manifest-path "$OLDPWD/Cargo.toml"
```

Expected Verus result after EP-M3:

```plaintext
verification results:: <N> verified, 0 errors
```

Focused loops:

```bash
cargo nextest run -p cabochon-lapidary --test hit_test_properties
cargo nextest run -p cabochon-lapidary --test damage_properties
```

## Validation and acceptance

- Tests: `make test` passes, including every obligation's tests and
  `scene.feature`.
- Verification: `make kani` discharges `OB-LAP-UNITS`, `OB-LAP-FIXED`, and
  `OB-LAP-BACKEND`; `make verus` discharges `OB-LAP-XFORM-BOUNDS`,
  `OB-LAP-WINDING`, `OB-LAP-HIT`, `OB-LAP-DAMAGE`, and `OB-LAP-SNAP` with no
  construct banned by C8 and no extra Verus arguments; the proof-hygiene and
  tooling contracts pass.
- Lint and formatting: `make check-fmt` and `make lint` pass with no
  `#[expect]` in Lapidary except where a reason cites a proven macro-expansion
  artefact (for example from `verus!`, if the single-source route is taken).
- Documentation: `make markdownlint`, `make nixie`, and `make spelling` pass.
- Compatibility: the `msrv` command passes.
- Kani precedence: `make kani` runs through `prover-tools kani exec`; if
  Cranelift was expunged, `OB-KANI-COMPAT` passes and the revisit issue is open.

Record red, green, and refactor evidence per milestone in `Artefacts and notes`.

## Idempotence and recovery

Every step is additive and rerunnable. The workspace conversion is one commit
and reverts cleanly; each dependency addition is its own commit.
`make install-kani` and `make install-verus` are idempotent: the prover tool
verifies versions and checksums and reuses existing installations
(`.verus/<version>` is ignored by Git). If a gate fails, fix the cause and
rerun the whole sequence. Accept `insta` snapshot changes only after confirming
they reflect an intended contract change.

## Artefacts and notes

Record spike results, Kani run times, the Verus verified-item count, and red,
green, and refactor transcripts here as work proceeds.

## Interfaces and dependencies

### Workspace and Makefile

```toml
[workspace]
members = ["crates/*"]
resolver = "3"

[workspace.package]
edition = "2024"
license = "ISC"
repository = "https://github.com/leynos/cabochon"

[workspace.lints.rust]
# the existing [lints.rust] entries, unchanged, plus:
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)'] }

[lints]
workspace = true
```

```makefile
override PROVER_TOOLS_REF_FILE := tools/rust-prover-tools/REF
override PROVER_TOOLS_REF := $(shell awk '/^ref:/ { print $$2 }' $(PROVER_TOOLS_REF_FILE))
ifeq ($(shell printf '%s' '$(PROVER_TOOLS_REF)' | grep -Ec '^[0-9a-f]{40}$$'),0)
$(error PROVER_TOOLS_REF missing or malformed; expected "ref: <40-hex>" in $(PROVER_TOOLS_REF_FILE))
endif
override PROVER_TOOLS_SOURCE := git+https://github.com/leynos/rust-prover-tools.git@$(PROVER_TOOLS_REF)
PROVER_TOOLS_ORIGINS := $(origin PROVER_TOOLS_CONTRACT_TEST),$(origin PROVER_TOOLS)
ifeq ($(PROVER_TOOLS_ORIGINS),command line,command line)
else
override PROVER_TOOLS := $(UVX) --from "$(PROVER_TOOLS_SOURCE)" prover-tools
endif
KANI_VERSION := $(shell cat tools/kani/VERSION)
KANI_INSTALLATION ?= $(HOME)/.kani/kani-$(KANI_VERSION)
VERUS_PROOF_FILE ?= verus/lapidary_proofs.rs

install-kani: ## Install the pinned Kani release
	$(PROVER_TOOLS) kani install --repo-root .

kani: ## Run the Kani harnesses for the Lapidary kernels (added in EP-M0b)
	$(PROVER_TOOLS) kani check-version --repo-root .
	$(PROVER_TOOLS) kani exec --installation "$(KANI_INSTALLATION)" --cwd crates/cabochon-lapidary

install-verus: ## Install the pinned Verus release
	$(PROVER_TOOLS) verus install --repo-root .

verus: ## Run the Verus proofs
	$(PROVER_TOOLS) verus run --repo-root . --proof-file "$(VERUS_PROOF_FILE)"
```

`PROVER_TOOLS` is locked to the pinned source unless both
`PROVER_TOOLS_CONTRACT_TEST` and `PROVER_TOOLS` are given on the `make` command
line, which is the seam the contract test uses; environment values never unlock
it. `KANI_INSTALLATION` names the directory `cargo kani setup` creates for the
pinned release. The prover tool runs in the same clean Git environment as other
Git-backed tools.

### Lapidary (`cabochon-lapidary`)

```rust,no_run
pub struct Emu(i64);
impl Emu {
    pub const LIMIT: i64 = 1 << 47;
    pub const fn from_points(points: i64) -> Result<Self, LengthOutOfRange>;
    pub const fn from_millimetres(mm: i64) -> Result<Self, LengthOutOfRange>;
    pub const fn from_inches(inches: i64) -> Result<Self, LengthOutOfRange>;
    pub const fn from_reference_pixels(px: i64) -> Result<Self, LengthOutOfRange>;
    pub fn parse(text: &str) -> Result<Self, LengthParseError>; // "10.5pt", "3mm", "1in", "12px"
}

pub struct Fixed16(i32);
pub struct Transform { pub xx: Fixed16, pub xy: Fixed16, pub yx: Fixed16, pub yy: Fixed16, pub tx: Emu, pub ty: Emu }

pub struct Scene {
    pub page: PageGeometry,
    pub output_intent: OutputIntent,
    pub colour_intent: ColourIntent,
    pub layers: Vec<LayerSpec>,
    pub root: Group,
    pub images: ImageStore,
}

pub enum Node {
    Group(Group),
    Shape(Shape),
    Text(TextNode),
    Image(ImageNode),
    HitRegion(HitRegion),
    Subscene(Subscene), // origin: OpaqueOrigin, transform, clip, scene: Arc<Scene>
}

pub enum OutputIntent { Screen, Document, Print(PrintIntent) }

pub trait TextLayout {
    fn layout(&self, run: &TextRun) -> Result<ShapedRun, TextLayoutError>;
    fn font_data(&self, key: FontKey) -> Option<FontData>;
}

pub struct ShapedRun {
    pub font: FontKey,
    pub glyphs: Vec<ShapedGlyph>,
    pub ink_bounds: Rect,
    pub caret_stops: Vec<CaretStop>,
}

pub fn resolve(scene: &Scene, layout: &dyn TextLayout) -> Result<ResolvedScene, ResolveError>;

pub struct DeviceMapping { pub origin: Point, pub scale: Scale120 }
impl DeviceMapping {
    pub fn scene_to_device(&self, point: Point) -> DevicePoint;
    pub fn device_to_scene(&self, point: DevicePoint) -> Point;
    pub fn snap_out(&self, rect: Rect) -> PixelRect;
    pub fn to_backend_units(&self, point: Point) -> Result<BackendPoint, GeometryOutOfRange>; // 1/256 px, |v| < 2^24
}

pub struct PageMapping { pub origin: Point }
impl PageMapping {
    pub fn to_page_units(&self, point: Point) -> Result<BackendPoint, GeometryOutOfRange>; // 1/256 pt, |v| < 2^24
}

impl ResolvedScene {
    pub fn hit_test(&self, point: Point) -> Option<HitPath>;
    pub fn hit_test_all(&self, point: Point) -> Vec<HitPath>;
    pub fn accessibility_nodes(&self) -> AccessibilityNodes;
    pub fn fonts(&self) -> &FontSet;
}

pub fn damage(old: &ResolvedScene, new: &ResolvedScene) -> Damage;

impl SceneDocument {
    pub fn encode(scene: &Scene) -> Result<String, SceneEncodeError>;
    pub fn decode(text: &str) -> Result<Scene, SceneDecodeError>;
}
```

`ResolveError` covers `GeometryOutOfRange`, `TextLayout(TextLayoutError)`
(including `NoFontCovers`, which the 1.0.4 plan's text adapter raises), and
`Invalid(ValidationError)`.

### Dependencies

Caret requirements in `[workspace.dependencies]`; `Cargo.lock` records exact
versions. Use the newest compatible release when each milestone starts and
record deviations in `Decision log`.

- Lapidary: `serde` 1, `serde_json` 1, `base64` 0.22, `thiserror` 2. On the
  single-source route, `vstd` and `verus_builtin_macros` at the dated release
  matching `tools/verus/VERSION` (pre-release versions, so the requirement is
  exact; the documented exception to caret requirements).
- Development: `rstest` 0.26, `rstest-bdd` 0.6, `rstest-bdd-macros` 0.6,
  `googletest` 0.14, `pretty_assertions` 1.4, `insta` 1.49, `proptest` 1.11, and
  `proc-macro2` 1 (spec-mirror fallback only).
- Tooling: Kani 0.67.0 (`tools/kani/VERSION`), Verus (`tools/verus/VERSION`),
  and `rust-prover-tools` (`tools/rust-prover-tools/REF`).

## Documentation obligations

- ADR 0009 (integer EMU geometry and fixed-point transforms) under
  `docs/adrs/`, named per the style guide and referenced from
  `docs/cabochon-design.md` §15.
- `docs/cabochon-design.md`: the crate-ownership subsection (EP-M0a) and, in
  §8.3, the Lapidary contract summary and proof inventory, with the
  proof-driven representation decisions stated as design facts: half-open
  integer rectangles, checked transforms, non-zero winding with half-open
  crossings, topmost-wins scanning, prefix and suffix damage over interned
  keys, and edge snapping.
- `docs/developers-guide.md`: the workspace, the dependency rule and its
  contract test, test conventions and budgets, the prover tooling, and the
  proof inventory with what each proof does not cover. Repository layout stays
  in `docs/repository-layout.md`.
- A prose reference for scene document version 1.
- If EP-M0b expunges Cranelift: ADR 0013, the revisit issue, and updates to
  `AGENTS.md`, `docs/developers-guide.md`, `docs/users-guide.md`, and
  `docs/repository-layout.md` describing the LLVM development standard and
  linking the issue.
- `docs/repository-layout.md` and `docs/contents.md` list every new directory
  and document.

## Revision notes

- 2026-10-23: created by decomposing the reviewed step-level plan
  (`docs/execplans/1-0-establish-gem-substrate-contracts.md` at commit
  `616a8fa`) into four task-level plans. This plan carries the step plan's
  EP-M0 (workspace, tooling, guards), EP-M1, EP-M2a, and EP-M2b, and its
  Lapidary obligations. New here: Kani and Verus run through `rust-prover-tools`
  (`kani exec` replaces the hand-rolled recipe; decision T2), and backend-unit
  conversion moves from Burin into Lapidary so Kani checks a small crate
  (decision T3).
- 2026-10-23: applied the cross-plan consistency review.
  `prover-tools kani exec` refuses this repository's codegen-backend
  configuration at the pinned commit, so `make kani` runs `cargo kani` from the
  parent directory (as `lint-whitaker` does) and joins the held-out target
  list, with S-KANI re-checking `kani exec` (decision T2). The `FORMAL-SKIP`
  stub became S-VERUS's verified toy kernel, S-VERUS now checks that the
  single-source kernel builds under Kani's older toolchain, the testkit gained
  a dependency-contract rule, `formal.yml` installs `mold`, the 2^24
  backend-unit limit is a recorded risk, and decision T8 records one pull
  request per plan.
- 2026-10-23: following the reviewer's direction that Kani is more important
  than Cranelift, replaced the handwritten `cargo kani` route with
  `prover-tools kani exec` (constraint C13, decisions T2 and T10). EP-M0 is now
  EP-M0a (workspace, tooling, guards, spikes) and EP-M0b (the Kani route). If
  `kani exec` cannot run while Cranelift is configured, as at the pinned
  commit, EP-M0b expunges Cranelift, guards against its return
  (`OB-KANI-COMPAT`), records ADR 0013, and raises an issue to revisit the
  matter six months later. ADR 0013 is reserved for that decision (T9).
- 2026-10-23: raised
  [leynos/rust-prover-tools#206](https://github.com/leynos/rust-prover-tools/issues/206)
  asking `kani exec` to support a configured development codegen backend, and
  recorded the reviewer's acceptance of losing Cranelift until it is resolved.
  The planned Cabochon revisit issue now falls due when #206 is resolved or six
  months after removal, whichever comes first (decision T10).
