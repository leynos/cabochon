.PHONY: help all clean test build release coverage lint lint-clippy lint-whitaker typecheck fmt check-fmt markdownlint install-markdownlint spelling nixie audit rust-audit install-build-tools check-build-tools test-workflow-contracts

SHELL := bash


TARGET ?= cabochon

# Prefer the verified local linker installation for both the preflight and
# Cargo, so a system package cannot shadow the repository's pinned version.
BUILD_TOOLS_PREFIX ?= $(HOME)/.local
export PATH := $(BUILD_TOOLS_PREFIX)/bin:$(HOME)/.cargo/bin:$(HOME)/.bun/bin:$(PATH)
INSTALL_BUILD_TOOLS ?= scripts/install-build-tools.sh
CHECK_BUILD_TOOLS ?= scripts/check-build-tools.sh
CARGO ?= cargo
BUILD_JOBS ?=
RUST_FLAGS ?=
RUST_FLAGS := -D warnings $(RUST_FLAGS)
RUSTDOC_FLAGS ?=
RUSTDOC_FLAGS := --cfg docsrs -D warnings $(RUSTDOC_FLAGS)
CARGO_FLAGS ?= --workspace --all-targets --all-features
CLIPPY_FLAGS ?= $(CARGO_FLAGS) -- $(RUST_FLAGS)
TEST_FLAGS ?= $(CARGO_FLAGS)
TEST_CMD := $(if $(shell command -v cargo-nextest 2>/dev/null),nextest run,test)
COVERAGE_LINKER_FLAGS ?= -fuse-ld=lld
COVERAGE_RUST_FLAGS ?= $(RUST_FLAGS) -C link-arg=$(COVERAGE_LINKER_FLAGS)
MDLINT ?= markdownlint-cli2
BUN ?= bun
UVX ?= uvx
# `make fmt` and `make check-fmt` call mdtablefix directly. `--git` selects the
# Markdown files Git tracks and `--include-untracked` adds the untracked files
# Git does not ignore, so a new document is formatted before it is staged.
# Both modes need mdtablefix 0.6.0 or later; CI pins the version at the
# install-mdtablefix step.
MDTABLEFIX ?= mdtablefix
MDTABLEFIX_SELECT = --git --include-untracked
MDTABLEFIX_RULES = --wrap --renumber --breaks --ellipsis --fences
NIXIE ?= nixie
WHITAKER ?= whitaker

UV ?= uv
UV_ENV = UV_CACHE_DIR=.uv-cache UV_TOOL_DIR=.uv-tools
# The CV-005 CodeScene contracts live in shared-actions and run from a full
# commit, so a fix is a pin bump. `.github/cv005.toml` holds this repository's
# only parameters.
CV005_CONTRACTS_REF ?= a38feb9be25755c30eca5bda96bd3786a5b89c6b
CV005_CONTRACTS = $(UV_ENV) $(UV) tool run --python 3.13 \
	--from 'git+https://github.com/leynos/shared-actions@$(CV005_CONTRACTS_REF)\#subdirectory=packages/cv005-contracts' \
	cv005-contracts

test-workflow-contracts: ## Check the CV-005 CodeScene workflow contracts
	$(CV005_CONTRACTS) check --repository .
# The development build standard (concordat rule `rust-build-defaults`):
# Cranelift is selected by the dev profile in .cargo/config.toml; the flags
# below restate the parallel frontend and target-scoped Linux `mold` linker.
# An assigned RUSTFLAGS replaces every `rustflags` table in .cargo/config.toml, so each
# recipe that sets it composes these onto any inherited value (CI's
# setup-rust exports one). Coverage, release, and Whitaker select LLVM.
BUILD_HOST_OS := $(shell uname -s)

# The resolver follows Cargo's --target precedence, stops at a Clippy `--`,
# and asks the pinned rustc for target_os. An unreadable target stops Make.
# Pass its result through `call` once; supported Ubuntu runners ship Make 4.3,
# which does not provide the newer `let` function.
TARGET_ROUTE = $(call CHECK_TARGET_ROUTE,$(shell BUILD_HOST_OS="$(BUILD_HOST_OS)" scripts/resolve-build-target.sh $(1) || printf route-error),$(1))
CHECK_TARGET_ROUTE = $(if $(filter route-error,$(1)),$(error Cannot classify Cargo target for $(2)),$(1))
NEEDS_MOULD = $(filter linux linux-clang,$(call TARGET_ROUTE,$(1)))
NEEDS_CLANG = $(filter linux-clang,$(call TARGET_ROUTE,$(1)))
STANDARD_RUSTFLAGS = -Zthreads=8$(if $(call NEEDS_MOULD,$(1)), -Clink-arg=-fuse-ld=mold)

# Target-specific values reach the prerequisite before each compilation route.
# A test's nextest and doctest commands can select different targets.
MOULD_REQUIRED = $(call NEEDS_MOULD,)
CLANG_REQUIRED = $(call NEEDS_CLANG,)
# `strip` expands both routes, so a bad later Cargo target cannot be hidden by
# an earlier route that already needs a tool.
test: MOULD_REQUIRED = $(strip $(call NEEDS_MOULD,$(TEST_FLAGS) $(BUILD_JOBS)) $(call NEEDS_MOULD,$(BUILD_JOBS)))
test: CLANG_REQUIRED = $(strip $(call NEEDS_CLANG,$(TEST_FLAGS) $(BUILD_JOBS)) $(call NEEDS_CLANG,$(BUILD_JOBS)))
target/debug/$(TARGET): MOULD_REQUIRED = $(call NEEDS_MOULD,$(BUILD_JOBS))
target/debug/$(TARGET): CLANG_REQUIRED = $(call NEEDS_CLANG,$(BUILD_JOBS))
lint-clippy: MOULD_REQUIRED = $(strip $(call NEEDS_MOULD,) $(call NEEDS_MOULD,$(CLIPPY_FLAGS)))
lint-clippy: CLANG_REQUIRED = $(strip $(call NEEDS_CLANG,) $(call NEEDS_CLANG,$(CLIPPY_FLAGS)))
typecheck: MOULD_REQUIRED = $(call NEEDS_MOULD,$(CARGO_FLAGS))
typecheck: CLANG_REQUIRED = $(call NEEDS_CLANG,$(CARGO_FLAGS))

build: target/debug/$(TARGET) ## Build debug binary
release: target/release/$(TARGET) ## Build release binary

# Keep composite gates ordered even when the caller passes `make -j`.
all: ## Perform a comprehensive check of code
	$(MAKE) check-fmt
	$(MAKE) markdownlint
	$(MAKE) spelling
	$(MAKE) lint
	$(MAKE) test
	$(MAKE) test-workflow-contracts

install-build-tools: ## Install the pinned Rust toolchain and verified `mold` binary
	@$(INSTALL_BUILD_TOOLS)
	@$(MAKE) check-build-tools

check-build-tools: ## Verify development build prerequisites
	@CHECK_MOULD="$(if $(MOULD_REQUIRED),yes,no)" CHECK_CLANG="$(if $(CLANG_REQUIRED),yes,no)" $(CHECK_BUILD_TOOLS)

clean: ## Remove build artefacts
	$(CARGO) clean

test: check-build-tools ## Run tests with warnings treated as errors
	RUSTFLAGS="$${RUSTFLAGS:+$$RUSTFLAGS }$(RUST_FLAGS) $(call STANDARD_RUSTFLAGS,$(TEST_FLAGS) $(BUILD_JOBS))" $(CARGO) $(TEST_CMD) $(TEST_FLAGS) $(BUILD_JOBS)
	RUSTFLAGS="$${RUSTFLAGS:+$$RUSTFLAGS }$(RUST_FLAGS) $(call STANDARD_RUSTFLAGS,$(BUILD_JOBS))" $(CARGO) test --doc --workspace --all-features $(BUILD_JOBS)

target/debug/$(TARGET): | check-build-tools ## Build the debug binary
	RUSTFLAGS="$${RUSTFLAGS:+$$RUSTFLAGS }$(RUST_FLAGS) $(call STANDARD_RUSTFLAGS,$(BUILD_JOBS))" $(CARGO) build $(BUILD_JOBS) --bin $(TARGET)

target/release/$(TARGET): ## Build the release binary with LLVM and the platform linker
	cd "$(dir $(CURDIR))" && env -u CARGO_ENCODED_RUSTFLAGS -u CARGO_PROFILE_DEV_CODEGEN_BACKEND -u CARGO_PROFILE_TEST_CODEGEN_BACKEND -u CARGO_PROFILE_RELEASE_CODEGEN_BACKEND RUSTFLAGS="" CARGO_TARGET_DIR="$(CURDIR)/target" $(CARGO) +stable build $(BUILD_JOBS) --release --manifest-path "$(CURDIR)/Cargo.toml" --bin $(TARGET)

coverage: ## Generate lcov coverage with lld for llvm-tools compatibility
	@echo "coverage linker flags: $(COVERAGE_LINKER_FLAGS)"
	env -u CARGO_ENCODED_RUSTFLAGS \
		CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang \
		CARGO_PROFILE_DEV_CODEGEN_BACKEND=llvm \
		CARGO_PROFILE_TEST_CODEGEN_BACKEND=llvm \
		RUSTFLAGS="$(COVERAGE_RUST_FLAGS)" \
		CFLAGS="$(COVERAGE_LINKER_FLAGS)" \
		LDFLAGS="$(COVERAGE_LINKER_FLAGS)" \
		$(CARGO) llvm-cov --lcov --output-path lcov.info $(TEST_FLAGS)

lint: ## Run rustdoc, Clippy, then Whitaker sequentially
	$(MAKE) lint-clippy
	$(MAKE) lint-whitaker

lint-clippy: check-build-tools ## Run rustdoc and Clippy under the development standard
	RUSTDOCFLAGS="$(RUSTDOC_FLAGS)" RUSTFLAGS="$${RUSTFLAGS:+$$RUSTFLAGS }$(RUST_FLAGS) $(call STANDARD_RUSTFLAGS,)" $(CARGO) doc --workspace --no-deps
	RUSTFLAGS="$${RUSTFLAGS:+$$RUSTFLAGS }$(RUST_FLAGS) $(call STANDARD_RUSTFLAGS,$(CLIPPY_FLAGS))" $(CARGO) clippy $(CLIPPY_FLAGS)

lint-whitaker: ## Run the installer-managed rolling Whitaker suite
	CARGO_ENCODED_RUSTFLAGS= RUSTFLAGS="" DYLINT_RUSTFLAGS="-D warnings" CARGO_PROFILE_DEV_CODEGEN_BACKEND=llvm CARGO_PROFILE_TEST_CODEGEN_BACKEND=llvm $(WHITAKER) --all -- $(CARGO_FLAGS)

typecheck: check-build-tools ## Type-check without building
	RUSTFLAGS="$${RUSTFLAGS:+$$RUSTFLAGS }$(RUST_FLAGS) $(call STANDARD_RUSTFLAGS,$(CARGO_FLAGS))" $(CARGO) check $(CARGO_FLAGS)

fmt: ## Format Rust and Markdown sources
	$(CARGO) fmt --all
	$(MDTABLEFIX) --in-place $(MDTABLEFIX_SELECT) $(MDTABLEFIX_RULES)
	$(MDLINT) --fix "**/*.md"

check-fmt: ## Verify formatting
	$(CARGO) fmt --all -- --check
	$(MDTABLEFIX) --check $(MDTABLEFIX_SELECT) $(MDTABLEFIX_RULES)

markdownlint: ## Lint Markdown files
	$(MDLINT) '**/*.md'

install-markdownlint: ## Install the pinned Markdown linter for local Make targets
	BUN_INSTALL_BIN="$(BUILD_TOOLS_PREFIX)/bin" $(BUN) add --global --exact markdownlint-cli2@0.22.1

spelling: ## Regenerate the spelling configuration and check source and prose
	$(UVX) --from "git+https://github.com/leynos/typos-config-builder.git@v0.1.3" typos-config-builder gate --scope all

nixie: ## Validate Mermaid diagrams
	$(NIXIE) --no-sandbox

audit: rust-audit ## Audit dependencies for known vulnerabilities

rust-audit: ## Audit the Rust workspace for known vulnerabilities
	set -eo pipefail; \
	manifest_list=$$(mktemp); \
	trap 'rm -f "$$manifest_list"' EXIT; \
	printf "Audit metadata phase: deriving workspace manifests\n"; \
	$(CARGO) metadata --no-deps --format-version 1 | python3 -c 'import json, sys; metadata = json.load(sys.stdin); members = set(metadata["workspace_members"]); print(metadata["workspace_root"]); [print(package["manifest_path"]) for package in metadata["packages"] if package["id"] in members]' > "$$manifest_list"; \
	workspace_root=$$(sed -n '1p' "$$manifest_list"); \
	audit_flags=(); \
	for advisory in $$CARGO_AUDIT_IGNORES; do \
		audit_flags+=(--ignore "$$advisory"); \
	done; \
	printf "Auditing Rust workspace %s\n" "$$workspace_root"; \
	sed -n '2,$$p' "$$manifest_list" | while IFS= read -r manifest; do \
		manifest_dir=$$(dirname "$$manifest"); \
		printf "Workspace Rust manifest %s\n" "$$manifest_dir/Cargo.toml"; \
	done; \
	printf "Audit execution phase: running cargo audit\n"; \
	printf "Audit failures may indicate RustSec advisories, cargo metadata errors, or documented ignores that need CARGO_AUDIT_IGNORES entries.\n"; \
	(cd "$$workspace_root" && $(CARGO) audit "$${audit_flags[@]}")

help: ## Show available targets
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS=":"; printf "Available targets:\n"} {printf "  %-20s %s\n", $$1, $$2}'
