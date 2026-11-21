# Define the minimum Clerestory application contract (roadmap 1.0.1)

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
`Tolerances`, `Risks`, `Progress`, `Surprises & discoveries`, `Decision log`,
`Outcomes & retrospective`, `Conformance basis`, and `Verification plan` must
be kept up to date as work proceeds.

Status: DRAFT (refined 2026-11-21 against the Cabochon UI guidelines; awaiting
user approval)

Branch: `1-0-1-clerestory-application-contract`

Roadmap item: `1.0.1` in `docs/roadmap.md`.

The reviewer approved an earlier revision of this plan on 2026-11-04. That
approval does not cover this revision, which changes the acceptance surface,
adds a formal proof milestone, and proposes amendments to two sibling plans.
Implementation must not begin until the user approves this revision, the pull
request carrying it has merged, and the 1.0.3, 1.0.4, and 1.0.2 plans
(`docs/execplans/1-0-3-lapidary-scene-contract.md`,
`docs/execplans/1-0-4-burin-output-contract.md`, and
`docs/execplans/1-0-2-escutcheon-resource-format.md`) have been implemented and
merged. Implement on the branch above, in its own pull request.

## Purpose / big picture

Cabochon models its application environment on the Graphics Environment Manager
(GEM), the 1980s desktop whose Application Environment Services (AES) gave
every program the same windows, menus, and dialogs from resource files.
Clerestory is Cabochon's AES analogue. It owns windows, menus, commands,
standard dialogs, focus, keyboard traversal, text input, inspector panels,
accessibility roles, and adaptation to a host desktop such as GNOME or KDE
Plasma. It builds on Escutcheon (resources), Lapidary (scenes), and Burin
(rendering), which the three earlier task plans of roadmap step 1.0 deliver.
Architectural decision record (ADR) 0007 makes all four production contracts in
hosted mode, and the roadmap requires them to work without Enfilade, Cabochon's
semantic object layer.

Today the repository is a single crate, `cabochon`, whose `src/main.rs` prints
a greeting. Nothing draws a window. After this plan, a developer can run a
small application, `hello-window`, inside an ordinary GNOME or KDE Plasma
session. The Cabochon UI guidelines call this program "the smallest application
that proves the Cabochon application contract"
(`book/36-hello-window.md` in `leynos/cabochon-ui-guidelines`). Its window,
menu bar, commands, shortcuts, strings, icon, status line, and dialogs all come
from Escutcheon resources rather than hand-built widgets. Its document is one
Lapidary scene drawn by Burin. Every control is reachable by keyboard and
described to assistive technology. The same scene exports to every Burin output
with one command.

Clerestory's contract is wider than `hello-window`'s surface. The roadmap names
inspector panels, and the reviewer accepted a text field with input-method
support (decision D14). The guidelines place neither in `hello-window`, so a
second, test-only application, the contract probe, exercises them. Nothing in
the contract goes unproved, and `hello-window` stays "entirely explained by the
contract".

Observable success:

1. `cargo run -p cabochon-hello-window -- run` opens one window. Its menu bar
   reads File, View, Help. File holds Export, Print, and Quit. View holds
   Command palette, a Zoom submenu, and an Appearance submenu. Help holds About
   and Diagnostics. The document shows a greeting, a short explanation, a
   primary Export button, and a secondary About button. A status line sits at
   the bottom. Control+Q quits, Control+K opens the command palette, F1
   describes the focused control in the status line, Alt focuses the menu bar,
   and Tab visits every control once per cycle.
2. `--locale en-XA` shows pseudo-localized strings and `--locale ar-XB` shows
   the furniture mirrored for right-to-left text, with no code changes and no
   clipped labels.
3. `cargo run -p cabochon-hello-window -- review --out "$(mktemp -d)"` writes
   the conformance bundle: `content.pdf`, `content-print.pdf`, `content.png`,
   `thumbnail.png`, `accessibility.json`, `clipboard/`, `hit-map.json`, and
   `export.json` (the manifest).
4. `make test` passes, including `furniture.feature` and
   `hello_window.feature` under the headless shell. `make verus` proves the
   focus kernel. `make test-hosted` runs the application under a headless
   Wayland compositor and prints:

   ```plaintext
   hello-window: shell=hosted windows=1 frames=1 accessibility_nodes=<N> exit=0
   ```

5. Roadmap tasks 1.0.1 and 1.0.2 are marked done (decision T1).

How to read identifiers: `RM-` roadmap task, `TDD-` technical-design
requirement, `TOR-` terms-of-reference measure, `UIG-` UI-guidelines rule
(followed by the book's own checklist identifier where one exists), `REQ-` a
requirement this plan derives, `GAP-` a baseline-to-target gap, `EP-M`
milestone, `OB-` verification obligation, `INV-` invariant, `L-` lemma,
`AXIOM-` trusted assumption, `C` constraint, `D` decision inherited from the
reviewed step-level plan, `T` decision made when the step plan was split into
task plans, `R` decision made in this refinement, and `AM-` a proposed
amendment to a sibling plan.

## Constraints

- C1, GEM-derived invariants. Applications reach the desktop only through
  Clerestory, draw only through Lapidary scenes, render only through Burin, and
  take user-interface structure from Escutcheon (`docs/cabochon-design.md` §3,
  invariants 1 to 4). No crate in this plan names an Enfilade type (invariant
  5, decision D12).
- C2, dependency direction, enforced by allow-lists counting normal and build
  edges (decision T13). `OB-ARCH-DEPS` rows: `cabochon-clerestory` may depend
  only on `cabochon-lapidary`, `cabochon-escutcheon`, `thiserror`, `tracing`,
  and `metrics`, plus `vstd` and `verus_builtin_macros` if the 1.0.3 plan's
  single-source proof route was adopted (decision D11);
  `cabochon-clerestory-hosted` on `cabochon-clerestory`, `cabochon-burin`,
  `cabochon-lapidary`, `winit`, `softbuffer`, `accesskit`, `accesskit_winit`,
  `thiserror`, and `tracing`; and `cabochon-hello-window` on
  `cabochon-clerestory`, `cabochon-clerestory-hosted`, `cabochon-escutcheon`,
  `cabochon-burin`, `cabochon-lapidary`, `cabochon-fonts`, `clap`, `eyre`,
  `serde`, `serde_json`, `tracing`, `tracing-subscriber`, `metrics`,
  `metrics-util`, `mockable`, `cap-std`, and `camino`. Only the hosted and
  application crates combine Clerestory with Burin in normal edges, and only
  the hosted crate reaches `winit`, `softbuffer`, or AccessKit.
- C3, module containment (`OB-ARCH-IMPORTS` rows). In Clerestory, no module
  outside `shell::headless` may import `shell::headless`, so the reference
  adapter stays outside the application model; `kernel` imports nothing from
  the rest of the crate. In the hosted crate, `winit` and `softbuffer` appear
  only under `event_loop` and `present`, AccessKit only under `accessibility`,
  and float or cast lint expectations only under `float_boundary`. In the
  application, `clap` appears only under `cli`, `tracing-subscriber` and
  `metrics-util` only under `telemetry`, `eyre` only in `main.rs`, and `serde`
  and `serde_json` only under `export::wire`, whose private data-transfer types
  write `export.json` and `accessibility.json` (decision T15).
- C4, no third-party types in Clerestory's public interface: no `winit`,
  Wayland, X11, AccessKit, GTK, or Qt types (ADR 0002's consequence that host
  concerns stay behind adapter ports). `OB-ARCH-PUBAPI` row:
  `cabochon-clerestory`'s public interface may name only `std`, `core`,
  `alloc`, `cabochon_lapidary`, and `cabochon_escutcheon`. The hosted adapter
  and the application are composition crates and are exempt.
- C5, integer geometry. Floating-point arithmetic appears only in
  `cabochon_clerestory_hosted::float_boundary` (scale factors and pointer
  positions from `winit`), with reasoned, item-scoped lint expectations.
- C6, repository policy as stated in `AGENTS.md` and enforced by `Cargo.toml`
  and `clippy.toml`: no file over 400 lines, cognitive complexity at most 9,
  functions at most 70 lines and four parameters, no `unsafe`, no `unwrap` or
  `expect` outside tests, a `//!` comment on every module, Rustdoc with an
  example on every public item, a message on every assertion, and en-GB
  Oxford spelling.
- C7, existing contracts keep passing: the `msrv` continuous integration (CI)
  job, the workflow contract tests under `tests/`, and the build-standard
  contracts. Member crates declare no Cargo features.
- C8, no proof shortcuts. Verus sources contain no `assume`, `admit`,
  `#[verifier::external_body]`, `#[verifier::external]`, or trusted proof body,
  and no axiom introduced for a theorem in this plan. The 1.0.3 plan's
  `OB-PROOF-MIRROR` guard enforces this textually.
- C9, tests never mutate the process environment. Locale, display, and
  appearance reach the application through arguments or an injected
  `mockable::Env`; child processes in end-to-end tests receive environment
  values through `Command::env`. No `serial_test`, shared lock, or `temp_env`.
- C10, only the `cabochon-hello-window` binary installs a `tracing`
  subscriber or a `metrics` recorder. Library crates emit and never install.
- C11, shared host. Recipes never kill processes they did not start; the
  hosted test stops only the `weston` and D-Bus session it started.
- C12, the UI guidelines bind user experience, not platform scope. Where the
  book and the roadmap, design document, or an ADR disagree about scope, the
  programme documents win and the difference is recorded in `Decision log`
  (the book's own rule, `book/appendix-references.md` "Contract basis").
- C13, completed and approved sibling plans are not edited by this plan's
  implementation. The amendments `AM-ESC-1`, `AM-BUR-1`, and `AM-LAP-1` are
  proposals for the user to accept or reject.
- C14, implementation begins only after the user approves this revision.

If satisfying the objective requires violating a constraint, stop, record the
conflict in `Decision log`, and escalate.

## Tolerances (exception triggers)

Comparison base: the commit on `main` from which the implementation branch is
cut, recorded in `Artefacts and notes` at the start of EP-M0. Limits are net
lines (additions minus deletions), measured per category with:

```bash
git diff --numstat "$BASE"...HEAD -- <category paths>
```

Each category is enforced independently; spare capacity in one never offsets
another. ExecPlan files under `docs/execplans/` are excluded from every count.

- Code: Rust under `crates/*/src`, `crates/*/tests`, `tests/`, and `verus/`,
  plus `Makefile` recipes and `scripts/`. Limit: 3,500 net lines in any one
  milestone and 16,000 for the plan. Generated `*.snap` files and
  `proptest-regressions/` are excluded.
- Configuration: `Cargo.toml` manifests, `.config/nextest.toml`,
  `.github/workflows/`, `typos.local.toml`, and hand-written resource bundles
  (`*.kdl`, `*.ftl`, `*.svg`). Limit: 1,200 net lines for the plan.
  `Cargo.lock` and the generated `typos.toml` are excluded.
- Documentation: `docs/` except `docs/execplans/`, and `README.md`. Limit:
  1,500 net lines for the plan.
- Files: more than 70 non-ExecPlan files touched in one milestone, or 260 for
  the plan.
- Interface: a Lapidary, Escutcheon, or Burin contract must change beyond the
  amendments listed in `Decision log`, or a Clerestory contract changes in a
  way that alters roadmap tasks 2.1.2 and 2.1.3.
- Dependencies: a dependency not listed in `Interfaces and dependencies` is
  needed, or a listed crate must move to a new major or pre-1.0 minor series.
- Proof: a lemma in `Verification plan` cannot be proved after three
  restructurings of the kernel, or proving it needs a construct C8 bans.
- Hosted environment: a headless Wayland compositor cannot be provisioned in
  CI or locally during EP-M0.
- Coverage: the coverage ratchet fails because hosted-only code cannot run
  without a display, and thinning the adapter does not resolve it.
- Upstream conflict: evidence contradicts `docs/cabochon-design.md`, an ADR,
  the roadmap, or a **must** rule of the UI guidelines that `Decision log`
  does not already record as a departure. Set `Status: BLOCKED` and record the
  proposed deviation.
- Iterations: a red test cannot be turned green after five attempts with
  distinct hypotheses.
- Ambiguity: two readings of a guideline rule lead to different observable
  behaviour and `Decision log` does not choose between them.

The initial budgets are justified in `Context and orientation`.

## Risks

- Risk: the guidelines' `hello-window` needs resource features the approved
  Escutcheon plan does not define (nested menus, state marks, a command's input
  requirement and unavailability reason, dialog purpose). Severity: high.
  Likelihood: certain. Mitigation: `AM-ESC-1` proposes them before 1.0.2 is
  implemented; EP-M0 checks they were delivered and otherwise blocks.
- Risk: scope. The refined surface adds a status line, command palette, zoom,
  appearance, print, and diagnostics to an already large plan. Severity: high.
  Likelihood: high. Mitigation: each addition has a minimal, stated semantics
  (decisions R4 to R9); eight milestones with per-milestone budgets; the
  exclusions in `Context and orientation` are enforced by tests where a
  predicate can be checked.
- Risk: headless `weston` implements neither the `text-input-v3` protocol nor
  fractional scaling, so input-method events and fractional scales cannot be
  exercised under the hosted adapter in CI. Severity: medium. Likelihood:
  certain. Mitigation: both are verified through the headless shell and pure
  mapping functions; the hosted residual is named for roadmap 5.1.4.
- Risk: AccessKit's Linux adapter activates only when an assistive technology
  has announced itself on the session bus, so a hosted test may publish no
  tree. Severity: medium. Likelihood: high. Mitigation: `OB-CLR-HOST` compares
  the session journal, which is independent of activation, and checks the
  conversion with `accesskit_consumer`; EP-M0 probes activation by setting
  `org.a11y.Status.IsEnabled`; reading the tree back over the Assistive
  Technology Service Provider Interface (AT-SPI) stays a residual.
- Risk: `winit` 0.30 has no popup surfaces, so menus, sheets, and the palette
  are drawn inside the window. Severity: low. Likelihood: certain. Mitigation:
  menus flip or scroll inside the window (`UIG` menus rule), and session start
  rejects a bundle whose menu bar or widest dialog exceeds the declared minimum
  window size in any shipped locale.
- Risk: `winit` 0.31 (beta) changes the event-loop interface and
  `accesskit_winit` 0.34 supports only `winit` 0.30. Severity: low.
  Likelihood: high. Mitigation: stable `winit` 0.30.13, confined to the hosted
  crate behind the `HostShell` port.
- Risk: the Verus proof of the focus cycle needs induction over a rank
  function and may be costly; Verus releases pin an exact Rust toolchain.
  Severity: medium. Likelihood: medium. Mitigation: the kernel is index-only
  and total; EP-M0 writes the lemma statements and inspects the open goals
  before the session is designed around them; the proof tolerance applies.
- Risk: the coverage ratchet fails because the hosted event loop cannot run
  without a display. Severity: medium. Likelihood: medium. Mitigation: keep
  the hosted crate thin, with key, scale, and accessibility mapping in pure,
  unit-tested functions.
- Risk: `rstest-bdd` 0.6 declares a minimum supported Rust version of 1.88,
  above the root's 1.85. Severity: low. Likelihood: certain. Mitigation: it is
  a development dependency, and the `msrv` job checks only the root library;
  confirm in EP-M0.
- Risk: the sibling plans still describe Cranelift as the development
  backend, while `main` disabled it on 2026-11-14. Severity: low. Likelihood:
  certain. Mitigation: this plan depends on neither backend; it records the
  drift and leaves the reconciliation to the 1.0.3 plan's owner.

## Progress

- [x] (2026-10-18) Step-level plan drafted, reviewed by four
  community-of-experts panels, and revised (commit `616a8fa`).
- [x] (2026-10-22) Reviewer accepted decisions D1, D6, D7, D8, D10, D11, D12,
  D13, and D14 on the step-level plan.
- [x] (2026-10-23) Decomposed into four task-level plans; this is the last.
- [x] (2026-11-04) Reviewer approval of the earlier revision.
- [x] (2026-11-21) Refined against the UI guidelines, current tooling, and the
  proof-sketch-first verification policy; expert review applied.
- [ ] User approval of this revision, including decisions R1 to R14 and the
  amendments `AM-ESC-1`, `AM-BUR-1`, and `AM-LAP-1`.
- [ ] The 1.0.3, 1.0.4, and 1.0.2 plans implemented and merged.
- [ ] EP-M0: feasibility probes and prerequisite check.
- [ ] EP-M1: focus kernel, command and key routing, session, headless shell.
- [ ] EP-M2: Verus proofs of the focus kernel.
- [ ] EP-M3: metrics, theme, layout, furniture, and the accessibility tree.
- [ ] EP-M4: text input, command palette, inspector panel, contract probe.
- [ ] EP-M5: the hello-window application under the headless shell.
- [ ] EP-M6: the hosted shell adapter and its CI job.
- [ ] EP-M7: documentation reconciliation and step closure.

## Surprises & discoveries

- Observation: `accesskit_winit` 0.34.2 requires `winit` ^0.30.5 and does not
  support the 0.31 betas. Evidence: crates.io metadata, 2026-11-21. Impact:
  `winit` 0.30.13 is the only compatible stable release.
- Observation: `muda` (native menus) needs GTK on Linux and attaches only to
  GTK windows. Impact: supports in-window menus (decision D5).
- Observation: `mockable` is at 3.0.0, while
  `docs/reliable-testing-in-rust-via-dependency-injection.md` cites `0.3` and
  recommends `temp_env`, which `AGENTS.md` forbids. Impact: use `mockable` 3
  and correct the guide in EP-M5.
- Observation: the UI guidelines (`leynos/cabochon-ui-guidelines`, commit
  `9852939`, 2026-11-21) define `hello-window` differently from the earlier
  revision of this plan: no inspector and no editable field, a status line, and
  Print, Command palette, Zoom, Appearance, and Diagnostics commands; F1 means
  help for the focused object, not About. Evidence: `book/36-hello-window.md`,
  `book/18-focus-keyboard.md` "Keys that must work". Impact: decisions R2 to
  R9.
- Observation: the book's profile table makes "Inspector and project window"
  a semantic-workspace capability, while roadmap 1.0.1 lists inspector panels.
  Evidence: `book/appendix-references.md` "Conformance profiles and delivery
  phases". Impact: decision R3 separates the generic inspector panel (substrate
  furniture, this plan) from the book's identity-bearing Inspector (later).
- Observation: headless `weston` 13 to 15 lacks `text-input-v3` and
  `fractional-scale-v1`. Evidence: wayland.app protocol support tables,
  2026-11-21. Impact: the hosted residuals above.
- Observation: AccessKit's Unix adapter builds no tree until
  `org.a11y.Status` reports an enabled client, and Accerciser does not set that
  flag at start-up. Evidence: AccessKit discussion 502. Impact: `OB-CLR-HOST`
  design and the manual check's instructions.
- Observation: the workflow contract tests refuse a raw `cargo nextest` line
  in CI but accept `make test-hosted`, and treat it as a suite job that needs a
  Linux runner and `setup-rust` with `install-mold: 'true'` and
  `expect-cache: any`. Evidence: `tests/workflow_suite_contract.rs`,
  `tests/build_backend_contract/suite_jobs.rs`. Impact: EP-M6's CI job shape.
- Observation: `docs/roadmap.md` task 5.1.3 is packaging; automated
  GNOME-like and KDE-like acceptance is task 5.1.4. Impact: residuals now cite
  5.1.4.

## Decision log

Inherited from the reviewed step-level plan (accepted ones say so):

- Decision D5: Clerestory owns the furniture model (commands, menus, dialogs,
  focus, modality, roles) and draws furniture inside the window as Lapidary
  scenes. It adapts to hosts through a `HostShell` port; the hosted adapter
  uses `winit` 0.30, `softbuffer`, and `accesskit_winit`. Considered: native
  menus through `muda` (needs GTK), GTK4 (a second toolkit), Slint, egui, or
  iced (each duplicates Lapidary), and Masonry as the hosted furniture renderer
  (a second render path outside Burin's parity tests). `FrameUpdate` stays at
  the scene level so the Masonry option remains open. Recorded as ADR 0012.
- Decision D8 (accepted 2026-10-22): standard dialogs are alert,
  confirmation, and About. File dialogs wait for roadmap 5.1.2; export takes
  its directory from the command line.
- Decision D10 (accepted 2026-10-22): device scale in 1/120 units; snapping
  applies to edges; a scale change damages the whole window.
- Decision D11 (accepted 2026-10-22, 1.0.3 plan): Verus kernels are written
  once and compiled into production if the 1.0.3 plan's S-VERUS spike
  succeeded; otherwise a Verus copy is kept token-identical by a drift guard.
  This plan follows whichever route 1.0.3 delivered.
- Decision D12 (accepted 2026-10-22): Enfilade integration lives in bridge
  crates; EP-M7 proposes redrawing the design's Figure 1 accordingly.
- Decision D13 (accepted 2026-10-22): drag-and-drop and host clipboard
  integration are deferred; EP-M7 proposes a roadmap task for them under step
  5.1.
- Decision D14 (accepted 2026-10-22): a single-line text field with input
  method editor (IME) support is in scope: `HostEvent` carries IME events,
  `HostShell` exposes the IME cursor area, and `Role::TextInput` exists.
- Decision D15: font policy. Furniture always uses the bundled fonts in
  `cabochon-fonts`. A `--fonts bundled|bundled+system` option selects the
  content font collection: `run` defaults to `bundled+system`, and `export` and
  `review` to `bundled`. The manifest records every font and fallback. Tests
  pass `--fonts bundled` and never use system fonts. `Session::new` therefore
  takes two text-layout handles, one for furniture and one for content.
- Decision D20: assertion style (`googletest` matchers, `pretty_assertions`
  for whole values, `insta` for stable summaries; `#[gtest]` before
  `#[rstest]`, because the reverse order registers the test twice).

Task-level decisions (2026-10-23):

- Decisions T13, T14, and T15 (recorded in the 1.0.3 plan): the architecture
  is enforced by tested guards with allow-lists at crate level
  (`OB-ARCH-DEPS`), lint inheritance (`OB-ARCH-LINTS`), module level
  (`OB-ARCH-IMPORTS`, a `syn` import lint), and public-interface level
  (`OB-ARCH-PUBAPI`, using the `public-api` crate), and `serde` is confined to
  private data-transfer types. The 1.0.3 plan builds the guards and their rule
  tables in `tests/architecture/rules.rs`; this plan adds its crates' rows,
  each with a seeded violation that must fail.
- Decision T1: this plan runs last and marks roadmap tasks 1.0.1 and 1.0.2
  done, because the hello-window application proves both success criteria.
- Decision T5 (superseded by R3): move the hello-window fixture out of
  Escutcheon's tests.
- Decision T7 (narrowed by R5): the in-app Export command writes to the
  directory given by `run --export-dir`, because file dialogs are deferred.
- Decision T8: one pull request per task plan.
- Decision T9: ADR numbers are reserved: 0009 for the 1.0.3 plan, 0010 for
  1.0.2, 0011 for 1.0.4, and 0012 for this plan; 0013 for the 1.0.3 plan's
  Cranelift contingency. Any other pull request needing an ADR takes 0014 or
  later.
- Decision T11: the hello-window binary embeds its resource bundle with
  `include_str!` and loads it through Escutcheon's `MemorySource` by default;
  `--resources DIR` loads a directory through the `cap_std` adapter instead.
- Decision T12: behavioural scenarios split by crate: `furniture.feature`
  drives the contract probe in Clerestory's tests, and `hello_window.feature`
  drives the real application.

Refinement decisions (2026-11-21, author: planning agent; all await approval):

- Decision R1: the UI guidelines at commit `9852939` join the conformance
  basis at the book's substrate profile. Rules are cited by chapter file and
  heading, and by checklist identifier where `book/40-checklist.md` gives one.
  Rationale: the user introduced the book as a required reference, and it
  names this plan as a contract basis. Precedence follows C12.
- Decision R2: `hello-window`'s surface is the one in
  `book/36-hello-window.md`: the three menus and their items, a read-only
  document with a greeting, an explanation, and Export and About buttons, and a
  status line reporting the document and the last export time. The title is the
  document's name, a resource string. Edit, Window, New, Open, and Save are
  absent as the book's recorded fixture exemptions. Rejected: keeping the
  earlier surface (File, View, Help with an editable greeting inspector), which
  the book now contradicts on six points. Supersedes observable-success item 1
  of the earlier revision.
- Decision R3: Clerestory's contract areas that `hello-window` does not show
  (inspector panel, text field and IME, alert and confirmation dialogs) are
  exercised by a second resource-defined application, the contract probe. The
  bundle that the 1.0.2 plan authors as its `hello-window` fixture, which has
  an inspector and a text field, is that probe: EP-M4 renames its directory to
  `crates/cabochon-escutcheon/tests/fixtures/contract-probe/` with `git mv`.
  EP-M5 authors the real bundle in `crates/cabochon-hello-window/resources/`.
  Rationale: the roadmap requires inspector panels and decision D14 requires
  text input, while the book forbids `hello-window` from adding features. The
  generic inspector panel is furniture: a titled panel of property rows bound
  to an `InspectorModel`. The book's Inspector, with identity and authority
  blocks, is semantic-workspace scope and is excluded. Supersedes T5.
- Decision R4: command semantics follow the book's command record
  (`book/10-menus.md` "Command records"). Each command has one record with a
  semantic identifier, localized label, input requirement, availability with a
  single reason, optional shortcut, and menu location. Availability has three
  values: available, unavailable with a reason (shown disabled, still reachable
  by arrow keys), and inapplicable (absent). Menus, buttons, the palette, and
  shortcuts are views of the record and dispatch identically.
- Decision R5: Export and Print are distinct records. In-app Export
  (`file.export`) writes one screen-oriented `content.pdf` and its manifest
  entry to the `--export-dir` directory, and reports "unavailable: no export
  directory" as its reason when none was given. The command-line `export`
  subcommand does the same without a window. The `review` subcommand writes the
  eight-artefact conformance bundle, staged and renamed as a unit (`UIG`
  `CON-04`). Rationale: the book says an ordinary export yields one file and
  the bundle comes from "the review command". Print (`file.print`) calls a new
  `HostShell::request_print` operation with the print intent; the headless
  shell records the request and returns a scripted outcome. The hosted adapter
  returns `PrintOutcome::Unavailable`, which Clerestory reports through the
  four-part error anatomy. Rejected: calling the desktop print portal now,
  because portals belong to roadmap 5.1.2 and would add a D-Bus client and an
  asynchronous runtime. This is a recorded departure from the book's "opens
  the print surface" in hosted mode only.
- Decision R6: the command palette is in scope at its minimum: Control+K
  opens an application-modal overlay with a text field and a list of command
  records; matching is case-insensitive substring over localized labels;
  unavailable commands show their reason and never run; Escape closes through
  the focus-return chain. Objects, tools, places, and query prefixes are
  excluded because their predicates are false without Enfilade. Rationale: the
  book calls the palette a universal command surface (`CTL-12`), and it gives
  the text field a legitimate home in `hello-window`. Rejected: a disabled
  menu item, which would ship a must-rule failure.
- Decision R7: View holds two one-level submenus. Zoom holds Zoom in
  (Control+=), Zoom out (Control+-), and Actual size (Control+0), stepping
  through 50, 75, 100, 125, 150, 200, 300, and 400 per cent. Zoom scales only
  the document plane; arrow and page keys pan when the document has focus and
  the wheel scrolls. Scroll bars are excluded. Appearance holds Light and Dark
  as radio items; the initial value comes from the host when it reports one.
  Rejected: single cycling items, which hide state.
- Decision R8: F1 dispatches the built-in `help.context`, which shows the
  focused control's resource-defined description in the status line, or a
  "no help is available" message. About moves to Help with no shortcut.
  Diagnostics (`help.diagnostics`) opens an information sheet showing the
  application and renderer versions, shell adapter, locale chain, font
  collection, command-dispatch counters, and the last successful and last
  failed operation with timestamps from an injected clock. The copyable bundle
  is excluded until the clipboard exists (decision D13).
- Decision R9: dialogs are document-modal sheets drawn inside their window.
  Each declares a purpose (information, input, confirmation, recovery) and
  whether its confirming action is destructive; initial focus and the Return
  default follow `book/22-dialogs.md` "Defaults and focus". Actions sit on the
  trailing edge, safe action first. The order is Cabochon's on every host; host
  adaptation does not reorder buttons.
- Decision R10: focus is one state machine per window over an explicit
  scope stack, built on a small index-only kernel that is proved with Verus.
  The proof sketch fixed four representation choices before any interface was
  written: the focus owner is a single optional index, not a flag per control;
  eligibility is a precomputed boolean mask, so the kernel is total; modal
  dialogs, the palette, and menu-bar activation all push frames on one stack
  instead of keeping private "previous focus" fields; and menu items, palette
  rows, and tab stops share one cyclic-successor function. Rationale: keyboard
  reachability, the absence of keyboard traps, and never losing focus are
  promises every later application inherits, they must hold for any number of
  controls and any nesting depth, and the kernel is small and stable. Rejected:
  property tests alone, which sample; and Kani, whose bound would have to be
  chosen below real window sizes.
- Decision R11: the hosted adapter always presents the book's
  server-decorated profile: the host (or `winit`'s own frame where the
  compositor offers no decoration) draws the single title and window controls,
  and Clerestory draws no document header. Client-decorated and undecorated
  profiles are excluded until a shell adapter needs them (roadmap 6.1.2).
  Rationale: no speculative variants; the one-title rule is testable now.
- Decision R12: Clerestory keeps its own `Role` and `State` vocabulary,
  aligned with AccessKit's names, and reports accessible bounds. The hosted
  adapter maps roles with an exhaustive `match`; content nodes map from
  Lapidary's `SemanticRole`. The accessibility order equals the focus order.
- Decision R13: the session records a journal (dispatch records, focus
  changes, announcements, and a digest of each published accessibility tree)
  itself, independent of the shell. Rationale: headless and hosted runs can be
  compared without a decorator around the port, and AccessKit's lazy activation
  cannot hide a difference.
- Decision R14: the application binary installs a `metrics-util` debugging
  recorder and shows its snapshot in Diagnostics and in the session journal.
  Rationale: `AGENTS.md` requires new telemetry to be exercised through the
  application's collection path, and the design's session service does not
  exist yet.
- Proposed amendment `AM-ESC-1` (1.0.2 plan, schema version 1): one level of
  nested `menu`; `command` attributes `input` (none or required, which drives
  the ellipsis) and `unavailable` (a string key for the reason); a `help`
  string key on commands and controls; dialog `purpose` and a `destructive`
  flag on buttons; a `window` node with `title`, `min-size`, and `status-line`
  keys; built-ins `help.context` and `view.command-palette`; rejection of
  shortcuts on the reserved keys (Tab, Enter, Space, Escape, F1, F6,
  Control+Q, Control+K) other than the built-in bindings; and the example
  fragment's `app.about shortcut="F1"` removed.
- Proposed amendment `AM-BUR-1` (1.0.4 plan): Burin exposes its renderer
  version and declared parity tolerances as constants so the manifest can
  record them (`UIG` `CON-05`).
- Proposed amendment `AM-LAP-1` (1.0.3 plan): `TextLayout` is usable through
  `Arc<dyn TextLayout + Send + Sync>`, and the types this plan consumes are
  named in its interface section (`Scale120`, `Damage`, `HitPath`,
  `CaretStop`, `DevicePoint`). EP-M0 records the delivered names either way.

## Outcomes & retrospective

Not started.

## Context and orientation

Inspected base: `main` at commit `d295397` (2026-11-21). At that revision the
repository is one package, `cabochon` (`Cargo.toml`, `src/lib.rs`,
`src/main.rs`), with no `[workspace]`, no `crates/` directory, and no
`.config/` directory. Integration tests under `tests/` check the build
standard, the Makefile, and the CI workflows.

When implementation starts, the three earlier plans will have changed that.
The workspace will contain `cabochon-lapidary` (scenes, `resolve`, hit testing,
`damage`, `DeviceMapping`, accessibility nodes, the `TextLayout` port),
`cabochon-burin` (`ParleyTextLayout`, every output, `InteractiveSurface`,
`render_with_fallback`), `cabochon-fonts` (bundled font data),
`cabochon-escutcheon` (`compile`, `ResourceBundle`, `Localizer`,
`MemorySource`, built-in commands, the `escutcheon-check` binary, and a fixture
bundle), `cabochon-substrate-testkit` (`MonospaceTextLayout`, generators),
architecture guards under `tests/`, `.config/nextest.toml`, and prover tooling:
`make install-verus`, `make verus`, `make install-kani`, `make kani`, with
proof files under `verus/`. EP-M0 confirms each of these.

Gate commands that exist today: `make check-fmt`, `make lint` (Rustdoc,
Clippy, Whitaker), `make test` (nextest plus doctests, with `-D warnings`),
`make typecheck`, `make markdownlint`, `make spelling`, `make nixie`, and
`make audit`. `make test-hosted` does not exist; EP-M6 adds it. CI
(`.github/workflows/ci.yml`) runs on `ubuntu-latest`: an `msrv` job, and a
`build-test` job whose coverage step runs the suite under a ratchet.

Governing documents:

- `docs/cabochon-design.md` §3 (invariants), §6 (the runtime owns no
  top-level windows in hosted mode), §8.1 (Clerestory's responsibilities,
  identical under the hosted adapter and the future Mullion shell), §8.9, §11,
  §13, and §14 (spans and low-cardinality metrics; libraries install nothing).
- `docs/adrs/adr-0002-keep-the-hosted-runtime-independent-of-the-shell.md` and
  `docs/adrs/adr-0007-make-the-gem-substrate-a-production-boundary.md`.
- `docs/terms-of-reference.md`, "Success measures": "a developer can create a
  small resource-defined Cabochon application that opens a window, exposes
  standard menus and commands, renders a Lapidary scene, exports the same scene
  through Burin, and integrates with accessibility and keyboard traversal
  without writing host-desktop-specific code." The next measure says the
  hosted-product hypothesis is validated only by a workflow that also exercises
  Enfilade; this plan does not claim it.
- `docs/context.md` and `references/cabochon-white-paper.md` (background).
- `docs/roadmap.md` tasks 1.0.1 and 1.0.2.
- The UI guidelines,
  <https://github.com/leynos/cabochon-ui-guidelines/blob/main/book/README.md>,
  at commit `9852939`: chapters 5, 9, 10, 11, 12, 14, 17, 18, 19, 22, 23, 25,
  36, 37, 38, 39, and 40.
- Testing and style guides: `docs/rust-testing-with-rstest-fixtures.md`,
  `docs/rust-doctest-dry-guide.md`,
  `docs/reliable-testing-in-rust-via-dependency-injection.md`,
  `docs/complexity-antipatterns-and-refactoring-strategies.md`,
  `docs/scripting-standards.md`, and `docs/documentation-style-guide.md`.

Terms:

- Host shell: the desktop that owns windows and input (GNOME or KDE Plasma).
  In this plan "hosted" always describes the shell adapter, not the runtime.
- Headless shell: a Clerestory adapter that runs applications without a
  display, recording frames, IME cursor areas, and print requests.
- Furniture: standard application chrome: menu bar, menus, status line,
  sheets, the palette, and inspector panels.
- Sheet: a dialog attached to, and drawn inside, one window.
- Logical unit (lu): the guidelines' interface length at 100 per cent scale;
  Clerestory converts it to device pixels through the 1/120 scale.
- Tab stop: a control that Tab can reach. A composite (the menu bar, a menu,
  the palette's list) is one stop; arrow keys move a cursor inside it.
- Focus scope: the set of stops a surface owns. A trapping scope (a sheet or
  the palette) keeps Tab inside itself.
- Focus-return chain: where focus goes when a surface closes: the control
  that opened it, else the surviving context, else a safe fallback.
- Responder chain: the order in which a command is offered to handlers:
  focused element, sheet, window, application.
- IME (input method editor): host software that composes text through
  intermediate "preedit" text before committing it.
- Conformance bundle: the eight artefacts the `review` subcommand writes.
- Kernel: a small, total, integer-only function proved with Verus.

Deliberate exclusions, each with the predicate that makes the owning guideline
inapplicable or the roadmap task that owns it: file dialogs (decision D8,
roadmap 5.1.2); host clipboard and drag-and-drop (D13); the print portal (R5,
roadmap 5.1.2); client-side decoration and the document header (R11); tool
strip and masthead (optional bands, absent); panes, splitters, and F6 (one
pane; `help.context` aside, F6 has nothing to cycle); the "More" overflow menu
(session start proves the menu bar fits the minimum width in every shipped
locale); context menus (no object under the pointer has commands of its own);
the state chip (no live relationship exists without Enfilade); palette objects,
tools, and places (R6); the book's Inspector identity and authority blocks
(R3); undo (the document is read-only, and the probe's field commits directly);
window geometry persistence (roadmap 5.1.1 owns per-user state); multiple
windows per session beyond what the port admits; motion (no finite transition
is drawn, so reduced motion is trivially satisfied); and per-user runtime
activation (roadmap 5.1.1).

Budget survey. No comparable implemented plan exists: the four step-1.0 plans
are approved and unimplemented, and their revision notes record no budget
change. Each used one trigger, 3,000 net Rust lines or 60 files per milestone,
with no configuration or documentation limit. This plan estimates, by
milestone: EP-M0 200 lines of throwaway probes; EP-M1 2,600; EP-M2 900 of
proof; EP-M3 3,000; EP-M4 2,500; EP-M5 2,900; EP-M6 1,900; EP-M7 100. The
total, about 14,100, plus 13 per cent contingency gives the 16,000 limit, and
the per-milestone limit of 3,500 gives the largest milestone 17 per cent.
Configuration (three manifests, two bundles in three locales, one CI job,
nextest entries) is estimated at 950 lines. Documentation (ADR 0012, a design
section, users' and developers' guide sections, layout and contents entries) is
estimated at 1,200 lines.

Skills to load: `execplans`, `rust-router` (then `rust-state-machines` for
focus, menus, and text editing, `rust-types-and-apis`, `rust-errors`, and
`domain-cli-and-daemons` for the application), `hexagonal-architecture`,
`rust-verification` and `verus` (EP-M0, EP-M2), `rust-unit-testing`,
`proptest`, `nextest`, `arch-decision-records`, `en-gb-oxendict`,
`codegraph-mcp`, and `firecrawl-mcp` for `winit`, `softbuffer`, and AccessKit
documentation.

## Conformance basis

Upstream revisions inspected: `leynos/cabochon` `main` at `d295397` for the
design document, terms of reference, roadmap, and ADRs 0001 to 0008;
`leynos/cabochon-ui-guidelines` at `9852939`. The book records `de40019` as the
platform revision it was checked against; no document cited here changed
between `de40019` and `d295397`. ADR 0012 does not exist yet; this plan writes
it. No supply-chain or security standard beyond `make audit` governs this work.

Derived requirements:

- `REQ-WINDOW`: a session opens one document window from resources, with one
  title, a declared minimum size, and vetoable close (`RM-1.0.1`, `TDD-CLR`,
  `UIG` windows).
- `REQ-COMMAND`: every command is one record; menus, buttons, palette, and
  shortcuts dispatch it identically (`RM-1.0.1`, `UIG` `CTL-12`).
- `REQ-MENU`: an in-window menu bar with one level of submenus, state marks,
  disabled-but-reachable items, and the guideline keys (`RM-1.0.1`).
- `REQ-DIALOG`: alert, confirmation, and About sheets with purpose-scoped
  defaults (`RM-1.0.1`, decision D8).
- `REQ-FOCUS`: exactly one focus owner whenever any control is eligible; Tab
  reaches every eligible stop; traps only in modal surfaces; focus returns
  through the chain (`RM-1.0.1`, `UIG` focus).
- `REQ-KEYS`: the routing decision table of `book/18-focus-keyboard.md`.
- `REQ-TEXT`: a single-line text field with IME composition (decision D14).
- `REQ-INSPECTOR`: a generic inspector panel bound to a model (`RM-1.0.1`).
- `REQ-A11Y`: roles, names, states, bounds, and order for every control
  (`RM-1.0.1`, `TOR-SM-GEM`, `UIG` accessibility).
- `REQ-L10N`: `en-XA` and `ar-XB` run without clipping or reversed order
  (`UIG` `CON-07`).
- `REQ-HOST`: the same session runs under the headless and hosted shells
  with no host type in the application (`RM-1.0.1`, `TDD-INV-6`, ADR 0002).
- `REQ-APP`: `hello-window` matches chapter 36 and takes its interface from
  resources (`RM-1.0.2`, `UIG` `CON-01` to `CON-06`).
- `REQ-OBS`: dispatch is traced and counted through the application's own
  collection path (`TDD-OBS`).

Gaps at the base revision: `GAP-NO-CLERESTORY` (no crate, port, or adapter
exists), `GAP-NO-APP` (no resource-defined application), and `GAP-NO-HOSTED-CI`
(no CI job has a display).

```plaintext
RM-1.0.1 -> REQ-FOCUS -> EP-M1, EP-M2 -> OB-CLR-FOCUS-KERNEL, OB-CLR-FOCUS, OB-CLR-MODAL
RM-1.0.1 -> REQ-COMMAND, REQ-KEYS -> EP-M1 -> OB-CLR-ROUTE, OB-CLR-KEYS
RM-1.0.1 -> REQ-WINDOW, REQ-MENU, REQ-DIALOG -> EP-M3 -> OB-CLR-MENU, OB-CLR-DIALOG, OB-CLR-LAYOUT
RM-1.0.1 -> REQ-A11Y, REQ-L10N -> EP-M3 -> OB-CLR-A11Y, OB-CLR-LAYOUT
RM-1.0.1 -> REQ-TEXT, REQ-INSPECTOR -> EP-M4 -> OB-CLR-TEXT, OB-CLR-PALETTE, furniture.feature
RM-1.0.1 -> TDD-INV-6, ADR-0002 -> REQ-HOST -> EP-M6 -> OB-CLR-HOST, make test-hosted
RM-1.0.2 -> REQ-APP -> EP-M5 -> OB-APP-RESOURCES, OB-APP-SURFACE, hello_window.feature
TOR-SM-GEM -> RM-1.0.1..4 -> EP-M5, EP-M6 -> OB-APP-EXPORT, OB-APP-E2E, OB-CLR-HOST
TDD-OBS -> REQ-OBS -> EP-M1, EP-M5 -> OB-CLR-OBS
GAP-NO-CLERESTORY -> EP-M1; GAP-NO-APP -> EP-M5; GAP-NO-HOSTED-CI -> EP-M6
```

## Verification plan

Clerestory's obligations fall into three kinds. The focus rules are a
transition system whose guarantees must hold for every window layout and every
nesting depth; they get a formal proof. The key-routing rules are a finite
table; they get an exhaustive table test. Everything else is orchestration,
layout, or integration, and gets property tests against independent oracles,
named cases, snapshots, behavioural scenarios, and end-to-end tests.

### Abstract model

The focus model is written before the implementation and independently of it.

- A window has a finite sequence of stops, `stops`, in reading order. Each
  stop `i` has a scope identifier `scope(i)` and a flag `operable(i)`, true
  when the control is visible and enabled.
- A scope stack `frames` is a non-empty sequence. Each frame has a scope
  identifier, a flag `traps`, and an ordered list of return candidates (stop
  indices). The bottom frame is the window's own scope and never traps. The
  active scope is the top frame's.
- The eligibility mask is `eligible(i) = operable(i) ∧ scope(i) = active`.
- The state is `(stops, frames, owner)`, where `owner` is an optional index.
- `succ(mask, from)` is the first index after `from`, cyclically, whose mask
  bit is true; `pred` is its mirror image. With no start, they return the first
  and last true index.
- Transitions: `Tab`, `ShiftTab`, `FocusPointer(i)`, `SetOperable(i, b)`,
  `Insert(i, stop)`, `Remove(i)`, `Push(frame)`, and `Pop`. `Pop` on the bottom
  frame is rejected and leaves the state unchanged, as is any transition naming
  an index out of range; the kernel has no preconditions.
- `Pop` resolves the popped frame's candidates in order and takes the first
  that is eligible in the new active scope, else `succ` with no start.
- Whenever a transition leaves the owner ineligible, the owner is re-homed
  with `succ(mask, owner)`.

Roving cursors (items in a menu, rows in the palette) are not stops. Each is
an independent `(mask, cursor)` pair that uses the same `succ` and `pred`, with
a mask that keeps disabled items and drops separators.

### Invariants and lemmas

- `INV-FOCUS-WF` (well-formed focus, `REQ-FOCUS`): after every transition the
  stack is non-empty and its bottom frame does not trap; `owner` is either
  absent or an in-range, eligible index; and `owner` is absent only when no
  stop is eligible. One owner at most holds by representation.
- `L-RING-TOTAL`: `succ` returns no index exactly when the mask has no true
  bit; any index it returns is in range and its bit is true.
- `L-RING-NEAREST`: when `succ(mask, i)` is `j`, no true bit lies strictly
  between `i` and `j` in cyclic order. With `L-RING-TOTAL` this determines
  `succ` uniquely.
- `L-RING-INVERSE`: for a true bit `i`, `pred(mask, succ(mask, i)) = i` and
  `succ(mask, pred(mask, i)) = i`.
- `L-RING-CYCLE` (keyboard reachability): if the mask has `k ≥ 1` true bits
  and `i` is one of them, the `k` successive applications of `succ` from `i`
  are pairwise distinct true bits and the `k`-th returns to `i`. Hence one
  full Tab cycle visits every eligible stop exactly once.
- `L-SCOPE-CONTAIN` (trap exactly in modal surfaces): `Tab` and `ShiftTab`
  never change `frames`, and the owner they produce is eligible in the active
  scope. So focus cannot leave a trapping scope by traversal, and by
  `L-RING-CYCLE` every stop of a non-trapping scope remains reachable.
- `L-SCOPE-RESTORE` (focus-return chain): if a frame is pushed with the
  current owner as its first candidate, and that stop is still operable when
  the frame is popped with the stack beneath unchanged, the owner afterwards
  equals the owner before the push. If it is not, the owner is the first
  eligible candidate, else the first eligible stop. By induction on depth,
  nested pushes unwind to the original owner.
- `L-STEP-PRESERVES`: every transition preserves `INV-FOCUS-WF`. This is the
  inductive step from which the invariant holds over every operation sequence.
- `INV-ROUTE` (`REQ-COMMAND`): a dispatch reaches the first responder in the
  chain that accepts it, an unavailable command is never delivered, and a
  record's dispatch is the same from every view.
- `INV-LAYOUT` (`REQ-WINDOW`, `REQ-L10N`): children lie inside their parent,
  siblings do not overlap, and layout is a function of its inputs.
- `INV-A11Y` (`REQ-A11Y`): every operable control has a role and a non-empty
  name unique within its group; parent and child links agree; the tree's
  pre-order of operable nodes equals the focus order; the focus owner is in the
  tree.
- `INV-TEXT` (`REQ-TEXT`): the caret rests only on caret stops; preedit text
  is never committed without a commit event; the reported IME cursor area
  contains the caret.
- `INV-PAN`: the pan offset stays within the zoomed document's overflow.

### Axioms (trusted, not verified here)

- `AXIOM-VERUS`: the pinned Verus and Z3 are sound, and on the single-source
  route `verus_builtin_macros` erases only ghost code (source: the 1.0.3 plan's
  `A-VERUS`). Depends on it: `OB-CLR-FOCUS-KERNEL`.
- `AXIOM-LAP-HIT`: Lapidary's hit testing returns the topmost covering
  region (proved for its kernel and property-tested at its public interface by
  the 1.0.3 plan's `OB-LAP-HIT`). Depends on it: pointer routing.
- `AXIOM-LAP-CARET`: the `TextLayout` port reports caret stops at grapheme
  cluster boundaries. Depends on it: `OB-CLR-TEXT`. Checked at the boundary by
  a named case with a combining sequence and an emoji sequence.
- `AXIOM-BUR-INVAL`: Burin redraws no pixel outside the damage it is given
  (the 1.0.4 plan's `OB-BUR-INVAL`). Depends on it: redraw.
- `AXIOM-WINIT`: `winit` and `softbuffer` deliver input and present buffers
  as documented, and headless `weston` represents GNOME and KDE Plasma closely
  enough for a smoke test. Residual: host-specific behaviour, IME, fractional
  scale, and AT-SPI read-back (roadmap 5.1.4).
- `AXIOM-ACCESSKIT`: `accesskit_winit` publishes the supplied tree to AT-SPI
  faithfully once activated.

### Methods per obligation

Property suites follow the 1.0.3 plan's non-vacuity mechanics, restated here:
a manual `proptest::test_runner::TestRunner` with a fixed seed, per-class
counters with asserted minimum counts, directly constructed important classes,
256 cases per suite (state-machine suites use sequences of up to 32
operations), and committed `proptest-regressions/` files. A seeded fault is a
temporary edit, applied to see the named test fail and then reverted; its
transcript goes in `Artefacts and notes`, and no fault is committed.

- Obligation `OB-CLR-FOCUS-KERNEL`: `L-RING-TOTAL`, `L-RING-NEAREST`,
  `L-RING-INVERSE`, `L-RING-CYCLE`, `L-SCOPE-CONTAIN`, `L-SCOPE-RESTORE`, and
  `L-STEP-PRESERVES` over the kernel in
  `crates/cabochon-clerestory/src/kernel/`. Method: Verus proof. Rationale: the
  properties quantify over all mask lengths, stack depths, and operation
  sequences, and the kernel is pure and index-only. Domain: all inputs; no
  preconditions. Artefact: `verus/clerestory_proofs.rs` (which includes
  `clerestory_ring.rs` and `clerestory_scope.rs`), or the kernel modules
  themselves on the single-source route. Evidence: `make verus`; red first by
  running it with the lemma statements and empty proof bodies and recording the
  open goals; discharged when it reports every function verified with no error
  and `OB-PROOF-MIRROR` passes. Non-vacuity: no lemma has a `requires` clause
  beyond "the bit at `i` is true", which the witness mask `[true]` satisfies; a
  proof function exhibits concrete states reaching each transition, including a
  two-deep stack and a pop whose first candidate is inoperable; seeded faults:
  starting the scan at `from` rather than `from + 1` must fail
  `L-RING-NEAREST`, and omitting the re-home in `SetOperable` must fail
  `L-STEP-PRESERVES`. Residual: the proof covers the kernel, not the session
  code that computes masks and candidates.
- Obligation `OB-CLR-FOCUS` (`INV-FOCUS-WF` at the public interface, and the
  executable twin of each lemma). Method: `proptest-state-machine` driving
  `Session` through host events against a reference model written directly
  from `Abstract model` over `Vec` and linear search, sharing no code with the
  kernel; plus named cases. Rationale: it ties the proved kernel to the masks
  and candidates the session really computes. Domain: generated windows of 0 to
  24 stops with sheets, the palette, and menu activation; operations Tab,
  Shift+Tab, enable, disable, show, hide, pointer focus, open and close of each
  surface. Artefact: `crates/cabochon-clerestory/tests/focus_machine.rs`.
  Evidence: `cargo nextest run -p cabochon-clerestory focus_machine`; red
  against a session whose Tab ignores scope. Non-vacuity: counters require at
  least 20 per cent of cases with a trapping scope open, 10 per cent with an
  inoperable invoker at close, and 5 per cent with no eligible stop; seeded
  fault: restoring focus to the invoker without checking operability.
- Obligation `OB-CLR-MODAL`: while a sheet or the palette is open, no key or
  pointer event reaches a control outside it, and the document behind is
  inert. Method: the same state machine with an event-delivery oracle, plus the
  "Dialogs are modal" scenario. Non-vacuity: at least 20 per cent of cases
  deliver a pointer event outside an open sheet; seeded fault: hit testing the
  window before the sheet.
- Obligation `OB-CLR-KEYS` (`REQ-KEYS`): the pure function
  `route_key(mode, key)` returns the disposition the routing decision table
  gives. Method: an `rstest` table with one case per table cell, plus a
  totality test that iterates every `InputMode` and every `RoutedKey` variant
  and asserts each pair appears in the table. Rationale: the domain is finite
  and each cell is normative. Artefact:
  `crates/cabochon-clerestory/src/key_routing/tests.rs`. Non-vacuity: the
  totality test fails when a variant is added without a row; seeded fault:
  letting Return activate the default while a button has focus.
- Obligation `OB-CLR-ROUTE` (`INV-ROUTE`). Method: a property test over
  generated responder chains and availability, with an oracle that scans the
  chain independently; named cases for the vetoable built-in `window.close`,
  for session start failing fast when the application declares a command the
  bundle lacks, and for a shortcut on a reserved key. A proof is not used: the
  specification of first-match dispatch is the same fold as its
  implementation, so a proof would restate it. Non-vacuity: at least 30 per
  cent of chains have two or more accepting responders and 20 per cent an
  unavailable command; seeded fault: offering the command to the application
  first.
- Obligation `OB-CLR-MENU` (`REQ-MENU`): no menu stays open while focus is
  outside the menu bar's frame; arrows reach disabled items and skip
  separators; Enter on a disabled item dispatches nothing; Escape and Left or
  Right close a submenu; a menu taller than the space below flips or scrolls
  and stays inside the window. Method: the focus state machine extended with
  menu operations, named cases, and a property on generated menu sizes and
  window heights. Seeded fault: skipping disabled items.
- Obligation `OB-CLR-DIALOG` (`REQ-DIALOG`): for each purpose and
  destructive flag, initial focus and the Return default are those of
  `book/22-dialogs.md`; Escape performs the safe exit; a sheet never opens
  another sheet. Method: an `rstest` table over purpose, destructiveness, and
  key. Seeded fault: making the destructive button the Return default.
- Obligation `OB-CLR-TEXT` (`INV-TEXT`). Method: `proptest-state-machine`
  over key and IME events against a reference model on a vector of grapheme
  clusters; named cases for the routing rows (Return commits the field's own
  action, Escape reverts the draft). Non-vacuity: at least 25 per cent of cases
  contain a preedit that is cancelled rather than committed; seeded fault:
  committing preedit on focus loss.
- Obligation `OB-CLR-PALETTE`: the result list is exactly the applicable
  commands whose label contains the query, in menu order; unavailable results
  are present with their reason and never run. Method: property test with a
  filter oracle; named cases. Seeded fault: dropping unavailable commands.
- Obligation `OB-CLR-LAYOUT` (`INV-LAYOUT`, `REQ-L10N`). Method: property
  test over generated layout trees with a geometric oracle that checks
  containment and disjointness directly from the output rectangles; `insta`
  snapshots of the probe's dialog and of `hello-window` in `en-GB`, `en-XA`,
  and `ar-XB`, as text summaries of named rectangles in logical units, with no
  paths, timestamps, or font metrics; named cases for the band heights (menu
  bar 29 lu, status line 26 lu), control targets (28, 32, 36 lu), dialog width
  (420 to 560 lu), and the 512 by 384 lu floor. A proof is not used: the layout
  engine will change with later profiles and is not yet a stable kernel.
  Non-vacuity: at least 30 per cent of trees nest three levels deep and 20 per
  cent have a child whose natural size exceeds its share; seeded fault:
  dropping the inter-child gap from the extent sum.
- Obligation `OB-CLR-A11Y` (`INV-A11Y`). Method: property test over
  generated bundles; the hosted conversion is checked by feeding its
  `TreeUpdate` to `accesskit_consumer::Tree` and comparing roles, names,
  states, and bounds, with a test that the role mapping covers every `Role`.
  Seeded fault: emitting the tree in paint order.
- Obligation `OB-CLR-OBS` (`REQ-OBS`): dispatch emits a span and increments
  `clerestory_command_dispatch_total` with the fixed `outcome` labels
  `dispatched`, `unavailable`, `unhandled`, and `vetoed`. Method: a named case
  with a local recorder in Clerestory, and in `OB-APP-E2E` a non-zero counter
  read through the binary's own recorder in the journal.
- Obligation `OB-APP-RESOURCES` (`REQ-APP`): the application builds its
  window, menus, sheets, strings, shortcuts, and icon only from the compiled
  bundle, loaded through `MemorySource` by default; changing a string in a copy
  of the bundle passed with `--resources` changes the presented title without
  recompiling. Method: end-to-end test with `assert_cmd`.
- Obligation `OB-APP-SURFACE` (`REQ-APP`): the presented menus, items,
  buttons, and status line are exactly those of `book/36-hello-window.md`.
  Method: a named test comparing the journal's first accessibility tree with a
  table transcribed from the chapter. Seeded fault: adding an Edit menu.
- Obligation `OB-APP-EXPORT`: `review` writes all eight artefacts from one
  scene revision, staged in a temporary directory and renamed into place;
  `export` writes one file and its manifest entry; both refuse a non-empty
  directory they did not create unless `--force` is given; a failed export
  leaves the previous output intact; the manifest records intents, scene
  revision, versions, tolerances, fonts and substitutions, and which identity
  fields apply; `en-XA` and `ar-XB` also produce the bundle. Method: end-to-end
  tests; `insta` snapshot of the manifest with versions and timestamps
  normalized.
- Obligation `OB-APP-E2E`: a scripted headless session
  (`run --shell headless --script FILE --journal FILE`) exercising menus,
  shortcuts, the palette with IME text, About, Diagnostics, zoom, appearance,
  print, Tab traversal, and quit produces the expected journal and exit status
  in `en-GB`, `en-XA`, and `ar-XB`. Method: end-to-end test.
- Obligation `OB-CLR-HOST` (`REQ-HOST`): the same script without its IME
  steps, with `ParleyTextLayout` and bundled fonts, run under the headless
  shell and under the hosted shell inside headless `weston`, yields identical
  journals; the hosted run presents at least one frame and the window shows one
  title. Method: end-to-end test run by `make test-hosted`. Non-vacuity: the
  test asserts the journal holds at least one dispatch, one focus change, and
  one tree digest, so two empty journals cannot pass.

### Formal verification decision

Verus is used for the focus kernel (decision R10) and nowhere else. A proof
adds a guarantee over all layouts and all operation sequences that sampling
cannot give, for the property users with keyboards and screen readers depend on
most. It leaves unverified the session code that derives masks and candidates,
which `OB-CLR-FOCUS` covers through the public interface. Kani is not used: no
obligation here is a bounded arithmetic or memory question, and scale
conversion belongs to Lapidary's `DeviceMapping`, already covered by the 1.0.3
plan. Command routing, layout, text editing, diagnostics formatting, the
command-line interface, and adapter plumbing carry no proof obligation, for the
reasons given beside each.

### Behavioural specification

The runner is `rstest-bdd`: each scenario binds with
`#[scenario(path = "...", name = "...")]`, and step functions take fixtures by
parameter name. The command is `make test`.

`crates/cabochon-clerestory/tests/features/furniture.feature` (EP-M3 and
EP-M4; drives the contract probe; `REQ-FOCUS`, `REQ-DIALOG`, `REQ-TEXT`,
`REQ-INSPECTOR`, `REQ-A11Y`):

```gherkin
Feature: Clerestory furniture behaves consistently

  Scenario: Every view of a command dispatches the same record
    Given the contract probe under the headless shell
    When the user presses "Control+K", types "about", and presses "Enter"
    And the user presses "Escape"
    And the user chooses "Help" then "About"
    Then the journal shows "help.about" twice with identical records

  Scenario: Dialogs are modal and restore focus
    Given the probe's greeting field has focus
    And the About sheet is open
    When the user presses "Tab" five times
    Then focus never leaves the sheet
    When the user presses "Escape"
    Then focus returns to the greeting field

  Scenario: Focus falls back when the invoker disappears
    Given a confirmation sheet opened from a button
    When the application disables that button and the user presses "Escape"
    Then focus moves to the next operable control in the same panel

  Scenario: A destructive confirmation defaults to the safe action
    Given a destructive confirmation sheet is open
    When the user presses "Enter"
    Then the journal shows "dialog.cancel" and no destructive command

  Scenario: Input methods compose text before committing it
    Given the probe's greeting field has focus
    When the input method sends preedit "ni" and then commits "你"
    Then the field shows "你" and the inspector model's greeting is updated
    And only the greeting's bounds are damaged

  Scenario: A disabled menu item is reachable and inert
    Given the probe with "Export" unavailable
    When the user opens "File" and presses "Down" until "Export" is highlighted
    Then the item's reason is announced
    When the user presses "Enter"
    Then nothing is dispatched and the menu stays open

  Scenario: Furniture is accessible
    Given the contract probe under the headless shell
    Then every operable control has a role, a unique non-empty name, and bounds
    And the accessibility order equals the Tab order
```

`crates/cabochon-hello-window/tests/features/hello_window.feature` (EP-M5;
`REQ-APP`, `REQ-L10N`):

```gherkin
Feature: The hello-window application is defined by its resources

  Scenario: The window matches the reference surface
    Given the hello-window application under the headless shell
    When the session starts
    Then the menu bar lists "File", "View", and "Help" and no other menu
    And "File" lists "Export", "Print", and "Quit"
    And the document shows a primary "Export" button and an "About" button
    And a status line is present

  Scenario: Strings change without recompiling
    Given a copy of the resource bundle whose document title is "Bonjour"
    When the application starts with that bundle through --resources
    Then the presented window is titled "Bonjour"

  Scenario: Export reports where no directory was given
    Given the hello-window application started without an export directory
    When the user opens "File"
    Then "Export" is unavailable with the reason "No export directory was given"

  Scenario: Printing reports an unavailable host honestly
    Given the headless shell is scripted to report printing as unavailable
    When the user chooses "File" then "Print"
    Then a message says what happened, what survived, why, and what to do next
    And the status line keeps the message until it is dismissed

  Scenario: The interface survives the pseudo-locales
    Given the hello-window application with locale "ar-XB"
    Then the menu bar starts at the right edge
    And no label is clipped
```

## Plan of work

Red-Green-Refactor throughout: write the failing test or lemma statement, see
it fail for the stated reason, make it pass with the smallest change, then
tidy. Each milestone ends with `make check-fmt`, `make lint`, and `make test`
passing, run sequentially through the `scrutineer` agent, and a commit. Each
dependency addition is its own commit followed by `make audit`.

### EP-M0: feasibility probes and prerequisite check (prototype)

This milestone resolves four named uncertainties and commits no production
code. Probes live under a scratch directory outside the repository and are
deleted afterwards; their transcripts go in `Artefacts and notes`.

1. Prerequisites. Confirm the sibling plans' deliverables listed in `Context
   and orientation` exist, record the delivered names of the Lapidary, Burin,
   and Escutcheon types this plan consumes, and confirm `AM-ESC-1`,
   `AM-BUR-1`, and `AM-LAP-1` were delivered. Go: all present. No-go: set
   `Status: BLOCKED` and name the missing item.
2. Hosted provisioning. In a throwaway crate, open a `winit` 0.30 window
   under `weston --backend=headless` with a private `XDG_RUNTIME_DIR`,
   `WAYLAND_DISPLAY` set, and `DISPLAY` unset; present one `softbuffer` frame;
   under `dbus-run-session`, set `org.a11y.Status.IsEnabled` and observe
   whether the AccessKit activation handler runs. Expected: one frame
   presented. Go: a frame is presented locally (EPEL packages or a rootless
   Podman image) and on an `ubuntu-latest` runner. No-go: the hosted tolerance
   triggers. The activation result decides only whether `make test-hosted`
   also asserts activation.
3. Proof shape. Write the kernel's `succ` and the statements of
   `L-RING-NEAREST` and `L-RING-CYCLE` in a scratch Verus file and run the
   pinned Verus. Expected: open goals for the two lemmas, with no error about
   unsupported constructs. Go: the statements are accepted. No-go: restructure
   the kernel once; a second failure triggers the proof tolerance.
4. Tooling. Confirm `rstest-bdd` builds with the workspace's `rstest`, that
   the `msrv` job is unaffected, and that `#[gtest]` before `#[rstest]`
   compiles under the pinned toolchain.

### EP-M1: focus kernel, command and key routing, session, headless shell

Assigned: `REQ-FOCUS`, `REQ-COMMAND`, `REQ-KEYS`, `REQ-OBS`,
`GAP-NO-CLERESTORY`; decisions R4, R10, R13.

Create `crates/cabochon-clerestory` and add its rows to the architecture
guards, running each seeded violation red first. Write, in this order:

1. `kernel::ring` (`succ`, `pred`) and `kernel::scope` (the frame stack and
   candidate resolution), shaped for proof as the 1.0.3 plan shapes its
   kernels: slices, `while` loops, no preconditions. Red: unit tests for the
   named cases in `Abstract model`.
2. `focus::FocusState` and its `step` function over the kernel, then
   `OB-CLR-FOCUS`'s reference model and state machine. Red: the machine fails
   against a `step` that ignores scope.
3. `key_routing::route_key` and `OB-CLR-KEYS`'s table and totality test.
4. `command` (records, availability, responder chain, dispatch) with
   `OB-CLR-ROUTE` and `OB-CLR-OBS`.
5. The `Application` trait, `Session`, `HostEvent`, the `HostShell` and
   `SessionWaker` ports, the session journal, and `shell::headless`.
6. `OB-CLR-MODAL` at the event-delivery level with placeholder surfaces.

Draft ADR 0012 as `docs/adrs/adr-0012-<title-slug>.md` with status Proposed,
and record the crate's ownership and reuse policy in
`docs/developers-guide.md`.

### EP-M2: Verus proofs of the focus kernel

Assigned: `REQ-FOCUS`; `OB-CLR-FOCUS-KERNEL`.

Add `verus/clerestory_proofs.rs`. Write every lemma statement first and run
`make verus` to record the open goals. Then prove the ring lemmas, the scope
lemmas, and `L-STEP-PRESERVES`, with supporting lemmas as the verification
conditions require; those details are deferred to this point on purpose. Extend
the `verus` recipe to run the Clerestory proof file as a second
`$(PROVER_TOOLS) verus run` invocation, extend
`tests/prover_tooling_contract.rs` to expect both invocations, and extend
`OB-PROOF-MIRROR` to count the Clerestory kernel regions. Apply and revert the
two seeded faults. If a proof forces a kernel change, update `Abstract model`,
the reference model, and `OB-CLR-FOCUS` in the same commit. Record the proof
inventory, and what it does not cover, in `docs/developers-guide.md` and in the
design document's §8.1 draft.

### EP-M3: metrics, theme, layout, furniture, and the accessibility tree

Assigned: `REQ-WINDOW`, `REQ-MENU`, `REQ-DIALOG`, `REQ-A11Y`, `REQ-L10N`;
decisions R7 (state marks and submenus), R9, R11, R12.

Implement `measure` (the guideline lengths as named constants in logical
units, with their chapter cited), `theme` (light and dark colour roles from
`book/12-palette.md`, including the focus ring), the layout engine (`column`,
`row` with `align`, `label`, `button`, `text-field`, `separator`, `spacer`,
`property-row`; measured through the `TextLayout` port; mirrored for
right-to-left locales), and the furniture scene builders for the menu bar,
menus and submenus with state marks, the status line (message, progress, and a
polite live region), and sheets. Implement `accessibility::AccessibilityTree`,
grafting the content scene's nodes under the document node. Write
`OB-CLR-MENU`, `OB-CLR-DIALOG`, `OB-CLR-LAYOUT`, `OB-CLR-A11Y`, and the menu,
dialog, and accessibility scenarios of `furniture.feature`. Clerestory's unit
tests use a small monospace `TextLayout` defined in the crate's own test
support; integration tests use the testkit's `MonospaceTextLayout`; the feature
uses Burin's `ParleyTextLayout` with bundled fonts, a development-only edge.

### EP-M4: text input, command palette, inspector panel, contract probe

Assigned: `REQ-TEXT`, `REQ-INSPECTOR`, `REQ-COMMAND`; decisions R3, R6, D14.

Implement the single-line text field (caret movement by caret stop, selection,
preedit, commit, revert, IME cursor area), the command palette over it, and the
inspector panel bound to an `InspectorModel`. Rename Escutcheon's fixture to
`contract-probe` (decision R3) and repoint Escutcheon's tests, its schema
reference, and `docs/developers-guide.md`. Write `OB-CLR-TEXT`,
`OB-CLR-PALETTE`, and the remaining scenarios of `furniture.feature`.

### EP-M5: the hello-window application under the headless shell

Assigned: `REQ-APP`, `REQ-OBS`, `GAP-NO-APP`; decisions R2, R5, R7, R8, R14,
T7, T11.

Create `crates/cabochon-hello-window` and add its rows to the architecture
guards. Author its bundle under `crates/cabochon-hello-window/resources/` in
`en-GB`, with the commands `file.export`, `file.print`, `app.quit`,
`view.zoom-in`, `view.zoom-out`, `view.zoom-actual`, `view.appearance-light`,
`view.appearance-dark`, `help.about`, and `help.diagnostics`, and embed it.
Implement the `run`, `export`, and `review` subcommands; `--locale`,
`--shell headless`, `--script`, `--journal`, `--exit-after-first-frame`,
`--export-dir`, `--resources`, `--fonts`, and `--appearance`; the document
scene and its two buttons; zoom and pan; the print request and its error
message; Diagnostics; a `tracing` subscriber writing to standard error and the
metrics recorder; a clock port; and environment reading through `mockable`.
`main` returns `eyre::Result`; every other function returns a typed error.
Write `OB-APP-RESOURCES`, `OB-APP-SURFACE`, `OB-APP-EXPORT`, `OB-APP-E2E`, and
`hello_window.feature`. Correct the `mockable` version and the `temp_env`
advice in `docs/reliable-testing-in-rust-via-dependency-injection.md`.

### EP-M6: the hosted shell adapter and its CI job

Assigned: `REQ-HOST`, `GAP-NO-HOSTED-CI`; decision R11.

Create `crates/cabochon-clerestory-hosted` and add its rows to the
architecture guards: the `winit` event loop (`ApplicationHandler`),
`softbuffer` presentation of `InteractiveSurface` frames with damage, scale
handling in 1/120 units, key, pointer, wheel, and IME mapping, close requests,
host appearance, and `accesskit_winit` publication, with all mapping in pure,
unit-tested functions. Hosted tests carry
`#[ignore = "needs Wayland; run make test-hosted"]`.

Add `make test-hosted` and `scripts/run-hosted-tests.sh`, a shell script
checked by `shellcheck`, consistent with the repository's existing shell
helpers. The recipe depends on `check-build-tools`, composes `RUSTFLAGS` as
the `test` recipe does, and runs the command below directly, so the
build-standard contract can read it; the script, given that command as its
arguments, creates a private `XDG_RUNTIME_DIR`, starts
`weston --backend=headless`, waits for its socket with a deadline, runs the
command under `dbus-run-session` with `DISPLAY` unset, and on any exit stops
only the processes it started.

```bash
cargo nextest run --run-ignored only \
  -p cabochon-clerestory-hosted -p cabochon-hello-window
```

Add `test-hosted` to `DEVELOPMENT_TARGETS` in
`tests/build_standard_support/make.rs` and to `.PHONY`. Add a CI job `hosted`
on `ubuntu-latest` that installs `weston`, `dbus-daemon`, and `at-spi2-core`,
sets up Rust through the pinned `setup-rust` action with
`install-mold: 'true'`, `install-clang-lld: 'true'`, and `expect-cache: any`,
and runs `make test-hosted`; keep the workflow contract tests green. Write
`OB-CLR-HOST`. Accept ADR 0012.

### EP-M7: documentation reconciliation and step closure

Update `docs/cabochon-design.md` §8.1 with the Clerestory contract summary,
the focus model, and the proof inventory; propose the Figure 1 bridge edges
(decision D12); add the §15 rows for ADR 0012 and for the deferrals (D8, D13,
R5, R11). Replace the template text in `docs/users-guide.md` with a
hello-window section (`run`, `export`, `review`, `--locale`, `--fonts`,
`--resources`, keyboard reference, and `escutcheon-check`). Finish
`docs/developers-guide.md` (the session model, the headless shell as the
reference adapter, the contract probe, `make test-hosted` and its
provisioning, the hosted residuals, and the proof inventory). Update
`docs/repository-layout.md` and `docs/contents.md`. Record the guidelines
checklist result for `hello-window` in the pull request description: each
applicable must rule with its evidence, and each inapplicable rule with its
false predicate. Propose the roadmap task for host clipboard and drag-and-drop
under step 5.1; mark roadmap tasks 1.0.1 and 1.0.2 done; reconcile every
discovery with the conformance basis; and set this plan's status to
`COMPLETE`.

## Milestones and plateaus

At each boundary, check: the assigned requirements are met; the design and
ADRs are still followed; no upstream assumption was falsified; no public
interface, dependency, or persisted format was added beyond this plan; and the
trace links are current. Compatibility decision for every milestone: none. All
interfaces are new and unreleased, so each change updates its callers together.

- EP-M0. Outcome: the four uncertainties are resolved and nothing is
  committed but this plan's notes. Recovery: delete the scratch directory.
  Remaining gaps: all implementation.
- EP-M1. Outcome: a session runs under the headless shell with focus,
  command routing, and key routing. Requirements: `REQ-FOCUS`, `REQ-COMMAND`,
  `REQ-KEYS`, `REQ-OBS`. Evidence: `OB-CLR-FOCUS`, `OB-CLR-MODAL`,
  `OB-CLR-KEYS`, `OB-CLR-ROUTE`, `OB-CLR-OBS`, and the Clerestory guard rows.
  Recovery: revert the milestone's commits. Remaining gaps: proof, furniture.
- EP-M2. Outcome: the focus kernel is proved. Evidence: `make verus` and
  `OB-PROOF-MIRROR`. Recovery: revert; EP-M1's tests still hold. Remaining
  gaps: none for focus beyond the stated residual.
- EP-M3. Outcome: furniture is laid out, drawn, and described to assistive
  technology. Requirements: `REQ-WINDOW`, `REQ-MENU`, `REQ-DIALOG`, `REQ-A11Y`,
  `REQ-L10N`. Evidence: `OB-CLR-MENU`, `OB-CLR-DIALOG`, `OB-CLR-LAYOUT`,
  `OB-CLR-A11Y`.
- EP-M4. Outcome: text input, the palette, and the inspector panel work in
  the contract probe. Requirements: `REQ-TEXT`, `REQ-INSPECTOR`. Evidence:
  `OB-CLR-TEXT`, `OB-CLR-PALETTE`, and all of `furniture.feature`.
- EP-M5. Outcome: `hello-window` runs headless, exports, and reviews.
  Requirements: `REQ-APP`; completes the `RM-1.0.2` success criterion;
  `TOR-SM-GEM` except the hosted window. Evidence: `OB-APP-RESOURCES`,
  `OB-APP-SURFACE`, `OB-APP-EXPORT`, `OB-APP-E2E`, `hello_window.feature`.
- EP-M6. Outcome: the application runs under a real compositor.
  Requirements: `REQ-HOST`, `TDD-INV-6`, `TOR-SM-GEM`. Evidence: `OB-CLR-HOST`
  locally and in CI; coverage ratchet green. Recovery: the headless path from
  EP-M5 survives a reverted EP-M6.
- EP-M7. Outcome: documentation and roadmap match the delivered contracts;
  roadmap 1.0.1 and 1.0.2 are marked done; step 1.0 is complete.

## Concrete steps

Run from the repository root. Gates run one at a time, in this order.

```bash
set -euo pipefail

BRANCH=$(git branch --show-current | tr '/' '-')
make check-fmt 2>&1 | tee "/tmp/check-fmt-cabochon-${BRANCH}.out"
make lint 2>&1 | tee "/tmp/lint-cabochon-${BRANCH}.out"
make test 2>&1 | tee "/tmp/test-cabochon-${BRANCH}.out"
make markdownlint 2>&1 | tee "/tmp/markdownlint-cabochon-${BRANCH}.out"
make spelling 2>&1 | tee "/tmp/spelling-cabochon-${BRANCH}.out"
make nixie 2>&1 | tee "/tmp/nixie-cabochon-${BRANCH}.out"
```

From EP-M2, and from EP-M6:

```bash
set -euo pipefail

BRANCH=$(git branch --show-current | tr '/' '-')
make install-verus
make verus 2>&1 | tee "/tmp/verus-cabochon-${BRANCH}.out"
make test-hosted 2>&1 | tee "/tmp/test-hosted-cabochon-${BRANCH}.out"
```

Focused commands, with the result to expect before and after each change:

```plaintext
Working directory: repository root
Red:   cargo nextest run -p cabochon-clerestory focus_machine -> the reference model disagrees on a scoped Tab
Green: the same command -> all cases pass and the class counters meet their minimums
Red:   make verus -> open goals for L-RING-NEAREST and L-RING-CYCLE
Green: make verus -> every function verified, 0 errors
Red:   cargo nextest run -p cabochon-hello-window hello_window -> the scenario finds no window
Green: the same command -> five scenarios pass
```

From EP-M5:

```bash
# `review` refuses a foreign non-empty directory, so each run takes a fresh one.
REVIEW_DIR=$(mktemp -d)
cargo run -p cabochon-hello-window -- review --out "$REVIEW_DIR"
ls "$REVIEW_DIR"
```

```plaintext
accessibility.json  clipboard  content-print.pdf  content.pdf  content.png  export.json  hit-map.json  thumbnail.png
```

The `msrv` CI job must keep passing (constraint C7). Reproduce it from outside
the checkout:

```bash
rustup toolchain install 1.85.0 --profile minimal
(cd /tmp && cargo +1.85.0 check --lib --all-features --locked \
  --manifest-path "$OLDPWD/Cargo.toml")
```

## Validation and acceptance

- Tests: `make test` passes with every obligation's tests,
  `furniture.feature`, and `hello_window.feature`; `make test-hosted` passes
  locally and in CI. Each new test is recorded failing before its
  implementation.
- Verification: `make verus` reports every function in
  `verus/clerestory_proofs.rs` verified with no error; `OB-PROOF-MIRROR` finds
  no banned construct; the two seeded proof faults and every seeded test fault
  were seen to fail and were reverted.
- Lint and formatting: `make check-fmt` and `make lint` pass with no
  `#[expect]` in this plan's crates outside the hosted float boundary.
- Architecture: the Clerestory, hosted, and application rows of
  `OB-ARCH-DEPS`, `OB-ARCH-IMPORTS`, and `OB-ARCH-PUBAPI` pass, and each row's
  seeded violation (Clerestory depending on `winit`, `session` importing
  `shell::headless`, and a public Clerestory function returning an AccessKit
  type) fails.
- Security: no `unsafe`, no direct process-environment access, and
  `make audit` passes after each dependency addition.
- Documentation: `make markdownlint`, `make nixie`, and `make spelling` pass.
- Behaviour, automated: under headless `weston`, `make test-hosted` shows
  the window, menus, sheets, and status line coming from resources and the same
  journal as the headless shell. This is the regression guarantee.
- Behaviour, manual: in a real GNOME or KDE Plasma session with Orca running
  (which enables the accessibility bus), the same is true, Accerciser lists the
  roles, names, states, and document text, and typing in the palette through an
  input method composes text. Record the desktop, its version, and the outcome
  in `Artefacts and notes`. This is a one-off check backing `AXIOM-WINIT`, not
  a regression guarantee; automated GNOME-like and KDE-like acceptance belongs
  to roadmap 5.1.4.
- Performance: no threshold is set. The white paper's "opens quickly" goal is
  measured by later work; this plan records the cold-start time of
  `--exit-after-first-frame` once, as a baseline.

## Idempotence and recovery

Every step is additive and rerunnable. `export` and `review` write into a
temporary directory, rename it into place, and refuse foreign non-empty
directories without `--force`. `make test-hosted` stops only the processes it
started, even on failure. Seeded faults are reverted with `git restore` before
any commit, and `git status` must be clean of them. Accept an `insta` snapshot
change only after confirming an intended contract change.

## Artefacts and notes

Record here, labelled by milestone and revision: the implementation base
commit; EP-M0's probe transcripts and the delivered sibling type names; red,
green, and refactor transcripts; the open goals from the first `make verus`
run; seeded-fault transcripts; the hosted test output; and the manual desktop
check.

Reconnaissance observations (2026-11-21, base `d295397`): the facts in
`Surprises & discoveries` dated today, and the crate versions in the next
section, were read from crates.io and upstream documentation on that date.

## Interfaces and dependencies

The signatures below are the contract at the end of EP-M4. Names of Lapidary,
Burin, and Escutcheon types are those of the sibling plans; EP-M0 records any
that were delivered under another name. Every public item is owned by
`cabochon-clerestory`; applications use it through `Application` and `Session`,
shell adapters through `HostShell`, and no other crate reimplements focus,
routing, or layout.

```rust,no_run
/// Total, index-only kernel (crates/cabochon-clerestory/src/kernel/ring.rs).
pub fn succ(mask: &[bool], from: Option<usize>) -> Option<usize>;
pub fn pred(mask: &[bool], from: Option<usize>) -> Option<usize>;

pub trait Application {
    fn commands(&self) -> &[CommandId];
    fn handle_command(&mut self, command: &CommandId, context: &mut CommandContext) -> CommandOutcome;
    // Available, Unavailable { reason: StringKey }, or Inapplicable; plus an optional checked mark.
    fn command_state(&self, command: &CommandId) -> CommandState;
    // ContentFrame { scene: Arc<Scene>, revision: ContentRevision }
    fn content(&self, view: &ViewContext) -> Option<ContentFrame>;
    fn inspector(&self, window: WindowId) -> Option<InspectorModel>;
}

pub enum HostEvent {
    Configured { window: HostWindowId, size: DeviceSize, scale: Scale120, appearance: Option<Appearance> },
    CloseRequested { window: HostWindowId },
    Key { window: HostWindowId, key: KeyEvent },          // logical key, physical key, modifiers, text
    Pointer { window: HostWindowId, event: PointerEvent }, // move, button, wheel
    Ime { window: HostWindowId, event: ImeEvent },        // Enabled, Preedit { text, caret }, Commit(text), Disabled
    FocusChanged { window: HostWindowId, focused: bool },
    RedrawRequested { window: HostWindowId },
    Woken(WakeToken),
}

pub trait HostShell {
    fn open_window(&mut self, spec: &WindowSpec) -> Result<HostWindowId, ShellError>;
    fn present(&mut self, window: HostWindowId, frame: &FrameUpdate<'_>) -> Result<(), ShellError>;
    fn publish_accessibility(&mut self, window: HostWindowId, tree: &AccessibilityTree) -> Result<(), ShellError>;
    fn set_ime_cursor_area(&mut self, window: HostWindowId, area: PixelRect) -> Result<(), ShellError>;
    fn request_print(&mut self, window: HostWindowId, request: &PrintRequest) -> Result<PrintOutcome, ShellError>;
    fn close_window(&mut self, window: HostWindowId) -> Result<(), ShellError>;
}

pub trait SessionWaker: Send + Sync {
    fn wake(&self, token: WakeToken);
}

pub struct FrameUpdate<'a> { pub scene: &'a ResolvedScene, pub damage: &'a Damage, pub mapping: &'a DeviceMapping }

pub struct TextLayouts {
    pub furniture: Arc<dyn TextLayout + Send + Sync>,
    pub content: Arc<dyn TextLayout + Send + Sync>,
}

pub struct Session<A: Application> { /* windows, focus, menus, sheets, palette, status, journal */ }
impl<A: Application> Session<A> {
    // Fails fast on an undeclared command, a reserved-key shortcut, or a
    // menu bar or dialog wider than the declared minimum window.
    pub fn new(application: A, resources: ResourceBundle, layouts: TextLayouts, locale: LocaleChain)
        -> Result<Self, SessionError>;
    pub fn handle(&mut self, event: HostEvent, shell: &mut dyn HostShell) -> Result<SessionStatus, SessionError>;
    pub fn journal(&self) -> &Journal;
}

pub enum Role {
    Window, MenuBar, Menu, MenuItem, MenuItemCheckBox, MenuItemRadio, Dialog, AlertDialog,
    Button, Label, TextInput, Group, Pane, Document, Status, ListBox, ListBoxOption,
}
pub struct States { /* unavailable, checked, expanded, selected, busy, read_only, modal, live_polite */ }
pub enum PrintOutcome { Submitted, Cancelled, Unavailable { reason: StringKey } }
```

`FrameUpdate` carries a resolved scene, its damage, and the device mapping,
never pixels; the hosted adapter rasterizes through Burin's
`InteractiveSurface`. `Session` calls Lapidary's `resolve` and embeds the
content scene as a subscene; `SessionError` wraps `ResolveError` and
`ShellError`, and the hosted adapter maps Burin's `RenderError` into
`ShellError`. Error enums use `thiserror`; a large payload sits behind `Box`.
A shortcut matches a `KeyEvent` by Escutcheon's canonical form, falling back to
the physical key on non-Latin layouts, as the 1.0.2 plan defines.

Dependencies (caret requirements; newest compatible release when each milestone
starts; versions read on 2026-11-21):

- `cabochon-clerestory`: `cabochon-lapidary`, `cabochon-escutcheon`,
  `thiserror` 2, `tracing` 0.1, `metrics` 0.24.
- `cabochon-clerestory-hosted`: `cabochon-clerestory`, `cabochon-burin`,
  `cabochon-lapidary`, `winit` 0.30.13, `softbuffer` 0.4.8, `accesskit`
  0.25.2, `accesskit_winit` 0.34.2, `thiserror` 2, `tracing` 0.1.
- `cabochon-hello-window`: the three Clerestory-side crates above as listed
  in C2, `cabochon-fonts`, `clap` 4, `eyre` 0.6, `serde` 1 and `serde_json` 1
  (through `export::wire`), `tracing-subscriber` 0.3, `metrics` 0.24,
  `metrics-util` 0.20, `mockable` 3, `cap-std` 4, `camino` 1.
- Development: `rstest`, `rstest-bdd` and `rstest-bdd-macros` 0.6, `proptest`
  1, `proptest-state-machine` 0.9, `googletest` 0.14, `pretty_assertions` 1,
  `insta` 1, `mockall` 0.15, `accesskit_consumer` 0.39, `tracing-test` 0.2,
  `assert_cmd` 2, and `cabochon-substrate-testkit`, at the versions the 1.0.3
  plan's workspace already pins where it pins them.

## Documentation obligations

- ADR 0012 (in-window furniture, the host-shell port, the focus model and its
  proof, and the considered alternatives) under `docs/adrs/`, indexed in
  `docs/contents.md` and referenced from `docs/cabochon-design.md` §15.
- `docs/cabochon-design.md` §8.1: the Clerestory contract summary, focus
  model, and proof inventory; the Figure 1 bridge proposal; the §15 deferrals.
- `docs/users-guide.md`: the hello-window section.
- `docs/developers-guide.md`: the session model, ownership and reuse policy
  for the three crates, the headless shell, the contract probe,
  `make test-hosted`, the hosted residuals, and the proof inventory.
- `docs/repository-layout.md` and `docs/contents.md`: the new crates.
- `docs/reliable-testing-in-rust-via-dependency-injection.md`: the `mockable`
  version and the `temp_env` advice.
- `docs/roadmap.md`: 1.0.1 and 1.0.2 marked done; the proposed clipboard and
  drag-and-drop task.

## Revision notes

- 2026-10-23: created by decomposing the reviewed step-level plan (commit
  `616a8fa`), then revised the same day by the cross-plan consistency review
  and the architecture-guard decisions (T5 to T15).
- 2026-11-04: the reviewer approved the plan; milestones unchanged.
- 2026-11-05: three review rounds: scenario ordering, the automated and manual
  hosted checks, fresh export directories, and the `#[expect]` scope.
- 2026-11-21: refined for implementation readiness. Why: the UI guidelines
  became a required reference and redefine `hello-window`; the current
  ExecPlan standard requires a proof-sketch-first verification plan,
  per-category budgets, and non-vacuity evidence; and reconnaissance found
  stale claims (roadmap task numbers, the conformance base, `make` targets, the
  Cranelift state) and hosted-test limits (no IME or fractional scale under
  headless `weston`, lazy AccessKit activation). What changed: status returns
  to DRAFT; decisions R1 to R14 and amendments `AM-ESC-1`, `AM-BUR-1`, and
  `AM-LAP-1` are added; T5 is superseded and T7 narrowed; the acceptance
  surface follows chapter 36, with a contract probe for inspector panels and
  text input; focus gains an abstract model, a Verus-proved kernel, and a new
  proof milestone; milestones grow from five to eight (EP-M0 probes, EP-M2
  proofs, and furniture split in two); the plan is now self-contained rather
  than deferring to the 1.0.3 plan. Effect on remaining work: all of it; no
  implementation had started.
