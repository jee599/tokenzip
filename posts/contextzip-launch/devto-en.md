---
title: "Claude Code Is Wasting Your Tokens. I Fixed It in 3 Weeks Without Writing a Single Line of Code."
published: false
description: "1,056 tests. 102 benchmarks. 60-90% token savings. Built entirely by Claude Code agents."
tags: ai, opensource, rust, productivity
---

I burned through my Claude Code context window three times in one session. The third time, I snapped.

I was debugging a Node.js app. Claude ran `npm install`, and 150 lines of deprecated warnings flooded my context. Then a stacktrace — 30 lines of `node_modules` frames, only 2 lines of my actual code. Then `docker build` dumped 50 lines of layer hashes.

By the time I got back to the actual bug, Claude had forgotten the code I'd shown it 10 minutes ago. The context window was full of noise.

That night I found RTK — Rust Token Killer. An open-source CLI proxy that compresses command output before it reaches Claude Code. 28k stars, 60-90% savings on git, test, and ls output. Impressive.

But it didn't touch error stacktraces. Or web pages. Or npm install noise. Or Docker builds.

So I forked it.


## The Experiment: Can Claude Code Build Its Own Tool?

Here's the part that gets weird. I built ContextZip — a 40-module Rust CLI with 1,056 tests — using Claude Code itself. The tool that compresses Claude Code's context was built by Claude Code.

I didn't write Rust. I wrote prompts.

The entire project took 3 weeks. Here's what actually happened.


## Week 1: Fork RTK, Rename Everything, Ship the Installer

The first task was mechanical: clone RTK's source (34 command modules, 60+ TOML filters, 950 tests), rename every reference from "rtk" to "contextzip", and verify nothing broke.

I dispatched a Claude Code subagent to do the rename. 70 files changed, 1,544 insertions, 1,182 deletions. 950 tests still passing. The `--version` output now reads `contextzip 0.1.0 (based on rtk 0.30.1)`.

Then I had three agents work in parallel: one writing the install script, one setting up GitHub Actions CI/CD for 5 platform builds, one extending the SQLite tracking system with a `feature` column.

By the end of Week 1: `curl | bash` installs the binary, Claude Code hook activates automatically, and `contextzip gain --by-feature` shows which filter saved the most tokens.


## Week 2: The Three Filters RTK Doesn't Have

This is where ContextZip diverges from RTK.

**Error stacktraces.** A Node.js error with 30 Express middleware frames becomes 3 lines: the error message, your code frames, and "(+ 27 framework frames hidden)." Same for Python (hides site-packages), Rust (hides std::panicking), Go (hides runtime/), and Java (hides java.lang.reflect).

The before/after:

```
Before (30 lines, ~1,500 tokens):
TypeError: Cannot read properties of undefined (reading 'id')
    at getUserProfile (/app/src/api/users.ts:47:23)
    at processAuth (/app/src/middleware/auth.ts:12:5)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    ... 25 more node_modules frames

After (3 lines, ~100 tokens):
TypeError: Cannot read properties of undefined (reading 'id')
  → src/api/users.ts:47         getUserProfile()
  → src/middleware/auth.ts:12   processAuth()
  (+ 27 framework frames hidden)
```

93% saved. Claude sees the error and your code. Not Express internals.

**ANSI preprocessor.** Every command output passes through this filter first. It strips escape codes, spinner characters, progress bars, and decoration lines. But it preserves error/warning lines and timestamps. 82.5% average savings across 15 test cases.

**Web page extraction.** `contextzip web https://docs.example.com` fetches a page and strips nav, footer, sidebar, cookie banners, ads, scripts. Keeps the article content, code blocks, and tables. Built with the `scraper` crate.


## Week 3: The Conditional Filters + Honesty

Three more filters: build error grouping (40 identical TS2322 errors become one group with all line numbers preserved), package install log compression (npm deprecated noise removed, security warnings kept), and Docker build log compression (success = 1 line, failure = context preserved).

Then I ran 102 benchmark tests with production-like inputs. The results were honest — and that matters.

| Category | Cases | Avg Savings | Best | Worst |
|:---------|------:|------------:|-----:|------:|
| Docker build | 10 | 88.2% | 97% | 77% |
| ANSI/spinners | 15 | 82.5% | 98% | 41% |
| Error stacktraces | 20 | 58.7% | 97% | 2% |
| Build errors | 15 | 55.6% | 90% | -10% |
| **Overall** | **102** | **57.4%** | | |

Some filters have negative savings on tiny inputs. The formatting overhead exceeds the noise removed. I put that in the README because lying about benchmarks is worse than imperfect numbers.

Weighted across all tests: **61.1% savings** (326K chars in → 127K chars out).


## What Claude Code Actually Did

Every task followed this cycle: I dispatched a subagent with a detailed prompt, it implemented the feature with tests, I dispatched a reviewer subagent, issues got fixed, next task.

I ran up to 3 implementation agents in parallel for independent tasks (install.sh, CI/CD, and tracking extension at the same time). Each agent got a fresh context — no pollution from previous work.

The final stats:
- 1,056 tests, 0 failures
- 40+ command modules (34 from RTK + 6 new)
- 5-platform CI/CD (Linux x86/musl, macOS arm64/x86, Windows)
- Homebrew tap, curl installer, cargo install
- README in 4 languages
- 102 benchmark test cases

I reviewed every commit. I made architectural decisions. I caught bugs the agents missed (Rust panic compression was at 2% — I rewrote the function and got it to 80%). But the actual Rust code? That was Claude.


## Try It

```bash
curl -fsSL https://raw.githubusercontent.com/jee599/contextzip/main/install.sh | bash
```

Restart Claude Code. Every command is now compressed automatically.

After each command, you'll see:

```
💾 contextzip: 200 → 40 tokens (saved 80%)
```

Check your total savings anytime: `contextzip gain`

---

- [GitHub: jee599/contextzip](https://github.com/jee599/contextzip)
- [Benchmark results (102 tests)](https://github.com/jee599/contextzip/blob/main/docs/benchmark-results.md)
- Built on [RTK](https://github.com/rtk-ai/rtk) by rtk-ai

> The best AI tool is the one that makes AI work better. Even if AI built it.
