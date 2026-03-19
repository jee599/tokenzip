#!/bin/bash
# RTK suggest hook for Claude Code PreToolUse:Bash
# Emits system reminders when rtk-compatible commands are detected.
# Outputs JSON with systemMessage to inform Claude Code without modifying execution.

set -euo pipefail

INPUT=$(cat)
CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$CMD" ]; then
  exit 0
fi

# Extract the first meaningful command (before pipes, &&, etc.)
FIRST_CMD="$CMD"

# Skip if already using rtk
case "$FIRST_CMD" in
  rtk\ *|*/rtk\ *) exit 0 ;;
esac

# Skip commands with heredocs, variable assignments, etc.
case "$FIRST_CMD" in
  *'<<'*) exit 0 ;;
esac

SUGGESTION=""

# --- Git commands ---
if echo "$FIRST_CMD" | grep -qE '^git\s+status(\s|$)'; then
  SUGGESTION="contextzip git status"
elif echo "$FIRST_CMD" | grep -qE '^git\s+diff(\s|$)'; then
  SUGGESTION="contextzip git diff"
elif echo "$FIRST_CMD" | grep -qE '^git\s+log(\s|$)'; then
  SUGGESTION="contextzip git log"
elif echo "$FIRST_CMD" | grep -qE '^git\s+add(\s|$)'; then
  SUGGESTION="contextzip git add"
elif echo "$FIRST_CMD" | grep -qE '^git\s+commit(\s|$)'; then
  SUGGESTION="contextzip git commit"
elif echo "$FIRST_CMD" | grep -qE '^git\s+push(\s|$)'; then
  SUGGESTION="contextzip git push"
elif echo "$FIRST_CMD" | grep -qE '^git\s+pull(\s|$)'; then
  SUGGESTION="contextzip git pull"
elif echo "$FIRST_CMD" | grep -qE '^git\s+branch(\s|$)'; then
  SUGGESTION="contextzip git branch"
elif echo "$FIRST_CMD" | grep -qE '^git\s+fetch(\s|$)'; then
  SUGGESTION="contextzip git fetch"
elif echo "$FIRST_CMD" | grep -qE '^git\s+stash(\s|$)'; then
  SUGGESTION="contextzip git stash"
elif echo "$FIRST_CMD" | grep -qE '^git\s+show(\s|$)'; then
  SUGGESTION="contextzip git show"

# --- GitHub CLI ---
elif echo "$FIRST_CMD" | grep -qE '^gh\s+(pr|issue|run)(\s|$)'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^gh /contextzip gh /')

# --- Cargo ---
elif echo "$FIRST_CMD" | grep -qE '^cargo\s+test(\s|$)'; then
  SUGGESTION="contextzip cargo test"
elif echo "$FIRST_CMD" | grep -qE '^cargo\s+build(\s|$)'; then
  SUGGESTION="contextzip cargo build"
elif echo "$FIRST_CMD" | grep -qE '^cargo\s+clippy(\s|$)'; then
  SUGGESTION="contextzip cargo clippy"
elif echo "$FIRST_CMD" | grep -qE '^cargo\s+check(\s|$)'; then
  SUGGESTION="contextzip cargo check"
elif echo "$FIRST_CMD" | grep -qE '^cargo\s+install(\s|$)'; then
  SUGGESTION="contextzip cargo install"
elif echo "$FIRST_CMD" | grep -qE '^cargo\s+nextest(\s|$)'; then
  SUGGESTION="contextzip cargo nextest"
elif echo "$FIRST_CMD" | grep -qE '^cargo\s+fmt(\s|$)'; then
  SUGGESTION="contextzip cargo fmt"

# --- File operations ---
elif echo "$FIRST_CMD" | grep -qE '^cat\s+'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^cat /contextzip read /')
elif echo "$FIRST_CMD" | grep -qE '^(rg|grep)\s+'; then
  SUGGESTION=$(echo "$CMD" | sed -E 's/^(rg|grep) /contextzip grep /')
elif echo "$FIRST_CMD" | grep -qE '^ls(\s|$)'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^ls/contextzip ls/')
elif echo "$FIRST_CMD" | grep -qE '^tree(\s|$)'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^tree/contextzip tree/')
elif echo "$FIRST_CMD" | grep -qE '^find\s+'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^find /contextzip find /')
elif echo "$FIRST_CMD" | grep -qE '^diff\s+'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^diff /contextzip diff /')
elif echo "$FIRST_CMD" | grep -qE '^head\s+'; then
  # Suggest contextzip read with --max-lines transformation
  if echo "$FIRST_CMD" | grep -qE '^head\s+-[0-9]+\s+'; then
    LINES=$(echo "$FIRST_CMD" | sed -E 's/^head +-([0-9]+) +.+$/\1/')
    FILE=$(echo "$FIRST_CMD" | sed -E 's/^head +-[0-9]+ +(.+)$/\1/')
    SUGGESTION="contextzip read $FILE --max-lines $LINES"
  elif echo "$FIRST_CMD" | grep -qE '^head\s+--lines=[0-9]+\s+'; then
    LINES=$(echo "$FIRST_CMD" | sed -E 's/^head +--lines=([0-9]+) +.+$/\1/')
    FILE=$(echo "$FIRST_CMD" | sed -E 's/^head +--lines=[0-9]+ +(.+)$/\1/')
    SUGGESTION="contextzip read $FILE --max-lines $LINES"
  fi

# --- JS/TS tooling ---
elif echo "$FIRST_CMD" | grep -qE '^(pnpm\s+)?vitest(\s|$)'; then
  SUGGESTION="contextzip vitest run"
elif echo "$FIRST_CMD" | grep -qE '^pnpm\s+test(\s|$)'; then
  SUGGESTION="contextzip vitest run"
elif echo "$FIRST_CMD" | grep -qE '^pnpm\s+tsc(\s|$)'; then
  SUGGESTION="contextzip tsc"
elif echo "$FIRST_CMD" | grep -qE '^(npx\s+)?tsc(\s|$)'; then
  SUGGESTION="contextzip tsc"
elif echo "$FIRST_CMD" | grep -qE '^pnpm\s+lint(\s|$)'; then
  SUGGESTION="contextzip lint"
elif echo "$FIRST_CMD" | grep -qE '^(npx\s+)?eslint(\s|$)'; then
  SUGGESTION="contextzip lint"
elif echo "$FIRST_CMD" | grep -qE '^(npx\s+)?prettier(\s|$)'; then
  SUGGESTION="contextzip prettier"
elif echo "$FIRST_CMD" | grep -qE '^(npx\s+)?playwright(\s|$)'; then
  SUGGESTION="contextzip playwright"
elif echo "$FIRST_CMD" | grep -qE '^pnpm\s+playwright(\s|$)'; then
  SUGGESTION="contextzip playwright"
elif echo "$FIRST_CMD" | grep -qE '^(npx\s+)?prisma(\s|$)'; then
  SUGGESTION="contextzip prisma"

# --- Containers ---
elif echo "$FIRST_CMD" | grep -qE '^docker\s+(ps|images|logs)(\s|$)'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^docker /contextzip docker /')
elif echo "$FIRST_CMD" | grep -qE '^kubectl\s+(get|logs)(\s|$)'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^kubectl /contextzip kubectl /')

# --- Network ---
elif echo "$FIRST_CMD" | grep -qE '^curl\s+'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^curl /contextzip curl /')
elif echo "$FIRST_CMD" | grep -qE '^wget\s+'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^wget /contextzip wget /')

# --- pnpm package management ---
elif echo "$FIRST_CMD" | grep -qE '^pnpm\s+(list|ls|outdated)(\s|$)'; then
  SUGGESTION=$(echo "$CMD" | sed 's/^pnpm /contextzip pnpm /')
fi

# If no suggestion, allow command as-is
if [ -z "$SUGGESTION" ]; then
  exit 0
fi

# Output suggestion as system message
jq -n \
  --arg suggestion "$SUGGESTION" \
  '{
    "hookSpecificOutput": {
      "hookEventName": "PreToolUse",
      "permissionDecision": "allow",
      "systemMessage": ("⚡ RTK available: `" + $suggestion + "` (60-90% token savings)")
    }
  }'
