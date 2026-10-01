//! Checks that every reachable Linux suite job installs `mold` before running.

use rstest::rstest;
use serde_norway::Value;

use super::{CI, SETUP_RUST, action_step, field, is_binding};

/// Resolves a matrix runner label from literal values declared by the job.
fn resolve_label(label: &str, job: &Value) -> Result<Vec<String>, String> {
    let Some((prefix, expression)) = label.split_once("${{ matrix.") else {
        return Ok(vec![label.to_owned()]);
    };
    let (key, suffix) = expression
        .split_once(" }}")
        .ok_or_else(|| format!("unreadable matrix runner label: {label}"))?;
    let matrix = job
        .get("strategy")
        .and_then(|strategy| strategy.get("matrix"))
        .ok_or_else(|| format!("no matrix for runner label: {label}"))?;
    let direct = matrix
        .get(key)
        .and_then(Value::as_sequence)
        .map(|items| items.iter().filter_map(Value::as_str).collect::<Vec<_>>());
    let included = matrix
        .get("include")
        .and_then(Value::as_sequence)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.get(key).and_then(Value::as_str))
                .collect::<Vec<_>>()
        });
    if direct.is_none() && included.is_none() {
        return Err(format!("matrix {key} has no values"));
    }
    let values: Vec<&str> = direct
        .into_iter()
        .flatten()
        .chain(included.into_iter().flatten())
        .collect();
    if values.is_empty() {
        return Err(format!("matrix {key} has no readable values"));
    }
    values
        .into_iter()
        .map(|value| {
            let resolved = format!("{prefix}{value}{suffix}");
            if resolved.contains("${{") {
                Err(format!("unresolved runner label: {resolved}"))
            } else {
                Ok(resolved)
            }
        })
        .collect()
}

/// Returns whether a runner label denotes Linux, rejecting opaque suite jobs.
fn linux_runner(job: &Value, value: &Value) -> Result<bool, String> {
    let labels: Vec<&str> = match value {
        Value::String(label) => vec![label],
        Value::Sequence(items) => items.iter().filter_map(Value::as_str).collect(),
        Value::Mapping(fields) => fields
            .values()
            .flat_map(|item| match item {
                Value::String(label) => vec![label.as_str()],
                Value::Sequence(items) => items.iter().filter_map(Value::as_str).collect(),
                _ => Vec::new(),
            })
            .collect(),
        _ => Vec::new(),
    };
    let resolved = labels
        .into_iter()
        .map(|label| resolve_label(label, job))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    if resolved
        .iter()
        .any(|label| label.contains("ubuntu") || label.contains("linux"))
    {
        Ok(true)
    } else if resolved
        .iter()
        .any(|label| label.contains("windows") || label.contains("macos"))
    {
        Ok(false)
    } else {
        Err(format!("cannot resolve suite runner: {value:?}"))
    }
}

/// Finds every suite job, including one added after this contract was written.
///
/// # Errors
///
/// Returns a parse or reachability error if the workflow cannot be read.
pub fn suite_job_problems(text: &str) -> Result<Vec<String>, String> {
    let workflow: Value = serde_norway::from_str(text).map_err(|error| error.to_string())?;
    let jobs = workflow
        .get("jobs")
        .and_then(Value::as_mapping)
        .ok_or_else(|| "workflow has no jobs".to_owned())?;
    let mut problems = Vec::new();
    let mut suite_count = 0;
    for (name, job) in jobs {
        let Some(steps) = job.get("steps").and_then(Value::as_sequence) else {
            continue;
        };
        let suite_position = steps.iter().position(|step| {
            step.get("uses")
                .and_then(Value::as_str)
                .is_some_and(|uses| uses.contains("/generate-coverage@"))
                || step.get("run").and_then(Value::as_str).is_some_and(|run| {
                    run.contains("cargo test")
                        || run.contains("cargo nextest")
                        || run.contains("make test")
                        || run.contains("make all")
                })
        });
        let Some(suite_at) = suite_position else {
            continue;
        };
        suite_count += 1;
        let job_name = name.as_str().unwrap_or("<unknown>");
        let runner = job
            .get("runs-on")
            .ok_or_else(|| format!("{job_name} has no runner"))?;
        match linux_runner(job, runner) {
            Ok(false) => (),
            Err(error) => problems.push(format!("{job_name}: {error}")),
            Ok(true) => {
                let installer = action_step(steps, SETUP_RUST);
                if !installer.is_some_and(|(at, step)| {
                    at < suite_at
                        && is_binding(step)
                        && field(step, "with", "install-mold") == Some("true")
                }) {
                    problems.push(format!(
                        "{job_name}: pinned `mold` setup must precede the suite"
                    ));
                }
            }
        }
    }
    if suite_count == 0 {
        problems.push("workflow has no readable suite job".to_owned());
    }
    Ok(problems)
}

/// Scenario: a new Linux suite job in each supported runner form lacks setup.
#[rstest]
#[case::scalar("ubuntu-latest", "cargo nextest run")]
#[case::list("[self-hosted, linux]", "cargo nextest run")]
#[case::group("{ group: linux-runners }", "cargo nextest run")]
#[case::make_all("ubuntu-latest", "make all")]
fn newly_reachable_linux_suite_jobs_need_mold(#[case] runner: &str, #[case] suite_command: &str) {
    let mutated = format!(
        "{CI}\n  extra-suite:\n    runs-on: {runner}\n    steps:\n      - run: {suite_command}\n"
    );
    let problems = suite_job_problems(&mutated).expect("read mutated suite workflow");
    assert!(
        problems
            .iter()
            .any(|problem| problem.contains("extra-suite: pinned `mold` setup")),
        "{problems:?}"
    );
}

/// Scenario: a suite job reaches Linux through a matrix expression.
#[rstest]
#[case::direct("{ os: [ubuntu-latest, windows-latest] }")]
#[case::included("{ include: [{ os: ubuntu-latest }, { os: windows-latest }] }")]
#[case::combined("{ os: [windows-latest], include: [{ os: ubuntu-latest }] }")]
fn matrix_linux_suite_jobs_need_mold(#[case] matrix: &str) {
    let mutated = format!(
        "{CI}\n  matrix-suite:\n    strategy:\n      matrix: {matrix}\n    runs-on: ${{{{ \
         matrix.os }}}}\n    steps:\n      - run: cargo test\n"
    );
    let problems = suite_job_problems(&mutated).expect("read matrix suite workflow");
    assert!(
        problems
            .iter()
            .any(|problem| problem.contains("matrix-suite: pinned `mold` setup")),
        "{problems:?}"
    );
}
