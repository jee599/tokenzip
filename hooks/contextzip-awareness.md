# ContextZip - Token-Optimized CLI (based on RTK)

**Usage**: Token-optimized CLI proxy (60-90% savings on dev operations)

## Meta Commands (always use contextzip directly)

```bash
contextzip gain              # Show token savings analytics
contextzip gain --history    # Show command usage history with savings
contextzip discover          # Analyze Claude Code history for missed opportunities
contextzip proxy <cmd>       # Execute raw command without filtering (for debugging)
```

## Installation Verification

```bash
contextzip --version         # Should show: contextzip X.Y.Z (based on rtk 0.30.1)
contextzip gain              # Should work (not "command not found")
which contextzip             # Verify correct binary
```

## Hook-Based Usage

All other commands are automatically rewritten by the Claude Code hook.
Example: `git status` → `contextzip git status` (transparent, 0 tokens overhead)

Refer to CLAUDE.md for full command reference.
