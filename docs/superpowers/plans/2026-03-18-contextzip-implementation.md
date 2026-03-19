# ContextZip Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fork RTK and build ContextZip — a Claude Code context optimizer that adds 6 noise filters (error stacktrace, web page, ANSI, build error, package install, Docker) on top of RTK's existing 34 CLI compression modules.

**Architecture:** Full fork of rtk-ai/rtk with rtk→contextzip renaming. New modules plug into RTK's existing Clap enum router and SQLite tracking. ANSI filter runs as a preprocessor on all output before command-specific modules. Error stacktrace detection runs as a post-processor after all modules.

**Tech Stack:** Rust, Clap 4, rusqlite, regex, scraper (HTML parsing), serde, colored, chrono

**Spec:** `docs/superpowers/specs/2026-03-18-contextzip-design.md`
**Original Spec:** `/Users/jidong/Downloads/spoon-context-impl-spec.md`

---

## File Structure

### Existing (from RTK fork, renamed)
- `Cargo.toml` — crate name/bin → contextzip, add scraper dependency
- `src/main.rs` — CLI router, add new Commands enum variants
- `src/tracking.rs` — SQLite tracking, add `feature` column
- `src/gain.rs` — gain dashboard, add `--by-feature`, `--graph`, `--history`
- `src/init.rs` — hook installation, rtk→contextzip paths
- `src/config.rs` — config paths rtk→contextzip
- `src/discover/registry.rs` — rewrite rules rtk→contextzip
- `hooks/rtk-rewrite.sh` → `hooks/contextzip-rewrite.sh`
- All other `src/*.rs` — string replacements rtk→contextzip

### New Files
- `src/ansi_filter.rs` — ANSI/spinner/decoration preprocessor
- `src/error_cmd.rs` — Error stacktrace compression
- `src/web_cmd.rs` — Web page content extraction
- `src/build_cmd.rs` — Build error grouping
- `src/pkg_cmd.rs` — Package install log compression
- `src/docker_cmd.rs` — Docker build log compression (replaces container.rs docker handling)
- `install.sh` — One-line installer script
- `.github/workflows/release.yml` — Cross-platform build + release
- `tests/ansi_filter_test.rs` — ANSI filter integration tests
- `tests/error_cmd_test.rs` — Error compression integration tests
- `tests/web_cmd_test.rs` — Web compression integration tests
- `tests/build_cmd_test.rs` — Build grouping integration tests
- `tests/pkg_cmd_test.rs` — Package log integration tests
- `tests/docker_cmd_test.rs` — Docker log integration tests

---

## Week 1: RTK Fork + Install Experience

### Task 1: Clone RTK and Set Up Base

**Files:**
- Create: all files (RTK source copy)
- Modify: `Cargo.toml`

- [ ] **Step 1: Clone RTK source into contextzip repo**

```bash
cd /Users/jidong
git clone https://github.com/rtk-ai/rtk.git /tmp/rtk-source
# Copy all source files (excluding .git) into contextzip
cp -r /tmp/rtk-source/* /Users/jidong/contextzip/
cp /tmp/rtk-source/.gitignore /Users/jidong/contextzip/
```

- [ ] **Step 2: Verify RTK builds as-is**

Run: `cd /Users/jidong/contextzip && cargo build --release`
Expected: Successful build, binary at `target/release/rtk`

- [ ] **Step 3: Verify existing RTK tests pass**

Run: `cargo test`
Expected: All tests pass

- [ ] **Step 4: Commit RTK base**

```bash
git add -A
git commit -m "chore: import rtk source as fork base"
```

---

### Task 2: Rename rtk → contextzip (Cargo.toml + binary)

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Update Cargo.toml**

Change:
```toml
[package]
name = "contextzip"
# ...

[[bin]]
name = "contextzip"
path = "src/main.rs"
```

Add dependencies:
```toml
scraper = "0.21"
```

Note: RTK already includes `lazy_static`, `regex`, and other needed deps. Verify they're present in the existing Cargo.toml after fork.

- [ ] **Step 2: Build to confirm binary name change**

Run: `cargo build --release`
Expected: Binary at `target/release/contextzip`

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: rename crate and binary to contextzip"
```

---

### Task 3: Rename all source-level rtk references

**Files:**
- Modify: all `src/*.rs`, `hooks/*`, `build.rs`

This is a bulk rename operation. "rtk" → "contextzip" in:
- CLI help text, error messages, version strings
- Data directory paths (`rtk` → `contextzip` in dirs:: calls)
- Config file paths
- SQLite database paths
- Hook script references
- README, ARCHITECTURE.md references

- [ ] **Step 1: Bulk rename in source files**

Use `sed` or manual edits to replace:
- `"rtk"` → `"contextzip"` in user-facing strings
- `rtk` → `contextzip` in directory/path constants
- `RTK` → `ContextZip` in display names
- `rtk-rewrite.sh` → `contextzip-rewrite.sh`
- `RTK.md` → `CONTEXTZIP.md`

Careful NOT to rename:
- Rust std library references (no false positives)
- External crate names
- Git URLs pointing to rtk-ai/rtk (keep as attribution)

- [ ] **Step 2: Rename hook files**

```bash
mv hooks/rtk-rewrite.sh hooks/contextzip-rewrite.sh
mv hooks/rtk-awareness.md hooks/contextzip-awareness.md
```

Update hook script contents to reference `contextzip` binary.

- [ ] **Step 3: Build and run tests**

Run: `cargo build --release && cargo test`
Expected: All pass. Binary is `contextzip`.

- [ ] **Step 4: Verify CLI output**

Run: `./target/release/contextzip --version`
Expected: Contains "contextzip" and RTK base version (e.g., "contextzip 0.1.0 (based on rtk 0.30.0)")

Run: `./target/release/contextzip --help`
Expected: No "rtk" references in help text

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "refactor: rename all rtk references to contextzip"
```

---

### Task 4: Add MIT License with RTK Attribution

**Files:**
- Modify: `LICENSE`

- [ ] **Step 1: Update LICENSE**

Keep MIT license. Add at top:
```
Based on rtk (https://github.com/rtk-ai/rtk) by rtk-ai
Original work Copyright (c) rtk-ai contributors
Modifications Copyright (c) 2026 jee599
```

- [ ] **Step 2: Commit**

```bash
git add LICENSE
git commit -m "docs: add RTK attribution to license"
```

---

### Task 5: Extend SQLite Tracking with Feature Column

**Files:**
- Modify: `src/tracking.rs`
- Modify: `src/gain.rs`

- [ ] **Step 1: Write test for feature column**

In `src/tracking.rs` test module, add test that creates a record with `feature` field and queries it back.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test tracking`
Expected: FAIL — no `feature` field yet

- [ ] **Step 3: Add feature column to schema**

In `tracking.rs`:
- Add `feature: String` to `CommandRecord`
- ALTER TABLE migration: add `feature TEXT DEFAULT 'cli'` column
- Update `track()` method signature to accept `feature: &str`
- Update INSERT statement to include feature

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test tracking`
Expected: PASS

- [ ] **Step 5: Write test for gain --by-feature**

In `src/gain.rs` test module, add test that groups savings by feature.

- [ ] **Step 6: Implement gain --by-feature**

In `gain.rs`:
- Add `ByFeatureEntry { feature, commands, saved, avg_pct }`
- SQL: `SELECT feature, COUNT(*), SUM(saved_tokens), AVG(savings_pct) FROM tracking GROUP BY feature ORDER BY SUM(saved_tokens) DESC`
- Format output as table

In `main.rs`:
- Add `--by-feature` flag to gain subcommand

- [ ] **Step 7: Implement gain --graph**

In `gain.rs`:
- Add ASCII bar chart rendering for daily savings (last 30 days)
- Each day shows a bar proportional to tokens saved
- Format: `2026-03-18 ████████ 12.3K`

In `main.rs`:
- Add `--graph` flag to gain subcommand

- [ ] **Step 8: Implement gain --history**

In `gain.rs`:
- Add per-command history view: list recent commands with input/output/saved tokens
- Format: timestamp, command, input tokens, output tokens, savings %

In `main.rs`:
- Add `--history` flag to gain subcommand

- [ ] **Step 9: Run all tests**

Run: `cargo test`
Expected: All pass

- [ ] **Step 10: Commit**

```bash
git add src/tracking.rs src/gain.rs src/main.rs
git commit -m "feat: add feature column to tracking, gain --by-feature/--graph/--history"
```

---

### Task 6: Install Script

**Files:**
- Create: `install.sh`

- [ ] **Step 1: Write install.sh**

The script must:
1. Detect OS/arch (Linux x86_64, macOS arm64/x86_64, Windows)
2. Download binary from GitHub Releases (`https://github.com/jee599/contextzip/releases/latest/download/contextzip-{os}-{arch}`)
3. Install to `~/.local/bin/contextzip`
4. Check PATH for `~/.local/bin`
5. Run `contextzip init -g --hook-only --auto-patch`
6. Detect existing RTK installation, offer replace/coexist/cancel
7. Print success message with `contextzip gain` hint

```bash
#!/bin/bash
set -euo pipefail

VERSION="${CONTEXTZIP_VERSION:-latest}"
INSTALL_DIR="${HOME}/.local/bin"

# OS/arch detection
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64|amd64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Platform mapping
case "$OS" in
  linux) TARGET="contextzip-linux-${ARCH}" ;;
  darwin) TARGET="contextzip-macos-${ARCH}" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Download
if [ "$VERSION" = "latest" ]; then
  DOWNLOAD_URL="https://github.com/jee599/contextzip/releases/latest/download/${TARGET}"
else
  DOWNLOAD_URL="https://github.com/jee599/contextzip/releases/download/${VERSION}/${TARGET}"
fi
echo "Downloading contextzip..."
curl -fsSL "$DOWNLOAD_URL" -o /tmp/contextzip
chmod +x /tmp/contextzip

# Install
mkdir -p "$INSTALL_DIR"
mv /tmp/contextzip "$INSTALL_DIR/contextzip"

# PATH check
if ! echo "$PATH" | tr ':' '\n' | grep -q "$INSTALL_DIR"; then
  echo "⚠ Add to PATH: export PATH=\"$INSTALL_DIR:\$PATH\""
fi

# RTK detection
if command -v rtk &>/dev/null; then
  echo ""
  echo "RTK detected. contextzip includes all RTK features."
  echo "  [1] Replace RTK hooks (recommended)"
  echo "  [2] Install alongside RTK"
  echo "  [3] Cancel"
  read -r -p "Choice [1]: " choice
  case "${choice:-1}" in
    1) REPLACE_RTK=true ;;
    2) REPLACE_RTK=false ;;
    3) echo "Cancelled."; exit 0 ;;
  esac

  # Replace RTK hooks if chosen
  if [ "$REPLACE_RTK" = "true" ]; then
    # Remove RTK hook from settings.json
    if [ -f "$HOME/.claude/settings.json" ]; then
      sed -i.bak 's/rtk-rewrite\.sh/contextzip-rewrite.sh/g' "$HOME/.claude/settings.json"
      rm -f "$HOME/.claude/settings.json.bak"
    fi
    rm -f "$HOME/.claude/hooks/rtk-rewrite.sh"
    echo "✓ RTK hooks replaced with contextzip"
  fi
fi

# Hook installation
"$INSTALL_DIR/contextzip" init -g --hook-only --auto-patch

echo ""
echo "✓ contextzip installed to $INSTALL_DIR/contextzip"
echo "✓ Claude Code hook installed"
echo "✓ Ready! Restart Claude Code to activate."
echo ""
echo "  Quick check:  contextzip gain"
echo "  Full status:  contextzip init --show"
```

- [ ] **Step 2: Make executable**

```bash
chmod +x install.sh
```

- [ ] **Step 3: Commit**

```bash
git add install.sh
git commit -m "feat: add one-line install script"
```

---

### Task 7: GitHub Actions CI/CD

**Files:**
- Create: `.github/workflows/release.yml`
- Create: `.github/workflows/ci.yml`

- [ ] **Step 1: Write CI workflow**

`.github/workflows/ci.yml`:
```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test
      - run: cargo clippy -- -D warnings
```

- [ ] **Step 2: Write release workflow**

`.github/workflows/release.yml`:
- Trigger: tag push `v*`
- Matrix: linux-x86_64 (gnu+musl), macos-arm64, macos-x86_64, windows-x86_64
- Steps: checkout → rust toolchain → cross-compile → upload to release
- Generate SHA256 checksums

- [ ] **Step 3: Commit**

```bash
git add .github/
git commit -m "ci: add CI and cross-platform release workflows"
```

---

### Task 8: Update init and uninstall commands

**Files:**
- Modify: `src/init.rs`

- [ ] **Step 1: Verify init --show works with contextzip paths**

Run: `cargo run -- init --show`
Expected: Shows contextzip paths (not rtk):
```
✅ Binary: ~/.local/bin/contextzip (v0.1.0)
✅ Hook: ~/.claude/hooks/contextzip-rewrite.sh (executable)
✅ Settings: ~/.claude/settings.json (PreToolUse registered)
✅ Database: ~/.local/share/contextzip/tracking.db
```

- [ ] **Step 2: Verify init -g installs hook + awareness file**

Run: `cargo run -- init -g` (in a temp environment)
Expected: Creates `~/.claude/hooks/contextzip-rewrite.sh` AND `CONTEXTZIP.md` awareness file.

- [ ] **Step 3: Verify init -g --hook-only installs hook only**

Run: `cargo run -- init -g --hook-only`
Expected: Creates hook but NOT the awareness file. This is what install.sh uses.

- [ ] **Step 4: Add uninstall subcommand if not present**

In `main.rs` add `Uninstall` command variant.
In `init.rs` or new module: remove hook, remove settings entry, remove binary.
Print message about SQLite data preservation (`--purge` to delete).

- [ ] **Step 5: Add update subcommand**

Self-update: download latest release binary and replace current.

- [ ] **Step 6: Test init, uninstall, update commands**

Run: `cargo test init`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add src/init.rs src/main.rs
git commit -m "feat: add uninstall and update commands"
```

---

### ★ Week 1 Checkpoint

**Verification:**
1. `cargo build --release` succeeds
2. `cargo test` — all tests pass
3. `./target/release/contextzip --version` — shows "contextzip 0.1.0 (based on rtk 0.30.0)"
4. `./target/release/contextzip --help` — no "rtk" references
5. `./target/release/contextzip gain` — works (shows "no data yet" message)
6. `./target/release/contextzip gain --by-feature` — works
6a. `./target/release/contextzip gain --graph` — works
6b. `./target/release/contextzip gain --history` — works
7. `./target/release/contextzip init --show` — shows contextzip paths
8. `./target/release/contextzip init -g` — installs hook + awareness file
9. `./target/release/contextzip init -g --hook-only` — installs hook only
10. All existing RTK command modules work (e.g., `contextzip git status`, `contextzip ls`)
11. `./target/release/contextzip discover` — works
12. Hook script references contextzip binary

**Stop here. Report results to user for review before proceeding to Week 2.**

---

## Week 2: Safe Features — ANSI, Error, Web

### Task 9: ANSI Filter Preprocessor

**Files:**
- Create: `src/ansi_filter.rs`
- Modify: `src/main.rs` (add mod declaration, wire into pipeline)

- [ ] **Step 1: Write failing tests for ANSI stripping**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_escape() {
        let input = "\x1b[32mSuccess\x1b[0m: done";
        assert_eq!(filter_ansi(input), "Success: done");
    }

    #[test]
    fn test_strip_spinner() {
        let input = "⠋ Loading...\n⠙ Loading...\n⠹ Loading...\n✓ Done";
        let result = filter_ansi(input);
        assert!(result.contains("✓ Done"));
        assert!(!result.contains("⠋"));
    }

    #[test]
    fn test_strip_progress_bar_keep_final() {
        let input = "████░░░░ 45%\n████████ 100%";
        let result = filter_ansi(input);
        assert!(result.contains("100%"));
        assert!(!result.contains("45%"));
    }

    #[test]
    fn test_strip_decoration_lines() {
        let input = "Header\n═══════════\nContent\n───────────\nFooter";
        let result = filter_ansi(input);
        assert!(!result.contains("═══"));
        assert!(result.contains("Header"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_preserve_error_lines() {
        let input = "\x1b[31merror: something failed\x1b[0m";
        let result = filter_ansi(input);
        assert!(result.contains("error: something failed"));
    }

    #[test]
    fn test_preserve_timestamp_lines() {
        let input = "═══════\n2026-03-18T10:00:00 INFO started\n═══════";
        let result = filter_ansi(input);
        assert!(result.contains("2026-03-18T10:00:00 INFO started"));
    }

    #[test]
    fn test_carriage_return_keep_last() {
        let input = "Progress: 10%\rProgress: 50%\rProgress: 100%";
        let result = filter_ansi(input);
        assert!(result.contains("100%"));
        assert!(!result.contains("10%"));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test ansi_filter`
Expected: FAIL — module doesn't exist

- [ ] **Step 3: Implement ansi_filter.rs**

```rust
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref ANSI_RE: Regex = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    static ref SPINNER_RE: Regex = Regex::new(r"^[⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏]\s").unwrap();
    static ref PROGRESS_RE: Regex = Regex::new(r"[█░▓▒]{3,}.*\d+%").unwrap();
    static ref DECORATION_RE: Regex = Regex::new(r"^(.)\1{4,}\s*$").unwrap();
    static ref TIMESTAMP_RE: Regex = Regex::new(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}").unwrap();
    static ref ERROR_RE: Regex = Regex::new(r"(?i)\b(error|warn|fail)\b").unwrap();
}

pub fn filter_ansi(input: &str) -> String {
    let stripped = strip_ansi_codes(input);
    let lines = handle_carriage_returns(&stripped);
    filter_lines(&lines)
}

fn strip_ansi_codes(input: &str) -> String {
    ANSI_RE.replace_all(input, "").to_string()
}

fn handle_carriage_returns(input: &str) -> String {
    input.lines()
        .map(|line| {
            if line.contains('\r') {
                line.rsplit('\r').next().unwrap_or(line)
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn filter_lines(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut result = Vec::new();
    let mut last_progress_line: Option<&str> = None;

    for line in &lines {
        // Always preserve error/warn/fail lines
        if ERROR_RE.is_match(line) {
            if let Some(pl) = last_progress_line.take() {
                result.push(pl);
            }
            result.push(line);
            continue;
        }
        // Always preserve timestamp lines
        if TIMESTAMP_RE.is_match(line) {
            result.push(line);
            continue;
        }
        // Skip spinner lines
        if SPINNER_RE.is_match(line) {
            continue;
        }
        // Track progress lines, keep only last
        if PROGRESS_RE.is_match(line) {
            last_progress_line = Some(line);
            continue;
        }
        // Skip decoration lines
        if DECORATION_RE.is_match(line.trim()) {
            continue;
        }
        result.push(line);
    }

    if let Some(pl) = last_progress_line {
        result.push(pl);
    }

    result.join("\n")
}
```

- [ ] **Step 4: Add mod declaration in main.rs**

```rust
mod ansi_filter;
```

- [ ] **Step 5: Wire ANSI filter into pipeline**

In the main execution path, before command-specific handling, apply `ansi_filter::filter_ansi()` to stdout/stderr output.

- [ ] **Step 6: Run tests**

Run: `cargo test ansi_filter`
Expected: All PASS

- [ ] **Step 7: Commit**

```bash
git add src/ansi_filter.rs src/main.rs
git commit -m "feat: add ANSI/spinner/decoration preprocessor filter"
```

---

### Task 10: Error Stacktrace Compression

**Files:**
- Create: `src/error_cmd.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Write failing tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodejs_stacktrace() {
        let input = r#"TypeError: Cannot read properties of undefined (reading 'id')
    at getUserProfile (/app/src/api/users.ts:47:23)
    at processAuth (/app/src/middleware/auth.ts:12:5)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    at next (/app/node_modules/express/lib/router/route.js:144:13)
    at Route.dispatch (/app/node_modules/express/lib/router/route.js:114:3)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)"#;
        let result = compress_errors(input);
        assert!(result.contains("TypeError"));
        assert!(result.contains("users.ts:47"));
        assert!(result.contains("auth.ts:12"));
        assert!(!result.contains("node_modules"));
        assert!(result.contains("framework frames hidden"));
    }

    #[test]
    fn test_python_traceback() {
        let input = r#"Traceback (most recent call last):
  File "/app/main.py", line 10, in handler
    process(data)
  File "/usr/lib/python3.11/importlib/__init__.py", line 126, in import_module
    return _bootstrap._find_and_load(name, _gcd_import)
  File "/app/venv/lib/python3.11/site-packages/flask/app.py", line 1498, in __call__
    return self.wsgi_app(environ, start_response)
ValueError: invalid literal for int()"#;
        let result = compress_errors(input);
        assert!(result.contains("ValueError"));
        assert!(result.contains("main.py"));
        assert!(!result.contains("site-packages"));
        assert!(!result.contains("importlib"));
    }

    #[test]
    fn test_rust_panic() {
        let input = r#"thread 'main' panicked at 'index out of bounds', src/handler.rs:42:5
stack backtrace:
   0: std::panicking::begin_panic
   1: std::rt::lang_start_internal
   2: myapp::handler::process
             at ./src/handler.rs:42:5
   3: myapp::main
             at ./src/main.rs:15:3
   4: std::rt::lang_start
   5: tokio::runtime::enter"#;
        let result = compress_errors(input);
        assert!(result.contains("panicked at"));
        assert!(result.contains("handler.rs:42"));
        assert!(!result.contains("std::panicking"));
        assert!(!result.contains("tokio::runtime"));
    }

    #[test]
    fn test_repeated_errors() {
        let input = "TypeError: x is not a function\n  at foo.ts:1\nTypeError: x is not a function\n  at foo.ts:2\nTypeError: x is not a function\n  at foo.ts:3\n";
        let result = compress_errors(input);
        assert!(result.contains("repeated 3 times") || result.contains("(×3)"));
    }

    #[test]
    fn test_no_stacktrace_passthrough() {
        let input = "Hello world\nThis is normal output";
        let result = compress_errors(input);
        assert_eq!(result.trim(), input.trim());
    }

    #[test]
    fn test_go_stacktrace() {
        let input = r#"goroutine 1 [running]:
runtime/debug.Stack()
	/usr/local/go/src/runtime/debug/stack.go:24
runtime.gopanic({0x1234, 0x5678})
	/usr/local/go/src/runtime/panic.go:884
main.handler()
	/app/handler.go:42 +0x1a4
main.main()
	/app/main.go:15 +0x58"#;
        let result = compress_errors(input);
        assert!(result.contains("handler.go:42"));
        assert!(result.contains("main.go:15"));
        assert!(!result.contains("runtime/debug"));
        assert!(!result.contains("runtime/panic"));
    }

    #[test]
    fn test_java_stacktrace() {
        let input = r#"java.lang.NullPointerException: Cannot invoke method on null
	at com.myapp.UserService.getUser(UserService.java:42)
	at com.myapp.Controller.handle(Controller.java:15)
	at java.lang.reflect.Method.invoke(Method.java:498)
	at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)
	at org.springframework.web.servlet.FrameworkServlet.service(FrameworkServlet.java:897)"#;
        let result = compress_errors(input);
        assert!(result.contains("NullPointerException"));
        assert!(result.contains("UserService.java:42"));
        assert!(!result.contains("java.lang.reflect"));
        assert!(!result.contains("sun.reflect"));
        assert!(!result.contains("springframework"));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test error_cmd`
Expected: FAIL

- [ ] **Step 3: Implement error_cmd.rs**

Key components:
- `detect_stacktrace(input: &str) -> Option<Language>` — pattern matching for each language
- `compress_errors(input: &str) -> String` — main entry point
- `compress_stacktrace(lines: &[&str], lang: Language) -> String` — per-language compression
- `is_framework_frame(line: &str, lang: Language) -> bool` — framework frame detection
- `deduplicate_errors(input: &str) -> String` — repeated error compression

Framework frame patterns:
- Node.js: `node_modules/`, `node:internal/`
- Python: `site-packages/`, `/usr/lib/python`, `importlib`, `_bootstrap`
- Java: `java.lang.reflect.`, `sun.reflect.`, `org.springframework.`
- Go: `runtime/`, `runtime/debug.`, `net/http.`
- Rust: `std::rt::`, `tokio::runtime::`, `std::panicking::`

Output format:
```
ErrorMessage
  → file:line    function()
  → file:line    function()
  (+ N framework frames hidden)
```

- [ ] **Step 4: Add mod and wire as post-processor**

In `main.rs`:
- `mod error_cmd;`
- Add `Err` command variant for explicit `contextzip err <cmd>`
- Wire `error_cmd::compress_errors()` as post-processor on all command output

- [ ] **Step 5: Run tests**

Run: `cargo test error_cmd`
Expected: All PASS

- [ ] **Step 6: Commit**

```bash
git add src/error_cmd.rs src/main.rs
git commit -m "feat: add error stacktrace compression with 5-language support"
```

---

### Task 11: Web Page Compression

**Files:**
- Create: `src/web_cmd.rs`
- Modify: `src/main.rs`
- Modify: `Cargo.toml` (scraper already added in Task 2)

- [ ] **Step 1: Write failing tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_nav_footer() {
        let html = r#"<html><body>
            <nav><a href="/">Home</a><a href="/about">About</a></nav>
            <main><h1>Hello</h1><p>Content here</p></main>
            <footer>© 2026 Corp</footer>
        </body></html>"#;
        let result = extract_content(html);
        assert!(result.contains("Hello"));
        assert!(result.contains("Content here"));
        assert!(!result.contains("Home"));
        assert!(!result.contains("© 2026"));
    }

    #[test]
    fn test_preserve_code_blocks() {
        let html = r#"<html><body>
            <article><pre><code>fn main() { println!("hello"); }</code></pre></article>
        </body></html>"#;
        let result = extract_content(html);
        assert!(result.contains("fn main()"));
    }

    #[test]
    fn test_strip_cookie_banner() {
        let html = r#"<html><body>
            <div class="cookie-consent">We use cookies</div>
            <main><p>Real content</p></main>
        </body></html>"#;
        let result = extract_content(html);
        assert!(result.contains("Real content"));
        assert!(!result.contains("cookies"));
    }

    #[test]
    fn test_strip_scripts_styles() {
        let html = r#"<html><head><style>body{color:red}</style></head><body>
            <script>alert('x')</script>
            <main><p>Content</p></main>
        </body></html>"#;
        let result = extract_content(html);
        assert!(result.contains("Content"));
        assert!(!result.contains("alert"));
        assert!(!result.contains("color:red"));
    }

    #[test]
    fn test_preserve_tables() {
        let html = r#"<html><body><main>
            <table><tr><td>Name</td><td>Value</td></tr></table>
        </main></body></html>"#;
        let result = extract_content(html);
        assert!(result.contains("Name"));
        assert!(result.contains("Value"));
    }

    #[test]
    fn test_preserve_img_alt() {
        let html = r#"<html><body><article>
            <img src="photo.jpg" alt="Architecture diagram">
        </article></body></html>"#;
        let result = extract_content(html);
        assert!(result.contains("Architecture diagram"));
    }

    #[test]
    fn test_non_html_passthrough() {
        let input = "Just plain text output";
        let result = extract_content(input);
        assert_eq!(result.trim(), input.trim());
    }

    #[test]
    fn test_strip_ad_social() {
        let html = r#"<html><body>
            <div class="advertisement">Buy now!</div>
            <div id="social-share">Share on Twitter</div>
            <main><p>Article text</p></main>
        </body></html>"#;
        let result = extract_content(html);
        assert!(result.contains("Article text"));
        assert!(!result.contains("Buy now"));
        assert!(!result.contains("Share on Twitter"));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test web_cmd`
Expected: FAIL

- [ ] **Step 3: Implement web_cmd.rs**

Using `scraper` crate:
- `is_html(input: &str) -> bool` — detect `<!DOCTYPE` or `<html`
- `extract_content(input: &str) -> String` — main entry
- Remove: `nav, header, footer, aside, script, style, noscript`
- Remove by class/id patterns: `cookie|consent|banner|newsletter|subscribe|signup|social|share|follow|ad|advertisement|sponsor`
- Preserve: `main, article, pre, code, table`
- Extract img alt text
- `run_web_command(url: &str) -> Result<String>` — for `contextzip web <url>` (fetch + extract)

- [ ] **Step 4: Add mod and CLI command**

In `main.rs`:
- `mod web_cmd;`
- Add `Web { url: String }` command variant
- Auto-detect HTML in curl/wget output and apply web filter

- [ ] **Step 5: Run tests**

Run: `cargo test web_cmd`
Expected: All PASS

- [ ] **Step 6: Commit**

```bash
git add src/web_cmd.rs src/main.rs
git commit -m "feat: add web page content extraction with HTML noise removal"
```

---

### ★ Week 2 Checkpoint

**Verification:**
1. `cargo test` — all tests pass (existing + new)
2. ANSI filter strips escape codes, spinners, progress bars, decorations
3. ANSI filter preserves error/warn lines and timestamps
4. Error compression works for Node.js, Python, Rust, Go, Java stacktraces
5. Error compression preserves error message + user code frames
6. Web extraction strips nav/footer/sidebar/ads/cookies
7. Web extraction preserves main/article content, code blocks, tables
8. `contextzip err <cmd>` works
9. `contextzip web <url>` works
10. Post-processor auto-detects stacktraces in any command output

**Stop here. Report results to user for review before proceeding to Week 3.**

---

## Week 3: Conditional Features + Integration

### Task 12: Build Error Grouping

**Files:**
- Create: `src/build_cmd.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Write failing tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsc_grouping() {
        let input = r#"src/api/users.ts:47:5 - error TS2322: Type 'string' is not assignable to type 'number'.
src/api/users.ts:83:5 - error TS2322: Type 'string' is not assignable to type 'number'.
src/api/orders.ts:12:5 - error TS2322: Type 'string' is not assignable to type 'number'.
src/api/orders.ts:45:5 - error TS2322: Type 'string' is not assignable to type 'number'.
Found 4 errors in 2 files."#;
        let result = group_build_errors(input);
        assert!(result.contains("TS2322"));
        assert!(result.contains("×4"));
        assert!(result.contains(":47"));
        assert!(result.contains(":83"));
        assert!(result.contains(":12"));
        assert!(result.contains(":45"));
        // Grouped — fewer lines than input
        assert!(result.lines().count() < input.lines().count());
    }

    #[test]
    fn test_eslint_grouping() {
        let input = r#"src/App.tsx:5:1 - error no-unused-vars: 'x' is defined but never used
src/App.tsx:12:1 - error no-unused-vars: 'y' is defined but never used
src/Home.tsx:3:1 - error no-unused-vars: 'z' is defined but never used"#;
        let result = group_build_errors(input);
        assert!(result.contains("no-unused-vars"));
        assert!(result.contains(":5"));
        assert!(result.contains(":12"));
        assert!(result.contains(":3"));
    }

    #[test]
    fn test_cargo_grouping() {
        let input = r#"error[E0308]: mismatched types
 --> src/main.rs:10:5
error[E0308]: mismatched types
 --> src/lib.rs:20:10"#;
        let result = group_build_errors(input);
        assert!(result.contains("E0308"));
        assert!(result.contains("main.rs:10"));
        assert!(result.contains("lib.rs:20"));
    }

    #[test]
    fn test_preserves_all_line_numbers() {
        let input = (1..=40).map(|i| format!("src/file{}.ts:{}:5 - error TS2322: Type mismatch", i % 8, i * 10))
            .collect::<Vec<_>>().join("\n");
        let result = group_build_errors(&input);
        // All 40 line numbers must be present
        for i in 1..=40 {
            assert!(result.contains(&format!(":{}", i * 10)), "Missing line {}", i * 10);
        }
    }

    #[test]
    fn test_no_build_errors_passthrough() {
        let input = "Build succeeded\n0 errors, 0 warnings";
        let result = group_build_errors(input);
        assert_eq!(result.trim(), input.trim());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test build_cmd`
Expected: FAIL

- [ ] **Step 3: Implement build_cmd.rs**

Key components:
- `detect_build_tool(input: &str) -> Option<BuildTool>` — detect tsc/eslint/cargo/mypy/pylint
- `group_build_errors(input: &str) -> String` — main entry
- `parse_error_entry(line: &str, tool: BuildTool) -> Option<ErrorEntry>` — extract code, file, line
- `group_by_code(entries: Vec<ErrorEntry>) -> BTreeMap<String, Vec<ErrorEntry>>` — group same codes
- `format_grouped(groups: BTreeMap<...>) -> String` — output format

Output format:
```
TS2322: Type 'string' is not assignable to type 'number' (×40)
  src/api/users.ts      :47, :83
  src/api/orders.ts     :12, :45, :67
```

**Safety: ALL file:line locations preserved.**

- [ ] **Step 4: Wire into main.rs**

- `mod build_cmd;`
- Detect tsc/eslint/cargo build output and apply grouping

- [ ] **Step 5: Run tests**

Run: `cargo test build_cmd`
Expected: All PASS

- [ ] **Step 6: Commit**

```bash
git add src/build_cmd.rs src/main.rs
git commit -m "feat: add build error grouping with line number preservation"
```

---

### Task 13: Package Install Log Compression

**Files:**
- Create: `src/pkg_cmd.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Write failing tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npm_install_compression() {
        let input = r#"npm warn deprecated inflight@1.0.6: This module is not supported
npm warn deprecated rimraf@3.0.2: Rimraf v3 is no longer supported
added 847 packages, and audited 848 packages in 32s
143 packages are looking for funding
  run `npm fund` for details
8 vulnerabilities (2 moderate, 6 high)"#;
        let result = compress_pkg_log(input);
        assert!(result.contains("847 packages"));
        assert!(result.contains("8 vulnerabilities"));
        assert!(!result.contains("inflight"));
        assert!(!result.contains("funding"));
    }

    #[test]
    fn test_preserve_security_deprecated() {
        let input = "npm warn deprecated bcrypt@3.0.0: known security vulnerability (CVE-2023-31484)";
        let result = compress_pkg_log(input);
        assert!(result.contains("CVE-2023-31484"));
        assert!(result.contains("bcrypt"));
    }

    #[test]
    fn test_preserve_vulnerability_warnings() {
        let input = "6 high severity vulnerabilities\nRun `npm audit` for details";
        let result = compress_pkg_log(input);
        assert!(result.contains("vulnerabilities"));
    }

    #[test]
    fn test_pip_compression() {
        let input = r#"Requirement already satisfied: requests in ./venv/lib/python3.11/site-packages (2.31.0)
Using cached urllib3-2.1.0-py3-none-any.whl
Successfully installed flask-3.0.0 jinja2-3.1.2"#;
        let result = compress_pkg_log(input);
        assert!(!result.contains("already satisfied"));
        assert!(!result.contains("Using cached"));
        assert!(result.contains("flask") || result.contains("installed"));
    }

    #[test]
    fn test_preserve_ghsa() {
        let input = "npm warn deprecated pkg@1.0.0: GHSA-1234-5678-9abc";
        let result = compress_pkg_log(input);
        assert!(result.contains("GHSA-1234"));
    }

    #[test]
    fn test_preserve_critical_keyword() {
        let input = "npm warn deprecated pkg@1.0.0: critical security fix required";
        let result = compress_pkg_log(input);
        assert!(result.contains("critical"));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test pkg_cmd`
Expected: FAIL

- [ ] **Step 3: Implement pkg_cmd.rs**

Key components:
- `compress_pkg_log(input: &str) -> String` — main entry
- `is_security_warning(line: &str) -> bool` — check for vulnerability/security/critical/CVE-/GHSA-
- `detect_pkg_manager(input: &str) -> Option<PkgManager>` — npm/yarn/pnpm/pip/cargo
- Remove: deprecated (non-security), funding, progress, "already satisfied", "Using cached"
- Preserve: security warnings, vulnerability counts, final summary
- Output: `✓ N packages (time)\n⚠ vulnerabilities\n⚠ security deprecations`

- [ ] **Step 4: Wire into main.rs**

- `mod pkg_cmd;`
- Detect npm/pip/cargo install output and apply compression

- [ ] **Step 5: Run tests**

Run: `cargo test pkg_cmd`
Expected: All PASS

- [ ] **Step 6: Commit**

```bash
git add src/pkg_cmd.rs src/main.rs
git commit -m "feat: add package install log compression with security preservation"
```

---

### Task 14: Docker Build Log Compression

**Files:**
- Create: `src/docker_cmd.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Write failing tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_build() {
        let input = r#"Step 1/12 : FROM node:20-alpine
 ---> abc123def456
Step 2/12 : WORKDIR /app
 ---> Using cache
 ---> 789ghi012jkl
Step 3/12 : COPY package*.json ./
 ---> Using cache
 ---> mno345pqr678
Successfully built abc123final
Successfully tagged my-app:latest"#;
        let result = compress_docker_log(input);
        assert!(result.contains("my-app:latest"));
        assert!(result.contains("12 steps"));
        assert!(!result.contains("abc123def456"));
        assert!(!result.contains("Using cache"));
        assert!(result.lines().count() <= 2);
    }

    #[test]
    fn test_failed_build() {
        let input = r#"Step 1/12 : FROM node:20-alpine
 ---> abc123
Step 5/12 : COPY package*.json ./
 ---> Using cache
Step 6/12 : RUN npm install
 ---> Using cache
Step 7/12 : RUN npm run build
 ---> Running in xyz789
error: Module not found: 'react-dom/client'
The command '/bin/sh -c npm run build' returned a non-zero code: 1"#;
        let result = compress_docker_log(input);
        assert!(result.contains("FAILED") || result.contains("failed"));
        assert!(result.contains("step 7"));
        assert!(result.contains("Module not found"));
        // Should preserve context steps (5, 6) and failed step (7)
        assert!(result.contains("Step 5") || result.contains("COPY"));
        assert!(result.contains("Step 6") || result.contains("npm install"));
    }

    #[test]
    fn test_cached_count() {
        let input = (1..=10).map(|i| format!("Step {}/10 : CMD {}\n ---> Using cache\n ---> hash{}", i, i, i))
            .collect::<Vec<_>>().join("\n");
        let input = format!("{}\nSuccessfully built final\nSuccessfully tagged app:v1", input);
        let result = compress_docker_log(&input);
        assert!(result.contains("cached"));
    }

    #[test]
    fn test_preserve_exit_code() {
        let input = "Step 3/5 : RUN make\nError: compilation failed\nThe command returned a non-zero code: 2";
        let result = compress_docker_log(input);
        assert!(result.contains("2") || result.contains("exit"));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test docker_cmd`
Expected: FAIL

- [ ] **Step 3: Implement docker_cmd.rs**

Key components:
- `compress_docker_log(input: &str) -> String` — main entry
- `parse_docker_steps(input: &str) -> Vec<DockerStep>` — parse Step N/M lines
- `detect_failure(steps: &[DockerStep]) -> Option<usize>` — find failed step
- Success: `✓ built {tag} ({N} steps, {M} cached)`
- Failure: show failed step + 2 prior steps + error message + exit code

- [ ] **Step 4: Wire into main.rs**

- `mod docker_cmd;`
- Detect docker build output and apply compression
- Integrate with existing container.rs if applicable

- [ ] **Step 5: Run tests**

Run: `cargo test docker_cmd`
Expected: All PASS

- [ ] **Step 6: Commit**

```bash
git add src/docker_cmd.rs src/main.rs
git commit -m "feat: add Docker build log compression with failure context preservation"
```

---

### Task 15: Integration Testing

**Files:**
- Modify: existing test files
- Possibly create: `tests/integration.rs`

- [ ] **Step 1: Full test suite**

Run: `cargo test`
Expected: ALL tests pass (RTK original + all new modules)

- [ ] **Step 2: Test ANSI → Error pipeline**

Write integration test: ANSI-colored stacktrace → ANSI filter strips colors → error_cmd compresses frames.

- [ ] **Step 3: Test ANSI → Docker pipeline**

Write integration test: Docker build output with ANSI colors and progress bars → both filters applied correctly.

- [ ] **Step 4: Test gain --by-feature accuracy**

Create test entries with different features, verify `gain --by-feature` aggregates correctly.

- [ ] **Step 5: Test CLI commands end-to-end**

```bash
echo "test output" | ./target/release/contextzip err cat
./target/release/contextzip --version
./target/release/contextzip gain
./target/release/contextzip gain --by-feature
./target/release/contextzip init --show
```

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "test: add integration tests for full pipeline"
```

---

### Task 16: README and Final Polish

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Write README**

Structure:
- One-line description
- Install (1 command)
- Feature comparison table (RTK vs ContextZip)
- Before/After examples for each new feature
- CLI reference
- Configuration
- Attribution (RTK fork)

- [ ] **Step 2: Clippy clean**

Run: `cargo clippy -- -D warnings`
Fix any warnings.

- [ ] **Step 3: Final commit**

```bash
git add -A
git commit -m "docs: add README with install instructions and feature comparison"
```

---

### ★ Week 3 Checkpoint (Final)

**Verification — Full Completion Criteria:**

1. `cargo build --release` succeeds
2. `cargo test` — ALL tests pass
3. `cargo clippy -- -D warnings` — clean
4. RTK 기존 CLI 압축 기능 전부 동작
5. Error stacktrace: 프레임워크 프레임 제거, 사용자 코드 보존
6. Web: nav/footer/sidebar 제거, 본문 추출
7. ANSI: 모든 출력에서 이스케이프/스피너/장식 제거
8. Build: 에러 그룹화 + 모든 줄 번호 유지
9. Pkg: 로그 압축 + 보안 경고 보존
10. Docker: 성공 1줄 요약, 실패 시 컨텍스트 보존
11. `contextzip gain` — 전체 절약량
12. `contextzip gain --by-feature` — 기능별 절약량
13. install.sh 동작
14. CI/CD workflow 설정 완료
15. README 완성

**이 전부 확인되면 Phase 1 완료.**
