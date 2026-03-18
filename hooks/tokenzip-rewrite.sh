#!/usr/bin/env bash
# tokenzip-hook-version: 2
# TokenZip Claude Code hook — rewrites commands to use tokenzip for token savings.
# Requires: tokenzip >= 0.1.0, jq
#
# This is a thin delegating hook: all rewrite logic lives in `tokenzip rewrite`,
# which is the single source of truth (src/discover/registry.rs).
# To add or change rewrite rules, edit the Rust registry — not this file.

if ! command -v jq &>/dev/null; then
  echo "[tokenzip] WARNING: jq is not installed. Hook cannot rewrite commands. Install jq: https://jqlang.github.io/jq/download/" >&2
  exit 0
fi

if ! command -v tokenzip &>/dev/null; then
  echo "[tokenzip] WARNING: tokenzip is not installed or not in PATH. Hook cannot rewrite commands." >&2
  exit 0
fi

INPUT=$(cat)
CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$CMD" ]; then
  exit 0
fi

# Delegate all rewrite logic to the Rust binary.
# tokenzip rewrite exits 1 when there's no rewrite — hook passes through silently.
REWRITTEN=$(tokenzip rewrite "$CMD" 2>/dev/null) || exit 0

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
      "permissionDecisionReason": "TokenZip auto-rewrite",
      "updatedInput": $updated
    }
  }'
