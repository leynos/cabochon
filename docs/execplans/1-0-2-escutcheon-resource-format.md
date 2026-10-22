# Define the minimum Escutcheon resource format (roadmap 1.0.2)

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
`Tolerances`, `Risks`, `Progress`, `Surprises & discoveries`, `Decision log`,
`Outcomes & retrospective`, `Conformance basis`, and `Verification plan` must
be kept up to date as work proceeds.

Status: DRAFT

Implementation must not begin until a human reviewer explicitly approves this
plan and the 1.0.3 plan (`docs/execplans/1-0-3-lapidary-scene-contract.md`) has
been implemented and merged. This plan does not depend on the 1.0.4 (Burin)
plan and may proceed in parallel with it. Silence is not approval. Implement on
a branch named `1-0-2-escutcheon-resource-format`, in its own pull request.

## Purpose / big picture

Cabochon models its application environment on the Graphics Environment Manager
(GEM), whose applications described menus and dialogs in `.RSC` resource files
rather than in code. Cabochon's equivalent is Escutcheon. Clerestory (the
application environment) consumes Escutcheon resources, Lapidary is the scene
model, and Burin the renderer; ADR 0007 makes all four production contracts in
hosted mode. Roadmap step 1.0 delivers them as four task plans: 1.0.3 Lapidary
(done before this), 1.0.4 Burin, 1.0.2 Escutcheon (this plan), and 1.0.1
Clerestory (which consumes this plan's output).

After this plan, an application author can describe a Cabochon application's
furniture in text files: commands with labels, tooltips, icons, shortcuts, and
accessibility descriptions; a menu bar; windows; dialogs with a simple layout
tree; inspector panels; icons as SVG; and strings as Project Fluent messages in
one or more locales. Escutcheon compiles those files into a typed,
cross-referenced `ResourceBundle`, or reports every problem with its file,
line, and column. Lookups fall back through a locale chain to the base locale,
and two pseudo-locales (`en-XA`, accented and lengthened, and `ar-XB`,
right-to-left) exercise localization without translators. A command-line
checker lets authors who do not write Rust validate a bundle.

Observable success:

1. `cargo run -p cabochon-escutcheon --bin escutcheon-check -- crates/cabochon-escutcheon/tests/fixtures/hello-window`
   prints a one-line summary such as
   `ok: <n> commands, <n> menus, <n> dialogs, 1 inspector, 1 icon, 1 locale`
   (EP-M2 fixes the counts when it authors the fixture) and exits zero.
2. Running it on `tests/fixtures/broken/dangling-command` prints a diagnostic
   naming the undeclared command with its file, line, and column, and exits
   non-zero.
3. `make test` passes, including `resources.feature`.

Roadmap task 1.0.2's success criterion is that the hello-window application
loads its user interface from resources. That application belongs to the 1.0.1
plan, so the 1.0.1 plan marks 1.0.2 done (decision T1). This plan delivers the
hello-window resource bundle as a fixture that the 1.0.1 plan moves into the
application crate (decision T5).

How to read identifiers: `RM-` roadmap task, `TDD-` technical-design
requirement, `EP-` milestone, `OB-` verification obligation, `A-` axiom, `C`
constraint, `D` decision inherited from the reviewed step-level plan, `T`
decision made for the task-level plans.

## Constraints

- C1, GEM-derived invariants. Escutcheon owns declarative user-interface
  resources (`docs/cabochon-design.md` §3, invariant 4); it contains no
  rendering or layout code and names no Enfilade type.
- C2, dependency direction. `cabochon-escutcheon` depends only on
  `cabochon-lapidary` among Cabochon crates (normal edges), for compiled icon
  geometry. This plan adds its row to `tests/substrate_dependency_contract.rs`.
- C3, no third-party types in Escutcheon's public API: no `kdl`, Fluent,
  `unic-langid`, `usvg`, or `miette` types. Escutcheon owns `LocaleChain`,
  `Diagnostic`, and message-argument types.
- C4, integer geometry. The only floating-point code is
  `cabochon_escutcheon::icon::float_boundary`, which converts `usvg`'s `f32`
  path data into Lapidary EMU with reasoned lint expectations.
- C5, lint, safety, and documentation policy as in the repository, including
  the style guide's doc-comment rules.
- C6, existing contracts, including the `msrv` CI job, keep passing; member
  crates declare no features.
- C7, the resource schema is versioned (version 1) and is an authoring format,
  not a persisted user-document format.
- C9, no direct process-environment access; the checker reads only its
  arguments and the named directory.
- C10, library instrumentation only.

## Tolerances (exception triggers)

- Scope: a milestone exceeds 3,000 net added lines of Rust (excluding
  fixtures and snapshots) or 60 files.
- Interface: the `ResourceBundle` shape must change in a way that alters the
  1.0.1 plan.
- Dependencies: a dependency not listed here is needed, or a listed crate must
  move to a new major or pre-1.0 minor series.
- Format: KDL 2.0 or Fluent proves unable to express a required resource
  without escape hatches; present the alternatives before continuing.
- Upstream conflict: evidence contradicts `docs/cabochon-design.md`, an ADR,
  or the roadmap; set `Status: BLOCKED` and record the proposed deviation.
- Iterations: a red test cannot be turned green after five attempts with
  distinct hypotheses.

## Risks

- Risk: `fluent-bundle` and `fluent-pseudo` release about once a year.
  Severity: low. Likelihood: medium. Mitigation: wrap them behind an
  Escutcheon-owned `Localizer`; the public API never exposes Fluent types.
- Risk: the `kdl` crate's minimum supported Rust version (reported as 1.95)
  interacts with the root package's `rust-version = "1.85"` under the
  Rust-version-aware resolver. Severity: low. Likelihood: medium. Mitigation:
  the crate declares its own `rust-version`; the `msrv` job checks only the
  root library; confirm `Cargo.lock` picks the intended `kdl` release.
- Risk: KDL 2.0 syntax (for example `#true`) surprises authors used to JSON.
  Severity: low. Likelihood: medium. Mitigation: the prose schema reference and
  checker diagnostics explain the idioms.
- Risk: `cap_std` directory listing order is unspecified, making compilation
  order-dependent. Severity: medium. Likelihood: medium. Mitigation:
  `OB-ESC-DET` sorts inputs and diagnostics.

## Progress

- [x] (2026-10-18) Step-level plan drafted, reviewed by four
  community-of-experts panels, and revised (commit `616a8fa`).
- [x] (2026-10-22) Reviewer accepted decisions D1, D6, D7, D8, D10, D11, D12,
  D13, and D14 on the step-level plan.
- [x] (2026-10-23) Decomposed into four task-level plans; this is the third.
- [ ] Reviewer approval of this plan; the 1.0.3 plan implemented and merged.
- [ ] EP-M1: schema, parser, diagnostics, shortcuts, and the resource source
  port.
- [ ] EP-M2: localization, pseudo-locales, icons, accessibility annotations,
  and the hello-window fixture.
- [ ] EP-M3: the checker, prose schema reference, and ADR 0010 accepted.

## Surprises & discoveries

- Observation: the white paper's own resource sketch
  (`menu main { item command("file.new") }`, near line 568 of
  `references/cabochon-white-paper.md`) is already KDL-shaped. Impact: supports
  decision D4.
- Observation: KDL 2.0's specification is final, and the `kdl` crate (6.7)
  keeps spans and formatting. Impact: located diagnostics need no separate
  parser.

## Decision log

Inherited from the reviewed step-level plan (accepted ones say so):

- Decision D4: Escutcheon resources use KDL 2.0 (the `kdl` crate) for
  structure, Project Fluent `.ftl` files for strings, and SVG for icons,
  compiled into a typed, cross-referenced `ResourceBundle`. RON (weak
  diagnostics), TOML (awkward for deep ordered trees), and Qt `.ui` or
  GtkBuilder XML (toolkit-specific) were considered. Recorded as ADR 0010.
- Decision D8 (accepted 2026-10-22): the minimum standard dialogs are alert,
  confirmation, and About. The schema has no file open or save dialogs; those
  arrive with the portal work in roadmap 5.1.2.
- Decision D13 (accepted 2026-10-22): drag-and-drop and host clipboard
  integration are deferred, so the built-in command catalogue has no clipboard
  commands in this step.
- Decision D14 (accepted 2026-10-22): text input with input-method support is
  in scope; the schema therefore includes a single-line `text-field` control.
- Decision D19: Escutcheon's canonical emitter (writing a bundle back to KDL)
  is deferred; nothing in roadmap 1.0 or 2.1 needs it.
- Decision D20: assertion style (`googletest` matchers, `pretty_assertions`
  for whole values, `insta` for stable summaries; `#[gtest]` before
  `#[rstest]`).

Task-level decisions:

- Decision T1: implementation order 1.0.3, then 1.0.4 and 1.0.2, then 1.0.1;
  the 1.0.1 plan marks 1.0.2 done once the hello-window application loads its
  user interface from these resources. Date: 2026-10-23.
- Decision T5: this plan authors the hello-window bundle as a fixture at
  `crates/cabochon-escutcheon/tests/fixtures/hello-window/`. The 1.0.1 plan
  moves it with `git mv` to `crates/cabochon-hello-window/resources/` and
  points this crate's tests at the new location, so one copy exists. Rationale:
  Cargo rejects a `crates/*` workspace member directory without a manifest, so
  the application crate cannot exist before the 1.0.1 plan. Date: 2026-10-23.
- Decision T6: the checker is a `bin` target, `escutcheon-check`, in
  `cabochon-escutcheon`, with handwritten argument handling (one directory and
  an optional `--locale`), so the library gains no command-line parsing
  dependency. Date: 2026-10-23.
- Decision T8: one pull request per task plan, replacing the step plan's
  decision D18, following the reviewer's direction on 2026-10-23. Date:
  2026-10-23.
- Decision T9: ADR numbers are reserved by this decomposition: 0009 for the
  1.0.3 plan, 0010 for the 1.0.2 plan, 0011 for the 1.0.4 plan, and 0012 for
  the 1.0.1 plan. 0013 is reserved for the 1.0.3 plan's Cranelift decision, if
  its EP-M0b expunges Cranelift (decision T10 there). Any other pull request
  needing an ADR takes 0014 or later. Date: 2026-10-23.

## Outcomes & retrospective

Not started.

## Context and orientation

When this plan starts, the repository is a Cargo workspace (from the 1.0.3
plan) with `crates/cabochon-lapidary`, `crates/cabochon-substrate-testkit`,
prover tooling, `tests/substrate_dependency_contract.rs`, and
`.config/nextest.toml`. Escutcheon uses Lapidary only for `Path`, `Emu`, and
`Rect`, to hold compiled icon geometry, and for `TextDirection`, which the
localizer reports for each locale.

Governing documents: `docs/cabochon-design.md` §3 (invariant 4) and §8.2
(Escutcheon);
`docs/adrs/adr-0007-make-the-gem-substrate-a-production-boundary.md`;
`docs/adrs/adr-0004-require-interoperable-authoring-paths.md` (contracts
neutral to the authoring language); `docs/context.md` (Escutcheon, Resource);
`docs/terms-of-reference.md` (the developer audience knows Python or
JavaScript, not Rust); and `docs/roadmap.md` task 1.0.2.

Terms:

- Resource bundle: one application's directory of resources: a
  `<app>.escutcheon.kdl` file, `locales/<tag>/<app>.ftl` files, and
  `icons/*.svg`.
- Command identifier: a dotted name such as `app.quit`; string key: a
  hyphenated Fluent message identifier such as `menu-file`.
- Base locale: the locale whose strings must be complete (`en-GB` for the
  hello-window bundle).
- Locale chain: the ordered locales consulted for a lookup, ending in the
  base locale.
- Pseudo-locale: a generated locale that transforms base strings to expose
  layout and direction bugs.
- Built-in command: a command Escutcheon declares for every application:
  `dialog.ok`, `dialog.cancel`, and `window.close`.
- Layout control: a node in a dialog or inspector layout tree. Schema version
  1 defines `column`, `row` (with `align`), `label`, `button` (with `command`,
  `default`, and `cancel`), `text-field` (with a bound property key and an
  accessibility label), `separator`, `spacer`, and `property-row` (a label and
  a value or text field). Escutcheon validates structure and references only;
  Clerestory owns measuring and placing controls.

Skills: `execplans`, `rust-router` (then `rust-types-and-apis` and
`rust-errors`), `hexagonal-architecture`, `rust-unit-testing`, `proptest`,
`arch-decision-records`, `en-gb-oxendict-style`, `codegraph-mcp`, and
`firecrawl-mcp` for the KDL and Fluent documentation. References as listed in
the 1.0.3 plan.

## Conformance basis

Upstream at `main` commit `0de2b57`. Traced items: `RM-1.0.2`; `TDD-ESC`
(design §8.2); `TDD-INV-4` (§3); ADR 0004; ADR 0007.

```plaintext
RM-1.0.2 -> TDD-ESC, TDD-INV-4 -> EP-M1, EP-M2 -> OB-ESC-REFS, OB-ESC-SHORTCUT, OB-ESC-L10N, OB-ESC-ICON
RM-1.0.2 -> TDD-ESC (testable, versionable) -> EP-M1, EP-M3 -> OB-ESC-VERSION, OB-ESC-DET, OB-ESC-CHECK
ADR-0004 -> EP-M3 -> escutcheon-check, prose schema reference
RM-1.0.2 (success criterion) -> 1.0.1 plan -> hello-window loads these resources
```

## Verification plan

Escutcheon's obligations are validation, lookup, and determinism rules over
generated and named inputs. Property tests, parameterized cases, snapshots of
diagnostics, behavioural scenarios, and an end-to-end checker test give the
right assurance; no obligation here is a contractual algebraic kernel, so
neither Verus nor Kani is warranted. Non-vacuity mechanics follow the 1.0.3
plan (fixed-seed `TestRunner`, per-class counters, constructed classes); pure
suites run 256 cases.

Axioms: `A-KDL`, `A-FLUENT`, and `A-USVG`: those parsers conform to their
specifications for the constructs the schema uses.

`OB-ESC-REFS`, referential integrity. A compiled bundle has no dangling
reference: every menu item's command (declared or built in), every command's
label, tooltip, and accessibility strings, every icon, every window's menu and
inspector, and every dialog control's command or string resolves. Method: a
property test that generates valid bundles, seeds exactly one dangling
reference, and asserts compilation fails with a diagnostic naming it and its
span; named cases; an `insta` snapshot of rendered diagnostics for a
multi-error file. Non-vacuity: counters for each reference kind seeded.

`OB-ESC-SHORTCUT`, shortcuts. No two commands share a shortcut within one scope;
`Shortcut::parse(s.to_string()) == s`. Shortcuts name logical keys; matching
falls back to the Latin layout's key for the same physical key, so Ctrl+Q works
on a Cyrillic layout. Property tests plus named cases.

`OB-ESC-L10N`, localization. Lookup walks the locale chain to the base locale
and records whether a fallback occurred; the base locale must define every
referenced key; `en-XA` and `ar-XB` transform every string, and `ar-XB` reports
right-to-left direction; missing message arguments are reported, not dropped;
bidirectional isolation marks are on by default and pinned off in snapshot
tests. Named cases plus the behavioural feature.

`OB-ESC-ICON`, icon compilation. A compiled icon's Lapidary paths lie inside
its declared view box, preserve the SVG's fill rule, and come from
`float_boundary` rounding to nearest EMU; SVG text, scripts, and external
references are rejected with located diagnostics. Named cases with small
fixture icons.

`OB-ESC-DET`, determinism. Compilation is independent of directory listing
order, and diagnostics are sorted by file then position. Property test with a
shuffling in-memory resource source.

`OB-ESC-VERSION`, versioning. A bundle declaring an unknown schema version is
rejected with a located diagnostic. Named case.

`OB-ESC-CHECK`, the checker. Run as a child process on the hello-window fixture,
`escutcheon-check` prints the summary and exits zero; on each broken fixture
it prints sorted diagnostics and exits non-zero. End-to-end test.

`OB-ESC-OBS`, compilation emits one `tracing` span per compile and increments
`escutcheon_compile_total{outcome}`. Named case with a test recorder.

## Plan of work

Red-Green-Refactor throughout; each milestone ends with `make check-fmt`,
`make lint`, and `make test` passing (sequentially, through the `scrutineer`
agent) and a commit. Each dependency addition is its own commit with
`make audit`.

### EP-M1: schema, parser, diagnostics, shortcuts, and resource source

Create `crates/cabochon-escutcheon` and add its dependency-contract row and
ownership policy (`docs/cabochon-design.md`, `docs/repository-layout.md`).
Implement the `ResourceSource` port with a `cap_std` directory adapter and an
in-memory adapter for tests; the KDL schema version 1 parser for commands,
menus, windows, dialogs, inspectors, and icons; the built-in command catalogue;
the shortcut grammar; and Escutcheon-owned `Diagnostic`, `SourceSpan`, and
`DiagnosticCode` types. Write `OB-ESC-REFS`, `OB-ESC-SHORTCUT`, `OB-ESC-DET`,
and `OB-ESC-VERSION`. Draft ADR 0010 as `docs/adrs/adr-0010-<title-slug>.md`.

### EP-M2: localization, icons, annotations, and the hello-window fixture

Implement the `Localizer` over `fluent-bundle` with locale-chain fallback and
the `en-XA` and `ar-XB` pseudo-locales through `fluent-pseudo`; icon
compilation from SVG to Lapidary paths through `usvg` (its `text` feature
disabled) and `icon::float_boundary`; and accessibility annotations
(`a11y-label`, `a11y-description`) on commands and controls. Author the
hello-window bundle under `tests/fixtures/hello-window/` (commands `app.quit`,
`app.about`, `view.inspector`, and `content.export`; File, View, and Help
menus; the About dialog; the greeting inspector with a text field; one icon;
`en-GB` strings) and the broken fixtures. Write `OB-ESC-L10N`, `OB-ESC-ICON`,
`OB-ESC-OBS`, and `resources.feature`.

### EP-M3: checker, schema reference, and ADR 0010

Implement `escutcheon-check` and `OB-ESC-CHECK`. Write the prose reference for
Escutcheon schema version 1 (every node, property, and KDL idiom, with the
hello-window bundle as the worked example), linked from `docs/contents.md`.
Accept ADR 0010. Document the resource conventions in
`docs/developers-guide.md` and add a short "Validate resources" section to
`docs/users-guide.md`.

## Milestones and plateaus

Conformance check at each boundary as in the 1.0.3 plan. Recovery: revert the
milestone's commits. Compatibility decision: none.

- EP-M1. Outcome: bundles parse and validate with located diagnostics.
  Requirements: `RM-1.0.2` (menus, command metadata, shortcuts, dialog layout),
  `TDD-ESC`. Evidence: `OB-ESC-REFS`, `OB-ESC-SHORTCUT`, `OB-ESC-DET`, and
  `OB-ESC-VERSION` pass.
- EP-M2. Outcome: strings, icons, annotations, and the hello-window fixture.
  Requirements: `RM-1.0.2` (strings, icons, localization hooks), `TDD-INV-4`.
  Evidence: `OB-ESC-L10N`, `OB-ESC-ICON`, `OB-ESC-OBS`, and `resources.feature`
  pass.
- EP-M3. Outcome: authors can validate bundles without Rust. Requirements:
  ADR 0004. Evidence: `OB-ESC-CHECK` passes; ADR 0010 accepted. Roadmap 1.0.2
  stays open for the 1.0.1 plan.

## Behavioural specification

`crates/cabochon-escutcheon/tests/features/resources.feature`:

```gherkin
Feature: Escutcheon compiles textual, localizable resources

  Scenario: A valid bundle compiles
    Given the hello-window resource bundle
    When it is compiled for locale "en-GB"
    Then the menu "File" contains the command "app.quit" with shortcut "Ctrl+Q"

  Scenario: A dangling command reference is located
    Given a bundle whose menu item names the undeclared command "file.frobnicate"
    When it is compiled
    Then compilation fails with a diagnostic at the menu item's line and column

  Scenario: Missing translations fall back to the base locale
    Given a bundle whose "fr" strings omit "about-title"
    When "about-title" is looked up for locale "fr"
    Then the base-locale text is returned and a fallback is recorded

  Scenario: The pseudo-locales transform every string
    When the hello-window bundle is compiled for locales "en-XA" and "ar-XB"
    Then every visible string differs from its "en-GB" text
    And the "ar-XB" bundle declares right-to-left direction
```

## Concrete steps

```bash
BRANCH=$(git branch --show-current)
make check-fmt 2>&1 | tee /tmp/check-fmt-cabochon-${BRANCH}.out
make lint 2>&1 | tee /tmp/lint-cabochon-${BRANCH}.out
make test 2>&1 | tee /tmp/test-cabochon-${BRANCH}.out
cargo run -p cabochon-escutcheon --bin escutcheon-check -- \
  crates/cabochon-escutcheon/tests/fixtures/hello-window
```

```plaintext
ok: <commands> commands, <menus> menus, <dialogs> dialogs, 1 inspector, 1 icon, 1 locale
```

The `msrv` CI job must keep passing (constraint C6); reproduce it from outside
the checkout:

```bash
cd /tmp && cargo +1.85.0 check --lib --all-features --locked \
  --manifest-path "$OLDPWD/Cargo.toml"
```

## Validation and acceptance

- Tests: `make test` passes with every obligation's tests and
  `resources.feature`.
- Lint and formatting: `make check-fmt` and `make lint` pass with no
  `#[expect]` outside `icon::float_boundary`.
- Documentation: `make markdownlint`, `make nixie`, and `make spelling` pass.
- Security: `make audit` passes; no `unsafe`.

## Idempotence and recovery

Compilation and checking are read-only. Every step is additive. Accept `insta`
snapshot changes only after confirming an intended contract change.

## Artefacts and notes

Record red, green, and refactor transcripts here as work proceeds.

## Interfaces and dependencies

```rust,no_run
pub trait ResourceSource {
    fn read(&self, path: &ResourcePath) -> Result<String, ResourceReadError>;
    fn list(&self, dir: &ResourcePath) -> Result<Vec<ResourcePath>, ResourceReadError>;
}

pub fn compile(source: &dyn ResourceSource, locales: &LocaleChain) -> Result<ResourceBundle, CompileDiagnostics>;

pub struct LocaleChain(Vec<LocaleTag>);
pub struct Diagnostic { pub file: ResourcePath, pub span: SourceSpan, pub code: DiagnosticCode, pub message: String }

pub struct ResourceBundle { /* commands, menus, windows, dialogs, inspectors, icons, localizer */ }
pub struct CommandSpec {
    pub id: CommandId,
    pub label: StringKey,
    pub tooltip: Option<StringKey>,
    pub accessible_description: Option<StringKey>,
    pub icon: Option<IconId>,
    pub shortcuts: Vec<Shortcut>,
    pub enabled_by_default: bool,
}

pub trait Localizer {
    fn lookup(&self, key: &StringKey, args: &MessageArgs) -> Result<LocalizedText, LookupError>;
    fn direction(&self) -> TextDirection; // Lapidary's TextDirection
}

pub enum StandardDialogKind { Alert, Confirm, About }
```

Bundle layout for an application named `<app>`:

```plaintext
resources/
├── <app>.escutcheon.kdl        schema version, commands, menus, windows, dialogs, inspectors, icons
├── locales/en-GB/<app>.ftl     base locale (must be complete)
├── locales/fr/<app>.ftl        optional translations
└── icons/*.svg
```

Example fragment. Dotted identifiers such as `app.quit` are command
identifiers; hyphenated ones such as `menu-file` are Fluent string keys;
`#true` is KDL 2.0's boolean; `dialog.ok` is built in.

```plaintext
escutcheon version=1
command "app.quit" label="cmd-quit" shortcut="Ctrl+Q"
command "app.about" label="cmd-about" shortcut="F1" icon="hello"
menu-bar {
    menu label="menu-file" { item command="app.quit" }
    menu label="menu-help" { item command="app.about" }
}
dialog "about" kind="about" title="about-title" {
    column {
        label text="about-body"
        row align="end" { button command="dialog.ok" default=#true }
    }
}
```

Dependencies (caret requirements; newest compatible release when each milestone
starts): `cabochon-lapidary`, `kdl` 6.7, `fluent-bundle` 0.16, `fluent-pseudo`
0.3, `unic-langid` 0.9, `usvg` 0.48 (default features off), `cap-std` 4,
`camino` 1, `thiserror` 2, `tracing` 0.1, `metrics` 0.24. Development:
`metrics-util` (the release matching `metrics` 0.24, for its debugging
recorder) and `tracing-test` 0.2, plus the testing crates from the 1.0.3 plan.

## Documentation obligations

- ADR 0010 (KDL, Fluent, and SVG resources) under `docs/adrs/`, referenced
  from `docs/cabochon-design.md` §15.
- `docs/cabochon-design.md` §8.2: the resource contract summary and built-in
  command catalogue.
- The prose schema reference for Escutcheon schema version 1 (ADR 0004).
- `docs/developers-guide.md`: resource conventions and the fixture's move in
  the 1.0.1 plan; `docs/users-guide.md`: validating resources with
  `escutcheon-check`.
- `docs/repository-layout.md` and `docs/contents.md`: the new crate and
  reference.

## Revision notes

- 2026-10-23: created by decomposing the reviewed step-level plan (commit
  `616a8fa`). This plan carries the step plan's EP-M5 and Escutcheon
  obligations, adds `OB-ESC-ICON`, `OB-ESC-CHECK`, and `OB-ESC-OBS` as explicit
  obligations, authors the hello-window bundle as a fixture (decision T5), and
  leaves the 1.0.2 roadmap tick to the 1.0.1 plan (decision T1). The
  `edit.copy` built-in is dropped, because clipboard integration is deferred
  (decision D13).
- 2026-10-23: applied the cross-plan consistency review: listed the schema
  version 1 layout controls that the 1.0.1 plan's layout engine expects, noted
  Escutcheon's use of Lapidary's `TextDirection`, inlined the escalation
  triggers, added test-recorder dependencies and the `msrv` command, and
  recorded one pull request per plan (T8) and the ADR number reservation (T9).
- 2026-10-23: reserved ADR 0013 for the 1.0.3 plan's conditional Cranelift
  decision (T9); other new ADRs take 0014 or later.
