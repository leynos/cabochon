//! Readers for the Cargo configuration half of the build standard: the
//! toolchain pin, the flag lists, and the `rustflags` sources in
//! `.cargo/config.toml`. Everything is read as text, so the contract needs no
//! parser dependency.

pub const CONFIG: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/.cargo/config.toml"));
pub const TOOLCHAIN: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/rust-toolchain.toml"));

/// The parallel-frontend flag every `rustflags` source carries on a nightly pin.
pub const THREADS_FLAG: &str = "-Zthreads=8";
/// The linker flag the Linux source adds, normalized to one token.
pub const LINKER_FLAG: &str = "-Clink-arg=-fuse-ld=mold";

/// A list of complaints about the repository.
pub type Problems = Vec<String>;

/// The channel the toolchain file pins.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pin {
    Nightly,
    Stable,
}

/// Why a toolchain file cannot select a supported channel.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PinParseError {
    Missing,
    Malformed(String),
    Duplicate(String),
    Unsupported(String),
}

impl Pin {
    /// Reads the pin from a `rust-toolchain.toml`.
    ///
    /// ```text
    /// Pin::read("[toolchain]\nchannel = \"nightly-2026-05-28\"") == Ok(Pin::Nightly)
    /// Pin::read("[toolchain]\nchannel = \"1.94.0\"") == Ok(Pin::Stable)
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a typed reason when the toolchain table lacks a channel, its
    /// declaration is malformed or repeated, or the channel is not a stable
    /// or nightly route the development standard supports.
    pub fn read(toolchain: &str) -> Result<Self, PinParseError> {
        let channel = toolchain_channel(toolchain)?;
        if channel == "nightly"
            || channel
                .strip_prefix("nightly-")
                .is_some_and(|date| !date.is_empty())
        {
            return Ok(Self::Nightly);
        }
        if is_stable_channel(channel) {
            return Ok(Self::Stable);
        }
        Err(PinParseError::Unsupported(channel.to_owned()))
    }

    /// Returns whether the pin takes `-Zthreads`, which is a nightly flag.
    pub const fn takes_threads(self) -> bool { matches!(self, Self::Nightly) }
}

/// Returns whether a channel names the stable alias or a release version.
fn is_stable_channel(channel: &str) -> bool {
    if channel == "stable" {
        return true;
    }
    let mut parts = channel.split('.');
    let Some(major) = parts.next() else {
        return false;
    };
    let Some(minor) = parts.next() else {
        return false;
    };
    let Some(patch) = parts.next() else {
        return false;
    };
    parts.next().is_none()
        && [major, minor, patch]
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
}

/// Returns whether a channel name is empty or contains an embedded quote.
fn is_malformed_channel_name(channel_name: &str) -> bool {
    channel_name.is_empty() || channel_name.contains('"')
}

/// Records the first channel name and reports whether one was already present.
const fn is_duplicate_channel<'a>(channel: &mut Option<&'a str>, channel_name: &'a str) -> bool {
    channel.replace(channel_name).is_some()
}

/// Reads the channel declaration from the `[toolchain]` table.
fn toolchain_channel(toolchain: &str) -> Result<&str, PinParseError> {
    let mut in_toolchain_table = false;
    let mut channel = None;
    for raw_line in toolchain.lines() {
        let line = raw_line.split('#').next().unwrap_or_default().trim();
        if line.starts_with('[') {
            in_toolchain_table = line == "[toolchain]";
            continue;
        }
        if !in_toolchain_table || !line.starts_with("channel") {
            continue;
        }
        let Some((key, assigned)) = line.split_once('=') else {
            return Err(PinParseError::Malformed(line.to_owned()));
        };
        if key.trim() != "channel" {
            continue;
        }
        let declaration = assigned.trim();
        let Some(channel_name) = declaration
            .strip_prefix('"')
            .and_then(|quoted| quoted.strip_suffix('"'))
        else {
            return Err(PinParseError::Malformed(line.to_owned()));
        };
        if is_malformed_channel_name(channel_name) {
            return Err(PinParseError::Malformed(line.to_owned()));
        }
        if is_duplicate_channel(&mut channel, channel_name) {
            return Err(PinParseError::Duplicate(line.to_owned()));
        }
    }
    channel.ok_or(PinParseError::Missing)
}

/// A list of compiler flags, with `-C value` pairs joined into `-Cvalue` so
/// both spellings compare equal.
#[derive(Debug, PartialEq, Eq)]
pub struct Flags(Vec<String>);

impl Flags {
    /// Reads a flag list from its words.
    ///
    /// ```text
    /// Flags::from_words(["-C", "link-arg=-fuse-ld=mold"]) == Flags::from_words(["-Clink-arg=-fuse-ld=mold"])
    /// ```
    pub fn from_words<'a>(words: impl IntoIterator<Item = &'a str>) -> Self {
        let mut joined: Vec<String> = Vec::new();
        for word in words {
            match joined.last_mut() {
                Some(last) if last == "-C" => *last = format!("-C{word}"),
                _ => joined.push(word.to_owned()),
            }
        }
        Self(joined)
    }

    /// Returns whether the list names one flag.
    fn names(&self, flag: &str) -> bool { self.0.iter().any(|candidate| candidate == flag) }

    /// Returns whether the list names the frontend flag.
    pub fn names_threads(&self) -> bool { self.names(THREADS_FLAG) }

    /// Returns whether the list denies warnings in either Cargo spelling.
    pub fn names_warning_deny(&self) -> bool {
        self.names("-Dwarnings")
            || self.0.windows(2).any(
                |pair| matches!(pair, [deny, warnings] if deny == "-D" && warnings == "warnings"),
            )
    }

    /// Returns whether the list names the linker flag.
    pub fn names_linker(&self) -> bool { self.names(LINKER_FLAG) }

    /// Returns the list without the linker flag, which is the one that may differ.
    fn without_linker_flag(&self) -> Vec<&String> {
        self.0.iter().filter(|flag| *flag != LINKER_FLAG).collect()
    }

    /// Checks the list against a pin and whether the linker flag is expected.
    ///
    /// ```text
    /// Flags::from_words(["-Zthreads=8"]).meets(Pin::Nightly, false) == Ok(())
    /// Flags::from_words([]).meets(Pin::Nightly, false).is_err()
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the reason when the frontend or linker flag is wrong.
    pub fn meets(&self, pin: Pin, takes_linker_flag: bool) -> Result<(), String> {
        if self.names_threads() != pin.takes_threads() {
            return Err(format!("gets {THREADS_FLAG} wrong: {:?}", self.0));
        }
        if self.names_linker() != takes_linker_flag {
            return Err(format!("gets `mold` wrong: {:?}", self.0));
        }
        Ok(())
    }
}

/// One `rustflags` source in a Cargo configuration.
struct Source {
    table: String,
    flags: Flags,
}

impl Source {
    /// Returns whether the table applies on Linux alone.
    fn is_linux(&self) -> bool { self.table.starts_with("target.") && self.table.contains("linux") }

    /// Returns what is wrong with the source's flags for a pin: the frontend flag
    /// on a nightly pin only, and `mold` in a Linux table only.
    fn problem(&self, pin: Pin) -> Option<String> {
        let reason = self.flags.meets(pin, self.is_linux()).err()?;
        Some(format!("[{}] {reason}", self.table))
    }
}

/// One line of a Cargo configuration, as far as the standard reads it.
enum Line {
    Table(String),
    Rustflags(Flags),
    Other,
}

/// Returns the quoted strings in one line, in order.
fn quoted(line: &str) -> Vec<&str> { line.split('"').skip(1).step_by(2).collect() }

/// Removes a TOML comment without treating a hash inside a quoted value as one.
fn without_comment(line: &str) -> &str {
    let mut quote: Option<char> = None;
    for (index, character) in line.char_indices() {
        match (quote, character) {
            (None, '"' | '\'') => quote = Some(character),
            (Some(open), _) if character == open => quote = None,
            (None, '#') => return line.get(..index).unwrap_or(line).trim_end(),
            _ => {}
        }
    }
    line
}

/// Reads one configuration line. A `rustflags` entry is a one-line array of
/// strings, which is the shape the standard prescribes; an entry spread over
/// several lines is refused rather than half read.
///
/// ```text
/// read_line("[build]")                     -> Line::Table("build")
/// read_line("rustflags = [\"-Zthreads=8\"]") -> Line::Rustflags(..)
/// ```
fn read_line(raw: &str) -> Result<Line, String> {
    let line = without_comment(raw);
    if line.starts_with('[') {
        return Ok(Line::Table(
            line.trim_matches(|c| c == '[' || c == ']')
                .trim()
                .to_owned(),
        ));
    }
    let Some(value) = line
        .strip_prefix("rustflags")
        .filter(|value| value.trim_start().starts_with('='))
    else {
        return Ok(Line::Other);
    };
    if !value.contains(']') {
        return Err("a `rustflags` array spans lines; keep it on one".to_owned());
    }
    Ok(Line::Rustflags(Flags::from_words(quoted(value))))
}

/// Returns every `rustflags` source in a Cargo configuration.
fn sources(config: &str) -> Result<Vec<Source>, String> {
    let mut table = String::new();
    let mut found = Vec::new();
    for line in config
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#'))
    {
        match read_line(line)? {
            Line::Table(name) => table = name,
            Line::Rustflags(flags) => found.push(Source {
                table: table.clone(),
                flags,
            }),
            Line::Other => {}
        }
    }
    Ok(found)
}

/// Returns the complaints about which sources exist: there must be a Linux
/// table, and a nightly pin needs a `[build]` source for the other hosts.
fn shape_problems(found: &[Source], pin: Pin) -> Problems {
    let checks = [
        (found.is_empty(), "no rustflags source"),
        (
            !found.iter().any(Source::is_linux),
            "no Linux target table carries rustflags",
        ),
        (
            pin.takes_threads() && !found.iter().any(|source| source.table == "build"),
            "no [build] rustflags for non-Linux hosts",
        ),
    ];
    checks
        .into_iter()
        .filter(|(failed, _)| *failed)
        .map(|(_, text)| text.to_owned())
        .collect()
}

/// Returns a complaint when the sources differ in anything but the linker, since
/// Cargo applies one source rather than merging them.
fn drift_problem(found: &[Source]) -> Option<String> {
    let mut stripped: Vec<Vec<&String>> = found
        .iter()
        .map(|source| source.flags.without_linker_flag())
        .collect();
    stripped.dedup();
    (stripped.len() > 1).then(|| format!("sources differ beyond the linker: {stripped:?}"))
}

/// Returns every complaint about the configuration sources.
///
/// ```text
/// config_problems(CONFIG, Pin::read(TOOLCHAIN)) == Ok(vec![])   // a compliant repository
/// ```
///
/// # Errors
///
/// Returns the reason when the configuration cannot be read.
pub fn config_problems(config: &str, pin: Pin) -> Result<Problems, String> {
    let found = sources(config)?;
    let mut problems = shape_problems(&found, pin);
    problems.extend(found.iter().filter_map(|source| source.problem(pin)));
    problems.extend(drift_problem(&found));
    Ok(problems)
}

#[cfg(test)]
mod pin_tests {
    //! Named positive and negative fixtures for the toolchain pin reader.

    use rstest::rstest;

    use super::{Pin, PinParseError};

    /// Scenario: toolchain files pinning each supported kind of channel.
    #[rstest]
    #[case::nightly("[toolchain]\nchannel = \"nightly-2026-05-28\"\n", Pin::Nightly)]
    #[case::stable("[toolchain]\nchannel = \"1.94.0\"\n", Pin::Stable)]
    fn the_pin_reader_tells_the_channels_apart(#[case] toolchain: &str, #[case] expected: Pin) {
        assert_eq!(Pin::read(toolchain), Ok(expected));
    }

    /// Scenario: invalid declarations cannot silently select a stable channel.
    #[rstest]
    #[case::missing("[toolchain]\n", PinParseError::Missing)]
    #[case::malformed_declaration("[toolchain]\nchannel: \"nightly\"\n", PinParseError::Malformed("channel: \"nightly\"".to_owned()))]
    #[case::bare_channel("[toolchain]\nchannel = 1.94.0\n", PinParseError::Malformed("channel = 1.94.0".to_owned()))]
    #[case::repeated_channel("[toolchain]\nchannel = \"nightly\"\nchannel = \"stable\"\n", PinParseError::Duplicate("channel = \"stable\"".to_owned()))]
    #[case::beta("[toolchain]\nchannel = \"beta\"\n", PinParseError::Unsupported("beta".to_owned()))]
    #[case::invalid_version("[toolchain]\nchannel = \"1.bad\"\n", PinParseError::Unsupported("1.bad".to_owned()))]
    fn the_pin_reader_rejects_indeterminate_channels(
        #[case] toolchain: &str,
        #[case] expected: PinParseError,
    ) {
        assert_eq!(Pin::read(toolchain), Err(expected));
    }
}
