//! Contract test for the Makefile's dev-fast wiring.
//!
//! `AGENTS.md`'s "Dev-fast is the standard development profile" section
//! promises that the standard `build`, `test`, `lint`, and `typecheck`
//! Makefile targets route every `cargo` invocation through
//! `--config tools/dev-fast/config.toml`, and that `coverage` never does.
//! This test reads the repository's own `Makefile` at a
//! `CARGO_MANIFEST_DIR`-relative path and checks that contract textually,
//! so an edit that silently drops the wiring fails locally before the
//! estate-wide audit (Concordat's DF-004 rule) ever runs.
//!
//! The standard-target assertions check every `cargo`-invoking recipe
//! line individually rather than the recipe block as a whole: a target
//! with several `cargo` lines (for example `test`'s nextest run plus
//! doc-tests, or `lint`'s `cargo doc` plus `cargo clippy`) would
//! otherwise pass a whole-block text match even when only one of its
//! lines carries `--config`, leaving the other silently unwired.

use std::{error::Error, process::Command};

use cap_std::{ambient_authority, fs_utf8::Dir};
use rstest::rstest;

/// Opens the repository root as a capability-scoped directory handle,
/// rooted at the crate manifest directory rather than the ambient working
/// directory, per the repository's `cap_std` convention for filesystem
/// access.
fn open_repo_root() -> Result<Dir, std::io::Error> {
    Dir::open_ambient_dir(env!("CARGO_MANIFEST_DIR"), ambient_authority())
}

/// Reads the repository's `Makefile` through a capability-scoped handle
/// rooted at the crate manifest directory, so the test works regardless
/// of the working directory it runs from.
fn read_makefile() -> Result<String, std::io::Error> {
    open_repo_root()?.read_to_string("Makefile")
}

/// Returns the recipe lines belonging to the first target whose header
/// line starts with `header_prefix`, or `None` if no such target exists.
///
/// A recipe line is any line immediately following the header that is
/// indented with a tab; the first non-tab line (including a blank line)
/// ends the recipe, matching GNU Make's own recipe-line convention.
///
/// # Examples
///
/// ```ignore
/// let makefile = "lint: ## Run Clippy\n\tcargo clippy\n\nother:\n";
/// let recipe = find_recipe(makefile, "lint:").expect("target present");
/// assert_eq!(recipe, vec!["\tcargo clippy"]);
/// ```
fn find_recipe<'a>(makefile: &'a str, header_prefix: &str) -> Option<Vec<&'a str>> {
    let mut lines = makefile.lines();
    for line in lines.by_ref() {
        if line.starts_with(header_prefix) {
            let recipe: Vec<&str> = lines.by_ref().take_while(|l| l.starts_with('\t')).collect();
            return Some(recipe);
        }
    }
    None
}

/// Reports whether `text` mentions the dev-fast configuration fragment,
/// matching `(?i)dev[-_]fast` without pulling in a regex dependency.
fn mentions_dev_fast(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("dev-fast") || lower.contains("dev_fast")
}

/// Returns the subset of `recipe` that actually invokes `cargo`, so
/// non-`cargo` recipe lines (a `@echo`, the Whitaker invocation) are not
/// held to the `--config`/dev-fast contract.
fn cargo_invoking_lines<'a>(recipe: &[&'a str]) -> Vec<&'a str> {
    recipe
        .iter()
        .copied()
        .filter(|line| line.contains("$(CARGO)") || line.trim_start().starts_with("cargo "))
        .collect()
}

/// Standard development targets: (name used in assertion messages, the
/// Makefile header line whose recipe implements that target). `build`'s
/// own header carries no recipe — it depends on the `target/%/$(TARGET)`
/// pattern rule, which is where its `cargo build` invocation actually
/// lives — so `build` is checked against that pattern rule's header
/// instead of its own.
// Note: the second case is named `run_tests`, not `test` — rstest's case
// naming collides internally with the `#[test]` attribute it generates
// when a case is literally named `test`, silently dropping every case in
// the set (reproduced locally; tracked upstream is unconfirmed).
#[rstest]
#[case::build("build", "target/%/$(TARGET):")]
#[case::run_tests("test", "test:")]
#[case::lint("lint", "lint:")]
#[case::typecheck("typecheck", "typecheck:")]
fn standard_targets_use_dev_fast_config(
    #[case] target: &str,
    #[case] header: &str,
) -> Result<(), Box<dyn Error>> {
    let makefile = read_makefile()?;
    let recipe = find_recipe(&makefile, header).ok_or_else(|| {
        format!(
            "Makefile target `{target}` (recipe header `{header}`) was not found; AGENTS.md's \
             \"Dev-fast is the standard development profile\" section expects it to route cargo \
             through --config tools/dev-fast/config.toml"
        )
    })?;
    let cargo_lines = cargo_invoking_lines(&recipe);
    if cargo_lines.is_empty() {
        return Err(format!(
            "Makefile target `{target}`'s recipe has no cargo-invoking line to check; AGENTS.md's \
             \"Dev-fast is the standard development profile\" section expects at least one"
        )
        .into());
    }
    for line in cargo_lines {
        if !line.contains("--config") {
            return Err(format!(
                "Makefile target `{target}`'s recipe line `{line}` does not pass --config; \
                 AGENTS.md's \"Dev-fast is the standard development profile\" section requires \
                 every cargo invocation in the standard build/test/lint/typecheck targets to \
                 route through tools/dev-fast/config.toml"
            )
            .into());
        }
        if !mentions_dev_fast(line) {
            return Err(format!(
                "Makefile target `{target}`'s recipe line `{line}` does not reference the \
                 dev-fast configuration fragment (tools/dev-fast/config.toml); see AGENTS.md's \
                 \"Dev-fast is the standard development profile\" section"
            )
            .into());
        }
    }
    Ok(())
}

/// `coverage` must keep the standard LLVM backend and platform linker, so
/// its recipe must never reference the dev-fast fragment.
#[rstest]
fn coverage_target_excludes_dev_fast_config() -> Result<(), Box<dyn Error>> {
    let makefile = read_makefile()?;
    if let Some(recipe) = find_recipe(&makefile, "coverage:") {
        let joined = recipe.join("\n");
        if mentions_dev_fast(&joined) {
            return Err(
                "Makefile target `coverage`'s recipe references the dev-fast configuration \
                 fragment; coverage builds must keep the standard LLVM backend and platform \
                 linker, per AGENTS.md's \"Dev-fast is the standard development profile\" section"
                    .into(),
            );
        }
    }
    Ok(())
}

/// Runs `make --dry-run <target> CARGO=probe-cargo` against the
/// repository's own Makefile and returns the printed (not executed)
/// recipe, so the `dev-build`/`dev-test` targets' `$(CARGO)` wiring can
/// be checked without needing the pinned nightly toolchain or `mold`
/// installed.
fn dry_run_with_probe_cargo(target: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("make")
        .args(["--dry-run", target, "CARGO=probe-cargo"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "`make --dry-run {target} CARGO=probe-cargo` exited with {status}; stderr: {stderr}",
            status = output.status,
            stderr = String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

/// `dev-build` and `dev-test` must call `$(CARGO)` rather than a
/// hard-coded `cargo`, so a caller can inject a probe or wrapper binary.
/// Checks the dry-run output shows the substituted binary name, then
/// `--config`, then a dev-fast reference, in that order, proving the
/// substitution reaches the actual invocation rather than being an inert
/// variable definition elsewhere in the file.
#[rstest]
#[case::dev_build("dev-build")]
#[case::dev_test("dev-test")]
fn dev_fast_targets_substitute_cargo_variable(#[case] target: &str) -> Result<(), Box<dyn Error>> {
    let output = dry_run_with_probe_cargo(target)?;
    let probe_pos = output.find("probe-cargo").ok_or_else(|| {
        format!(
            "`make --dry-run {target} CARGO=probe-cargo` did not substitute the CARGO variable; \
             the {target} recipe must invoke $(CARGO) rather than a hard-coded cargo"
        )
    })?;
    let after_probe = output
        .get(probe_pos..)
        .ok_or("internal error: `probe-cargo` match position was not a valid UTF-8 boundary")?;
    let config_offset = after_probe.find("--config").ok_or_else(|| {
        format!(
            "`make --dry-run {target} CARGO=probe-cargo` output does not show --config after the \
             substituted CARGO binary"
        )
    })?;
    let after_config = after_probe
        .get(config_offset..)
        .ok_or("internal error: `--config` match position was not a valid UTF-8 boundary")?;
    if !mentions_dev_fast(after_config) {
        return Err(format!(
            "`make --dry-run {target} CARGO=probe-cargo` output does not reference the dev-fast \
             configuration fragment after --config"
        )
        .into());
    }
    Ok(())
}

/// The dev-fast fragment referenced by the standard targets must actually
/// exist for `--config tools/dev-fast/config.toml` to do anything useful.
#[rstest]
fn dev_fast_config_fragment_exists() -> Result<(), Box<dyn Error>> {
    let is_file = open_repo_root()?
        .metadata("tools/dev-fast/config.toml")
        .is_ok_and(|metadata| metadata.is_file());
    if !is_file {
        return Err(
            "tools/dev-fast/config.toml is missing; the dev-fast wiring contract in AGENTS.md \
             depends on this fragment existing at the repository root"
                .into(),
        );
    }
    Ok(())
}
