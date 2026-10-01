//! Exercises the Makefile's build preflight and sequential lint failure paths.
//!
//! Controlled executables stand in for the tool check, Cargo, and Whitaker;
//! these tests do not compile the crate or depend on locally installed tools.

use std::process::Command;

use camino::Utf8Path;
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

/// Scenario: the old target names select the same default development route.
#[rstest]
#[case::build("dev-build", "build")]
#[case::run_tests("dev-test", "test")]
fn development_alias_uses_the_standard_route(#[case] alias: &str, #[case] standard: &str) {
    let alias_output = make(&["--dry-run", "--always-make", alias, "CARGO=probe-cargo"])
        .expect("read alias route");
    let standard_output = make(&["--dry-run", "--always-make", standard, "CARGO=probe-cargo"])
        .expect("read standard route");
    for (target, output) in [(alias, &alias_output), (standard, &standard_output)] {
        assert!(
            output.status.success(),
            "{target}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert_eq!(
        alias_output.stdout, standard_output.stdout,
        "{alias} differs from {standard}"
    );
    let stdout = String::from_utf8(alias_output.stdout).expect("read alias commands");
    assert!(stdout.contains("probe-cargo "), "{stdout}");
}

/// Scenario: each development target's preflight fails.
///
/// Invariant: a failed capability check stops before any Cargo invocation,
/// including under Make's forced rebuild mode.
#[rstest]
#[case::build("build")]
#[case::run_tests("test")]
#[case::dev_build("dev-build")]
#[case::dev_test("dev-test")]
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

/// Scenario: Whitaker's temporary driver must not inherit Cargo's nightly
/// development profile setting while it builds under its own toolchain.
#[test]
fn whitaker_runs_outside_the_development_configuration() {
    let output = make(&["--dry-run", "lint-whitaker", "WHITAKER=probe-whitaker"])
        .expect("read evaluated Whitaker route");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("read Whitaker route");
    let command = stdout
        .lines()
        .find(|line| line.contains("probe-whitaker "))
        .expect("dry run includes a Whitaker recipe");
    let parent = Utf8Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest directory has a parent");
    assert!(
        command.starts_with(&format!("cd \"{parent}/\" && env ")),
        "{command}"
    );
    assert!(
        command.contains(&format!(
            "--manifest-path \"{}/Cargo.toml\" --all --",
            env!("CARGO_MANIFEST_DIR")
        )),
        "{command}"
    );
    for key in [
        "CARGO_ENCODED_RUSTFLAGS",
        "CARGO_PROFILE_DEV_CODEGEN_BACKEND",
        "CARGO_PROFILE_TEST_CODEGEN_BACKEND",
        "CARGO_PROFILE_RELEASE_CODEGEN_BACKEND",
    ] {
        assert!(command.contains(&format!("-u {key}")), "{command}");
        assert!(!command.contains(&format!("{key}=llvm")), "{command}");
    }
}

/// Scenario: Make is asked to run all gates with parallel jobs.
///
/// Invariant: the composite recipe keeps every gate in order.
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
    let positions = [
        "check-fmt",
        "markdownlint",
        "spelling",
        "lint",
        "test",
        "test-workflow-contracts",
    ]
    .map(|target| stdout.find(&format!("make {target}")));
    let [
        Some(format_at),
        Some(markdown_at),
        Some(spelling_at),
        Some(lint_at),
        Some(test_at),
        Some(contracts_at),
    ] = positions
    else {
        panic!("did not read all six gates in {stdout}");
    };
    assert!(
        format_at < markdown_at
            && markdown_at < spelling_at
            && spelling_at < lint_at
            && lint_at < test_at
            && test_at < contracts_at,
        "{stdout}"
    );
}

/// Scenario: spelling uses the pinned builder gate and propagates failure.
#[test]
fn spelling_gate_is_binding() {
    let dry_run =
        make(&["--dry-run", "spelling", "UVX=probe-uvx"]).expect("read evaluated spelling recipe");
    assert!(dry_run.status.success());
    let command = String::from_utf8(dry_run.stdout).expect("read spelling command");
    assert!(
        command.contains(concat!(
            "probe-uvx --from \"git+https://github.com/leynos/typos-config-builder.git@v0.1.3\" ",
            "typos-config-builder gate --scope all"
        )),
        "{command}"
    );
    let failed = make(&["--silent", "spelling", "UVX=false"]).expect("run failing spelling probe");
    assert!(
        !failed.status.success(),
        "spelling failure did not propagate"
    );
}

/// Scenario: local Markdown lint uses a pinned binary and propagates failures.
#[test]
fn markdownlint_installer_and_gate_are_binding() {
    let install = make(&[
        "--dry-run",
        "install-markdownlint",
        "BUN=probe-bun",
        "BUILD_TOOLS_PREFIX=/tmp/probe-build-tools",
    ])
    .expect("read Markdown lint installer recipe");
    assert!(install.status.success());
    let installer_command = String::from_utf8(install.stdout).expect("read installer command");
    assert!(
        installer_command.contains(concat!(
            "BUN_INSTALL_BIN=\"/tmp/probe-build-tools/bin\" ",
            "probe-bun add --global --exact markdownlint-cli2@0.22.1"
        )),
        "{installer_command}"
    );
    let installer_failure = make(&["--silent", "install-markdownlint", "BUN=false"])
        .expect("run failing Markdown lint installer probe");
    assert!(
        !installer_failure.status.success(),
        "installer failure did not propagate"
    );

    let lint = make(&["--dry-run", "markdownlint", "MDLINT=probe-markdownlint"])
        .expect("read Markdown lint recipe");
    assert!(lint.status.success());
    let lint_command = String::from_utf8(lint.stdout).expect("read Markdown lint command");
    assert!(
        lint_command.contains("probe-markdownlint '**/*.md'"),
        "{lint_command}"
    );
    let lint_failure = make(&["--silent", "markdownlint", "MDLINT=false"])
        .expect("run failing Markdown lint probe");
    assert!(
        !lint_failure.status.success(),
        "Markdown lint failure did not propagate"
    );
}
