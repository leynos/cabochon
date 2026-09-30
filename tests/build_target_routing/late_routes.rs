//! Regression checks for invalid targets in later Cargo invocations.

use super::{assignment, dry_run, failed_route};

/// A valid native nextest route cannot hide an invalid later doctest target.
#[test]
fn test_rejects_invalid_later_doctest_route_before_cargo() {
    let output = dry_run(
        "test",
        &[
            assignment("TEST_FLAGS", "--target=x86_64-unknown-linux-gnu --"),
            assignment("BUILD_JOBS", "--target=not-a-real-target"),
        ],
        None,
    )
    .expect("run the split-route test dry-run");
    failed_route(&output, "Cannot classify Cargo target")
        .expect("an invalid doctest target must fail before either Cargo command");
}

/// A valid native rustdoc route cannot hide an invalid later Clippy target.
#[test]
fn lint_rejects_invalid_later_clippy_route_before_cargo() {
    let output = dry_run(
        "lint-clippy",
        &[assignment(
            "CLIPPY_FLAGS",
            "--workspace --all-targets --target=not-a-real-target",
        )],
        None,
    )
    .expect("run the split-route lint dry-run");
    failed_route(&output, "Cannot classify Cargo target")
        .expect("an invalid Clippy target must fail before either Cargo command");
}
