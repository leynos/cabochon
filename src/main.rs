//! `Cabochon` application entry point.

use std::io::{self, Write};

/// Application entry point.
fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut output = stdout.lock();
    writeln!(output, "{}", cabochon::greet())
}
