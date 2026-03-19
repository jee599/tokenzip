//! CLI smoke tests: verify contextzip commands don't panic.
//!
//! These tests build and run the actual binary, checking that each
//! subcommand exits without crashing (exit code 0 or 2 for --help/--version).

use std::process::Command;

/// Get the path to the cargo-built binary.
fn contextzip_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_contextzip"))
}

#[test]
fn smoke_version() {
    let output = contextzip_bin()
        .arg("--version")
        .output()
        .expect("Failed to execute contextzip --version");
    assert!(
        output.status.success(),
        "contextzip --version failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("contextzip"),
        "Version output should contain 'contextzip': {}",
        stdout
    );
}

#[test]
fn smoke_help() {
    let output = contextzip_bin()
        .arg("--help")
        .output()
        .expect("Failed to execute contextzip --help");
    assert!(
        output.status.success(),
        "contextzip --help failed: {}",
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
    let output = contextzip_bin()
        .arg("gain")
        .output()
        .expect("Failed to execute contextzip gain");
    // gain should succeed (shows summary or "no data yet" message)
    assert!(
        output.status.success(),
        "contextzip gain failed with code {:?}: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn smoke_gain_by_feature() {
    let output = contextzip_bin()
        .args(["gain", "--by-feature"])
        .output()
        .expect("Failed to execute contextzip gain --by-feature");
    assert!(
        output.status.success(),
        "contextzip gain --by-feature failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn smoke_gain_graph() {
    let output = contextzip_bin()
        .args(["gain", "--graph"])
        .output()
        .expect("Failed to execute contextzip gain --graph");
    assert!(
        output.status.success(),
        "contextzip gain --graph failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn smoke_gain_history() {
    let output = contextzip_bin()
        .args(["gain", "--history"])
        .output()
        .expect("Failed to execute contextzip gain --history");
    assert!(
        output.status.success(),
        "contextzip gain --history failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn smoke_init_show() {
    let output = contextzip_bin()
        .args(["init", "--show"])
        .output()
        .expect("Failed to execute contextzip init --show");
    assert!(
        output.status.success(),
        "contextzip init --show failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
