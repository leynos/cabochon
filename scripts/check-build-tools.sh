#!/usr/bin/env bash
# Check the capabilities needed by the default development build.

set -euo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
toolchain=$(sed -nE 's/^channel = "([^"]+)"$/\1/p' "$repo_root/rust-toolchain.toml")
status=0

if [[ ${CHECK_MOULD-} != yes && ${CHECK_MOULD-} != no ]] ||
  [[ ${CHECK_CLANG-} != yes && ${CHECK_CLANG-} != no ]]; then
  echo 'Build-tool route is missing; run the check through make check-build-tools' >&2
  exit 1
fi

if [[ ${CARGO_ENCODED_RUSTFLAGS+x} == x ]]; then
  echo 'CARGO_ENCODED_RUSTFLAGS overrides the development standard; unset it before using standard Make targets' >&2
  status=1
fi
if [[ ${CARGO_PROFILE_DEV_CODEGEN_BACKEND+x} == x && ${CARGO_PROFILE_DEV_CODEGEN_BACKEND-} != cranelift ]]; then
  echo 'CARGO_PROFILE_DEV_CODEGEN_BACKEND must select Cranelift for development; unset the override or set it to cranelift' >&2
  status=1
fi
if [[ ${CARGO_PROFILE_TEST_CODEGEN_BACKEND+x} == x && ${CARGO_PROFILE_TEST_CODEGEN_BACKEND-} != cranelift ]]; then
  echo 'CARGO_PROFILE_TEST_CODEGEN_BACKEND must select Cranelift for tests; unset the override or set it to cranelift' >&2
  status=1
fi

if [[ ! $toolchain =~ ^nightly-[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
  echo 'Invalid Rust toolchain pin; inspect rust-toolchain.toml' >&2
  exit 1
fi

if [[ $CHECK_CLANG == yes ]] && ! command -v clang >/dev/null 2>&1; then
  echo 'clang is required for the x86_64 GNU/Linux target; install it with your system package manager' >&2
  status=1
fi
if [[ $CHECK_MOULD == yes ]]; then
  if ! "$repo_root/scripts/clang-pinned-mold.sh" --check; then
    status=1
  fi
fi

if ! command -v rustup >/dev/null 2>&1; then
  echo 'rustup is required; install it from https://rustup.rs' >&2
  exit 1
fi
if ! rustup toolchain list | grep -q "^$toolchain"; then
  echo "Toolchain $toolchain is missing; run make install-build-tools" >&2
  status=1
else
  installed_components=$(rustup component list --toolchain "$toolchain" --installed)
  for component in rustfmt clippy rust-analyzer llvm-tools-preview rustc-codegen-cranelift-preview; do
    # rustup accepts the -preview aliases but lists their installed names
    # without that suffix.
    installed_name=${component%-preview}
    if ! grep -q "^$installed_name-" <<< "$installed_components"; then
      echo "Component $component is missing from $toolchain; run make install-build-tools" >&2
      status=1
    fi
  done
fi

exit "$status"
