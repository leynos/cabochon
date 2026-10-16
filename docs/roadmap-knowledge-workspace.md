# Knowledge-workspace roadmap

This roadmap tests individual value with notes and one rich object before
building the complete starter application set. [The programme map](roadmap.md)
defines G4 and G5. H and D refer to its hosted-runtime and developer-experience
roadmaps. The full proving ground still includes dataframes, Mermaid diagrams,
and bitmap images; the first experiment is not a unilateral scope reduction.

The authority is [terms of reference](terms-of-reference.md) §§4-8 and
[technical design](cabochon-design.md) §§7.3-7.4, 9.2, and 10-13. Later editor
work is admitted only after the earlier workflow test produces useful evidence.

## 1. Use linked notes and one editable rich embed

Idea: a small real knowledge workflow can test semantic continuity before
Cabochon invests in three applications and a broad note editor.

Entry: G2 for implementation. Evaluation planning may start earlier. Exit G4:
a user repeats the chosen workflow, including editing the same embedded object
outside the note and reopening the project, and the outcome supports a clear
continue, narrow, or stop decision.

### 1.1. Bound the workflow and the note model

This step chooses the evidence before choosing the application backlog. See
terms of reference §§5 and 7 and design §9.2.

- [ ] 1.1.1. Select representative tasks and record the first experiment's
  acceptance rubric, baseline workflow, and supported limits.
  - Start with prose, linked notes, and one source-edited Mermaid diagram
    unless the observed task makes a dataframe demonstrably more useful.
  - Record any change of first provider and resequence the relevant tasks;
    the other provider must not disappear from G5 scope.
  - Measure semantic continuity, recovery, task completion, and application
    switching against the user's existing workflow, not an entire product.
- [ ] 1.1.2. Implement one local project's note identities, links, and tree.
  - Requires 1.1.1. Renames and moves preserve link targets; project export,
    import, and reopen preserve the note graph. Reuse H's storage contract.
- [ ] 1.1.3. Add bounded prose and link editing with transaction-backed undo.
  - Requires 1.1.2. Support the chosen paragraphs, headings, and links only;
    defer plugin compatibility and a general rich-text or page-layout editor.
  - Exercise keyboard editing and accessible text semantics with the feature.

### 1.2. Prove embedding and a focused editor across a process boundary

Requires step 1.1. See design §§7.3-7.4, 8.9, and 9.2. Reuse H's minimal
currency host and provider harness instead of starting a new document system.

- [ ] 1.2.1. Implement the common embed envelope and explicit placeholders for
  available, opaque, unavailable, stale, and malformed objects.
  - Retain identity, payload references, and recoverable presentations.
    Provider removal must not delete or silently flatten an embed.
- [ ] 1.2.2. Add undo for inserting, removing, moving, and changing the
  presentation of an existing embed.
  - Requires 1.2.1. Preserve object identity and enforce revision conflicts.
    Restrict mutations to the supported atomic domain; reject unsupported
    cross-provider compound edits rather than approximating atomic undo.
- [ ] 1.2.3. Implement the Mermaid source provider and a focused source editor.
  - Requires 1.2.1. Open the same object in a separate application process;
    source edits update the note's presentation without importing a copy.
  - Use source editing and preview, not a graphical diagramming application.
    Invalid source remains editable with located diagnostics. Validate the
    representative diagram fixtures with the repository's Mermaid checks.
- [ ] 1.2.4. Export a note through a page-oriented Lapidary presentation.
  - Requires 1.2.3 and H 3.1. Include headings, text, the supported embed,
    page geometry, and unavailable-provider diagnostics. Extend shared output
    conformance rather than introducing a note-specific export renderer.

### 1.3. Decide whether the first workflow earns another editor

Requires step 1.2. The outcome gates phase 2; it is not a marketing milestone.
See terms of reference §7.

- [ ] 1.3.1. Exercise the chosen workflow repeatedly with realistic project
  data, including provider absence, undo, export, and restart.
  - Record task completion, switching, stale embeds, and failure recovery with
    consent. A manual observation log is sufficient; collect no document
    content in metrics and require no telemetry service.
- [ ] 1.3.2. Record the value result and the next scope decision.
  - Requires 1.3.1. Compare against the predeclared rubric. Continue with one
    additional editor, narrow and repair the workflow, or stop expansion.
  - A failure cannot be answered solely by adding dataframes and painting.

## 2. Add the remaining focused editors one at a time

Idea: each additional provider should prove reuse and add an observed workflow,
not introduce another application-sized prerequisite to initial value.

Entry: G4 recommends expansion. Complete one editor's acceptance loop before
starting the next. Exit: notes support all three agreed rich object types
without conversion or loss of identity. This is still not full G5 acceptance.

### 2.1. Add a bounded dataframe workflow

See design §§8.9 and 9.2. If the dataframe was selected first, complete its
tasks in phase 1 and perform the Mermaid work here; record the sequencing
change.

- [ ] 2.1.1. Implement typed scalar cells, stable row and column identities,
  selections, and the dataframe provider's persistence fixtures.
  - Bound sizes and supported scalar types. Defer spreadsheet compatibility,
    arbitrary formulas, pivots, and external data connectors.
- [ ] 2.1.2. Implement focused cell, row, and column editing through selectors.
  - Requires 2.1.1. Include revision checks, supported undo, keyboard access,
    and accessible names and values; use the existing authority path.
- [ ] 2.1.3. Embed and export that same dataframe and test the focused workflow.
  - Requires 2.1.2. Note and editor share identity; edits update presentations.
    Bound page overflow explicitly and test provider absence and reopen.

### 2.2. Add a bounded bitmap workflow

Requires step 2.1 unless the gate review explicitly changes the order. See
design §9.2. The experiment is editable embedding, not a general paint package.

- [ ] 2.2.1. Implement a bitmap payload and import path with decoded-size limits
  and malformed-image diagnostics.
  - Use supported file portals and preserve the imported object's identity.
- [ ] 2.2.2. Implement a focused editor with a small brush and crop operation.
  - Requires 2.2.1. Bound image sizes, colour handling, and undo storage.
    Defer layers, filters, colour-management breadth, and file-format breadth.
- [ ] 2.2.3. Embed, reopen, and export the bitmap after edits in its editor.
  - Requires 2.2.2. Every presentation refers to the same object; shared scene
    geometry, output conformance, and failure diagnostics remain authoritative.

## 3. Establish the full hosted proving ground

Idea: three working demonstrations become a product only when real work
survives installation, upgrades, unavailable providers, and ordinary mistakes.

Entry: phase 2 and G3. Exit G5: the accepted note, dataframe, Mermaid, and
bitmap workflows meet the recorded user and developer criteria. G5 may justify
a separate shell decision; it does not automatically approve shell development.

### 3.1. Exercise the combined workspace's failure boundaries

See design §§10-14. Grow the matrix from delivered workflows rather than
requiring every future combination to be tested in advance.

- [ ] 3.1.1. Add targeted cross-application identity and recovery cases.
  - Cover provider removal and restart, opaque objects, project reopen, and
    undo. Test high-risk denial, crash, and restart combinations explicitly;
    use pairwise coverage only for lower-risk independent dimensions.
- [ ] 3.1.2. Exercise export, restore, and provider upgrades with all supported
  rich objects and unknown fields in the same project.
  - Requires 3.1.1. Test mixed available and unavailable content, interrupted
    migration, and retained user data. Never flatten an unsupported embed.
- [ ] 3.1.3. Package the complete workspace for the two accepted host profiles.
  - Requires 3.1.2 and H 3.3. Reuse activation and portal tests. Check fresh
    install, update, removal, and data recovery with the real application set.

### 3.2. Record whether the hosted product earns further investment

Requires step 3.1. See terms of reference §§6-8. Thresholds must be declared
before the evaluation, not chosen afterwards to make the result look positive.

- [ ] 3.2.1. Revisit the workflow rubric for the complete proving ground.
  - Add developer-extension, operational recovery, and sustained individual
    use criteria to the early user tasks. Compare specific work with the
    established Obsidian workflow without promising wholesale feature parity.
- [ ] 3.2.2. Run the evaluation and record an ADR-backed product decision.
  - Requires 3.2.1. Record user and developer evidence, operational limits,
    maintenance cost, and whether to deepen, narrow, or stop the hosted scope.
  - E 3.1 owns any separate decision to pursue Mullion. A useful hosted product
    may remain hosted indefinitely.
