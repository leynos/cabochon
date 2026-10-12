//! Checks stable release builders, packaged artefacts, and config isolation.

use std::process::Command;

use rstest::rstest;
use serde_norway::Value;

use super::{RELEASE, field, named_step, steps};

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
fn release_artefact_problems(steps: &[Value]) -> Result<Vec<String>, String> {
    let (_, prepare) = named_step(steps, "Prepare artefact")
        .ok_or_else(|| "release artefact preparation is missing".to_owned())?;
    let prepare_run = prepare
        .get("run")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let mut problems = Vec::new();
    if !prepare_run.contains("mkdir -p artefacts/${{ matrix.os }}-${{ matrix.arch }}")
        || !prepare_run.contains("shasum -a 256")
    {
        problems.push(
            "release artefacts must retain the matrix path and macOS checksum tool".to_owned(),
        );
    }
    let (_, upload) = named_step(steps, "Upload release artefact")
        .ok_or_else(|| "release artefact upload is missing".to_owned())?;
    if field(upload, "with", "path") != Some("artefacts/${{ matrix.os }}-${{ matrix.arch }}") {
        problems.push("release upload must retain the matrix artefact path".to_owned());
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
    problems.extend(release_artefact_problems(steps)?);
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
#[case::cross_build_toolchain_comes_from_command(
    "cross +stable build --release",
    "cross +nightly build --release"
)]
#[case::native_manifest(
    "cargo +stable build --release --manifest-path \"$GITHUB_WORKSPACE/Cargo.toml\"",
    "cargo +stable build --release"
)]
#[case::artefact_path("path: artefacts/${{ matrix.os }}-${{ matrix.arch }}", "path: lost")]
#[case::mac_checksum("shasum -a 256", "sha256sum")]
fn release_mutations_are_detected(#[case] old: &str, #[case] new: &str) {
    let mutated = RELEASE.replacen(old, new, 1);
    assert_ne!(mutated, RELEASE);
    let problems = release_problems(&mutated).expect("read mutated release workflow");
    assert_ne!(
        problems,
        [] as [String; 0],
        "release mutation must be detected"
    );
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
