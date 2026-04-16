# I measured my Claude Code context — 85.8% is tool history

*Or: why your assistant forgets, and how I built a thing to fix it.*

---

## The problem you've felt

You start a long Claude Code session. It's helpful. Then somewhere around hour two, the model starts feeling… stale. It re-reads files it already read. It forgets what `Edit` you ran. The context-window indicator creeps toward the line.

I assumed — like most people — that this was about Claude's own verbose responses. So before I went off and shipped a "make Claude reply more tersely" project, I decided to actually measure.

## What's actually in your context

Claude Code stores every session as a single JSONL file under `~/.claude/projects/<project>/<session-id>.jsonl`. Every user prompt, every Claude response, every tool call, every tool result — appended one record per line. So the data is just sitting there, waiting to be counted.

I sampled 10 of my largest sessions, **6,850 assistant messages, 7.88 MB of text**, and asked: where does the context actually go?

```
Tool inputs (Edit/Write/Bash/Read/Agent args) | 46.4%
Tool results (Read/Bash/Agent outputs)         | 39.4%
User text                                       | 10.1%
Assistant text                                  |  4.1%
```

**85.8% of context is tool history.** Not Claude's prose. Not my prompts. The mechanical exhaust of every command that ever ran.

By tool:

```
Read    22.1%   — same files re-read 5–14× per session
Agent   20.0%   — subagent prompts that copy half the parent context
Write   18.9%   — file payloads frozen forever in input args
Edit    15.8%   — old_string/new_string pairs accumulating
Bash    15.0%   — already-filtered output, but accumulated history dominates
                  the live filter's gains
                  (ContextZip's existing live-stdout compression is just the
                   tip of the iceberg)
```

The top 5 tools are 91.8% of total context.

## Why this matters

Most context-saving advice points at the wrong place:

- *"Tell Claude to be more concise"* — fights for 4% of the budget.
- *"Compress live stdout"* — what RTK / ContextZip v0.1 already do, but it only ever sees one Bash result at a time, not the accumulated history.
- *"Just /compact in the UI"* — the built-in compactor preserves narrative but doesn't know which exact tool calls are redundant.

The leverage is in the JSONL itself, after the fact, where the same `Read /path/to/foo.rs` appears 14 times and a thousand-line cargo build sits in five different past tool_results.

## The fix

I built **`contextzip compact`** — a new CLI in [ContextZip v0.2](https://github.com/jee599/contextzip) that operates on the session JSONL directly:

```bash
contextzip compact <session-id>   # writes a .compressed sidecar
contextzip apply   <session-id>   # atomic swap; original kept as .bak
contextzip expand  <session-id>   # roll back to .bak; sidecar preserved
```

It ships with two safe axes only — the only two that survived an honest review of resume-safety risks:

1. **`ReadDedup`** — when the same file path is read multiple times, every result after the first becomes a short reference back to the first read. The reference includes the file path so a future `expand` can re-fetch from disk if anything diverged.
2. **`BashHistoryCompact`** — past `Bash` tool results are re-fed through ContextZip's existing line-based filters (ANSI strip, repeat-line tally, blank-run drop). Idempotent — re-running on already-compressed data is a no-op.

Three more axes (`WritePlaceholder`, `EditDelta`, `AgentPromptStrip`) were considered and dropped from v0.2: each had a failure mode where Claude needs the missing context to reason backwards. Better to leave them out than to ship a compressor that breaks resume.

## What it actually saves

First measurement on a real 57.3 MB session, 2,475 records:

```
57,347,003  →  53,516,418 bytes  (6.7% saved)
ReadDedup:        153 hits
BashHistoryCompact: 44 hits
```

6.7% isn't a headline number. It's an honest first data point. Sessions with more repeated reads (large monorepos, long debugging arcs) will see more. Sessions that are mostly novel exploration will see less. The honest target band is **6–12% with zero task-failure regressions** — verified on five real resumed sessions before I shipped it.

The `apply / expand` round-trip is **byte-for-byte lossless** when no source file has changed (verified by test). The original `.jsonl` is never modified. Worst-case rollback is `rm <session>.jsonl.compressed`.

## What I learned

- **Measure before you optimize.** I almost spent a week on Claude-output compression, which would have moved 0.5% of the budget at best.
- **The diff between "obvious target" and "actual target" is huge.** "Make AI replies shorter" is the obvious target. "Compact the JSONL after the fact" is the actual one.
- **Honest numbers beat impressive ones.** The first verification subagent told me my original "91.8% → 35%" pitch was hand-wavy and to ship the safe 8% instead. Right call.
- **Reversible everything.** Sidecar + .bak + expand makes the entire feature opt-in and refundable. People will actually try it.

## Try it

```bash
npx contextzip
contextzip compact <your-session-id>
```

Code: [github.com/jee599/contextzip](https://github.com/jee599/contextzip). 1,112 tests, clippy clean, MIT licensed, fork of [rtk-ai/rtk](https://github.com/rtk-ai/rtk).

If you measure your own sessions, I'd love to see the breakdown — open an issue with the four-bucket percentages.

---

*ContextZip is built by [Jidong Lee](https://github.com/jee599). It's a fork of RTK with a session-history layer added on top. The empirical baseline data in this post is reproducible — the analysis script is in the repo under `docs/`.*
