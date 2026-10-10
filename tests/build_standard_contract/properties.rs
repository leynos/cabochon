//! Property checks for pure build-standard readers.

use proptest::prelude::*;

use super::{
    config::{Flags, Pin, PinParseError},
    make::{Assignment, assigned_rustflags},
};

/// Produces supported and unsupported channel names from a TOML-safe alphabet.
fn channel_names() -> impl Strategy<Value = String> {
    prop_oneof![
        4 => prop::collection::vec(
            prop_oneof![
                Just('a'), Just('b'), Just('n'), Just('i'), Just('g'), Just('h'),
                Just('t'), Just('l'), Just('y'), Just('0'), Just('1'), Just('.'),
                Just('-'),
            ],
            0..32,
        )
        .prop_map(|characters| characters.into_iter().collect()),
        1 => Just(String::from("nightly")),
        2 => (0_u16..10_000, 1_u8..13, 1_u8..32)
            .prop_map(|(year, month, day)| format!("nightly-{year:04}-{month:02}-{day:02}")),
        1 => Just(String::from("stable")),
        2 => (0_u16..10_000, 0_u16..10_000, 0_u16..10_000)
            .prop_map(|(major, minor, patch)| format!("{major}.{minor}.{patch}")),
    ]
}

/// Renders a channel declaration without adding quoting edge cases.
fn toolchain(channel: &str) -> String { format!("[toolchain]\nchannel = \"{channel}\"\n") }

proptest! {
    #![proptest_config(ProptestConfig::with_cases(96))]

    /// Scenario: any generated numeric release channel selects stable.
    #[test]
    fn pin_reader_accepts_generated_stable_versions(
        major in 0_u16..10_000,
        minor in 0_u16..10_000,
        patch in 0_u16..10_000,
    ) {
        let input = toolchain(&format!("{major}.{minor}.{patch}"));
        prop_assert_eq!(Pin::read(&input), Ok(Pin::Stable));
    }

    /// Invariant: a generated safe channel is classified by the supported grammar.
    #[test]
    fn pin_reader_classifies_generated_channel_names(channel in channel_names()) {
        let input = toolchain(&channel);
        let result = Pin::read(&input);
        if channel.is_empty() {
            prop_assert!(matches!(result, Err(PinParseError::Malformed(_))));
        } else if channel == "nightly"
            || channel.strip_prefix("nightly-").is_some_and(|suffix| !suffix.is_empty())
        {
            prop_assert_eq!(result, Ok(Pin::Nightly));
        } else if is_numeric_release(&channel) || channel == "stable" {
            prop_assert_eq!(result, Ok(Pin::Stable));
        } else {
            prop_assert_eq!(result, Err(PinParseError::Unsupported(channel)));
        }
    }

    /// Invariant: split and joined `-C` words produce the same flags.
    #[test]
    fn compiler_control_words_have_one_normal_form(values in prop::collection::vec(0_u16..50_000, 0..24)) {
        let joined = values
            .iter()
            .map(|value| format!("-Cproperty={value}"))
            .collect::<Vec<_>>();
        let split = values
            .iter()
            .flat_map(|value| [String::from("-C"), format!("property={value}")])
            .collect::<Vec<_>>();
        prop_assert_eq!(
            Flags::from_words(split.iter().map(String::as_str)),
            Flags::from_words(joined.iter().map(String::as_str)),
        );
    }

    /// Invariant: either supported inheritance form preserves every own flag.
    #[test]
    fn rustflags_inheritance_forms_preserve_generated_words(values in prop::collection::vec(0_u16..50_000, 0..24)) {
        let own = values
            .iter()
            .map(|value| format!("-Cproperty={value}"))
            .collect::<Vec<_>>();
        let flags = own.join(" ");
        let quoted = assigned_rustflags(&format!("RUSTFLAGS=\"{flags}\" cargo test"));
        let separated = assigned_rustflags(&format!(
            "RUSTFLAGS=\"${{RUSTFLAGS:+$RUSTFLAGS }}{flags}\" cargo test"
        ));
        let defaulted = assigned_rustflags(&format!(
            "RUSTFLAGS=\"${{RUSTFLAGS-}} {flags}\" cargo test"
        ));
        let expected = || {
            Assignment::Flags(Flags::from_words(own.iter().map(String::as_str)), true)
        };
        let no_inheritance = Assignment::Flags(
            Flags::from_words(own.iter().map(String::as_str)),
            false,
        );
        prop_assert_eq!(quoted, Ok(no_inheritance));
        prop_assert_eq!(separated, Ok(expected()));
        prop_assert_eq!(defaulted, Ok(expected()));
    }

    /// Invariant: unquoted assignments remain errors for arbitrary flag payloads.
    #[test]
    fn unquoted_rustflags_assignments_are_rejected(values in prop::collection::vec(0_u16..50_000, 0..24)) {
        let flags = values
            .iter()
            .map(|value| format!("-Cproperty={value}"))
            .collect::<Vec<_>>()
            .join(" ");
        let command = format!("RUSTFLAGS={flags} cargo test");
        let assignment = assigned_rustflags(&command);
        prop_assert!(assignment.is_err());
    }
}

/// Returns whether a generated name has the supported stable release shape.
fn is_numeric_release(channel: &str) -> bool {
    let parts = channel.split('.').collect::<Vec<_>>();
    matches!(parts.as_slice(), [major, minor, patch]
        if [major, minor, patch]
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit())))
}
