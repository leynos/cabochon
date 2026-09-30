//! Holds Cabochon's development backend and explicit non-development routes.
//!
//! These tests cover repository-specific Cargo and workflow wiring. They use
//! breaching fixtures so deleting an installer or an exclusion cannot pass over
//! an empty set. The compiler invocation itself is checked separately with a
//! verbose pinned-nightly build.

use std::process::Command;

use rstest::rstest;
use serde_norway::Value;

#[path = "build_backend_contract/ci_installs.rs"]
mod ci_installs;
#[path = "build_backend_contract/pinned_linker.rs"]
mod pinned_linker;
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
    "leynos/shared-actions/.github/actions/setup-rust@6cec89bac47a21cf756d68d638a9a510998e57f8";
const INSTALL_WHITAKER: &str = "leynos/shared-actions/.github/actions/install-whitaker@\
                                6dea5677a84fec60ca51b07202570e3af12ffdb4";
const INSTALL_MDTABLEFIX: &str = "leynos/shared-actions/.github/actions/install-mdtablefix@\
                                  c5a54701c8603a0fa756a6b34c49bc2af75a6c11";
const MARKDOWNLINT_ACTION: &str =
    "DavidAnson/markdownlint-cli2-action@2df9e28eb87988518ef3880c34edad45d65b1668";

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
    assert!(!problems.is_empty());
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
    assert!(!problems.is_empty());
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
    assert!(!problems.is_empty());
}

/// Checks that the cross installer cannot discover development configuration.
fn cross_installer_problems(steps: &[Value]) -> Result<Vec<String>, String> {
    let (_, installer) = named_step(steps, "Install cross")
        .ok_or_else(|| "cross installer step is missing".to_owned())?;
    let install_command = installer
        .get("run")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let installer_requirements = [
        install_command.contains("cd \"$(dirname \"$GITHUB_WORKSPACE\")\""),
        install_command.contains("cargo +stable install cross"),
        install_command.contains(
            "unset CARGO_ENCODED_RUSTFLAGS CARGO_PROFILE_DEV_CODEGEN_BACKEND \
             CARGO_PROFILE_TEST_CODEGEN_BACKEND CARGO_PROFILE_RELEASE_CODEGEN_BACKEND",
        ),
        field(installer, "env", "RUSTFLAGS") == Some(""),
    ];
    let mut problems = Vec::new();
    if installer_requirements.contains(&false) {
        problems.push("cross installation must use the isolated stable route".to_owned());
    }
    Ok(problems)
}

/// Checks one release builder's stable toolchain and explicit config exclusion.
fn release_builder_problems(
    steps: &[Value],
    builder: &str,
    executable: &str,
) -> Result<Vec<String>, String> {
    let step_name = format!("Build release binary ({builder})");
    let (_, build) =
        named_step(steps, &step_name).ok_or_else(|| format!("{step_name} step is missing"))?;
    let command = build.get("run").and_then(Value::as_str).unwrap_or_default();
    let expected = format!(
        "{executable} +stable build --release --manifest-path \"$GITHUB_WORKSPACE/Cargo.toml\" \
         --target ${{{{ matrix.target }}}}"
    );
    let mut problems = Vec::new();
    if !command.contains("cd \"$(dirname \"$GITHUB_WORKSPACE\")\"") || !command.contains(&expected)
    {
        problems.push(format!(
            "stable {builder} must run outside config discovery with an absolute manifest and \
             matrix target"
        ));
    }
    if field(build, "env", "RUSTFLAGS") != Some("") {
        problems.push(format!("stable {builder} must clear development RUSTFLAGS"));
    }
    if field(build, "env", "CARGO_TARGET_DIR") != Some("${{ github.workspace }}/target") {
        problems.push(format!(
            "stable {builder} must write to the checkout target directory"
        ));
    }
    if !command.contains(
        "unset CARGO_ENCODED_RUSTFLAGS CARGO_PROFILE_DEV_CODEGEN_BACKEND \
         CARGO_PROFILE_TEST_CODEGEN_BACKEND CARGO_PROFILE_RELEASE_CODEGEN_BACKEND",
    ) {
        problems.push(format!(
            "stable {builder} must remove inherited encoded flags and backend overrides"
        ));
    }
    Ok(problems)
}

/// Checks that packaged artefacts retain their matrix path and checksum.
fn release_artifact_problems(steps: &[Value]) -> Result<Vec<String>, String> {
    let (_, prepare) = named_step(steps, "Prepare artifact")
        .ok_or_else(|| "release artifact preparation is missing".to_owned())?;
    let prepare_run = prepare
        .get("run")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let mut problems = Vec::new();
    if !prepare_run.contains("mkdir -p artifacts/${{ matrix.os }}-${{ matrix.arch }}")
        || !prepare_run.contains("shasum -a 256")
    {
        problems.push(
            "release artifacts must retain the matrix path and macOS checksum tool".to_owned(),
        );
    }
    let (_, upload) = named_step(steps, "Upload release artifact")
        .ok_or_else(|| "release artifact upload is missing".to_owned())?;
    if field(upload, "with", "path") != Some("artifacts/${{ matrix.os }}-${{ matrix.arch }}") {
        problems.push("release upload must retain the matrix artifact path".to_owned());
    }
    Ok(problems)
}

/// Checks both stable release builders' explicit configuration-discovery route.
fn release_problems(text: &str) -> Result<Vec<String>, String> {
    let workflow: Value = serde_norway::from_str(text).map_err(|error| error.to_string())?;
    let steps = steps(&workflow, "build")?;
    let mut problems = cross_installer_problems(steps)?;
    for (builder, executable) in [("native", "cargo"), ("cross", "cross")] {
        problems.extend(release_builder_problems(steps, builder, executable)?);
    }
    problems.extend(release_artifact_problems(steps)?);
    Ok(problems)
}

/// Scenario: the release keeps stable/cross and isolates the nightly config.
#[test]
fn stable_release_uses_external_working_directory() {
    let found = release_problems(RELEASE).expect("read release workflow");
    assert!(found.is_empty(), "{found:?}");
}

/// Scenario: the release invocation loses one part of the isolation.
#[rstest]
#[case::no_cwd(
    "cd \"$(dirname \"$GITHUB_WORKSPACE\")\"\n          cross +stable",
    "cross +stable"
)]
#[case::installer_uses_unpinned_cargo("cargo +stable install cross", "cargo install cross")]
#[case::no_manifest("--manifest-path \"$GITHUB_WORKSPACE/Cargo.toml\"", "")]
#[case::no_target_dir("CARGO_TARGET_DIR: ${{ github.workspace }}/target", "")]
#[case::encoded_flags_inherited("unset CARGO_ENCODED_RUSTFLAGS", "echo CARGO_ENCODED_RUSTFLAGS")]
#[case::native_cwd(
    "cd \"$(dirname \"$GITHUB_WORKSPACE\")\"\n          cargo +stable",
    "cargo +stable"
)]
#[case::native_manifest(
    "cargo +stable build --release --manifest-path \"$GITHUB_WORKSPACE/Cargo.toml\"",
    "cargo +stable build --release"
)]
#[case::artifact_path("path: artifacts/${{ matrix.os }}-${{ matrix.arch }}", "path: lost")]
#[case::mac_checksum("shasum -a 256", "sha256sum")]
fn release_mutations_are_detected(#[case] old: &str, #[case] new: &str) {
    let mutated = RELEASE.replacen(old, new, 1);
    assert_ne!(mutated, RELEASE);
    let problems = release_problems(&mutated).expect("read mutated release workflow");
    assert!(!problems.is_empty());
}

/// Scenario: evaluated Make release routing preserves the packaging path.
#[test]
fn make_release_uses_external_config_and_local_target_dir() {
    let output = Command::new("make")
        .args(["--dry-run", "--always-make", "release", "CARGO=probe-cargo"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("read evaluated Make release recipe");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let command = String::from_utf8(output.stdout).expect("read Make release command");
    let checkout = env!("CARGO_MANIFEST_DIR");
    assert!(command.contains("cd \""), "{command}");
    assert!(
        command.contains(&format!("CARGO_TARGET_DIR=\"{checkout}/target\"")),
        "{command}"
    );
    assert!(
        command.contains(&format!("--manifest-path \"{checkout}/Cargo.toml\"")),
        "{command}"
    );
    assert!(command.contains("RUSTFLAGS=\"\""), "{command}");
    assert!(command.contains("-u CARGO_ENCODED_RUSTFLAGS"), "{command}");
    assert!(
        command.contains("-u CARGO_PROFILE_RELEASE_CODEGEN_BACKEND"),
        "{command}"
    );
    assert!(command.contains("probe-cargo +stable build"), "{command}");
}
