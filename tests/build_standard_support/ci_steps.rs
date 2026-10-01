//! Reader for the CI half of the build standard: every workflow that builds under
//! the standard installs `mold` through `setup-rust`'s `install-mold` input, so the
//! Linux jobs have the linker the configuration names.
//!
//! The workflows are read as text, one step at a time. Release workflows are not
//! listed: a release stays on the platform linker and never uses `mold`.

use super::config::{Flags, Problems};

/// The Cargo arguments that identify CI's standalone doctest step.
const DOCTEST_ARGUMENTS: [&str; 4] = ["test", "--doc", "--workspace", "--all-features"];

/// The workflows that set up Rust and build under the standard, as name and text.
/// The list is this repository's own, so a workflow that stops setting up Rust
/// fails the contract rather than dropping out of it.
pub const WORKFLOWS: &[(&str, &str)] = &[
    (
        "ci.yml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/.github/workflows/ci.yml"
        )),
    ),
    (
        "coverage-main.yml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/.github/workflows/coverage-main.yml"
        )),
    ),
];

/// The workflows whose coverage lane leaves doctests to a separate command.
///
/// Each listed workflow must retain that command; otherwise coverage can omit
/// doctests without this contract noticing.
const DOCTEST_WORKFLOWS: &[(&str, &str)] = &[(
    "ci.yml",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/.github/workflows/ci.yml"
    )),
)];

/// Returns the number of leading spaces on a line.
fn indent(line: &str) -> usize { line.len() - line.trim_start().len() }

/// Returns whether a line opens a step: a `- ` list item.
fn opens_step(line: &str) -> bool { line.trim_start().starts_with("- ") }

/// Returns the lines of the step holding the line at `at`: from the step's own
/// list item to the line before the next step, or to the end of the job.
fn step_lines<'a>(lines: &[&'a str], at: usize) -> Vec<&'a str> {
    let here = lines.get(at).copied().unwrap_or_default();
    let starts_the_step = |index: &usize| {
        lines
            .get(*index)
            .is_some_and(|line| opens_step(line) && (*index == at || indent(line) < indent(here)))
    };
    let start = (0..=at).rev().find(starts_the_step).unwrap_or(at);
    let step_indent = lines.get(start).map_or(0, |line| indent(line));
    let end = (at + 1..lines.len())
        .find(|&index| {
            lines.get(index).is_some_and(|line| {
                !line.trim().is_empty()
                    && (indent(line) < step_indent
                        || (opens_step(line) && indent(line) <= step_indent))
            })
        })
        .unwrap_or(lines.len());
    lines.get(start..end).unwrap_or_default().to_vec()
}

/// Returns whether a step passes `install-mold: 'true'` (quoted or bare).
fn installs_mold(step: &[&str]) -> bool {
    step.iter().any(|line| {
        let squeezed: String = line
            .chars()
            .filter(|c| !matches!(c, ' ' | '\'' | '"'))
            .collect();
        squeezed == "install-mold:true"
    })
}

/// Returns whether a step runs the doctests that coverage leaves out.
fn runs_doctests(step: &[&str]) -> bool {
    step.iter().any(|line| {
        let Some(command) = line.trim().strip_prefix("run:") else {
            return false;
        };
        let words: Vec<&str> = command.split_whitespace().collect();
        words.first() == Some(&"cargo")
            && DOCTEST_ARGUMENTS
                .iter()
                .all(|argument| words.contains(argument))
    })
}

/// Returns the Rust flags assigned within one workflow step.
fn rustflags(step: &[&str]) -> Option<Flags> {
    step.iter().find_map(|line| {
        line.trim().strip_prefix("RUSTFLAGS:").map(|value| {
            Flags::from_words(
                value
                    .trim()
                    .trim_matches(|character| matches!(character, '\'' | '"'))
                    .split_whitespace(),
            )
        })
    })
}

/// Returns the complaints about one doctest step's explicit flag boundary.
fn doctest_step_problems(name: &str, at: usize, step: &[&str]) -> Problems {
    let Some(flags) = rustflags(step) else {
        return vec![format!(
            "{name}:{}: the doctest step does not set RUSTFLAGS",
            at + 1
        )];
    };
    let required = [
        (flags.names_warning_deny(), "-D warnings"),
        (flags.names_threads(), "-Zthreads=8"),
        (flags.names_linker(), "-Clink-arg=-fuse-ld=mold"),
    ];
    required
        .into_iter()
        .filter(|(is_present, _)| !is_present)
        .map(|(_, flag)| format!("{name}:{}: the doctest step lacks `{flag}`", at + 1))
        .collect()
}

/// Returns the complaints about a workflow's standalone doctest step.
///
/// A missing step is an error: coverage excludes doctests, so a contract that
/// only judges found steps would let the CI lane lose them silently.
pub fn doctest_problems(name: &str, workflow: &str) -> Problems {
    let lines: Vec<&str> = workflow.lines().collect();
    let doctest_steps: Vec<(usize, Vec<&str>)> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| opens_step(line))
        .map(|(at, _)| (at, step_lines(&lines, at)))
        .filter(|(_, step)| runs_doctests(step))
        .collect();
    if doctest_steps.is_empty() {
        return vec![format!("{name}: the listed workflow has no doctest step")];
    }
    doctest_steps
        .into_iter()
        .flat_map(|(at, step)| doctest_step_problems(name, at, &step))
        .collect()
}

/// Returns the complaint about each `setup-rust` step in one workflow that does
/// not pass `install-mold: 'true'`.
///
/// ```text
/// - uses: org/shared-actions/.github/actions/setup-rust@<sha>
///   with:
///     install-mold: 'true'      -> no complaint
/// ```
pub fn install_mold_problems(name: &str, workflow: &str) -> Problems {
    let lines: Vec<&str> = workflow.lines().collect();
    lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("setup-rust@") && !line.trim_start().starts_with('#'))
        .filter(|(at, _)| !installs_mold(&step_lines(&lines, *at)))
        .map(|(at, _)| {
            format!(
                "{name}:{}: a setup-rust step does not pass `install-mold: 'true'`",
                at + 1
            )
        })
        .collect()
}

/// Returns the complaints about one listed workflow: a step without the input,
/// or no `setup-rust` step at all, which would leave the check reading nothing.
fn listed_problems(name: &str, workflow: &str) -> Problems {
    let mut problems = install_mold_problems(name, workflow);
    if !workflow
        .lines()
        .any(|line| line.contains("setup-rust@") && !line.trim_start().starts_with('#'))
    {
        problems.push(format!(
            "{name}: the listed workflow has no setup-rust step, so the check proves nothing"
        ));
    }
    problems
}

/// Returns every complaint about the listed workflows.
pub fn workflow_problems() -> Problems {
    WORKFLOWS
        .iter()
        .flat_map(|(name, text)| listed_problems(name, text))
        .chain(
            DOCTEST_WORKFLOWS
                .iter()
                .flat_map(|(name, text)| doctest_problems(name, text)),
        )
        .collect()
}
