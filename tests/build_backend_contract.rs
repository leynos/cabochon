//! Holds Cabochon's development backend and explicit non-development routes.
//!
//! These tests cover repository-specific Cargo and workflow wiring. They use
//! breaching fixtures so deleting an installer or an exclusion cannot pass over
//! an empty set. The compiler invocation itself is checked separately with a
//! verbose pinned-nightly build.

use rstest::rstest;
use serde_norway::Value;

#[path = "build_backend_contract/ci_installs.rs"]
mod ci_installs;
#[path = "build_backend_contract/pinned_linker.rs"]
mod pinned_linker;
#[path = "build_backend_contract/release.rs"]
mod release;
#[path = "build_backend_contract/suite_jobs.rs"]
mod suite_jobs;

const CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/.cargo/config.toml"));
const CI: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/.github/workflows/ci.yml"
));
const PUBLISHER: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/.github/workflows/coverage-main.yml"
));
const RELEASE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/.github/workflows/release.yml"
));

const SETUP_RUST: &str =
    "leynos/shared-actions/.github/actions/setup-rust@b804b69fa7f978cf9091b9d9bd5481d8ce58c2ea";
const INSTALL_WHITAKER: &str = "leynos/shared-actions/.github/actions/install-whitaker@\
                                5a99aed78ce68dcebd3052b75c31e6f173162b1e";
const INSTALL_MDTABLEFIX: &str = "leynos/shared-actions/.github/actions/install-mdtablefix@\
                                  0606ca899ce2995a0c4bc78e660e80832e6e8855";
const MARKDOWNLINT_ACTION: &str =
    "DavidAnson/markdownlint-cli2-action@2df9e28eb87988518ef3880c34edad45d65b1668";
const SETUP_UV: &str = "astral-sh/setup-uv@12d13f90bc3a5a1971bebad4beb09a4dfa962e91";

/// Returns a YAML job's steps, rejecting absent or empty lists.
fn steps<'a>(workflow: &'a Value, job: &str) -> Result<&'a [Value], String> {
    workflow
        .get("jobs")
        .and_then(|jobs| jobs.get(job))
        .and_then(|job_value| job_value.get("steps"))
        .and_then(Value::as_sequence)
        .filter(|steps| !steps.is_empty())
        .map(Vec::as_slice)
        .ok_or_else(|| format!("{job} has no readable steps"))
}

/// Returns the first step using a pinned action prefix.
fn action_step<'a>(steps: &'a [Value], action: &str) -> Option<(usize, &'a Value)> {
    steps.iter().enumerate().find(|(_, step)| {
        step.get("uses")
            .and_then(Value::as_str)
            .is_some_and(|uses| uses == action)
    })
}

/// Returns the first named step.
fn named_step<'a>(steps: &'a [Value], name: &str) -> Option<(usize, &'a Value)> {
    steps
        .iter()
        .enumerate()
        .find(|(_, step)| step.get("name").and_then(Value::as_str) == Some(name))
}

/// Returns one environment or action-input value as a string.
fn field<'a>(step: &'a Value, table: &str, key: &str) -> Option<&'a str> {
    step.get(table)?.get(key)?.as_str()
}

/// Checks that a step cannot be skipped or softened.
fn is_binding(step: &Value) -> bool {
    step.get("if").is_none() && step.get("continue-on-error").is_none()
}

/// Checks that a coverage action selects LLVM and excludes development flags.
fn coverage_route(step: &Value) -> Vec<String> {
    let mut problems = Vec::new();
    for key in ["DEV", "TEST"] {
        let env_key = format!("CARGO_PROFILE_{key}_CODEGEN_BACKEND");
        if field(step, "env", &env_key) != Some("llvm") {
            problems.push(format!("{env_key} must select llvm"));
        }
    }
    let flags = field(step, "env", "RUSTFLAGS").unwrap_or_default();
    let has_development_flag = ["-Zthreads", "-fuse-ld=mold", "cranelift"]
        .into_iter()
        .any(|flag| flags.contains(flag));
    if !flags.contains("-fuse-ld=lld") || has_development_flag {
        problems.push("coverage RUSTFLAGS must select lld without development flags".to_owned());
    }
    if field(step, "env", "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER") != Some("clang") {
        problems.push("coverage must drive lld through clang".to_owned());
    }
    problems
}

/// Reports gaps in one coverage workflow's suite job.
fn coverage_problems(text: &str, job: &str) -> Result<Vec<String>, String> {
    let workflow: Value = serde_norway::from_str(text).map_err(|error| error.to_string())?;
    let steps = steps(&workflow, job)?;
    let coverage = steps
        .iter()
        .enumerate()
        .filter(|(_, step)| {
            step.get("uses")
                .and_then(Value::as_str)
                .is_some_and(|uses| uses.contains("/generate-coverage@"))
        })
        .collect::<Vec<_>>();
    if coverage.len() != 1 {
        return Err(format!("{job} must run exactly one coverage action"));
    }
    let Some(&(coverage_at, action)) = coverage.first() else {
        return Err(format!("{job} has no coverage action"));
    };
    let mut problems = coverage_route(action);
    let isolation = named_step(steps, "Check coverage flag isolation");
    if !isolation.is_some_and(|(at, step)| {
        at < coverage_at
            && is_binding(step)
            && step.get("run").and_then(Value::as_str).is_some_and(|run| {
                run.contains("[[ -v CARGO_ENCODED_RUSTFLAGS ]]") && run.contains("exit 1")
            })
    }) {
        problems.push("coverage must reject inherited encoded Rust flags before use".to_owned());
    }
    Ok(problems)
}

/// Scenario: the two live coverage lanes use the same excluded backend.
#[rstest]
#[case::pull_request(CI, "build-test")]
#[case::publisher(PUBLISHER, "coverage-upload")]
fn coverage_uses_llvm_in_both_lanes(#[case] workflow: &str, #[case] job: &str) {
    let found = coverage_problems(workflow, job).expect("read coverage workflow");
    assert!(found.is_empty(), "{job}: {found:?}");
}

/// Scenario: a coverage lane drops one exclusion or inherits a dev flag.
///
/// Invariant: each mutation is detected at the actual action step.
#[rstest]
#[case::missing_dev("CARGO_PROFILE_DEV_CODEGEN_BACKEND: llvm", "")]
#[case::missing_test("CARGO_PROFILE_TEST_CODEGEN_BACKEND: llvm", "")]
#[case::leaked_cranelift(
    "          RUSTFLAGS: -C link-arg=-fuse-ld=lld",
    "          RUSTFLAGS: -Zthreads=8 -Clink-arg=-fuse-ld=mold"
)]
#[case::encoded_guard_removed("[[ -v CARGO_ENCODED_RUSTFLAGS ]]", "[[ false ]]")]
fn coverage_mutations_are_detected(#[case] old: &str, #[case] new: &str) {
    let mutated = CI.replacen(old, new, 1);
    assert_ne!(mutated, CI, "fixture did not mutate the workflow");
    let problems = coverage_problems(&mutated, "build-test").expect("read mutated coverage");
    assert_ne!(
        problems,
        [] as [String; 0],
        "coverage mutation must be detected"
    );
}

/// Scenario: the publisher cannot silently fall back to development routing.
#[rstest]
#[case::llvm_removed("CARGO_PROFILE_DEV_CODEGEN_BACKEND: llvm", "")]
#[case::encoded_guard_removed("[[ -v CARGO_ENCODED_RUSTFLAGS ]]", "[[ false ]]")]
fn publisher_exclusion_mutations_are_detected(#[case] old: &str, #[case] new: &str) {
    let mutated = PUBLISHER.replacen(old, new, 1);
    assert_ne!(mutated, PUBLISHER, "fixture did not mutate the publisher");
    let problems =
        coverage_problems(&mutated, "coverage-upload").expect("read mutated publisher workflow");
    assert_ne!(
        problems,
        [] as [String; 0],
        "publisher mutation must be detected"
    );
}

/// Checks the default backend in Cargo's parsed configuration.
fn default_backend_problems(config: &str) -> Result<Vec<String>, String> {
    let parsed: toml::Value = toml::from_str(config).map_err(|error| error.to_string())?;
    let mut problems = Vec::new();
    if parsed
        .get("unstable")
        .and_then(|table| table.get("codegen-backend"))
        .and_then(toml::Value::as_bool)
        != Some(true)
    {
        problems.push("unstable.codegen-backend must be enabled".to_owned());
    }
    if parsed
        .get("profile")
        .and_then(|table| table.get("dev"))
        .and_then(|table| table.get("codegen-backend"))
        .and_then(toml::Value::as_str)
        != Some("cranelift")
    {
        problems.push("profile.dev must select cranelift".to_owned());
    }
    Ok(problems)
}

/// Scenario: Cargo auto-discovers this file for development commands.
#[test]
fn cargo_configuration_selects_cranelift() {
    let found = default_backend_problems(CONFIG).expect("read Cargo configuration");
    assert!(found.is_empty(), "{found:?}");
}

/// Scenario: a default key is removed or changed.
#[rstest]
#[case::backend_removed(CONFIG.replace("codegen-backend = \"cranelift\"", ""))]
#[case::feature_removed(CONFIG.replace("codegen-backend = true", ""))]
fn missing_cranelift_default_is_detected(#[case] config: String) {
    let problems = default_backend_problems(&config).expect("read mutated Cargo configuration");
    assert_ne!(
        problems,
        [] as [String; 0],
        "missing default must be detected"
    );
}

/// Scenario: a backend-looking string in another TOML key is not a setting.
#[test]
fn backend_text_in_an_environment_value_is_not_a_default() {
    let config = concat!(
        "[unstable]\ncodegen-backend = true\n",
        "[profile.dev]\ncodegen-backend = \"cranelift\"\n",
        "[env]\nBACKEND_HINT = \"codegen-backend=cranelift\"\n"
    );
    let problems = default_backend_problems(config).expect("read backend value fixture");
    assert!(problems.is_empty(), "{problems:?}");
}
