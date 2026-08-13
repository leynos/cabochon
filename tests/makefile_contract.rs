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

use std::error::Error;

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
    let joined = recipe.join("\n");
    if !joined.contains("--config") {
        return Err(format!(
            "Makefile target `{target}`'s recipe does not pass --config; AGENTS.md's \"Dev-fast \
             is the standard development profile\" section requires standard \
             build/test/lint/typecheck targets to route cargo through tools/dev-fast/config.toml"
        )
        .into());
    }
    if !mentions_dev_fast(&joined) {
        return Err(format!(
            "Makefile target `{target}`'s recipe does not reference the dev-fast configuration \
             fragment (tools/dev-fast/config.toml); see AGENTS.md's \"Dev-fast is the standard \
             development profile\" section"
        )
        .into());
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
