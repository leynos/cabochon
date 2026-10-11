# Architectural decision record (ADR) 006: `main` owns coverage publication

## Status

Accepted design. Adopts concordat's CV-005, `main-owned-codescene-coverage`.
GitHub read-back confirms `CS_ACCESS_TOKEN` is present in the protected
`codescene` environment and absent from repository-level secrets. Deployment is
restricted to `main`, with administrator bypass enabled. Publisher run
[38128492817](https://github.com/leynos/cabochon/actions/runs/38128492817)
succeeded for coverage, ratchet-baseline saving, and CodeScene upload.

## Date

2026-09-23.

## Context and problem statement

Coverage was measured on pull requests and sent to CodeScene from the same
lane. Both halves of that failed quietly. A pull request from a fork cannot read
`secrets.CS_ACCESS_TOKEN`, so the upload was skipped for exactly the changes
that most need review, and CodeScene accepts an upload only for a branch it
analyses, which a pull request head is not. With CodeScene's coverage gates
switched off, its pull-request check mode now fails outright. A lane that holds
the token is also a lane a pull request's workflow edit could try to reach.

## Decision drivers

- Keep a failing coverage ratchet on every pull request.
- Keep the CodeScene credential, host, and CLI off every workflow a pull
  request can reach, directly or through a called workflow.
- Give the ratchet baseline exactly one writer, so every pull request is
  measured against `main`.
- Hold the split with an executable contract rather than a convention.

## Decision outcome

Two workflows split the work. The pull-request lane runs the shared
`generate-coverage` action with `with-ratchet: 'true'` and
`publish-artefact: 'false'`, and nothing else touches CodeScene. The publisher,
`coverage-main.yml`, runs on a push to `main` and on dispatch: it measures the
same pinned selection, which writes the ratchet baseline on a push, and uploads
the report in explicit upload mode, passing the secret as the action's
`access-token` input and guarded on exactly
`steps.codescene-token.outputs.available == 'true' && github.ref == 'refs/heads/main'`,
where a check step reports whether the secret is set and no step holds it in
its `env`. The uploading job declares the `codescene` environment. GitHub
read-back confirms its deployment policy permits only `main`, with
administrator bypass enabled. Secret-name read-backs confirm `CS_ACCESS_TOKEN`
is present in the environment and absent from repository-level secrets.
Publisher run
[38128492817](https://github.com/leynos/cabochon/actions/runs/38128492817)
succeeded for coverage generation, ratchet-baseline saving, and CodeScene
upload. The publisher uses a never-cancelling concurrency group keyed on
workflow and ref; a manual re-run of an older run can republish older coverage.

`make test-workflow-contracts` runs the pinned shared `cv005-contracts check`.
`.github/cv005.toml` holds this repository's parameters. The shared contract
checks workflow reachability, coverage parity, token handling, and baseline
writer uniqueness; its own suite tests the failure cases.

## Options considered

- Keep the upload on the pull-request lane. Rejected: forks never upload, heads
  are not analysed branches, and the lane must hold the token.
- Drop CodeScene coverage altogether. Rejected: the trunk figure is still
  wanted, and the ratchet needs a trunk-written baseline anyway.
- Protect the publisher with a deployment environment restricted to `main`.
  Selected. GitHub read-back verifies the token's environment placement and
  absence from repository-level secrets. The deployment policy allows `main`
  only, with administrator bypass enabled.

## Consequences

- A pull request's coverage is judged only by the ratchet; CodeScene sees
  `main`.
- An eligible `main` dispatch measures without advancing the baseline; one that
  replaces a pending push leaves the baseline a commit behind until the next
  push. Other branch dispatches are blocked by the environment policy unless an
  administrator bypasses it.
- Adding a workflow that touches CodeScene, runs ratcheted coverage on a push,
  or changes the coverage selection fails the contract, which names the clause.
