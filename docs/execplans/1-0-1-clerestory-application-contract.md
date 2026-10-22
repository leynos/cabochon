# Define the minimum Clerestory application contract (roadmap 1.0.1)

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
`Tolerances`, `Risks`, `Progress`, `Surprises & discoveries`, `Decision log`,
`Outcomes & retrospective`, `Conformance basis`, and `Verification plan` must
be kept up to date as work proceeds.

Status: DRAFT

Implementation must not begin until a human reviewer explicitly approves this
plan and the 1.0.3, 1.0.4, and 1.0.2 plans
(`docs/execplans/1-0-3-lapidary-scene-contract.md`,
`docs/execplans/1-0-4-burin-output-contract.md`, and
`docs/execplans/1-0-2-escutcheon-resource-format.md`) have been implemented and
merged. Silence is not approval. Implement on a branch named
`1-0-1-clerestory-application-contract`, in its own pull request.

## Purpose / big picture

Cabochon models its application environment on the Graphics Environment Manager
(GEM). Clerestory is the Application Environment Services (AES) analogue:
windows, menus, commands, dialogs, focus, keyboard traversal, text input,
inspector panels, accessibility roles, and adaptation to a host desktop such as
GNOME or KDE Plasma. It builds on Escutcheon (resources), Lapidary (scenes),
and Burin (rendering), which the three earlier task plans of roadmap step 1.0
deliver. ADR 0007 makes all four production contracts in hosted mode, with no
dependency on Enfilade.

After this plan, a developer can run a small application, `hello-window`,
inside an ordinary GNOME or KDE Plasma session. Its window, menu bar, commands,
shortcuts, strings, icon, About dialog, and inspector panel all come from
Escutcheon resources rather than hand-built widgets. Its content is a Lapidary
scene drawn by Burin. Typing a new greeting into the inspector's text field,
including through an input method, redraws only the damaged part of the
content. The same content exports to every Burin output with one command. This
completes roadmap step 1.0 and the terms-of-reference success measure for the
GEM substrate.

Observable success:

1. `cargo run -p cabochon-hello-window -- run` opens a window titled
   "Hello, Cabochon" with File, View, and Help menus. Ctrl+Q quits, F1 opens
   the About dialog, Tab moves focus, and Accerciser (an accessibility
   inspector) shows a window, a menu bar, menu items, a dialog, buttons, a text
   field, and the content canvas with its text, each named.
2. `--locale en-XA` shows pseudo-localized strings and `--locale ar-XB` shows
   the furniture mirrored for right-to-left text, with no code changes.
3. `cargo run -p cabochon-hello-window -- export --out /tmp/hello` writes
   `content.pdf`, `content-print.pdf`, `content.png`, `thumbnail.png`,
   `accessibility.json`, `clipboard/`, `hit-map.json`, and `export.json` (the
   fonts and fallbacks used).
4. `make test` passes, including `furniture.feature` and
   `hello_window.feature` under the headless shell; `make test-hosted` runs the
   application under a headless Wayland compositor and prints
   `hello-window: shell=hosted windows=1 frames=1 accessibility_nodes=<N> exit=0`.
5. Roadmap tasks 1.0.1 and 1.0.2 are marked done (decision T1).

How to read identifiers: `RM-` roadmap task, `TDD-` technical-design
requirement, `TOR-` terms-of-reference measure, `EP-` milestone, `OB-`
verification obligation, `A-` axiom, `C` constraint, `D` decision inherited
from the reviewed step-level plan, `T` decision made for the task-level plans.

## Constraints

- C1, GEM-derived invariants. Applications reach the desktop only through
  Clerestory, draw only through Lapidary scenes, render only through Burin, and
  take UI structure from Escutcheon (`docs/cabochon-design.md` §3). No crate
  here names an Enfilade type (decision D12).
- C2, dependency direction (normal edges). `cabochon-clerestory` depends only
  on `cabochon-lapidary` and `cabochon-escutcheon`. Only
  `cabochon-clerestory-hosted` and `cabochon-hello-window` may combine
  Clerestory with Burin. Only the hosted crate may depend on `winit`,
  `softbuffer`, or `accesskit_winit`. `cabochon-clerestory` must not depend on
  `winit`, `softbuffer`, `accesskit*`, `kurbo`, `parley`, `krilla`, `vello*`,
  `kdl`, `fluent*`, `unic-langid`, or `miette`, and `cabochon-hello-window` may
  reach `winit` only through the hosted crate. This plan adds these rows to
  `tests/substrate_dependency_contract.rs`.
- C3, no third-party types in Clerestory's public API: no `winit`, Wayland,
  X11, AccessKit, GTK, or Qt types (ADR 0002).
- C4, integer geometry. Floating-point arithmetic appears only in
  `cabochon_clerestory_hosted::float_boundary` (scale factors and pointer
  positions from `winit`), with reasoned lint expectations.
- C5, lint, safety, and documentation policy as in the repository, including
  the style guide's doc-comment rules.
- C6, existing contracts, including the `msrv` CI job and the workflow
  contract tests, keep passing; member crates declare no features.
- C9, tests never mutate the process environment. Locale and display
  selection reach the application through arguments or an injected
  `mockable::Env`; child processes in end-to-end tests receive environment
  values through `Command::env`.
- C10, only the `cabochon-hello-window` binary installs a `tracing`
  subscriber or `metrics` recorder.
- C11, shared host. Recipes never kill processes they did not start; the
  hosted test stops only the `weston` process it started.

## Tolerances (exception triggers)

- Scope: a milestone exceeds 3,000 net added lines of Rust (excluding fixtures
  and snapshots) or 60 files.
- Interface: a Lapidary, Escutcheon, or Burin contract must change, or a
  Clerestory contract changes in a way that alters roadmap tasks 2.1.2 and
  2.1.3.
- Dependencies: a dependency not listed here is needed, or a listed crate must
  move to a new major or pre-1.0 minor series.
- Hosted environment: a headless Wayland compositor cannot be provisioned in
  continuous integration (CI) or locally at the start of EP-M4.
- Coverage: the coverage ratchet fails because hosted-only code cannot run
  without a display, and thinning the adapter does not resolve it.
- Upstream conflict: evidence contradicts `docs/cabochon-design.md`, an ADR,
  or the roadmap; set `Status: BLOCKED` and record the proposed deviation.
- Iterations: a red test cannot be turned green after five attempts with
  distinct hypotheses.

## Risks

- Risk: `winit` 0.31 is in beta with a changed event-loop API. Severity: low.
  Likelihood: high. Mitigation: stable `winit` 0.30.13, which `accesskit_winit`
  0.34 supports, confined to the hosted crate.
- Risk: CI runners and the development host lack a headless Wayland
  compositor; `winit` falls back to X11 without a Wayland socket; with no
  session bus AccessKit publishes nowhere. Severity: medium. Likelihood: high.
  Mitigation: provisioning is EP-M4's gated first step (`weston` from Ubuntu
  packages in CI; Extra Packages for Enterprise Linux (EPEL) or a rootless
  Podman image locally); tests force the Wayland backend; the test runs under
  `dbus-run-session`; `OB-CLR-HOST` checks the tree handed to AccessKit, and
  reading it back over the Assistive Technology Service Provider Interface
  (AT-SPI) is a named residual for roadmap 5.1.4.
- Risk: the coverage ratchet fails because the hosted event loop cannot run
  without a display. Severity: medium. Likelihood: medium. Mitigation: keep the
  hosted crate thin, with key, scale, decoration, and accessibility mapping in
  pure, unit-tested functions; investigate ratchet exclusions.
- Risk: in-window menus cannot extend beyond the window, because `winit` 0.30
  offers no `xdg_popup`. Severity: low. Likelihood: certain. Mitigation:
  windows have a minimum size that fits the longest menu (decision D5).
- Risk: headless `weston` reports integer scales only. Severity: low.
  Likelihood: certain. Mitigation: fractional scales are tested through the
  headless shell; hosted fractional scale is a residual for roadmap 5.1.3.

## Progress

- [x] (2026-10-18) Step-level plan drafted, reviewed by four
  community-of-experts panels, and revised (commit `616a8fa`).
- [x] (2026-10-22) Reviewer accepted decisions D1, D6, D7, D8, D10, D11, D12,
  D13, and D14 on the step-level plan.
- [x] (2026-10-23) Decomposed into four task-level plans; this is the last.
- [ ] Reviewer approval of this plan; the 1.0.3, 1.0.4, and 1.0.2 plans
  implemented and merged.
- [ ] EP-M1: session, state machines, routing, and the headless shell.
- [ ] EP-M2: layout, furniture, text input, inspector, and accessibility
  tree.
- [ ] EP-M3: the hello-window application under the headless shell.
- [ ] EP-M4: the hosted shell adapter and its CI job.
- [ ] EP-M5: documentation reconciliation and step closure.

## Surprises & discoveries

- Observation: `accesskit_winit` 0.34.1 requires `winit` ^0.30.5. Evidence:
  crates.io dependency metadata, 2026-10-18. Impact: `winit` 0.30.13 is
  compatible.
- Observation: `muda` (native menus) needs GTK on Linux and attaches only to
  GTK windows. Impact: supports in-window menus (decision D5).
- Observation: `mockable` is at 3.0.0, while
  `docs/reliable-testing-in-rust-via-dependency-injection.md` cites `0.3`.
  Impact: use `mockable` 3 and correct the guide in EP-M3.

## Decision log

Inherited from the reviewed step-level plan (accepted ones say so):

- Decision D5: Clerestory owns the furniture model (commands, menus, dialogs,
  focus, modality, roles) and draws furniture inside the window as Lapidary
  scenes. It adapts to hosts through a `HostShell` port; the hosted adapter uses
  `winit` 0.30, `softbuffer`, and `accesskit_winit`. Considered: native menus
  through `muda` (needs GTK), GTK4 (a second toolkit), Slint, egui, or iced
  (each duplicates Lapidary), and Masonry as the hosted furniture renderer
  (text editing and input methods from upstream, but a second render path
  outside Burin's parity tests). `FrameUpdate` stays at the scene level so the
  Masonry option remains open. Recorded as ADR 0012.
- Decision D8 (accepted 2026-10-22): standard dialogs are alert,
  confirmation, and About. File dialogs wait for roadmap 5.1.2; export takes
  its directory from the command line.
- Decision D10 (accepted 2026-10-22): device scale in 1/120 units; snapping
  snaps edges; a scale change damages the whole window.
- Decision D12 (accepted 2026-10-22): Enfilade integration lives in bridge
  crates; EP-M5 proposes redrawing the design's Figure 1 accordingly.
- Decision D13 (accepted 2026-10-22): drag-and-drop and host clipboard
  integration are deferred; EP-M5 proposes a roadmap task for them under step
  5.1.
- Decision D14 (accepted 2026-10-22): a single-line text field with input
  method editor (IME) support is in scope: `HostEvent` carries IME events,
  `HostShell` exposes the IME cursor area, and `Role::TextInput` exists.
- Decision D15: font policy. Furniture always uses the bundled fonts in
  `cabochon-fonts`. A `--fonts bundled|bundled+system` option selects the
  content font collection: `run` defaults to `bundled+system` and `export` to
  `bundled`. `export` records every font and fallback in `export.json`. Tests,
  including `OB-CLR-HOST` and every end-to-end test, pass `--fonts bundled` and
  never use system fonts.
- Decision D20: assertion style (`googletest` matchers, `pretty_assertions`
  for whole values, `insta` for stable summaries; `#[gtest]` before
  `#[rstest]`).

Task-level decisions:

- Decision T1: this plan runs last and marks roadmap tasks 1.0.1 and 1.0.2
  done, because the hello-window application proves both success criteria.
  Date: 2026-10-23.
- Decision T5: EP-M3 moves the hello-window resource bundle from
  `crates/cabochon-escutcheon/tests/fixtures/hello-window/` to
  `crates/cabochon-hello-window/resources/` with `git mv`, and repoints
  Escutcheon's and Clerestory's tests, the Escutcheon schema reference, and
  `docs/developers-guide.md` at the new location, so one copy exists. Date:
  2026-10-23.
- Decision T7: the in-app `content.export` command writes to the directory
  given by `run --export-dir`, and shows a resource-defined alert when none was
  given, because file dialogs are deferred (decision D8). Date: 2026-10-23.
- Decision T8: one pull request per task plan, replacing the step plan's
  decision D18, following the reviewer's direction on 2026-10-23. Date:
  2026-10-23.
- Decision T9: ADR numbers are reserved by this decomposition: 0009 for the
  1.0.3 plan, 0010 for the 1.0.2 plan, 0011 for the 1.0.4 plan, and 0012 for
  this plan. Any other pull request needing an ADR takes 0013 or later. Date:
  2026-10-23.
- Decision T10: the hello-window binary embeds its resource bundle with
  `include_str!` (and `include_bytes!` for icons) and loads it through
  Escutcheon's in-memory `ResourceSource` by default; `--resources DIR` loads a
  directory through the `cap_std` adapter instead. Rationale: `run` works from
  any working directory, and authors can still iterate on resources without
  recompiling. Date: 2026-10-23.
- Decision T11: behavioural scenarios split by crate. EP-M2 adds
  `crates/cabochon-clerestory/tests/features/furniture.feature`, driving a test
  application over the hello-window fixture bundle; EP-M3 adds
  `crates/cabochon-hello-window/tests/features/hello_window.feature`, driving
  the real application. Rationale: the application crate does not exist until
  EP-M3. Date: 2026-10-23.

## Outcomes & retrospective

Not started.

## Context and orientation

When this plan starts, the workspace contains `cabochon-lapidary` (scenes,
resolution, hit testing, damage, `DeviceMapping`, accessibility nodes, the
`TextLayout` port), `cabochon-burin` (`ParleyTextLayout`, every workstation,
`InteractiveSurface`, `render_with_fallback`), `cabochon-fonts` (bundled fonts),
`cabochon-escutcheon` (`compile`, `ResourceBundle`, `Localizer`, built-in
commands, `escutcheon-check`, and the hello-window fixture), the testkit,
prover tooling, and `.config/nextest.toml`. Read the earlier plans'
`Interfaces and dependencies` sections and the crates' rustdoc first.

Governing documents: `docs/cabochon-design.md` §3, §8.1 (Clerestory), §8.9,
§11, and §14;
`docs/adrs/adr-0002-keep-the-hosted-runtime-independent-of-the-shell.md`;
`docs/adrs/adr-0007-make-the-gem-substrate-a-production-boundary.md`;
`docs/terms-of-reference.md` lines 275 to 279 (the GEM substrate success
measure: a resource-defined application that opens a window, exposes standard
menus and commands, renders a Lapidary scene, exports it through Burin, and
integrates with accessibility and keyboard traversal, without host-specific
code); `docs/context.md`; and `docs/roadmap.md` tasks 1.0.1 and 1.0.2.

Terms:

- Host shell: the desktop that owns windows and input (GNOME or KDE Plasma).
- Headless shell: a Clerestory adapter that runs applications without a
  display, recording frames, accessibility trees, IME cursor areas, and
  dispatched commands.
- Responder chain: the order in which a command is offered to handlers:
  focused element, dialog, window, application.
- IME (input method editor): host software that composes text through
  intermediate "preedit" text before committing it.
- Furniture: standard application chrome: menu bar, menus, dialogs,
  inspector panels.

Skills: `execplans`, `rust-router` (then `rust-state-machines` for focus,
modality, menus, and text editing, `rust-types-and-apis`, `rust-errors`, and
`domain-cli-and-daemons` for the application), `hexagonal-architecture`,
`rust-unit-testing`, `proptest`, `arch-decision-records`, `nextest`,
`en-gb-oxendict-style`, `codegraph-mcp`, and `firecrawl-mcp` for `winit`,
`softbuffer`, and AccessKit documentation. References as listed in the 1.0.3
plan.

## Conformance basis

Upstream at `main` commit `0de2b57`. Traced items: `RM-1.0.1`, `RM-1.0.2`
(success criterion), `TOR-SM-GEM`, `TDD-CLR` (§8.1), `TDD-INV-1` and
`TDD-INV-6` (§3), `TDD-OBS` (§14), ADR 0002, and ADR 0007.

```plaintext
RM-1.0.1 -> TDD-CLR, TDD-INV-1 -> EP-M1, EP-M2 -> OB-CLR-FOCUS, OB-CLR-MODAL, OB-CLR-MENU, OB-CLR-ROUTE
RM-1.0.1 -> TDD-CLR (inspector, roles, text) -> EP-M2 -> OB-CLR-TEXT, OB-CLR-LAYOUT, OB-CLR-A11Y
RM-1.0.1 -> TDD-INV-6, ADR-0002 -> EP-M4 -> OB-CLR-HOST, make test-hosted
RM-1.0.2 -> TDD-ESC -> EP-M3 -> OB-APP-RESOURCES (hello-window loads its UI from resources)
TOR-SM-GEM -> RM-1.0.1..4 -> EP-M3, EP-M4 -> OB-APP-EXPORT, OB-APP-E2E, OB-CLR-HOST
TDD-OBS -> EP-M1 -> dispatch metric and span assertions
```

## Verification plan

Clerestory's obligations are state-machine and orchestration properties over
event sequences, plus end-to-end behaviour against a real compositor. The right
methods are `proptest-state-machine` against reference models, named cases,
behavioural scenarios, snapshots of layout, and end-to-end tests; no obligation
here is a stable algebraic kernel suited to Verus, and the bounded arithmetic
(scale and pointer conversion) is small enough for named cases. Clerestory
relies on Lapidary's proved hit testing (`OB-LAP-HIT`) for pointer routing and
Burin's tested invalidation (`OB-BUR-INVAL`) for redraw.

Non-vacuity mechanics follow the 1.0.3 plan. State-machine suites run 256 cases
of up to 32 operations; regression files are committed.

Axioms:

- `A-WINIT`: `winit` and `softbuffer` deliver input and present buffers as
  documented, and a headless `weston` session represents GNOME and KDE Plasma
  closely enough for a smoke test. Residual: host-specific behaviour,
  fractional scale, and AT-SPI read-back (roadmap 5.1.3 and 5.1.4).
- `A-ACCESSKIT`: `accesskit_winit` publishes the supplied tree to AT-SPI
  faithfully.

### Obligations

`OB-CLR-FOCUS`: in every window at most one element owns focus, and only a
focusable, enabled, visible element can own it; Tab then Shift+Tab returns
focus to where it started; a full Tab cycle visits every focusable element
exactly once. Method: `proptest-state-machine` with a reference model over Tab,
Shift+Tab, enable, disable, show, hide, open dialog, close dialog, pointer
focus, open menu, and close menu.

`OB-CLR-MODAL`: while a modal dialog is open, neither focus nor input reaches
elements outside it, and closing it restores the previous focus owner. Same
state machine.

`OB-CLR-MENU`: opening, arrow navigation, activation, and Escape never leave a
menu open while focus sits outside it; activating a disabled item dispatches
nothing. Same state machine.

`OB-CLR-ROUTE`: a command reaches the first responder that accepts it; a
disabled command is never dispatched; a shortcut and the matching menu item
produce identical dispatch records; a host close request is dispatched as the
vetoable built-in `window.close`; at session start every command the
application declares must exist in the bundle, or the session fails fast. Named
cases plus a property test over generated chains.

`OB-CLR-TEXT`: in the text field, caret movement, backspace, and delete move by
grapheme cluster; preedit text is shown but never committed without an IME
commit event; disabling the IME discards preedit; the reported IME cursor area
contains the caret. `proptest-state-machine` over key and IME events.

`OB-CLR-LAYOUT`: children lie inside their parent and do not overlap their
siblings; layout is deterministic; `en-XA`'s lengthened strings do not overflow
the dialog's minimum size; right-to-left locales mirror row order. Property
test over generated layouts plus `insta` snapshots of the hello-window dialog
layout in `en-GB`, `en-XA`, and `ar-XB`.

`OB-CLR-A11Y`: every interactive element has a role and a non-empty name;
parent and child links are consistent; the focused element is in the tree; the
canvas contains the content scene's accessibility nodes. Property test over
generated bundles; the hosted adapter's conversion is checked by feeding its
`TreeUpdate` into `accesskit_consumer::Tree`.

`OB-CLR-OBS`: dispatch emits a span and increments
`clerestory_command_dispatch_total{outcome}` with fixed labels. Named case with
a test recorder.

`OB-APP-RESOURCES`: the hello-window application constructs its window, menus,
dialogs, inspector, strings, shortcuts, and icon only from the compiled bundle,
embedded by default (decision T10); changing a string in
`resources/locales/en-GB/hello-window.ftl` changes the presented title without
recompiling Rust. End-to-end test with a temporary copy of the bundle passed
through `--resources`.

`OB-APP-EXPORT`: `export` writes all eight outputs, refuses a non-empty
directory it did not create unless `--force` is given, writes into a temporary
directory and renames it into place, and records fonts and fallbacks in
`export.json`. End-to-end test.

`OB-APP-E2E`: a scripted headless session (`run --shell headless --script`)
exercising menus, shortcuts, the About dialog, Tab traversal, IME text entry,
and quit produces the expected dispatch log, accessibility summary, and exit
status in `en-GB`, `en-XA`, and `ar-XB`. End-to-end test.

`OB-CLR-HOST`: the same scripted session, with the same `ParleyTextLayout` and
bundled fonts, run under the headless shell and under the hosted shell inside
headless `weston`, yields the same accessibility tree (as handed to AccessKit)
and the same command log. End-to-end test run by `make test-hosted`.

## Plan of work

Red-Green-Refactor throughout; each milestone ends with `make check-fmt`,
`make lint`, and `make test` passing (sequentially, through the `scrutineer`
agent) and a commit. Each dependency addition is its own commit with
`make audit`.

### EP-M1: session, state machines, routing, and the headless shell

Create `crates/cabochon-clerestory` with its dependency-contract row and
ownership policy. Implement the `Application` trait, `ContentFrame`,
`ViewContext`, `CommandContext` (with `invalidate`), the `Session` (windows,
menus, dialogs, focus, modality, routing), `HostEvent`, the `HostShell` and
`SessionWaker` ports, and `HeadlessShell` in
`cabochon_clerestory::shell::headless` (no infrastructure dependencies; the
reference adapter). Write `OB-CLR-FOCUS`, `OB-CLR-MODAL`, `OB-CLR-MENU`,
`OB-CLR-ROUTE`, and `OB-CLR-OBS`. Draft ADR 0012 as
`docs/adrs/adr-0012-<title-slug>.md`.

### EP-M2: layout, furniture, text input, inspector, and accessibility tree

Implement the layout engine (rows, columns, labels, buttons, text fields,
separators, property rows; measured through the `TextLayout` port; mirrored for
right-to-left locales), the furniture scene builders that draw the menu bar,
menus, dialogs, and inspector through Lapidary, the single-line text field with
IME support, the inspector bound to an `InspectorModel`, and the
`AccessibilityTree`, which grafts the content scene's accessibility nodes under
the canvas. Write `OB-CLR-TEXT`, `OB-CLR-LAYOUT`, `OB-CLR-A11Y`, and
`furniture.feature` (decision T11), whose test application reads the
hello-window fixture bundle from
`crates/cabochon-escutcheon/tests/fixtures/hello-window/`. Clerestory's unit
tests use the testkit's `MonospaceTextLayout`; the feature uses Burin's
`ParleyTextLayout` with bundled fonts (a development edge, which C2 permits).

### EP-M3: the hello-window application under the headless shell

Create `crates/cabochon-hello-window`. Move the resource bundle (decision T5)
and embed it (decision T10). Implement the `run` and `export` subcommands,
`--locale`, `--shell headless`, `--script`, `--exit-after-first-frame`,
`--export-dir` (decision T7), `--resources`, and `--fonts`; a `tracing`
subscriber writing to standard error; and an environment reader through
`mockable`. Write `OB-APP-RESOURCES`, `OB-APP-EXPORT`, `OB-APP-E2E`, and
`hello_window.feature` (decision T11). Correct the `mockable` version in
`docs/reliable-testing-in-rust-via-dependency-injection.md`.

### EP-M4: the hosted shell adapter and its CI job

First, provisioning: confirm `weston`, `dbus-run-session`, and `at-spi2-core`
are available in CI (Ubuntu packages) and locally (EPEL or a rootless Podman
image); otherwise the hosted tolerance triggers. Create
`crates/cabochon-clerestory-hosted`: the `winit` event loop (forcing Wayland in
tests), `softbuffer` presentation of `InteractiveSurface` frames with damage,
scale handling in 1/120 units, key, pointer, and IME mapping, close requests,
client-side decorations where required, and `accesskit_winit` publication, with
all mapping in pure, unit-tested functions. Hosted tests carry
`#[ignore = "needs Wayland; run make test-hosted"]`. Add `make test-hosted`: it
depends on `check-build-tools` and composes `RUSTFLAGS` as the `test` recipe
does (it is a development target); it creates a private `XDG_RUNTIME_DIR`,
starts `weston --backend=headless`, waits for its socket, runs the ignored
tests under `dbus-run-session` with
`cargo nextest run --run-ignored only -p cabochon-clerestory-hosted -p
cabochon-hello-window`,
and stops only the `weston` it started. Add `test-hosted` to
`DEVELOPMENT_TARGETS` in `tests/build_standard_support/make.rs` and to
`.PHONY`. Add a CI job that installs `weston` and `at-spi2-core`, sets up Rust
through `setup-rust` with `install-mold: 'true'`, and runs `make test-hosted`,
keeping the workflow contract tests green. Write `OB-CLR-HOST`. Accept ADR 0012.

### EP-M5: documentation reconciliation and step closure

Update `docs/cabochon-design.md` §8.1 with the Clerestory contract summary;
propose the Figure 1 bridge edges (decision D12) and record the §15 deferrals
(decisions D8 and D13); add a hello-window section to `docs/users-guide.md`
(`run`, `export`, `--locale`, `--fonts`, `--resources`, and
`escutcheon-check`); finish `docs/developers-guide.md`; propose the roadmap
task for host clipboard and drag-and-drop integration; mark roadmap tasks 1.0.1
and 1.0.2 done; and set this plan's status to `COMPLETE`.

## Milestones and plateaus

Conformance check at each boundary as in the 1.0.3 plan. Recovery: revert the
milestone's commits; the headless path from EP-M3 survives a reverted EP-M4.
Compatibility decision: none.

- EP-M1. Outcome: the session runs under the headless shell. Requirements:
  `RM-1.0.1` (windows, menus, commands, dialogs, focus, traversal),
  `TDD-INV-1`. Evidence: `OB-CLR-FOCUS`, `OB-CLR-MODAL`, `OB-CLR-MENU`,
  `OB-CLR-ROUTE`, and `OB-CLR-OBS` pass. Check: no host types and no Burin
  dependency in Clerestory.
- EP-M2. Outcome: furniture, text input, inspector, and accessibility tree.
  Requirements: `RM-1.0.1` (inspector panels, accessibility roles). Evidence:
  `OB-CLR-TEXT`, `OB-CLR-LAYOUT`, `OB-CLR-A11Y`, and `furniture.feature`.
- EP-M3. Outcome: the hello-window application runs headless and exports.
  Requirements: completes the `RM-1.0.2` success criterion; `TOR-SM-GEM` except
  the hosted window. Evidence: `OB-APP-RESOURCES`, `OB-APP-EXPORT`,
  `OB-APP-E2E`, and `hello_window.feature`.
- EP-M4. Outcome: the application runs under a real compositor. Requirements:
  `RM-1.0.1` (host-shell adaptation), `TDD-INV-6`, `TOR-SM-GEM`. Evidence:
  `OB-CLR-HOST` locally and in CI. Check: `winit`, `softbuffer`, and AccessKit
  only in the hosted crate; coverage ratchet green.
- EP-M5. Outcome: documentation and roadmap match the delivered contracts;
  roadmap 1.0.1 and 1.0.2 marked done; step 1.0 complete.

## Behavioural specification

`crates/cabochon-clerestory/tests/features/furniture.feature` (EP-M2; the test
application reads the hello-window fixture bundle):

```gherkin
Feature: Clerestory furniture behaves consistently

  Scenario: Shortcuts and menus dispatch the same command
    Given the fixture application under the headless shell
    When the user presses "F1"
    And the user chooses "Help" then "About Hello, Cabochon"
    Then the dispatch log shows "app.about" twice with identical records

  Scenario: Dialogs are modal and restore focus
    Given the About dialog is open
    When the user presses "Tab" five times
    Then focus never leaves the dialog
    When the user presses "Escape"
    Then focus returns to the element that had it before the dialog opened

  Scenario: Input methods compose text before committing it
    Given the inspector's greeting field has focus
    When the input method sends preedit "ni" and then commits "你"
    Then the field shows "你" and the content scene's greeting is updated
    And only the greeting's bounds are damaged

  Scenario: Furniture is accessible
    Given the fixture application under the headless shell
    Then the accessibility tree contains a window, a menu bar, and menu items
    And every interactive element has a role and a non-empty name
    And the canvas contains the content scene's text
```

`crates/cabochon-hello-window/tests/features/hello_window.feature` (EP-M3):

```gherkin
Feature: The hello-window application is defined by its resources

  Scenario: The window comes from resources
    Given the hello-window application under the headless shell
    When the session starts
    Then one window titled "Hello, Cabochon" is presented
    And its menu bar lists "File", "View", and "Help"

  Scenario: Strings change without recompiling
    Given a copy of the resource bundle whose title string is "Bonjour"
    When the application starts with that bundle through --resources
    Then the presented window is titled "Bonjour"

  Scenario: The content exports to every workstation
    Given the hello-window application under the headless shell with an export directory
    When the user chooses "File" then "Export"
    Then the export directory contains every Burin output and export.json
```

## Concrete steps

```bash
BRANCH=$(git branch --show-current)
make check-fmt 2>&1 | tee /tmp/check-fmt-cabochon-${BRANCH}.out
make lint 2>&1 | tee /tmp/lint-cabochon-${BRANCH}.out
make test 2>&1 | tee /tmp/test-cabochon-${BRANCH}.out
make test-hosted 2>&1 | tee /tmp/test-hosted-cabochon-${BRANCH}.out
cargo run -p cabochon-hello-window -- export --out /tmp/hello
ls /tmp/hello
```

```plaintext
accessibility.json  clipboard  content-print.pdf  content.pdf  content.png  export.json  hit-map.json  thumbnail.png
```

The `msrv` CI job must keep passing (constraint C6); reproduce it from outside
the checkout:

```bash
cd /tmp && cargo +1.85.0 check --lib --all-features --locked \
  --manifest-path "$OLDPWD/Cargo.toml"
```

## Validation and acceptance

- Tests: `make test` passes with every obligation's tests,
  `furniture.feature`, and `hello_window.feature`; `make test-hosted` passes
  locally and in CI.
- Lint and formatting: `make check-fmt` and `make lint` pass with no
  `#[expect]` outside the hosted float boundary.
- Security: no `unsafe`, no direct process-environment access, `make audit`
  passes.
- Documentation: `make markdownlint`, `make nixie`, and `make spelling` pass.
- Behaviour: under GNOME or KDE Plasma, the window, menus, dialogs, and
  inspector come from resources, Accerciser lists their roles and names and the
  canvas text, and typing into the greeting field redraws only the greeting.

## Idempotence and recovery

Every step is additive and rerunnable. `export` writes into a temporary
directory, renames it into place, and refuses foreign non-empty directories
without `--force`. `make test-hosted` stops only the `weston` it started, even
on failure. Accept `insta` snapshot changes only after confirming an intended
contract change.

## Artefacts and notes

Record provisioning results, red, green, and refactor transcripts, and the
hosted test output here.

## Interfaces and dependencies

```rust,no_run
pub trait Application {
    fn commands(&self) -> &[CommandId];
    fn handle_command(&mut self, command: &CommandId, context: &mut CommandContext) -> CommandOutcome;
    fn command_state(&self, command: &CommandId) -> CommandState;
    // ContentFrame { scene: Arc<Scene>, revision: ContentRevision }
    fn content(&self, view: &ViewContext) -> Option<ContentFrame>;
    fn inspector(&self, window: WindowId) -> Option<InspectorModel>;
}

pub enum HostEvent {
    Configured { window: HostWindowId, size: DeviceSize, scale: Scale120 },
    CloseRequested { window: HostWindowId },
    Key { window: HostWindowId, key: KeyEvent },          // logical key, physical key, modifiers, text
    Pointer { window: HostWindowId, event: PointerEvent },
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
    fn close_window(&mut self, window: HostWindowId) -> Result<(), ShellError>;
}

pub trait SessionWaker: Send + Sync {
    fn wake(&self, token: WakeToken);
}

pub struct FrameUpdate<'a> { pub scene: &'a ResolvedScene, pub damage: &'a Damage, pub mapping: &'a DeviceMapping }

pub struct Session<A: Application> { /* windows, menus, dialogs, focus, modality, routing */ }
impl<A: Application> Session<A> {
    pub fn new(
        application: A,
        resources: ResourceBundle,
        layout: Arc<dyn TextLayout + Send + Sync>,
        locale: LocaleChain,
    ) -> Result<Self, SessionError>; // fails fast on undeclared commands (OB-CLR-ROUTE)
    pub fn handle(&mut self, event: HostEvent, shell: &mut dyn HostShell) -> Result<SessionStatus, SessionError>;
}

pub enum Role { Window, MenuBar, Menu, MenuItem, Dialog, Alert, Button, Label, TextInput, Panel, Inspector, Canvas }
pub enum StandardDialog { Alert(DialogText), Confirm(DialogText), About }
```

`FrameUpdate` carries a resolved scene, its damage, and the device mapping,
never pixels; the hosted adapter rasterizes through Burin's
`InteractiveSurface`. `HeadlessShell` records windows, frames, accessibility
trees, IME cursor areas, and the dispatch log.

Dependencies (caret requirements; newest compatible release when each milestone
starts):

- `cabochon-clerestory`: `cabochon-lapidary`, `cabochon-escutcheon`,
  `thiserror` 2, `tracing` 0.1, `metrics` 0.24.
- `cabochon-clerestory-hosted`: `cabochon-clerestory`, `cabochon-burin`,
  `cabochon-lapidary`, `winit` 0.30.13, `softbuffer` 0.4.8, `accesskit` 0.25,
  `accesskit_winit` 0.34, `thiserror` 2, `tracing` 0.1.
- `cabochon-hello-window`: `cabochon-clerestory`, `cabochon-clerestory-hosted`,
  `cabochon-escutcheon`, `cabochon-burin`, `cabochon-lapidary`,
  `cabochon-fonts`, `clap` 4, `serde_json` 1 (for `export.json`),
  `tracing-subscriber` 0.3, `mockable` 3, `cap-std` 4, `camino` 1. It installs a
  `tracing` subscriber and no metrics recorder in this step.
- Development: `proptest-state-machine` 0.9, `accesskit_consumer` 0.39,
  `metrics-util` (the release matching `metrics` 0.24, for its debugging
  recorder), and `tracing-test` 0.2, plus the testing crates from the 1.0.3
  plan.

## Documentation obligations

- ADR 0012 (in-window furniture, the host-shell port, and the considered
  alternatives, including Masonry) under `docs/adrs/`, referenced from
  `docs/cabochon-design.md` §15.
- `docs/cabochon-design.md` §8.1: the Clerestory contract summary; the
  Figure 1 bridge proposal (D12); the §15 deferrals (D8, D13).
- `docs/users-guide.md`: the hello-window section.
- `docs/developers-guide.md`: the session model, the headless shell as the
  reference adapter, `make test-hosted` and its provisioning, and the hosted
  residuals.
- `docs/repository-layout.md` and `docs/contents.md`: the new crates.
- `docs/roadmap.md`: 1.0.1 and 1.0.2 marked done; the proposed clipboard and
  drag-and-drop task.

## Revision notes

- 2026-10-23: created by decomposing the reviewed step-level plan (commit
  `616a8fa`). This plan carries the step plan's EP-M6a to EP-M8, the Clerestory
  obligations, the hello-window application, the hosted adapter, and the
  roadmap ticks for 1.0.1 and 1.0.2. New here: explicit application obligations
  (`OB-APP-RESOURCES`, `OB-APP-EXPORT`, `OB-APP-E2E`), the fixture move
  (decision T5), and the export directory for the in-app command (decision T7).
- 2026-10-23: applied the cross-plan consistency review: restored the
  Clerestory third-party ban in the dependency contract, split the behavioural
  scenarios so none runs before the application crate exists (T11), embedded
  the resource bundle as the default (T10), widened the fixture move to every
  reference (T5), replaced `--system-fonts` with `--fonts` so tests can force
  bundled fonts, scoped `make test-hosted` to the hosted packages under the
  development-target rules, added `Session::new`, test-recorder dependencies,
  and the `msrv` command, inlined the escalation triggers, and recorded one
  pull request per plan (T8) and the ADR number reservation (T9).
