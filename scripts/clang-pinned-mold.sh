#!/usr/bin/env bash
# Drive Linux development links through the pinned `mold` chosen on PATH.

set -euo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
pinned_version=$(<"$repo_root/tools/mold/VERSION")
if [[ ! $pinned_version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Invalid \`mold\` pin; inspect tools/mold/VERSION" >&2
  exit 1
fi

linker_path=$(command -v mold || true)
if [[ ! $linker_path == /* || ! -x $linker_path ]]; then
  echo "Pinned \`mold\` is unavailable on PATH; run make install-build-tools" >&2
  exit 1
fi

linker_bin_dir=$(dirname -- "$linker_path")
adjacent_linker="$linker_bin_dir/ld.mold"
if [[ ! -x $adjacent_linker || ! $linker_path -ef $adjacent_linker ]]; then
  echo "ld.mold must resolve to the \`mold\` binary at $linker_path; run make install-build-tools" >&2
  exit 1
fi

if ! installed_version=$("$adjacent_linker" --version | awk 'NR == 1 { print $2 }'); then
  echo "Cannot read the ld.mold version at $adjacent_linker; run make install-build-tools" >&2
  exit 1
fi
if [[ $installed_version != "$pinned_version" ]]; then
  echo "ld.mold $installed_version does not match pinned $pinned_version; run make install-build-tools" >&2
  exit 1
fi
if [[ $# == 1 && $1 == --check ]]; then
  exit 0
fi

if ! command -v clang >/dev/null 2>&1; then
  echo 'clang is required for the x86_64 GNU/Linux target; install it with your system package manager' >&2
  exit 1
fi

# Clang searches this directory before its system toolchain for -fuse-ld=mold.
exec clang -B"$linker_bin_dir" "$@"
