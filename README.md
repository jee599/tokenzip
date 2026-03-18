# TokenZip

> Claude Code context optimizer. Reduce LLM token consumption by 60-90%.

[English](#) | [한국어](docs/README.ko.md) | [日本語](docs/README.ja.md) | [中文](docs/README.zh.md)

## What It Does

TokenZip wraps your CLI commands and compresses their output before it reaches Claude Code's context window. Less noise = more room for actual code.

Built on [RTK](https://github.com/rtk-ai/rtk) with 6 additional filters for noise RTK doesn't catch.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/jee599/tokenzip/main/install.sh | bash
```

That's it. Restart Claude Code.

## What Gets Compressed

| Noise Source | Before | After | Savings |
|---|---|---|---|
| Error stacktraces | 30 lines of node_modules frames | 3 lines: error + your code | ~93% |
| Web page fetch | 3,000 tokens of nav/footer/ads | 800 tokens of content | ~73% |
| ANSI/spinners | Escape codes, progress bars | Clean text | ~85% |
| Build errors | 40 identical TS2322 errors | Grouped by code, all locations kept | ~81% |
| Package install | 150 lines deprecated/funding | 3 lines: summary + security | ~95% |
| Docker build | 50 lines of layer hashes | 1 line: ✓ built app:latest | ~96% |
| CLI output | git/test/ls noise | Compressed (via RTK) | ~78% |

## Before / After

### Error Stacktrace
**Before** (30 lines, ~1,500 tokens):
```
TypeError: Cannot read properties of undefined (reading 'id')
    at getUserProfile (/app/src/api/users.ts:47:23)
    at processAuth (/app/src/middleware/auth.ts:12:5)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    at next (/app/node_modules/express/lib/router/route.js:144:13)
    ... 25 more node_modules frames
```

**After** (3 lines, ~100 tokens):
```
TypeError: Cannot read properties of undefined (reading 'id')
  → /app/src/api/users.ts:47         getUserProfile()
  → /app/src/middleware/auth.ts:12    processAuth()
  (+ 27 framework frames hidden)
```

### Package Install
**Before** (150 lines, ~2,000 tokens):
```
npm warn deprecated inflight@1.0.6: This module is not supported
npm warn deprecated rimraf@3.0.2: Rimraf v3 is no longer supported
... 47 more deprecated warnings
added 847 packages, and audited 848 packages in 32s
143 packages are looking for funding
8 vulnerabilities (2 moderate, 6 high)
```

**After** (3 lines, ~50 tokens):
```
✓ 847 packages (32s)
⚠ 8 vulnerabilities (6 high, 2 moderate)
⚠ deprecated bcrypt@3.0.0: security vulnerability (CVE-2023-31484)
```

### Docker Build (Success)
**Before** (50 lines): Step-by-step with hashes, cache lines, intermediate containers
**After** (1 line): `✓ built my-app:latest (12 steps, 8 cached)`

### Docker Build (Failure)
Preserves context: failed step + 2 prior steps + full error message + exit code.

## CLI

```bash
# Wrapped commands (automatic via hook)
tokenzip git status
tokenzip cargo test
tokenzip npm install

# New commands
tokenzip web https://docs.example.com    # Extract page content
tokenzip err node server.js              # Error-focused output

# Analytics
tokenzip gain                  # Total savings
tokenzip gain --by-feature     # Savings by filter type
tokenzip gain --graph          # Daily savings chart
tokenzip gain --history        # Recent command history

# Setup
tokenzip init -g               # Install hook globally
tokenzip init --show           # Check installation
tokenzip uninstall             # Clean removal
tokenzip update                # Self-update
```

## How It Works

1. Claude Code hook intercepts bash commands
2. Commands get routed through TokenZip
3. ANSI preprocessor strips escape codes from all output
4. Command-specific filter compresses the result
5. Error post-processor catches stacktraces in any output
6. Compressed output goes to Claude Code's context

## Configuration

```bash
# Config file
~/.config/tokenzip/config.toml

# Project-level filters
.tokenzip/filters.toml
```

## Requirements

- Claude Code (or any tool that uses PreToolUse hooks)
- macOS (arm64/x86_64) or Linux (x86_64)

## Attribution

Built on [RTK (Rust Token Killer)](https://github.com/rtk-ai/rtk) by rtk-ai. MIT License.
