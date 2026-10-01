#!/usr/bin/env bash
# Classify one Cargo invocation's compilation target for Make's build route.

set -euo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
toolchain=$(sed -nE 's/^channel = "([^"]+)"$/\1/p' "$repo_root/rust-toolchain.toml")
explicit_target=

fail() {
  echo "Cannot determine Cargo target: $*" >&2
  exit 1
}

set_target() {
  local candidate=$1
  [[ -n $candidate ]] || fail 'empty --target value'
  if [[ -n $explicit_target && $explicit_target != "$candidate" ]]; then
    fail "conflicting --target values: $explicit_target and $candidate"
  fi
  explicit_target=$candidate
}

while (($#)); do
  case $1 in
    --) break ;;
    --target)
      (($# >= 2)) || fail '--target has no value'
      set_target "$2"
      shift 2
      ;;
    --target=*)
      set_target "${1#--target=}"
      shift
      ;;
    *) shift ;;
  esac
done

[[ $toolchain =~ ^nightly-[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || fail 'invalid repository toolchain pin'
target=${explicit_target:-${CARGO_BUILD_TARGET:-}}
if [[ -z $target ]]; then
  version=$(rustup run "$toolchain" rustc -vV) || fail "pinned rustc $toolchain is unavailable"
  target=$(sed -n 's/^host: //p' <<< "$version")
  [[ -n $target ]] || fail "pinned rustc $toolchain reported no host triple"
fi

cfg=$(rustup run "$toolchain" rustc --print cfg --target "$target") || fail "rustc cannot classify $target"
target_os=$(sed -nE 's/^target_os="([^"]+)"$/\1/p' <<< "$cfg")
[[ -n $target_os && $target_os != *$'\n'* ]] || fail "rustc reported no unique target_os for $target"

host_os=${BUILD_HOST_OS:-$(uname -s)}
if [[ $target_os == linux ]]; then
  [[ $host_os == Linux ]] || fail "Linux target $target needs a supported Linux build host"
  if [[ $target == x86_64-unknown-linux-gnu ]]; then
    printf 'linux-clang\n'
  else
    printf 'linux\n'
  fi
else
  printf 'other\n'
fi
