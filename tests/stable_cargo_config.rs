//! Holds the stable release route outside this repository's development config.
//!
//! `build_backend_contract.rs` checks the configuration and release workflow
//! as text. These probes exercise Cargo's configuration discovery instead. A
//! stable Cargo run at the repository root must refuse the nightly-only
//! development profile. The release route starts outside the checkout and
//! passes an absolute manifest path, so it must build the real release binary.
//! That is the failure that broke catnap's v0.1.0 release.

use std::process::Command;

use camino::Utf8Path;
use rstest::rstest;

/// Stable Cargo's refusal of a configured profile.
const REFUSED: &str = "is not valid";
/// The command whose refusal proves stable Cargo discovers the development config.
const ROOT_PROBE_COMMAND: &str =
    "rustup run stable cargo build --release --offline --bin cabochon from the repository root";
/// The command whose success proves the release route bypasses it.
const RELEASE_PROBE_COMMAND: &str = concat!(
    "rustup run stable cargo build --release --offline --bin cabochon ",
    "--manifest-path <absolute manifest> from the repository parent"
);

/// Makes the stable Cargo command used by both configuration-discovery probes.
fn stable_cargo() -> Command {
    let mut command = Command::new("rustup");
    command
        .args([
            "run",
            "stable",
            "cargo",
            "build",
            "--release",
            "--offline",
            "--bin",
            "cabochon",
        ])
        // Match the release route even when coverage supplies LLVM profile
        // overrides: stable Cargo rejects those nightly-only config keys.
        .env("RUSTFLAGS", "")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_PROFILE_DEV_CODEGEN_BACKEND")
        .env_remove("CARGO_PROFILE_TEST_CODEGEN_BACKEND")
        .env_remove("CARGO_PROFILE_RELEASE_CODEGEN_BACKEND");
    command
}

/// Runs one stable Cargo probe with command-specific startup context.
fn run_probe(command: &mut Command, description: &str) -> Result<std::process::Output, String> {
    command
        .output()
        .map_err(|error| format!("could not start `{description}`: {error}"))
}

/// Judges the diagnostics from the external release-route probe.
///
/// A successful build proves the release route does not discover the nightly
/// development profile. Any failing exit status is reported with its diagnostics.
fn judge_release_probe(stderr: &str, succeeded: bool) -> Result<(), String> {
    if succeeded {
        Ok(())
    } else {
        Err(format!("`{RELEASE_PROBE_COMMAND}` failed:\n{stderr}"))
    }
}

/// Judges the diagnostics from the repository-root discovery probe.
fn judge_root_probe(stderr: &str, succeeded: bool) -> Result<(), String> {
    if succeeded {
        return Err(format!(
            "`{ROOT_PROBE_COMMAND}` unexpectedly succeeded; it must reject the development profile"
        ));
    }
    if stderr.contains(REFUSED) {
        Ok(())
    } else {
        Err(format!(
            "`{ROOT_PROBE_COMMAND}` did not reject the development profile:\n{stderr}"
        ))
    }
}

/// Scenario: the diagnostics stable Cargo prints for the probe in each state.
///
/// Invariant: the release route builds its binary, while the root route rejects
/// the development profile; unrelated output fails both.
#[rstest]
#[case::release_builds("", true, true, false)]
#[case::profile_refused(
    concat!(
        "error: config profile `dev` is not valid (defined in `.cargo/config.toml`)\n\n",
        "Caused by:\n  feature `codegen-backend` is required\n"
    ),
    false,
    false,
    true
)]
#[case::toolchain_missing(
    "error: toolchain 'stable-x86_64-unknown-linux-gnu' is not installed\n",
    false,
    false,
    false
)]
#[case::root_unexpectedly_succeeds("", true, true, false)]
#[case::no_output("", false, false, false)]
fn the_probe_output_is_judged_strictly(
    #[case] stderr: &str,
    #[case] succeeded: bool,
    #[case] release_passes: bool,
    #[case] root_passes: bool,
) {
    assert_eq!(
        judge_release_probe(stderr, succeeded).is_ok(),
        release_passes,
        "release route: {stderr:?}"
    );
    assert_eq!(
        judge_root_probe(stderr, succeeded).is_ok(),
        root_passes,
        "root route: {stderr:?}"
    );
}

/// Scenario: stable Cargo discovers the repository's development configuration.
///
/// Invariant: the nightly-only development profile is refused before Cargo can
/// reach the real release binary.
#[test]
fn stable_cargo_refuses_the_repository_development_configuration() -> Result<(), String> {
    let mut command = stable_cargo();
    let output = run_probe(
        command.current_dir(env!("CARGO_MANIFEST_DIR")),
        ROOT_PROBE_COMMAND,
    )?;
    judge_root_probe(
        &String::from_utf8_lossy(&output.stderr),
        output.status.success(),
    )
    .map_err(|reason| {
        format!(
            "`{ROOT_PROBE_COMMAND}` exited {} while probing the development configuration: \
             {reason}",
            output.status
        )
    })
}

/// Scenario: stable Cargo follows the release workflow from outside the checkout.
///
/// Invariant: the absolute manifest path identifies the package without
/// discovering its local development configuration, so Cargo builds the real
/// release binary.
#[test]
fn stable_cargo_builds_the_release_binary_from_the_repository_parent() -> Result<(), String> {
    let manifest = Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let repository_root = manifest
        .parent()
        .ok_or_else(|| format!("the manifest path `{manifest}` has no repository root"))?
        .to_owned();
    let parent = repository_root.parent().ok_or_else(|| {
        format!("the repository root `{repository_root}` has no parent directory")
    })?;
    let target_dir = repository_root.join("target");
    let mut command = stable_cargo();
    let output = run_probe(
        command
            .current_dir(parent)
            .arg("--manifest-path")
            .arg(manifest.as_std_path())
            .env("CARGO_TARGET_DIR", target_dir.as_str()),
        RELEASE_PROBE_COMMAND,
    )?;
    judge_release_probe(
        &String::from_utf8_lossy(&output.stderr),
        output.status.success(),
    )
    .map_err(|reason| {
        format!(
            "`{RELEASE_PROBE_COMMAND}` exited {} while probing the release route: {reason}",
            output.status
        )
    })
}
