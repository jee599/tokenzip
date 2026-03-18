# TokenZip - Token-Optimized CLI (based on RTK)

**Usage**: Token-optimized CLI proxy (60-90% savings on dev operations)

## Meta Commands (always use tokenzip directly)

```bash
tokenzip gain              # Show token savings analytics
tokenzip gain --history    # Show command usage history with savings
tokenzip discover          # Analyze Claude Code history for missed opportunities
tokenzip proxy <cmd>       # Execute raw command without filtering (for debugging)
```

## Installation Verification

```bash
tokenzip --version         # Should show: tokenzip X.Y.Z (based on rtk 0.30.1)
tokenzip gain              # Should work (not "command not found")
which tokenzip             # Verify correct binary
```

## Hook-Based Usage

All other commands are automatically rewritten by the Claude Code hook.
Example: `git status` → `tokenzip git status` (transparent, 0 tokens overhead)

Refer to CLAUDE.md for full command reference.
