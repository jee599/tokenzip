//! CLI smoke tests: verify tokenzip commands don't panic.
//!
//! These tests build and run the actual binary, checking that each
//! subcommand exits without crashing (exit code 0 or 2 for --help/--version).

use std::process::Command;

/// Get the path to the cargo-built binary.
fn tokenzip_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_tokenzip"))
}

#[test]
fn smoke_version() {
    let output = tokenzip_bin()
        .arg("--version")
        .output()
        .expect("Failed to execute tokenzip --version");
    assert!(
        output.status.success(),
        "tokenzip --version failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("tokenzip"),
        "Version output should contain 'tokenzip': {}",
        stdout
    );
}

#[test]
fn smoke_help() {
    let output = tokenzip_bin()
        .arg("--help")
        .output()
        .expect("Failed to execute tokenzip --help");
    assert!(
        output.status.success(),
        "tokenzip --help failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage") || stdout.contains("usage") || stdout.contains("USAGE"),
        "Help output should contain usage info: {}",
        stdout
    );
}

#[test]
fn smoke_gain() {
    let output = tokenzip_bin()
        .arg("gain")
        .output()
        .expect("Failed to execute tokenzip gain");
    // gain should succeed (shows summary or "no data yet" message)
    assert!(
        output.status.success(),
        "tokenzip gain failed with code {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn smoke_gain_by_feature() {
    let output = tokenzip_bin()
        .args(["gain", "--by-feature"])
        .output()
        .expect("Failed to execute tokenzip gain --by-feature");
    assert!(
        output.status.success(),
        "tokenzip gain --by-feature failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn smoke_gain_graph() {
    let output = tokenzip_bin()
        .args(["gain", "--graph"])
        .output()
        .expect("Failed to execute tokenzip gain --graph");
    assert!(
        output.status.success(),
        "tokenzip gain --graph failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn smoke_gain_history() {
    let output = tokenzip_bin()
        .args(["gain", "--history"])
        .output()
        .expect("Failed to execute tokenzip gain --history");
    assert!(
        output.status.success(),
        "tokenzip gain --history failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn smoke_init_show() {
    let output = tokenzip_bin()
        .args(["init", "--show"])
        .output()
        .expect("Failed to execute tokenzip init --show");
    assert!(
        output.status.success(),
        "tokenzip init --show failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
