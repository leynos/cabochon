//! Hermetic contracts for the pinned build-tool installer and its Make route.

#![cfg(target_os = "linux")]

use std::io;

#[path = "build_tools_installer_contract/fixture.rs"]
mod fixture;

use fixture::{InstallerFixture, LINKER_VERSION};
use rstest::rstest;

fn assert_success(output: &std::process::Output, context: &str) {
    assert!(
        output.status.success(),
        "{context}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_failure(output: &std::process::Output, context: &str) {
    assert!(!output.status.success(), "{context}");
}

fn assert_stderr_contains(output: &std::process::Output, expected: &str) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(expected), "{stderr}");
}

fn assert_contains(actual: &str, expected: &str) {
    assert!(
        actual.contains(expected),
        "{actual} does not contain {expected}"
    );
}

fn assert_excludes(actual: &str, unexpected: &str) {
    assert!(
        !actual.contains(unexpected),
        "{actual} unexpectedly contains {unexpected}"
    );
}

fn assert_empty(actual: &str) {
    assert!(actual.is_empty(), "{actual}");
}

fn assert_text_eq(actual: &str, expected: &str) {
    assert_eq!(actual, expected, "installer route log differs");
}

/// Scenario: each supported Linux architecture selects and verifies its archive.
#[rstest]
#[case::x86_64("x86_64")]
#[case::aarch64("aarch64")]
fn installer_checks_local_fixture_for_supported_architectures(
    #[case] arch: &str,
) -> io::Result<()> {
    let fixture = InstallerFixture::new()?;
    let output = fixture.run_installer(&[("FAKE_UNAME_MACHINE", arch)])?;
    assert_success(
        &output,
        "installer should support configured Linux architectures",
    );
    let archive = format!("mold-{}-{arch}-linux.tar.gz", LINKER_VERSION.trim());
    let log = fixture.log()?;
    assert_contains(
        &log,
        &format!(
            "https://fixture.invalid/releases/v{}/{archive}",
            LINKER_VERSION.trim()
        ),
    );
    assert_contains(&log, "sha256sum --check --status");
    assert_contains(&log, "tar --extract --gzip --strip-components=1");
    assert_contains(&log, "--file ");
    assert_contains(&log, &format!("--directory {}", fixture.prefix()));
    assert_contains(&log, "rustup toolchain install nightly-");
    assert_contains(&log, "--profile minimal");
    assert_contains(&log, "--component rustfmt");
    assert_contains(&log, "--component rustc-codegen-cranelift-preview");
    Ok(())
}

/// Scenario: an unsupported architecture fails before download or installation.
#[test]
fn installer_rejects_unsupported_architecture() -> io::Result<()> {
    let fixture = InstallerFixture::new()?;
    let output = fixture.run_installer(&[("FAKE_UNAME_MACHINE", "riscv64")])?;
    assert_failure(&output, "unsupported architecture passed");
    assert_stderr_contains(&output, "No pinned `mold` binary");
    let log = fixture.log()?;
    assert_empty(&log);
    Ok(())
}

/// Scenario: invalid pins or a missing checksum stop before invoking tools.
#[rstest]
#[case::toolchain("rust-toolchain.toml", "[toolchain]\nchannel = \"stable\"\n")]
#[case::mold_version("tools/mold/VERSION", "latest\n")]
#[case::checksum("tools/mold/SHA256SUMS", "no matching archive\n")]
fn installer_rejects_invalid_pins_and_checksums(
    #[case] file: &str,
    #[case] value: &str,
) -> io::Result<()> {
    let fixture = InstallerFixture::new()?;
    fixture.write_file(file, value)?;
    let output = fixture.run_installer(&[])?;
    assert_failure(&output, &format!("invalid {file} passed"));
    let log = fixture.log()?;
    assert_empty(&log);
    Ok(())
}

/// Scenario: a well-formed but incorrect digest prevents archive extraction.
#[test]
fn installer_rejects_checksum_mismatch_before_extraction() -> io::Result<()> {
    let fixture = InstallerFixture::new()?;
    let archive = format!("mold-{}-x86_64-linux.tar.gz", LINKER_VERSION.trim());
    fixture.write_file(
        "tools/mold/SHA256SUMS",
        &format!("{}  {archive}\n", "0".repeat(64)),
    )?;
    let output = fixture.run_installer(&[])?;
    assert_failure(&output, "checksum mismatch was accepted");
    assert_stderr_contains(&output, "Checksum mismatch");
    let log = fixture.log()?;
    assert_contains(&log, "curl ");
    assert_contains(&log, "sha256sum ");
    assert_excludes(&log, "tar ");
    assert_excludes(&log, "rustup ");
    Ok(())
}

/// Scenario: download, checksum, extraction and rustup failures propagate.
#[rstest]
#[case::download("FAKE_CURL_FAIL")]
#[case::checksum("FAKE_SHA256SUM_FAIL")]
#[case::extract("FAKE_TAR_FAIL")]
#[case::rustup("FAKE_RUSTUP_FAIL")]
fn installer_propagates_tool_failures(#[case] failure: &str) -> io::Result<()> {
    let fixture = InstallerFixture::new()?;
    let output = fixture.run_installer(&[(failure, "yes")])?;
    assert_failure(&output, &format!("{failure} was ignored"));
    let log = fixture.log()?;
    assert_contains(&log, "curl ");
    if failure != "FAKE_CURL_FAIL" {
        assert_contains(&log, "sha256sum ");
    }
    if failure == "FAKE_RUSTUP_FAIL" {
        assert_contains(&log, "tar ");
        assert_contains(&log, "rustup ");
    } else {
        assert_excludes(&log, "rustup ");
    }
    Ok(())
}

/// Scenario: the Linux installer fails closed when rustup is unavailable.
#[test]
fn installer_rejects_missing_rustup() -> io::Result<()> {
    let fixture = InstallerFixture::new()?;
    fixture.remove_rustup()?;
    let output = fixture.run_installer(&[])?;
    assert_failure(&output, "installer passed without rustup");
    assert_stderr_contains(&output, "rustup is required");
    let log = fixture.log()?;
    assert_contains(&log, "tar ");
    assert_excludes(&log, "rustup ");
    Ok(())
}

/// Scenario: non-Linux installation skips the linker download but installs Rust.
#[test]
fn installer_skips_linux_assets_on_other_hosts() -> io::Result<()> {
    let fixture = InstallerFixture::new()?;
    let output = fixture.run_installer(&[("FAKE_UNAME_SYSTEM", "Darwin")])?;
    assert_success(&output, "non-Linux installer route should succeed");
    let log = fixture.log()?;
    assert_excludes(&log, "curl ");
    assert_excludes(&log, "tar ");
    assert_contains(&log, "rustup toolchain install nightly-");
    Ok(())
}

/// Scenario: Make runs installation before its prerequisite check.
#[test]
fn make_installer_runs_install_then_check_and_propagates_failure() -> io::Result<()> {
    let fixture = InstallerFixture::new()?;
    fixture.make_route_probes()?;
    let success = fixture.run_make_route(&[])?;
    assert_success(&success, "Make installer route should succeed");
    assert_text_eq(&fixture.make_route_log()?, "install\ncheck\n");

    fixture.clear_make_route_log()?;
    let install_failure = fixture.run_make_route(&[("MAKE_PROBE_INSTALL_STATUS", "7")])?;
    assert_failure(&install_failure, "install failure passed");
    assert_text_eq(&fixture.make_route_log()?, "install\n");

    fixture.clear_make_route_log()?;
    let check_failure = fixture.run_make_route(&[("MAKE_PROBE_CHECK_STATUS", "8")])?;
    assert_failure(&check_failure, "check failure passed");
    assert_text_eq(&fixture.make_route_log()?, "install\ncheck\n");
    Ok(())
}
