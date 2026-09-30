//! Checks the starter application's observable command-line greeting.

use std::{io, process::Command};

/// Scenario: invoking the generated application prints its greeting once.
#[test]
fn starter_application_prints_its_greeting() -> io::Result<()> {
    let output = Command::new(env!("CARGO_BIN_EXE_cabochon")).output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "application exited {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    if output.stdout != b"Hello from Cabochon!\n" {
        return Err(io::Error::other(format!(
            "unexpected stdout: {:?}",
            String::from_utf8_lossy(&output.stdout)
        )));
    }
    Ok(())
}
