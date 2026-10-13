//! Property checks for target argument precedence and Linux linker routing.

use proptest::prelude::*;

use super::{NATIVE_LINUX, NON_LINUX, assignment, dry_run, successful_route};

const KNOWN_TARGETS: [&str; 3] = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-msvc",
    "aarch64-apple-darwin",
];

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    /// Invariant: an ordered explicit target overrides the environment target.
    #[test]
    fn explicit_target_forms_override_generated_environment_targets(
        environment_target in prop::sample::select(KNOWN_TARGETS.to_vec()),
        explicit_target in prop::sample::select(KNOWN_TARGETS.to_vec()),
        separated in any::<bool>(),
    ) {
        let argument = if separated {
            format!("--target {explicit_target}")
        } else {
            format!("--target={explicit_target}")
        };
        let output = dry_run(
            "build",
            &[assignment("BUILD_JOBS", &argument)],
            Some(environment_target),
        ).expect("run generated target routing dry-run");
        let expected = if explicit_target.contains("linux") {
            NATIVE_LINUX
        } else {
            NON_LINUX
        };
        let result = successful_route(&output, &[expected], expected);
        prop_assert!(
            result.is_ok(),
            "environment={environment_target}, explicit={explicit_target}, separated={separated}: {result:?}",
        );
    }
}
