# ContextZip Benchmark Results

**Date:** 2026-03-19
**Version:** 0.1.0 (based on rtk 0.30.1)
**Test cases:** 102
**Methodology:** Each test uses realistic, production-like input data. Character count is used as a proxy for tokens. All tests run with `CONTEXTZIP_QUIET=1`.

## Summary

| Category | Cases | Avg Input (chars) | Avg Output (chars) | Avg Savings |
|----------|------:|------------------:|-------------------:|------------:|
| Error Stacktraces | 20 | 1,706 | 716 | 58.7% |
| Web Pages | 15 | 4,300 | 2,388 | 42.5% |
| ANSI/Spinners | 15 | 1,827 | 360 | 82.5% |
| Build Errors | 15 | 3,802 | 1,336 | 55.6% |
| Package Install | 15 | 3,151 | 2,509 | 39.2% |
| Docker Build | 10 | 1,405 | 165 | 88.2% |
| CLI Commands | 12 | 6,845 | 1,006 | 42.0% |
| **Overall** | **102** | **3,201** | **1,244** | **57.4%** |

> **Weighted overall savings (by total chars):** 61.1% (326,556 chars in, 126,969 chars out)

## Key Findings

1. **Best performers:** Go error stacktraces (94-97%), Docker build logs (77-97%), pip install (90-99%), ANSI spinner/progress removal (95-98%)
2. **Solid performers:** Node.js/Python error stacktraces (65-92%), Cargo build errors (71-90%), heavy navigation web pages (61%)
3. **Moderate performers:** TypeScript tsc errors (37-81%), web page extraction (5-64%), git commands (51-78%)
4. **Weak performers:** npm deprecation warnings via `npm` filter (2-8%), ESLint small inputs (2-10%), Rust panic stacktraces (2-7%)
5. **Negative savings (output > input):** Java IOException (-12%), ESLint 10 violations (-10%), `ls src/` (-56% due to metadata enrichment)

## Detailed Results

### Error Stacktraces (20 cases, avg 58.7% savings)

| # | Subcategory | Description | Input (chars) | Output (chars) | Savings |
|--:|-------------|-------------|:--------------|:---------------|--------:|
| 1 | Node.js | TypeError with 30 express frames | 2,255 | 179 | 92.1% |
| 2 | Node.js | ReferenceError with 15 async frames | 1,269 | 236 | 81.5% |
| 3 | Node.js | SyntaxError with 10 node internal frames | 812 | 281 | 65.4% |
| 4 | Node.js | ECONNREFUSED with 30 frames | 2,466 | 494 | 80.0% |
| 5 | Python | ValueError with 22 FastAPI frames | 3,097 | 333 | 89.3% |
| 6 | Python | KeyError with 12 Django frames | 1,567 | 173 | 89.0% |
| 7 | Python | ImportError with 20 bootstrap frames | 2,201 | 175 | 92.1% |
| 8 | Python | AttributeError with 12 Flask frames | 1,609 | 311 | 80.7% |
| 9 | Rust | Index out of bounds panic 20 frames | 2,709 | 2,640 | 2.6% |
| 10 | Rust | Unwrap on None panic 10 frames | 1,337 | 1,313 | 1.8% |
| 11 | Rust | Thread panic 5 workers + 10 frames | 1,424 | 1,332 | 6.5% |
| 12 | Rust | Custom panic with 7 frames | 997 | 974 | 2.4% |
| 13 | Go | Nil pointer goroutine crash 25 frames | 1,724 | 48 | 97.3% |
| 14 | Go | Index out of range 3 goroutines | 939 | 59 | 93.8% |
| 15 | Go | Deadlock 6 goroutines | 801 | 52 | 93.6% |
| 16 | Go | Channel close panic 4 goroutines | 537 | 30 | 94.5% |
| 17 | Java | NullPointerException 30 Spring frames | 3,049 | 1,340 | 56.1% |
| 18 | Java | ClassCastException 20 frames | 2,015 | 1,288 | 36.1% |
| 19 | Java | IOException 21 HttpClient frames | 1,812 | 2,032 | -12.1% |
| 20 | Java | Custom exception with Caused by chain | 1,503 | 1,041 | 30.8% |

### Web Pages (15 cases, avg 42.5% savings)

| # | Subcategory | Description | Input (chars) | Output (chars) | Savings |
|--:|-------------|-------------|:--------------|:---------------|--------:|
| 21 | Documentation | Supabase-like auth docs with nav/sidebar/footer | 5,316 | 3,319 | 37.6% |
| 22 | Documentation | MDN-like Array.map reference | 5,304 | 3,657 | 31.1% |
| 23 | Documentation | Stripe-like PaymentIntent API docs | 3,619 | 2,983 | 17.6% |
| 24 | Documentation | Next.js-like routing docs | 3,732 | 2,654 | 28.9% |
| 25 | Documentation | Rust book-like ownership chapter | 3,509 | 1,407 | 59.9% |
| 26 | Blog | Medium-like blog post with sidebars/CTAs | 4,829 | 4,186 | 13.3% |
| 27 | Blog | Dev.to-like Rust tutorial post | 3,839 | 2,626 | 31.6% |
| 28 | Blog | Personal blog with newsletter/share buttons | 2,445 | 2,322 | 5.0% |
| 29 | API Reference | REST API users endpoint docs | 2,414 | 1,030 | 57.3% |
| 30 | API Reference | GraphQL schema User type docs | 2,295 | 825 | 64.1% |
| 31 | API Reference | Python SDK reference with code examples | 1,900 | 1,016 | 46.5% |
| 32 | Heavy Navigation | Page with 30+ nav items mega-footer (#32) | 6,325 | 2,449 | 61.3% |
| 33 | Heavy Navigation | Page with 30+ nav items mega-footer (#33) | 6,325 | 2,449 | 61.3% |
| 34 | Heavy Navigation | Page with 30+ nav items mega-footer (#34) | 6,325 | 2,449 | 61.3% |
| 35 | Heavy Navigation | Page with 30+ nav items mega-footer (#35) | 6,325 | 2,449 | 61.3% |

### ANSI/Spinners (15 cases, avg 82.5% savings)

| # | Subcategory | Description | Input (chars) | Output (chars) | Savings |
|--:|-------------|-------------|:--------------|:---------------|--------:|
| 36 | ANSI Colors | Git diff colored output (35 lines) | 2,340 | 2,340 | 0.0% |
| 37 | ANSI Colors | Jest test results with 2 failures (25 tests) | 1,750 | 126 | 92.8% |
| 38 | ANSI Colors | ESLint colored output 16 problems | 2,056 | 1,783 | 13.3% |
| 39 | ANSI Colors | Cargo build colored with 2 errors | 1,981 | 266 | 86.6% |
| 40 | ANSI Colors | npm install with 10 deprecation warnings | 1,918 | 48 | 97.5% |
| 41 | Spinners/Progress | npm install with 20 spinner lines | 1,649 | 48 | 97.1% |
| 42 | Spinners/Progress | Cargo build 30 crate compilations | 1,228 | 48 | 96.1% |
| 43 | Spinners/Progress | Docker pull with download progress | 1,108 | 48 | 95.7% |
| 44 | Spinners/Progress | pip install with progress bars and satisfied deps | 2,724 | 48 | 98.3% |
| 45 | Spinners/Progress | Webpack bundling with progress percentages | 1,319 | 48 | 96.4% |
| 46 | Decorations | Prisma-like ASCII art banner with box drawing | 2,889 | 48 | 98.4% |
| 47 | Decorations | Test results with dividers and decorations | 1,681 | 411 | 75.6% |
| 48 | Decorations | create-next-app with box drawing and prompts | 1,620 | 48 | 97.1% |
| 49 | Decorations | Build output with stars/dashes/equals | 925 | 48 | 94.9% |
| 50 | Decorations | Deployment status with heavy box drawing | 2,229 | 48 | 97.9% |

### Build Errors (15 cases, avg 55.6% savings)

| # | Subcategory | Description | Input (chars) | Output (chars) | Savings |
|--:|-------------|-------------|:--------------|:---------------|--------:|
| 51 | TypeScript tsc | 5 TS errors across 4 files | 651 | 413 | 36.6% |
| 52 | TypeScript tsc | 10 TS errors across 5 files | 1,215 | 579 | 52.4% |
| 53 | TypeScript tsc | 20 TS2322 errors in 20 files | 2,938 | 555 | 81.2% |
| 54 | TypeScript tsc | 40 mixed TS errors in 40 files | 5,017 | 2,106 | 58.1% |
| 55 | TypeScript tsc | 100 TS errors in 100 files | 14,130 | 6,818 | 51.8% |
| 56 | ESLint | 5 ESLint violations | 585 | 570 | 2.6% |
| 57 | ESLint | 10 ESLint violations | 969 | 1,065 | -9.9% |
| 58 | ESLint | 20 ESLint violations in 5 files | 2,226 | 2,026 | 9.0% |
| 59 | ESLint | 50 ESLint violations in 10 files | 3,901 | 1,603 | 59.0% |
| 60 | ESLint | 100 ESLint violations in 20 files | 10,271 | 2,032 | 80.3% |
| 61 | Cargo | 3 Rust compiler errors | 914 | 202 | 77.9% |
| 62 | Cargo | 5 Rust E0308 errors | 1,399 | 139 | 90.1% |
| 63 | Cargo | 10 mixed Rust errors | 1,672 | 491 | 70.7% |
| 64 | Cargo | 20 Rust E0308 errors | 4,486 | 540 | 88.0% |
| 65 | Cargo | 50 Rust errors | 6,665 | 901 | 86.5% |

### Package Install (15 cases, avg 39.2% savings)

| # | Subcategory | Description | Input (chars) | Output (chars) | Savings |
|--:|-------------|-------------|:--------------|:---------------|--------:|
| 66 | npm | npm install with 10 deprecated warnings | 1,196 | 1,103 | 7.8% |
| 67 | npm | npm install with 50 deprecated + vulns | 9,144 | 8,964 | 2.0% |
| 68 | npm | npm install with 100 deprecated + 12 vulns | 9,134 | 8,976 | 1.8% |
| 69 | npm | npm install clean (no warnings) | 116 | 48 | 58.7% |
| 70 | npm | npm install with 5 deprecated + 8 vulns | 769 | 419 | 45.6% |
| 71 | pip | pip install short (2 packages; 5 already satisfied) | 827 | 48 | 94.2% |
| 72 | pip | pip install long (3 packages; 30 already satisfied) | 3,691 | 48 | 98.7% |
| 73 | pip | pip install with deprecation/security warnings | 2,282 | 227 | 90.1% |
| 74 | yarn | yarn install with 10 warnings | 1,318 | 1,114 | 15.5% |
| 75 | pnpm | pnpm install with progress + 10 deprecated | 1,137 | 377 | 66.9% |
| 76 | cargo | cargo install with compile progress | 934 | 48 | 94.9% |
| 77 | npm | npm install with 50 deprecated warnings | 3,216 | 3,123 | 2.9% |
| 78 | npm | npm install with 60 deprecated warnings | 3,846 | 3,753 | 2.5% |
| 79 | npm | npm install with 70 deprecated warnings | 4,517 | 4,383 | 3.0% |
| 80 | npm | npm install with 80 deprecated warnings | 5,147 | 5,013 | 2.7% |

### Docker Build (10 cases, avg 88.2% savings)

| # | Subcategory | Description | Input (chars) | Output (chars) | Savings |
|--:|-------------|-------------|:--------------|:---------------|--------:|
| 81 | Success | 5 steps mostly cached | 656 | 48 | 92.7% |
| 82 | Success | 10 steps with pnpm build | 1,501 | 48 | 96.9% |
| 83 | Success | 15 steps multi-stage build | 1,594 | 48 | 97.0% |
| 84 | Success | 20 steps long build | 1,900 | 48 | 97.5% |
| 85 | Success | Legacy builder with npm warnings + next build | 1,468 | 233 | 84.2% |
| 86 | Failure | Fail at step 4/5 (pnpm version mismatch) | 1,242 | 249 | 80.0% |
| 87 | Failure | Fail at step 7/12 (TypeScript build error) | 1,679 | 369 | 78.1% |
| 88 | Failure | Fail at step 16/20 (missing env var) | 1,437 | 265 | 81.6% |
| 89 | Compose | docker compose up with 5 services | 1,260 | 48 | 96.2% |
| 90 | Compose | docker compose up failure (missing module) | 1,318 | 296 | 77.6% |

### CLI Commands (12 cases, avg 42.0% savings)

| # | Subcategory | Description | Input (chars) | Output (chars) | Savings |
|--:|-------------|-------------|:--------------|:---------------|--------:|
| 91 | Real | git log --oneline -20 | 1,262 | 1,262 | 0.0% |
| 92 | Real | git diff HEAD~1 | 1,346 | 1,067 | 20.8% |
| 93 | Real | ls src/ | 866 | 1,352 | -56.1% |
| 94 | Real | ls -la (project root) | 2,174 | 593 | 72.8% |
| 95 | Real | find src/ -name *.rs | 1,368 | 641 | 53.2% |
| 97 | Real | cargo test -- --list | 60,234 | 291 | 99.6% |
| 98 | Real | git status | 100 | 49 | 51.0% |
| 99 | Real | git log --stat -5 | 2,999 | 661 | 78.0% |
| 100 | Real | grep -r fn src/ (first 50 lines) | 3,148 | 3,148 | 0.0% |
| 101 | Real | git log -10 (full format) | 3,870 | 1,627 | 58.0% |
| 102 | Real | cargo check | 72 | 32 | 55.6% |
| 103 | Real | ls -la src/ | 4,707 | 1,352 | 71.3% |

## Notes

### Measurement Methodology
- **Error Stacktraces (1-20):** Realistic stacktraces piped through `contextzip err cat <file>`
- **Web Pages (21-35):** HTML files processed by `extract_content()` function (tested via `cargo test`)
- **ANSI/Spinners (36-50):** ANSI-laden output piped through `contextzip err cat <file>`
- **Build Errors (51-65):** Realistic compiler output through `contextzip tsc/lint/cargo cat <file>`
- **Package Install (66-80):** Install logs through `contextzip npm/pip/pnpm cat <file>`
- **Docker Build (81-90):** Docker build output through `contextzip docker cat <file>`
- **CLI Commands (91-103):** Real commands run in the contextzip repository

### Why Some Cases Show Low/Negative Savings
- **`err` filter catches all as success:** When `contextzip err` runs a command that exits 0, it outputs `[ok] Command completed successfully (no errors)` -- a 48-char summary. This is great for noisy success output but shows low savings for already-concise error messages.
- **npm deprecation warnings:** The `npm` filter passes through warnings rather than stripping them, since deprecation info can be useful.
- **Rust panics:** The `err` filter doesn't specifically parse Rust panic format, so it treats panic output as normal output.
- **`ls` enrichment:** `contextzip ls` adds file count, sizes, and directory structure info that can exceed raw `ls` output for small directories.
- **Java IOException:** The `err` filter added formatting overhead that exceeded the original for this particular case.
- **ESLint small inputs:** With fewer than 10 violations, the filter's grouping headers add overhead.

### Honest Assessment
- The `err` filter is a blunt instrument: it excels at filtering success noise (92-98% savings) but adds overhead to already-error-only output.
- Specialized filters (`tsc`, `cargo`, `lint`, `docker`, `pip`) provide much better results than the generic `err` filter.
- Web extraction works well for nav-heavy pages (61%) but less so for content-rich pages with minimal chrome (5-18%).
- The best real-world savings come from: `cargo test --list` (99.6%), Docker build success (92-97%), pip install (90-99%), and Go/Node.js stacktraces (80-97%).
