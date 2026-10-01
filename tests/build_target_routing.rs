//! Contracts for Make's target-aware development build route.
//!
//! Each probe asks GNU Make to print its evaluated recipes. The contract checks
//! every Cargo invocation as well as the preflight values selected for that
//! target, without compiling the project.

use std::process::{Command, ExitStatus};

const CARGO_PROBE: &str = "CARGO=probe-cargo";
const BUILD_HOST: &str = "BUILD_HOST_OS=Linux";
const NATIVE_LINUX: Route = Route {
    mold: true,
    clang: true,
};
const NON_LINUX: Route = Route {
    mold: false,
    clang: false,
};

/// The per-target tools required by the preflight and flags passed to Cargo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Route {
    /// Whether this route selects the repository's Linux linker.
    mold: bool,
    /// Whether this route requires Clang as the linker driver.
    clang: bool,
}

/// The result of one Make dry-run, including expected failing resolutions.
struct MakeOutput {
    /// Whether Make accepted the requested route.
    status: ExitStatus,
    /// The recipes printed by Make.
    stdout: String,
    /// Resolver and Make diagnostics.
    stderr: String,
}

/// Runs Make's route and reports a spawn error if the process cannot start.
fn dry_run(
    target: &str,
    assignments: &[String],
    cargo_build_target: Option<&str>,
) -> Result<MakeOutput, String> {
    let mut command = Command::new("make");
    command
        .args([
            "--dry-run",
            "--always-make",
            target,
            CARGO_PROBE,
            BUILD_HOST,
        ])
        .args(assignments)
        .env_remove("CARGO_BUILD_TARGET")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_PROFILE_DEV_CODEGEN_BACKEND")
        .env_remove("CARGO_PROFILE_TEST_CODEGEN_BACKEND")
        .env_remove("RUSTFLAGS")
        .current_dir(env!("CARGO_MANIFEST_DIR"));
    if let Some(build_target) = cargo_build_target {
        command.env("CARGO_BUILD_TARGET", build_target);
    }
    let output = command
        .output()
        .map_err(|error| format!("running Make dry-run for `{target}`: {error}"))?;
    Ok(MakeOutput {
        status: output.status,
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

/// Builds one command-line Make variable assignment.
fn assignment(name: &str, value: &str) -> String { format!("{name}={value}") }

/// Returns each printed invocation that uses the controlled Cargo executable.
fn cargo_lines(stdout: &str) -> Vec<&str> {
    stdout
        .lines()
        .filter(|line| line.split_whitespace().any(|word| word == "probe-cargo"))
        .collect()
}

/// Reads one quoted preflight variable or explains why it is unreadable.
fn preflight_value(line: &str, name: &str) -> Result<bool, String> {
    let key = format!("{name}=");
    let value = line
        .split_whitespace()
        .find_map(|word| word.strip_prefix(&key))
        .ok_or_else(|| format!("preflight recipe has no {name}: `{line}`"))?
        .trim_matches('"');
    match value {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(format!("preflight {name} is not yes or no: `{line}`")),
    }
}

/// Reads Make's single preflight route or reports a missing or duplicate check.
fn preflight_route(stdout: &str) -> Result<Route, String> {
    let lines: Vec<&str> = stdout
        .lines()
        .filter(|line| line.contains("scripts/check-build-tools.sh"))
        .collect();
    let [line] = lines.as_slice() else {
        return Err(format!(
            "expected one build-tool preflight, found {}",
            lines.len()
        ));
    };
    Ok(Route {
        mold: preflight_value(line, "CHECK_MOULD")?,
        clang: preflight_value(line, "CHECK_CLANG")?,
    })
}

/// Checks a Cargo recipe's frontend and linker flags against its target.
fn cargo_line_problem(line: &str, expected: Route) -> Result<(), String> {
    let (_, assignment) = line
        .split_once("RUSTFLAGS=\"")
        .ok_or_else(|| format!("Cargo recipe does not assign RUSTFLAGS: `{line}`"))?;
    let (raw_flags, command) = assignment
        .split_once('"')
        .ok_or_else(|| format!("Cargo recipe has an unterminated RUSTFLAGS value: `{line}`"))?;
    if command.split_whitespace().next() != Some("probe-cargo") {
        return Err(format!(
            "Cargo recipe lost the injectable command: `{line}`"
        ));
    }

    let flags = raw_flags.replace("${RUSTFLAGS:+$RUSTFLAGS }", " ");
    let words: Vec<&str> = flags.split_whitespace().collect();
    if !words
        .windows(2)
        .any(|pair| matches!(pair, [deny, warnings] if *deny == "-D" && *warnings == "warnings"))
    {
        return Err(format!("Cargo recipe lost `-D warnings`: `{line}`"));
    }
    if !words.contains(&"-Zthreads=8") {
        return Err(format!("Cargo recipe lost `-Zthreads=8`: `{line}`"));
    }
    let uses_mold = words.iter().any(|word| word.contains("fuse-ld=mold"));
    if uses_mold != expected.mold {
        return Err(format!(
            "Cargo recipe has mold={uses_mold}, expected mold={}: `{line}`",
            expected.mold
        ));
    }
    Ok(())
}

/// Verifies the preflight and every Cargo command on a successful route.
fn successful_route(
    output: &MakeOutput,
    expected_commands: &[Route],
    expected_preflight: Route,
) -> Result<(), String> {
    if !output.status.success() {
        return Err(format!(
            "Make rejected the route: {}{}",
            output.stdout, output.stderr
        ));
    }
    let actual_preflight = preflight_route(&output.stdout)?;
    if actual_preflight != expected_preflight {
        return Err(format!(
            "preflight selected {actual_preflight:?}, expected {expected_preflight:?}"
        ));
    }
    let lines = cargo_lines(&output.stdout);
    if lines.len() != expected_commands.len() {
        return Err(format!(
            "expected {} probe-cargo commands, found {}: {}",
            expected_commands.len(),
            lines.len(),
            output.stdout
        ));
    }
    for (line, expected) in lines.iter().zip(expected_commands) {
        cargo_line_problem(line, *expected)?;
    }
    Ok(())
}

/// Verifies failed target resolution stopped before Cargo.
fn failed_route(output: &MakeOutput, diagnostic: &str) -> Result<(), String> {
    if output.status.success() {
        return Err("Make accepted a target it could not classify".to_owned());
    }
    if !output.stderr.contains(diagnostic) {
        return Err(format!(
            "Make did not report `{diagnostic}`: {}",
            output.stderr
        ));
    }
    if !cargo_lines(&output.stdout).is_empty() {
        return Err(format!(
            "Make printed Cargo after target resolution failed: {}",
            output.stdout
        ));
    }
    Ok(())
}

/// Native Linux build: preflight and Cargo both select `mold` and threads.
#[test]
fn native_build_selects_mold_and_threads() {
    let output = dry_run("build", &[], None).expect("run the native build dry-run");
    successful_route(&output, &[NATIVE_LINUX], NATIVE_LINUX)
        .expect("native build should select the Linux development route");
}

/// Windows environment target: omit Linux linker requirements.
#[test]
fn cargo_build_target_windows_omits_linux_linker_tools() {
    let output = dry_run("build", &[], Some("x86_64-pc-windows-msvc"))
        .expect("run the Windows-target build dry-run");
    successful_route(&output, &[NON_LINUX], NON_LINUX)
        .expect("Windows build should use threads without Linux linker tools");
}

/// `BUILD_JOBS` Windows and macOS targets: omit Linux linker requirements.
#[test]
fn build_jobs_windows_and_macos_omit_linux_linker_tools() {
    for target in ["x86_64-pc-windows-msvc", "aarch64-apple-darwin"] {
        let output = dry_run(
            "build",
            &[assignment("BUILD_JOBS", &format!("--target={target}"))],
            None,
        )
        .expect("run a cross-target build dry-run");
        successful_route(&output, &[NON_LINUX], NON_LINUX)
            .expect("Windows and macOS routes should omit Linux linker tools");
    }
}

/// Explicit `--target` forms override `CARGO_BUILD_TARGET=Windows`.
#[test]
fn explicit_target_overrides_cargo_build_target() {
    for target_argument in [
        "--target=x86_64-unknown-linux-gnu",
        "--target x86_64-unknown-linux-gnu",
    ] {
        let output = dry_run(
            "build",
            &[assignment("BUILD_JOBS", target_argument)],
            Some("x86_64-pc-windows-msvc"),
        )
        .expect("run the command-line target dry-run");
        successful_route(&output, &[NATIVE_LINUX], NATIVE_LINUX)
            .expect("the explicit Linux target should override the Windows environment");
    }
}

/// Typecheck: resolve Windows from `CARGO_FLAGS` without Linux linker tools.
#[test]
fn typecheck_uses_its_effective_cargo_target() {
    let output = dry_run(
        "typecheck",
        &[assignment(
            "CARGO_FLAGS",
            "--workspace --all-targets --all-features --target=x86_64-pc-windows-msvc",
        )],
        None,
    )
    .expect("run the typecheck dry-run");
    successful_route(&output, &[NON_LINUX], NON_LINUX)
        .expect("typecheck should select the Windows route");
}

/// Clippy Windows target: rustdoc stays native and preflight covers both.
#[test]
fn lint_clippy_checks_rustdoc_and_clippy_routes_separately() {
    let output = dry_run(
        "lint-clippy",
        &[assignment(
            "CLIPPY_FLAGS",
            "--workspace --all-targets --all-features --target=x86_64-pc-windows-msvc",
        )],
        None,
    )
    .expect("run the Clippy dry-run");
    successful_route(&output, &[NATIVE_LINUX, NON_LINUX], NATIVE_LINUX)
        .expect("rustdoc and Clippy should retain their distinct routes");
}

/// Test Windows target: nextest is cross-target and doctests remain native.
#[test]
fn test_checks_nextest_and_doctest_routes_separately() {
    let output = dry_run(
        "test",
        &[assignment(
            "TEST_FLAGS",
            "--workspace --all-targets --all-features --target=x86_64-pc-windows-msvc",
        )],
        None,
    )
    .expect("run the test dry-run");
    successful_route(&output, &[NON_LINUX, NATIVE_LINUX], NATIVE_LINUX)
        .expect("nextest and doctests should retain their distinct routes");
}

/// Cases where an invalid later Cargo route must stop all compilation.
#[path = "build_target_routing/late_routes.rs"]
mod late_routes;

/// Clippy separator: a Windows-looking rustc argument does not select the route.
#[test]
fn clippy_stops_target_routing_at_the_cargo_separator() {
    let output = dry_run(
        "lint-clippy",
        &[assignment(
            "CLIPPY_FLAGS",
            "--workspace -- --target=x86_64-pc-windows-msvc",
        )],
        None,
    )
    .expect("run the Clippy separator dry-run");
    successful_route(&output, &[NATIVE_LINUX, NATIVE_LINUX], NATIVE_LINUX)
        .expect("arguments after Clippy's separator must not change Cargo routing");
}

/// Mutation contract: removing threads or leaking `mold` into a cross route fails.
#[test]
fn command_contract_rejects_missing_threads_and_cross_target_mold() {
    let output = dry_run(
        "build",
        &[assignment("BUILD_JOBS", "--target=x86_64-pc-windows-msvc")],
        None,
    )
    .expect("run the Windows build dry-run");
    successful_route(&output, &[NON_LINUX], NON_LINUX)
        .expect("the unmodified Windows route should pass first");
    let lines = cargo_lines(&output.stdout);
    let line = lines
        .first()
        .copied()
        .expect("the build has one Cargo command");
    let missing_threads = line.replace("-Zthreads=8", "");
    assert_ne!(
        missing_threads, line,
        "the probe command should contain threads"
    );
    let thread_problem = cargo_line_problem(&missing_threads, NON_LINUX);
    assert!(
        thread_problem
            .as_ref()
            .is_err_and(|problem| problem.contains("lost `-Zthreads=8`")),
        "removing the frontend flag was not detected: {thread_problem:?}"
    );

    let leaked_mold = line.replace("-Zthreads=8", "-Zthreads=8 -Clink-arg=-fuse-ld=mold");
    assert_ne!(
        leaked_mold, line,
        "the probe command should contain threads"
    );
    let linker_problem = cargo_line_problem(&leaked_mold, NON_LINUX);
    assert!(
        linker_problem
            .as_ref()
            .is_err_and(|problem| problem.contains("expected mold=false")),
        "adding `mold` to a cross target was not detected: {linker_problem:?}"
    );
}

/// Conflicting command-line targets fail before Cargo.
#[test]
fn conflicting_targets_fail_before_cargo() {
    let output = dry_run(
        "build",
        &[assignment(
            "BUILD_JOBS",
            "--target=x86_64-pc-windows-msvc --target=aarch64-apple-darwin",
        )],
        None,
    )
    .expect("run the conflicting-target dry-run");
    failed_route(&output, "conflicting --target values")
        .expect("conflicting target values should fail before Cargo");
}

/// An unknown target fails closed before Cargo.
#[test]
fn unclassifiable_target_fails_before_cargo() {
    let output = dry_run(
        "build",
        &[assignment("BUILD_JOBS", "--target=not-a-real-target")],
        None,
    )
    .expect("run the unclassifiable-target dry-run");
    failed_route(&output, "Cannot classify Cargo target")
        .expect("an unknown target should fail before Cargo");
}
