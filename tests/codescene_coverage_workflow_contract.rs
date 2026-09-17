//! Enforce source-equivalent local ratcheting and main-only `CodeScene` upload.

const CI: &str = include_str!("../.github/workflows/ci.yml");
const COVERAGE_MAIN: &str = include_str!("../.github/workflows/coverage-main.yml");

#[test]
fn pull_request_coverage_does_not_contact_codescene() {
    assert!(CI.contains("with-ratchet: 'true'"));
    assert!(!CI.contains("upload-codescene-coverage"));
    assert!(!CI.contains("CS_ACCESS_TOKEN"));
    assert!(!CI.contains("cs-coverage"));
    assert!(!CI.contains("project-url:"));
    assert!(!CI.contains("fetch-depth: 0"));
}

#[test]
fn main_coverage_keeps_lld_and_explicit_upload() {
    let main = COVERAGE_MAIN;
    assert!(main.contains("branches: [main]"));
    assert!(main.contains("RUSTFLAGS: -C link-arg=-fuse-ld=lld"));
    assert!(main.contains("mode: upload"));
    assert!(main.contains("upload-codescene-coverage@152d9c4784d0ae5877938a984fe6d1f04d718fd8"));
}
