//! Expected stable release commands for native and cross matrix legs.

pub const NATIVE_BUILD: &str = concat!(
    "unset CARGO_ENCODED_RUSTFLAGS CARGO_PROFILE_DEV_CODEGEN_BACKEND ",
    "CARGO_PROFILE_TEST_CODEGEN_BACKEND CARGO_PROFILE_RELEASE_CODEGEN_BACKEND ",
    "cd \"$(dirname \"$GITHUB_WORKSPACE\")\" ",
    "cargo +stable build --release --manifest-path \"$GITHUB_WORKSPACE/Cargo.toml\" ",
    "--target ${{ matrix.target }}"
);
pub const CROSS_BUILD: &str = concat!(
    "unset CARGO_ENCODED_RUSTFLAGS CARGO_PROFILE_DEV_CODEGEN_BACKEND ",
    "CARGO_PROFILE_TEST_CODEGEN_BACKEND CARGO_PROFILE_RELEASE_CODEGEN_BACKEND ",
    "cd \"$(dirname \"$GITHUB_WORKSPACE\")\" ",
    "cross +stable build --release --manifest-path \"$GITHUB_WORKSPACE/Cargo.toml\" ",
    "--target ${{ matrix.target }}"
);
pub const PUBLISH_IF: &str = concat!(
    "github.event_name == 'push' || (github.event_name == 'workflow_dispatch' && ",
    "inputs.dry-run == false && startsWith(github.ref, 'refs/tags/'))"
);
