#!/usr/bin/env bash
# contextzip-hook-version: 2
# ContextZip Claude Code hook — rewrites commands to use contextzip for token savings.
# Requires: contextzip >= 0.1.0, jq
#
# This is a thin delegating hook: all rewrite logic lives in `contextzip rewrite`,
# which is the single source of truth (src/discover/registry.rs).
# To add or change rewrite rules, edit the Rust registry — not this file.

if ! command -v jq &>/dev/null; then
  echo "[contextzip] WARNING: jq is not installed. Hook cannot rewrite commands. Install jq: https://jqlang.github.io/jq/download/" >&2
  exit 0
fi

if ! command -v contextzip &>/dev/null; then
  echo "[contextzip] WARNING: contextzip is not installed or not in PATH. Hook cannot rewrite commands." >&2
  exit 0
fi

INPUT=$(cat)
CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$CMD" ]; then
  exit 0
fi

# Delegate all rewrite logic to the Rust binary.
# contextzip rewrite exits 1 when there's no rewrite — hook passes through silently.
REWRITTEN=$(contextzip rewrite "$CMD" 2>/dev/null) || exit 0

# No change — nothing to do.
if [ "$CMD" = "$REWRITTEN" ]; then
  exit 0
fi

ORIGINAL_INPUT=$(echo "$INPUT" | jq -c '.tool_input')
UPDATED_INPUT=$(echo "$ORIGINAL_INPUT" | jq --arg cmd "$REWRITTEN" '.command = $cmd')

jq -n \
  --argjson updated "$UPDATED_INPUT" \
  '{
    "hookSpecificOutput": {
      "hookEventName": "PreToolUse",
      "permissionDecision": "allow",
      "permissionDecisionReason": "ContextZip auto-rewrite",
      "updatedInput": $updated
    }
  }'
