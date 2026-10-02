//! Capability-scoped fixtures for build-tool installer contracts.

use std::{
    env,
    io,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

use camino::{Utf8Path, Utf8PathBuf};
use cap_std::{
    ambient_authority,
    fs::{Permissions, PermissionsExt},
    fs_utf8::Dir,
};
const INSTALLER: &str = include_str!("../../scripts/install-build-tools.sh");
const TOOLCHAIN: &str = include_str!("../../rust-toolchain.toml");
/// Version from the repository's pinned linker fixture.
pub(super) const LINKER_VERSION: &str = include_str!("../../tools/mold/VERSION");
const FIXTURE_ARCHIVE: &[u8] = b"fixture linker archive\n";

const FAKE_CURL: &str = r#"#!/usr/bin/env bash
set -euo pipefail
printf 'curl %s\n' "$*" >> "$FIXTURE_LOG"
[[ ${FAKE_CURL_FAIL:-no} != yes ]] || exit 31
output=
while (($#)); do
  if [[ $1 == --output ]]; then
    output=$2
    shift 2
  else
    shift
  fi
done
cp "$FIXTURE_ARCHIVE" "$output"
"#;

const FAKE_SHA256SUM: &str = r#"#!/usr/bin/env bash
set -euo pipefail
printf 'sha256sum %s\n' "$*" >> "$FIXTURE_LOG"
[[ ${FAKE_SHA256SUM_FAIL:-no} != yes ]] || exit 32
exec "$REAL_SHA256SUM" "$@"
"#;

const FAKE_TAR: &str = r#"#!/usr/bin/env bash
set -euo pipefail
printf 'tar %s\n' "$*" >> "$FIXTURE_LOG"
[[ ${FAKE_TAR_FAIL:-no} != yes ]] || exit 33
destination=
while (($#)); do
  if [[ $1 == --directory ]]; then
    destination=$2
    shift 2
  else
    shift
  fi
done
mkdir -p "$destination"
printf 'linker fixture\n' > "$destination/mold"
printf 'linker fixture\n' > "$destination/ld.mold"
"#;

const FAKE_RUSTUP: &str = r#"#!/usr/bin/env bash
set -euo pipefail
printf 'rustup %s\n' "$*" >> "$FIXTURE_LOG"
[[ ${FAKE_RUSTUP_FAIL:-no} != yes ]] || exit 34
"#;

const FAKE_UNAME: &str = r#"#!/usr/bin/env bash
set -euo pipefail
case ${1-} in
  -s) printf '%s\n' "${FAKE_UNAME_SYSTEM:-Linux}" ;;
  -m) printf '%s\n' "${FAKE_UNAME_MACHINE:-x86_64}" ;;
  *) exit 35 ;;
esac
"#;

static NEXT_SCRATCH: AtomicU64 = AtomicU64::new(0);

/// Owns a capability-scoped temporary directory and removes it after the test.
struct Scratch {
    path: Utf8PathBuf,
    name: String,
    parent: Dir,
    dir: Dir,
}

impl Scratch {
    /// Creates a unique temporary directory without following a supplied path.
    pub(super) fn new() -> io::Result<Self> {
        let system_path = env::temp_dir();
        let parent_path = Utf8Path::from_path(&system_path)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "temporary path is not UTF-8")
            })?
            .to_owned();
        let parent = Dir::open_ambient_dir(&parent_path, ambient_authority())?;
        let name = format!(
            "cabochon-build-tools-{}-{}",
            std::process::id(),
            NEXT_SCRATCH.fetch_add(1, Ordering::Relaxed)
        );
        parent.create_dir_all(&name)?;
        let dir = parent.open_dir(&name)?;
        Ok(Self {
            path: parent_path.join(&name),
            name,
            parent,
            dir,
        })
    }

    /// Writes an executable fixture below the temporary root.
    fn executable(&self, path: &str, contents: &str) -> io::Result<()> {
        self.dir.write(path, contents)?;
        self.dir
            .set_permissions(path, Permissions::from_mode(0o755))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) { let _cleanup = self.parent.remove_dir_all(&self.name); }
}

/// Holds the fake checkout, tool executables, archive, and install prefix.
pub(super) struct InstallerFixture {
    scratch: Scratch,
    fake_bin: Utf8PathBuf,
    prefix: Utf8PathBuf,
    archive: Utf8PathBuf,
    log: Utf8PathBuf,
}

impl InstallerFixture {
    /// Builds an isolated checkout and tools that record every side effect.
    pub(super) fn new() -> io::Result<Self> {
        let scratch = Scratch::new()?;
        prepare_checkout(&scratch)?;
        let archive = prepare_archive(&scratch)?;
        prepare_fake_tools(&scratch)?;
        Ok(Self {
            fake_bin: scratch.path.join("fake-bin"),
            prefix: scratch.path.join("prefix"),
            archive,
            log: scratch.path.join("commands.log"),
            scratch,
        })
    }

    /// Runs the copied installer with only the fixture tools on its PATH.
    pub(super) fn run_installer(&self, overrides: &[(&str, &str)]) -> io::Result<Output> {
        let path = format!("{}:/usr/bin:/bin", self.fake_bin);
        let installer = self.scratch.path.join("scripts/install-build-tools.sh");
        let mut command = Command::new(installer.as_str());
        command
            .current_dir(self.scratch.path.as_str())
            .env("PATH", path)
            .env("BUILD_TOOLS_PREFIX", self.prefix.as_str())
            .env("MOLD_RELEASE_BASE_URL", "https://fixture.invalid/releases")
            .env("FIXTURE_ARCHIVE", self.archive.as_str())
            .env("FIXTURE_LOG", self.log.as_str())
            .env("REAL_SHA256SUM", "/usr/bin/sha256sum")
            .env_remove("BASH_ENV");
        for (name, value) in overrides {
            command.env(name, value);
        }
        command.output()
    }

    /// Returns the commands called by the installer fixtures.
    pub(super) fn log(&self) -> io::Result<String> {
        self.scratch.dir.read_to_string("commands.log")
    }

    /// Replaces one file in the temporary checkout.
    pub(super) fn write_file(&self, path: &str, value: &str) -> io::Result<()> {
        self.scratch.dir.write(path, value)
    }

    /// Returns the install prefix used by the script.
    pub(super) fn prefix(&self) -> &Utf8Path { &self.prefix }

    /// Installs command doubles for the real Make target's two recipe stages.
    pub(super) fn make_route_probes(&self) -> io::Result<()> {
        self.scratch.executable(
            "prefix/bin/probe-install",
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'install\\n' >> \
             \"$MAKE_PROBE_LOG\"\nexit \"${MAKE_PROBE_INSTALL_STATUS:-0}\"\n",
        )?;
        self.scratch.executable(
            "prefix/bin/probe-check",
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'check\\n' >> \
             \"$MAKE_PROBE_LOG\"\nexit \"${MAKE_PROBE_CHECK_STATUS:-0}\"\n",
        )?;
        self.scratch.dir.write("make-route.log", "")
    }

    /// Runs Make's installer route using the controlled install and check tools.
    pub(super) fn run_make_route(&self, overrides: &[(&str, &str)]) -> io::Result<Output> {
        let mut command = Command::new("make");
        command
            .args([
                "--silent",
                "install-build-tools",
                &format!("BUILD_TOOLS_PREFIX={}", self.prefix),
                "INSTALL_BUILD_TOOLS=probe-install",
                "CHECK_BUILD_TOOLS=probe-check",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .env("PATH", "/usr/bin:/bin")
            .env(
                "MAKE_PROBE_LOG",
                self.scratch.path.join("make-route.log").as_str(),
            )
            .env_remove("MAKEFLAGS")
            .env_remove("BASH_ENV");
        for (name, value) in overrides {
            command.env(name, value);
        }
        command.output()
    }

    /// Clears the recorded Make route events.
    pub(super) fn clear_make_route_log(&self) -> io::Result<()> {
        self.scratch.dir.write("make-route.log", "")
    }

    /// Removes the fake installer so the missing-prerequisite path is exercised.
    pub(super) fn remove_rustup(&self) -> io::Result<()> {
        self.scratch.dir.remove_file("fake-bin/rustup")
    }

    /// Returns the events from Make's installer route.
    pub(super) fn make_route_log(&self) -> io::Result<String> {
        self.scratch.dir.read_to_string("make-route.log")
    }
}

fn prepare_checkout(scratch: &Scratch) -> io::Result<()> {
    for directory in ["scripts", "tools/mold", "prefix/bin"] {
        scratch.dir.create_dir_all(directory)?;
    }
    scratch
        .dir
        .write("scripts/install-build-tools.sh", INSTALLER)?;
    scratch.dir.set_permissions(
        "scripts/install-build-tools.sh",
        Permissions::from_mode(0o755),
    )?;
    scratch.dir.write("rust-toolchain.toml", TOOLCHAIN)?;
    scratch.dir.write("tools/mold/VERSION", LINKER_VERSION)
}

fn prepare_archive(scratch: &Scratch) -> io::Result<Utf8PathBuf> {
    let archive = scratch.path.join("linker.archive");
    scratch.dir.write("linker.archive", FIXTURE_ARCHIVE)?;
    let output = Command::new("/usr/bin/sha256sum")
        .arg(archive.as_str())
        .output()?;
    if !output.status.success() {
        return Err(io::Error::other("sha256sum failed for fixture archive"));
    }
    let checksum_output = String::from_utf8(output.stdout)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let checksum = checksum_output
        .split_whitespace()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "digest is missing"))?;
    let version = LINKER_VERSION.trim();
    let checksums = format!(
        "{checksum}  mold-{version}-x86_64-linux.tar.gz\n{checksum}  \
         mold-{version}-aarch64-linux.tar.gz\n"
    );
    scratch.dir.write("tools/mold/SHA256SUMS", checksums)?;
    Ok(archive)
}

fn prepare_fake_tools(scratch: &Scratch) -> io::Result<()> {
    scratch.dir.create_dir_all("fake-bin")?;
    for (path, contents) in [
        ("fake-bin/curl", FAKE_CURL),
        ("fake-bin/sha256sum", FAKE_SHA256SUM),
        ("fake-bin/tar", FAKE_TAR),
        ("fake-bin/rustup", FAKE_RUSTUP),
        ("fake-bin/uname", FAKE_UNAME),
    ] {
        scratch.executable(path, contents)?;
    }
    scratch.dir.write("commands.log", "")
}
