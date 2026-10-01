#!/usr/bin/env bash
# Install the pinned Rust development toolchain and a verified `mold` binary.

set -euo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
prefix=${BUILD_TOOLS_PREFIX:-$HOME/.local}
mold_version=$(<"$repo_root/tools/mold/VERSION")
toolchain=$(sed -nE 's/^channel = "([^"]+)"$/\1/p' "$repo_root/rust-toolchain.toml")
component_list=(rustfmt clippy rust-analyzer llvm-tools-preview rustc-codegen-cranelift-preview)

if [[ ! $mold_version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ || ! $toolchain =~ ^nightly-[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
  echo "Invalid \`mold\` or nightly toolchain pin; inspect tools/mold/VERSION and rust-toolchain.toml" >&2
  exit 1
fi

if [[ $(uname -s) == Linux ]]; then
  case "$(uname -m)" in
    x86_64) mold_arch=x86_64 ;;
    aarch64) mold_arch=aarch64 ;;
    *) echo "No pinned \`mold\` binary for architecture $(uname -m)" >&2; exit 1 ;;
  esac
  archive="mold-$mold_version-$mold_arch-linux.tar.gz"
  checksum_file="$repo_root/tools/mold/SHA256SUMS"
  checksum=$(awk -v name="$archive" '$2 == name { print $1 }' "$checksum_file")
  if [[ ! $checksum =~ ^[[:xdigit:]]{64}$ ]]; then
    echo "Expected exactly one SHA-256 for $archive in $checksum_file" >&2
    exit 1
  fi
  workdir=$(mktemp -d)
  trap 'rm -rf -- "$workdir"' EXIT
  release_base=${MOLD_RELEASE_BASE_URL:-https://github.com/rui314/mold/releases/download}
  curl --fail --silent --show-error --location \
    --connect-timeout 15 --speed-limit 1024 --speed-time 60 \
    --output "$workdir/$archive" "$release_base/v$mold_version/$archive"
  printf '%s  %s\n' "$checksum" "$workdir/$archive" | sha256sum --check --status || {
    echo "Checksum mismatch for $archive; refusing to install" >&2
    exit 1
  }
  mkdir -p "$prefix"
  tar --extract --gzip --strip-components=1 --directory "$prefix" --file "$workdir/$archive"
  echo "Installed verified \`mold\` $mold_version into $prefix"
fi

command -v rustup >/dev/null 2>&1 || {
  echo 'rustup is required; install it from https://rustup.rs' >&2
  exit 1
}
component_args=()
for component in "${component_list[@]}"; do
  component_args+=(--component "$component")
done
rustup toolchain install "$toolchain" --profile minimal "${component_args[@]}"
echo 'Run make check-build-tools to verify the installation'
