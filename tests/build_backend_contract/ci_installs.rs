//! Checks CI tool provisioning, ordering, and binding gate steps.

use rstest::rstest;
use serde_norway::Value;

use super::{
    CI,
    INSTALL_MDTABLEFIX,
    INSTALL_WHITAKER,
    MARKDOWNLINT_ACTION,
    PUBLISHER,
    SETUP_RUST,
    SETUP_UV,
    action_step,
    field,
    is_binding,
    named_step,
    steps,
    suite_jobs,
};

/// Correct order of uv provisioning and the spelling gate in the CI fixture.
const UV_THEN_SPELLING: &str = concat!(
    "      - name: Setup uv\n",
    "        uses: astral-sh/setup-uv@",
    "12d13f90bc3a5a1971bebad4beb09a4dfa962e91\n",
    "      - name: Spelling\n",
    "        run: make spelling",
);

/// Reversed order used to prove that CI rejects late provisioning.
const SPELLING_THEN_UV: &str = concat!(
    "      - name: Spelling\n",
    "        run: make spelling\n",
    "      - name: Setup uv\n",
    "        uses: astral-sh/setup-uv@",
    "12d13f90bc3a5a1971bebad4beb09a4dfa962e91",
);

/// Stable check command, with an absolute manifest outside config discovery.
const MSRV_CHECK: &str = concat!(
    "cargo +1.85.0 check --lib --all-features --locked --manifest-path ",
    "\"$GITHUB_WORKSPACE/Cargo.toml\"",
);

/// Required explicit toolchain installation for the declared MSRV.
const MSRV_INSTALL: &str = "rustup toolchain install 1.85.0 --profile minimal";

/// Checks the isolated stable invocation without inherited development flags.
fn msrv_check_is_valid(step: &Value) -> bool {
    is_binding(step)
        && step.get("env").is_none()
        && step.get("working-directory").and_then(Value::as_str) == Some("${{ runner.temp }}")
        && step.get("run").and_then(Value::as_str) == Some(MSRV_CHECK)
}

/// Checks that the MSRV job is binding and has no inherited environment.
fn msrv_job_is_binding_without_environment(job: &Value) -> bool {
    is_binding(job) && job.get("env").is_none()
}

/// Reports absent, reordered, non-binding or incorrectly routed MSRV checks.
fn msrv_problems(text: &str) -> Result<Vec<String>, String> {
    let workflow: Value = serde_norway::from_str(text).map_err(|error| error.to_string())?;
    let Some(job) = workflow.get("jobs").and_then(|jobs| jobs.get("msrv")) else {
        return Ok(vec!["CI must have a binding msrv job".to_owned()]);
    };
    let job_steps = steps(&workflow, "msrv")?;
    let checkout = action_step(
        job_steps,
        "actions/checkout@900f2210b1d28bbbd0bd22d17926b9e224e8f231",
    );
    let install = named_step(job_steps, "Install MSRV toolchain");
    let check = named_step(job_steps, "Check MSRV");
    let ordered_steps = match (checkout, install, check) {
        (
            Some((checkout_at, checkout_step)),
            Some((install_at, install_step)),
            Some((check_at, check_step)),
        ) => {
            checkout_at < install_at
                && install_at < check_at
                && is_binding(checkout_step)
                && is_binding(install_step)
                && install_step.get("run").and_then(Value::as_str) == Some(MSRV_INSTALL)
                && msrv_check_is_valid(check_step)
        }
        _ => false,
    };
    if msrv_job_is_binding_without_environment(job) && ordered_steps {
        Ok(Vec::new())
    } else {
        Ok(vec![
            concat!(
                "MSRV must install Rust 1.85.0 before checking the library ",
                "outside Cargo config discovery",
            )
            .to_owned(),
        ])
    }
}

/// Scenario: the declared minimum compiler checks the library on an isolated route.
#[test]
fn ci_checks_the_declared_msrv() {
    let found = msrv_problems(CI).expect("read MSRV workflow");
    assert!(found.is_empty(), "{found:?}");
}

/// Scenario: removing or weakening any part of the MSRV route is detected.
#[rstest]
#[case::job_removed("  msrv:", "  missing-msrv:")]
#[case::job_conditional("  msrv:\n", "  msrv:\n    if: false\n")]
#[case::job_soft_failed("  msrv:\n", "  msrv:\n    continue-on-error: true\n")]
#[case::install_removed("name: Install MSRV toolchain", "name: Other toolchain")]
#[case::install_conditional(
    "name: Install MSRV toolchain",
    "name: Install MSRV toolchain\n        if: false"
)]
#[case::install_wrong_version(MSRV_INSTALL, "rustup toolchain install stable --profile minimal")]
#[case::check_removed("name: Check MSRV", "name: Other check")]
#[case::check_conditional("name: Check MSRV", "name: Check MSRV\n        if: false")]
#[case::check_soft_failed(
    "name: Check MSRV",
    "name: Check MSRV\n        continue-on-error: true"
)]
#[case::check_wrong_version("cargo +1.85.0 check", "cargo +stable check")]
#[case::library_missing("check --lib --all-features", "check --all-features")]
#[case::features_missing("check --lib --all-features", "check --lib")]
#[case::lock_missing("--locked --manifest-path", "--manifest-path")]
#[case::manifest_relative("\"$GITHUB_WORKSPACE/Cargo.toml\"", "Cargo.toml")]
#[case::directory_missing("working-directory: ${{ runner.temp }}", "shell: bash")]
#[case::directory_inside_checkout(
    "working-directory: ${{ runner.temp }}",
    "working-directory: ${{ github.workspace }}"
)]
fn ci_msrv_mutations_are_detected(#[case] old: &str, #[case] new: &str) {
    let mutated = CI.replacen(old, new, 1);
    assert_ne!(mutated, CI, "fixture must mutate the MSRV route");
    let found = msrv_problems(&mutated).expect("read mutated MSRV workflow");
    assert_ne!(
        found,
        [] as [String; 0],
        "mutation must violate the MSRV contract"
    );
}

/// Scenario: toolchain installation after the check cannot satisfy provisioning.
#[test]
fn ci_msrv_installation_cannot_follow_the_check() {
    let mut workflow: Value = serde_norway::from_str(CI).expect("read MSRV workflow");
    let job_steps = workflow
        .get_mut("jobs")
        .and_then(|jobs| jobs.get_mut("msrv"))
        .and_then(|job| job.get_mut("steps"))
        .and_then(Value::as_sequence_mut)
        .expect("MSRV step list");
    let (install_index, _) =
        named_step(job_steps, "Install MSRV toolchain").expect("MSRV installer step");
    let (check_index, _) = named_step(job_steps, "Check MSRV").expect("MSRV check step");
    job_steps.swap(install_index, check_index);
    let mutated = serde_norway::to_string(&workflow).expect("serialize reordered MSRV steps");
    let found = msrv_problems(&mutated).expect("read reordered MSRV workflow");
    assert_ne!(
        found,
        [] as [String; 0],
        "installation must precede the check"
    );
}

/// Checks that each pinned installer precedes its consumer without a soft skip.
fn install_order_problems(steps: &[Value]) -> Vec<String> {
    let mut problems = Vec::new();
    let required = [
        (SETUP_RUST, "Test and Measure Coverage"),
        (INSTALL_MDTABLEFIX, "Format"),
        (INSTALL_WHITAKER, "Lint"),
    ];
    for (action, consumer) in required {
        let Some((install_at, install)) = action_step(steps, action) else {
            problems.push(format!("missing pinned installer {action}"));
            continue;
        };
        let Some((use_at, _)) = named_step(steps, consumer) else {
            problems.push(format!("missing consumer {consumer}"));
            continue;
        };
        if install_at >= use_at || !is_binding(install) {
            problems.push(format!(
                "{action} must run unconditionally before {consumer}"
            ));
        }
    }
    problems
}

/// Checks whether an action declares a named input.
fn has_action_input(action: &Value, input: &str) -> bool {
    action
        .get("with")
        .and_then(|inputs| inputs.get(input))
        .is_some()
}

/// Detects missing Cranelift or explicit Whitaker rollout pins.
fn whitaker_inputs_are_invalid(action: &Value) -> bool {
    // GitHub accepts both YAML boolean true and the quoted action-input form.
    let uses_cranelift = action
        .get("with")
        .and_then(|inputs| inputs.get("cranelift"))
        .is_some_and(|value| value == &Value::Bool(true) || value.as_str() == Some("true"));
    let overrides_rolling_inputs =
        has_action_input(action, "suite-version") || has_action_input(action, "installer-version");

    !uses_cranelift || overrides_rolling_inputs
}

/// Checks the consumer-specific inputs on installers and Markdown lint.
fn installer_input_problems(steps: &[Value]) -> Vec<String> {
    let mut problems = Vec::new();
    if let Some((_, setup)) = action_step(steps, SETUP_RUST)
        && field(setup, "with", "install-mold") != Some("true")
    {
        problems.push("setup-rust must install `mold`".to_owned());
    }
    if let Some((_, whitaker)) = action_step(steps, INSTALL_WHITAKER)
        && whitaker_inputs_are_invalid(whitaker)
    {
        problems.push("Whitaker must use its pinned installer and rolling suite".to_owned());
    }
    if let Some((_, mdtablefix)) = action_step(steps, INSTALL_MDTABLEFIX)
        && field(mdtablefix, "with", "version") != Some("0.6.1")
    {
        problems.push("mdtablefix must install the pinned 0.6.1 binary".to_owned());
    }
    if let Some((_, markdown)) = action_step(steps, MARKDOWNLINT_ACTION) {
        if field(markdown, "with", "globs") != Some("**/*.md") {
            problems.push("Markdown lint must use the canonical glob".to_owned());
        }
    } else {
        problems.push("pinned Markdown lint action is missing".to_owned());
    }
    problems
}

/// Keeps auxiliary Cargo tools on the binary-only path before their consumers.
fn binary_tool_problems(steps: &[Value]) -> Vec<String> {
    let mut problems = Vec::new();
    for (name, package, consumer) in [
        (
            "Install test runner",
            "cargo-nextest",
            "Test and Measure Coverage",
        ),
        ("Install cargo-audit", "cargo-audit", "Audit dependencies"),
    ] {
        let Some((install_at, install)) = named_step(steps, name) else {
            problems.push(format!("missing binary installer {name}"));
            continue;
        };
        let Some((use_at, _)) = named_step(steps, consumer) else {
            problems.push(format!("missing consumer {consumer}"));
            continue;
        };
        let expected =
            format!("cargo binstall --no-confirm --disable-strategies compile {package}");
        let command = install.get("run").and_then(Value::as_str);
        let is_ready = install_at < use_at && is_binding(install);
        let is_binary_only = command == Some(expected.as_str());
        if !is_ready || !is_binary_only {
            problems.push(format!(
                "{name} must install a trusted binary before {consumer}"
            ));
        }
    }
    problems
}

/// Checks that uv setup is binding and precedes the spelling gate.
fn spelling_setup_is_valid(uv_at: usize, uv: &Value, gate_at: usize) -> bool {
    uv_at < gate_at && is_binding(uv)
}

/// Checks that the spelling step runs its binding Make gate.
fn spelling_gate_is_valid(gate: &Value) -> bool {
    is_binding(gate) && gate.get("run").and_then(Value::as_str) == Some("make spelling")
}

/// Reports missing, late or non-binding spelling setup and gate steps.
fn spelling_problems(steps: &[Value]) -> Vec<String> {
    match (action_step(steps, SETUP_UV), named_step(steps, "Spelling")) {
        (Some((uv_at, uv)), Some((gate_at, gate)))
            if spelling_setup_is_valid(uv_at, uv, gate_at) && spelling_gate_is_valid(gate) =>
        {
            Vec::new()
        }
        _ => vec!["CI must run the binding spelling gate after setup-uv".to_owned()],
    }
}

/// Checks action ordering and binding inputs in the PR CI job.
fn ci_install_problems(text: &str) -> Result<Vec<String>, String> {
    let workflow: Value = serde_norway::from_str(text).map_err(|error| error.to_string())?;
    let steps = steps(&workflow, "build-test")?;
    let mut problems = install_order_problems(steps);
    problems.extend(installer_input_problems(steps));
    problems.extend(binary_tool_problems(steps));
    problems.extend(spelling_problems(steps));
    Ok(problems)
}

/// Scenario: CI provisions every selected tool before use.
#[test]
fn ci_installers_precede_their_consumers() {
    let found = ci_install_problems(CI).expect("read CI workflow");
    assert!(found.is_empty(), "{found:?}");
    for workflow in [CI, PUBLISHER] {
        let suite = suite_jobs::suite_job_problems(workflow).expect("read suite jobs");
        assert!(suite.is_empty(), "{suite:?}");
    }
}

/// Scenario: installers are removed or made conditional.
#[rstest]
#[case::mold_removed("install-mold: 'true'", "install-mold: 'false'")]
#[case::whitaker_removed(
    INSTALL_WHITAKER,
    "example/incorrect@0000000000000000000000000000000000000000"
)]
#[case::suite_pinned("cranelift: true", "cranelift: true\n          suite-version: '1.0'")]
#[case::cranelift_disabled("cranelift: true", "cranelift: false")]
#[case::cranelift_absent("cranelift: true", "")]
#[case::cranelift_unrecognized("cranelift: true", "cranelift: maybe")]
#[case::installer_conditional(
    INSTALL_WHITAKER,
    "leynos/shared-actions/.github/actions/install-whitaker@\
     6dea5677a84fec60ca51b07202570e3af12ffdb4\n        if: false"
)]
#[case::installer_soft_failed(
    INSTALL_WHITAKER,
    "leynos/shared-actions/.github/actions/install-whitaker@\
     6dea5677a84fec60ca51b07202570e3af12ffdb4\n        continue-on-error: true"
)]
#[case::mdtablefix_version("version: \"0.6.1\"", "version: \"0.5.0\"")]
#[case::markdown_glob("globs: '**/*.md'", "globs: 'README.md'")]
#[case::nextest_source_fallback(
    "cargo binstall --no-confirm --disable-strategies compile cargo-nextest",
    "cargo binstall --no-confirm cargo-nextest"
)]
#[case::audit_source_fallback(
    "cargo binstall --no-confirm --disable-strategies compile cargo-audit",
    "cargo binstall --no-confirm cargo-audit"
)]
#[case::spelling_removed("run: make spelling", "run: true")]
#[case::spelling_before_setup(UV_THEN_SPELLING, SPELLING_THEN_UV)]
#[case::spelling_soft_failed(
    "run: make spelling",
    "run: make spelling\n        continue-on-error: true"
)]
fn ci_install_mutations_are_detected(#[case] old: &str, #[case] new: &str) {
    let mutated = CI.replacen(old, new, 1);
    assert_ne!(mutated, CI);
    let problems = ci_install_problems(&mutated).expect("read mutated CI workflow");
    assert_ne!(
        problems,
        [] as [String; 0],
        "installer mutation must be detected"
    );
}

/// Both YAML representations of the enabled input retain the approved route.
#[rstest]
#[case::boolean_true("cranelift: true")]
#[case::quoted_true("cranelift: 'true'")]
fn whitaker_accepts_enabled_cranelift(#[case] input: &str) {
    let workflow = CI.replacen("cranelift: true", input, 1);
    let found = ci_install_problems(&workflow).expect("read Whitaker input");
    assert!(found.is_empty(), "{found:?}");
}
