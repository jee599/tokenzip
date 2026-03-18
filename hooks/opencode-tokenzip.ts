import type { Plugin } from "@opencode-ai/plugin"

// TokenZip OpenCode plugin — rewrites commands to use rtk for token savings.
// Requires: tokenzip >= 0.1.0 in PATH.
//
// This is a thin delegating plugin: all rewrite logic lives in `tokenzip rewrite`,
// which is the single source of truth (src/discover/registry.rs).
// To add or change rewrite rules, edit the Rust registry — not this file.

export const TokenZipOpenCodePlugin: Plugin = async ({ $ }) => {
  try {
    await $`which tokenzip`.quiet()
  } catch {
    console.warn("[tokenzip] tokenzip binary not found in PATH — plugin disabled")
    return {}
  }

  return {
    "tool.execute.before": async (input, output) => {
      const tool = String(input?.tool ?? "").toLowerCase()
      if (tool !== "bash" && tool !== "shell") return
      const args = output?.args
      if (!args || typeof args !== "object") return

      const command = (args as Record<string, unknown>).command
      if (typeof command !== "string" || !command) return

      try {
        const result = await $`tokenzip rewrite ${command}`.quiet().nothrow()
        const rewritten = String(result.stdout).trim()
        if (rewritten && rewritten !== command) {
          ;(args as Record<string, unknown>).command = rewritten
        }
      } catch {
        // tokenzip rewrite failed — pass through unchanged
      }
    },
  }
}
