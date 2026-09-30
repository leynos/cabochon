//! Proves the configured Linux linker selects the pinned adjacent `ld.mold`.

use std::process::Command;

use rstest::rstest;

use super::CONFIG;

/// Make's executable preflight, which must inspect the Cargo linker route.
const PREFLIGHT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/scripts/check-build-tools.sh"
));
/// The version expected of the adjacent linker binary.
const PIN: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tools/mold/VERSION"));
/// Cargo's configured Linux linker launcher.
const LAUNCHER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/clang-pinned-mold.sh");

/// Builds controlled PATH entries and invokes the real launcher without compiling.
const PROBE: &str = r#"
set -euo pipefail
probe_dir=$(mktemp -d)
trap 'rm -rf -- "$probe_dir"' EXIT

if [[ $FIXTURE_CASE != missing_mold ]]; then
  cat > "$probe_dir/mold" <<'MOULD'
#!/usr/bin/env bash
printf 'mold %s\n' "$FAKE_MOLD_VERSION"
MOULD
  chmod +x "$probe_dir/mold"
fi

if [[ $FIXTURE_CASE == different_binary ]]; then
  cp "$probe_dir/mold" "$probe_dir/ld.mold"
elif [[ $FIXTURE_CASE != missing_linker && $FIXTURE_CASE != missing_mold ]]; then
  ln -s mold "$probe_dir/ld.mold"
fi

cat > "$probe_dir/clang" <<'CLANG'
#!/usr/bin/env bash
printf '%s\n' "$@"
CLANG
chmod +x "$probe_dir/clang"
for tool in bash dirname awk grep rm; do
  ln -s "$(command -v "$tool")" "$probe_dir/$tool"
done
export PATH="$probe_dir"

if [[ $FIXTURE_CASE == good ]]; then
  "$LAUNCHER" --check
  output=$("$LAUNCHER" -fuse-ld=mold --version)
  expected=$(printf '%s\n' "-B$probe_dir" '-fuse-ld=mold' '--version')
  [[ $output == "$expected" ]]
else
  if "$LAUNCHER" --check > "$probe_dir/check.out" 2> "$probe_dir/check.err"; then
    exit 1
  fi
  case "$FIXTURE_CASE" in
    stale_version) grep -Fq 'does not match pinned' "$probe_dir/check.err" ;;
    missing_mold) grep -Fq 'Pinned mold is unavailable on PATH' "$probe_dir/check.err" ;;
    missing_linker|different_binary)
      grep -Fq 'must resolve to the mold binary' "$probe_dir/check.err"
      ;;
  esac
fi
"#;

/// Checks that the live Cargo and Make routes use the one validated launcher.
#[test]
fn cargo_and_make_share_the_pinned_linker() {
    assert!(
        CONFIG.contains("linker = \"scripts/clang-pinned-mold.sh\""),
        "Cargo must use the pinned linker launcher"
    );
    assert!(
        PREFLIGHT.contains("scripts/clang-pinned-mold.sh\" --check"),
        "Make's preflight must validate the same linker launcher"
    );
}

/// Missing or stale `mold`, or missing or unrelated `ld.mold`, fails before linking.
#[rstest]
#[case::valid_pair("good", "pinned")]
#[case::stale_pair("stale_version", "0.0.0")]
#[case::missing_mold("missing_mold", "pinned")]
#[case::missing_linker("missing_linker", "pinned")]
#[case::different_binary("different_binary", "pinned")]
fn launcher_uses_only_the_pinned_linker(#[case] fixture_case: &str, #[case] version: &str) {
    let selected_version = if version == "pinned" {
        PIN.trim()
    } else {
        version
    };
    let output = Command::new("bash")
        .args(["-c", PROBE])
        .env("LAUNCHER", LAUNCHER)
        .env("FIXTURE_CASE", fixture_case)
        .env("FAKE_MOLD_VERSION", selected_version)
        .output()
        .expect("run controlled linker probe");
    assert!(
        output.status.success(),
        "{fixture_case}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
