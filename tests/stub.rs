//! Disposable generated-template test stub.
//!
//! This test exists only so `cargo nextest run` has at least one test in a
//! freshly generated project. Delete this file as soon as the project has real
//! functionality and real tests. Do not keep this stub as permanent coverage.

#[test]
fn replace_this_stub_when_real_tests_exist() {
    // `option_env!` reads the value Cargo bakes in at compile time, so this
    // stub stays clear of the `disallowed_methods` ban on runtime
    // `std::env` access without needing an injected environment reader.
    assert!(
        option_env!("CARGO_MANIFEST_DIR").is_some(),
        "CARGO_MANIFEST_DIR should be set by Cargo when running tests"
    );
}
