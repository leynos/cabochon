//! Exercises the Makefile's build preflight and sequential lint failure paths.
//!
//! Controlled executables stand in for the tool check, Cargo, and Whitaker;
//! these tests do not compile the crate or depend on locally installed tools.

use std::process::Command;

use rstest::rstest;

/// Runs Make from the checkout while replacing compilers with harmless probes.
///
/// # Errors
///
/// Returns an operating-system error if Make cannot be started.
fn make(args: &[&str]) -> Result<std::process::Output, std::io::Error> {
    Command::new("make")
        .args(args)
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_PROFILE_DEV_CODEGEN_BACKEND")
        .env_remove("CARGO_PROFILE_TEST_CODEGEN_BACKEND")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
}

/// Scenario: retired opt-in development targets cannot stand in for the suite.
#[rstest]
#[case::build("dev-build")]
#[case::run_tests("dev-test")]
fn retired_development_targets_are_absent(#[case] target: &str) {
    let output = make(&["--dry-run", target, "CARGO=echo"])
        .expect("ask Make for a retired development target");
    assert!(!output.status.success(), "{target} was still reachable");
    let stderr = String::from_utf8(output.stderr).expect("read Make error");
    assert!(stderr.contains("No rule to make target"), "{stderr}");
}

/// Scenario: each development target's preflight fails.
///
/// Invariant: a failed capability check stops before any Cargo invocation,
/// including under Make's forced rebuild mode.
#[rstest]
#[case::build("build")]
#[case::run_tests("test")]
#[case::lint("lint")]
#[case::typecheck("typecheck")]
fn a_failed_preflight_stops_the_target(#[case] target: &str) {
    let output = make(&[
        "--silent",
        "--always-make",
        target,
        "CHECK_BUILD_TOOLS=false",
        "CARGO=echo",
    ])
    .expect("run Make with a failed preflight");
    let stdout = String::from_utf8(output.stdout).expect("read Make output");
    assert!(
        !output.status.success(),
        "{target} ignored its failed preflight"
    );
    assert!(
        stdout.trim().is_empty(),
        "{target} invoked Cargo after the preflight failed: {stdout}"
    );
}

/// Scenario: inherited Cargo overrides would replace development defaults.
#[rstest]
#[case::encoded("CARGO_ENCODED_RUSTFLAGS", "-Dwarnings")]
#[case::dev_backend("CARGO_PROFILE_DEV_CODEGEN_BACKEND", "llvm")]
#[case::test_backend("CARGO_PROFILE_TEST_CODEGEN_BACKEND", "llvm")]
fn inherited_backend_override_fails_before_cargo(#[case] key: &str, #[case] value: &str) {
    let output = Command::new("make")
        .args(["--silent", "test", "CARGO=echo"])
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_PROFILE_DEV_CODEGEN_BACKEND")
        .env_remove("CARGO_PROFILE_TEST_CODEGEN_BACKEND")
        .env(key, value)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run Make with an inherited Cargo override");
    assert!(!output.status.success(), "{key} was ignored");
    let stderr = String::from_utf8(output.stderr).expect("read preflight error");
    assert!(stderr.contains(key), "{stderr}");
    let stdout = String::from_utf8(output.stdout).expect("read Make output");
    assert!(
        stdout.trim().is_empty(),
        "Cargo ran after failed preflight: {stdout}"
    );
}

/// Scenario: the evaluated lint recipe retains the docsrs rustdoc contract.
#[test]
fn rustdoc_lint_uses_docsrs_and_deny_warnings() {
    let output = make(&["--dry-run", "lint-clippy", "CARGO=probe-cargo"])
        .expect("read evaluated rustdoc lint recipe");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("read Make lint commands");
    assert!(
        stdout.contains("RUSTDOCFLAGS=\"--cfg docsrs -D warnings"),
        "{stdout}"
    );
    assert!(
        stdout.contains("probe-cargo doc --workspace --no-deps"),
        "{stdout}"
    );
}

/// Scenario: Whitaker reports a failing lint after rustdoc and Clippy pass.
///
/// Invariant: the composite lint target propagates that failure.
#[test]
fn whitaker_failure_propagates_through_lint() {
    let output = make(&[
        "--silent",
        "lint",
        "CHECK_BUILD_TOOLS=true",
        "CARGO=echo",
        "WHITAKER=false",
    ])
    .expect("run lint with a failing Whitaker probe");
    let stdout = String::from_utf8(output.stdout).expect("read Make output");
    assert!(stdout.contains("doc --workspace --no-deps"), "{stdout}");
    assert!(stdout.contains("clippy"), "{stdout}");
    assert!(!output.status.success(), "lint ignored Whitaker's failure");
}

/// Scenario: the evaluated Whitaker command promotes suite warnings to errors.
///
/// Invariant: Dylint receives its own rustc flags while the repository's
/// development backend and linker flags remain excluded.
#[test]
fn whitaker_denies_warnings_without_development_flags() {
    let output = make(&["--dry-run", "lint-whitaker", "WHITAKER=probe-whitaker"])
        .expect("read evaluated Whitaker command");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let command = String::from_utf8(output.stdout).expect("read Whitaker command");
    assert!(
        command.contains("DYLINT_RUSTFLAGS=\"-D warnings\""),
        "{command}"
    );
    assert!(command.contains("RUSTFLAGS=\"\""), "{command}");
    assert!(!command.contains("-Zthreads=8"), "{command}");
    assert!(!command.contains("-fuse-ld=mold"), "{command}");
}

/// Scenario: Make is asked to run all gates with parallel jobs.
///
/// Invariant: the composite recipe still reaches formatting before lint and
/// lint before the test suite.
#[test]
fn all_orders_its_gates_even_with_parallel_make() {
    let output = make(&[
        "--dry-run",
        "--jobs=3",
        "all",
        "CHECK_BUILD_TOOLS=true",
        "CARGO=echo",
        "MDTABLEFIX=echo",
        "MDLINT=echo",
        "WHITAKER=echo",
    ])
    .expect("run Make dry-run with parallel jobs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("read Make output");
    let positions = ["check-fmt", "markdownlint", "lint", "test"]
        .map(|target| stdout.find(&format!("make {target}")));
    let [
        Some(format_at),
        Some(markdown_at),
        Some(lint_at),
        Some(test_at),
    ] = positions
    else {
        panic!("did not read all four gates in {stdout}");
    };
    assert!(
        format_at < markdown_at && markdown_at < lint_at && lint_at < test_at,
        "{stdout}"
    );
}
