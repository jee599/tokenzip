//! ANSI/spinner/decoration preprocessor filter.
//!
//! Runs on ALL command output before command-specific modules.
//! Strips zero-information-value patterns: ANSI escapes, spinners,
//! progress bars (keeping final state), decoration lines, and
//! carriage-return overwrites.

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// ANSI escape sequences: \x1b[...m, \x1b[...H, etc.
    static ref ANSI_RE: Regex = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();

    /// Progress bar pattern: block chars (█░▓▒) repeated + optional percentage
    static ref PROGRESS_BAR_RE: Regex =
        Regex::new(r"[█░▓▒]{3,}.*\d+%|[█░▓▒]{5,}").unwrap();

    /// Percentage pattern for extracting progress value
    static ref PERCENT_RE: Regex = Regex::new(r"(\d+)%").unwrap();

    /// Decoration lines: same character repeated 5+ times
    static ref DECORATION_RE: Regex =
        Regex::new(r"^[\s]*([═─━━\-\*=~]{5,})[\s]*$").unwrap();

    /// Braille spinner characters (U+2800 block)
    static ref SPINNER_RE: Regex =
        Regex::new(r"^[\s]*[⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏]").unwrap();

    /// Error/warn/fail keywords (case-insensitive)
    static ref ERROR_KEYWORD_RE: Regex =
        Regex::new(r"(?i)\b(error|warn(ing)?|fail(ed|ure)?)\b").unwrap();

    /// Timestamp patterns: ISO 8601, syslog-style, HH:MM:SS, brackets with time
    static ref TIMESTAMP_RE: Regex =
        Regex::new(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}|\d{2}:\d{2}:\d{2}|\[[\d:T\-]+\]").unwrap();
}

/// Apply all preprocessing filters to command output.
///
/// Safe to call on any text — if no ANSI codes, spinners, etc. are present,
/// text passes through unchanged.
pub fn filter_ansi(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    // Step 1: Handle carriage returns — keep only last state per line
    let cr_resolved = resolve_carriage_returns(input);

    // Step 2: Strip ANSI escape sequences
    let ansi_stripped = strip_ansi_codes(&cr_resolved);

    // Step 3: Filter lines (spinners, progress bars, decorations)
    filter_lines(&ansi_stripped)
}

/// Resolve carriage returns: for each line, keep only the text after the last \r.
fn resolve_carriage_returns(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            if line.contains('\r') {
                // Split on \r and take the last non-empty segment
                line.rsplit('\r').find(|s| !s.is_empty()).unwrap_or("")
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Strip all ANSI escape sequences from text.
fn strip_ansi_codes(input: &str) -> String {
    ANSI_RE.replace_all(input, "").to_string()
}

/// Filter lines: remove spinners, intermediate progress, and decorations.
/// Preserves error/warn/fail lines and timestamp lines unconditionally.
fn filter_lines(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut result: Vec<&str> = Vec::with_capacity(lines.len());

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Always preserve error/warn/fail lines
        if ERROR_KEYWORD_RE.is_match(trimmed) {
            result.push(line);
            i += 1;
            continue;
        }

        // Always preserve timestamp lines
        if TIMESTAMP_RE.is_match(trimmed) {
            result.push(line);
            i += 1;
            continue;
        }

        // Skip spinner lines (but preserve check-mark completion lines)
        if SPINNER_RE.is_match(trimmed) {
            i += 1;
            continue;
        }

        // Handle progress bars: skip intermediate, keep 100% or final
        if is_progress_line(trimmed) {
            // Look ahead for the last consecutive progress line
            let mut last_progress_idx = i;
            let mut j = i + 1;
            while j < lines.len() {
                let next_trimmed = lines[j].trim();
                if is_progress_line(next_trimmed) || SPINNER_RE.is_match(next_trimmed) {
                    if is_progress_line(next_trimmed) {
                        last_progress_idx = j;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            // Keep only the final progress line (or 100% line)
            let final_line = lines[last_progress_idx];
            if let Some(pct) = extract_percent(final_line.trim()) {
                if pct == 100 {
                    result.push(final_line);
                }
                // Skip intermediate progress (not 100%)
            }
            i = j;
            continue;
        }

        // Skip decoration lines
        if is_decoration_line(trimmed) {
            i += 1;
            continue;
        }

        // Pass through everything else
        result.push(line);
        i += 1;
    }

    result.join("\n")
}

/// Check if a line looks like a progress bar.
fn is_progress_line(line: &str) -> bool {
    PROGRESS_BAR_RE.is_match(line)
}

/// Check if a line is purely decorative (repeated characters).
fn is_decoration_line(line: &str) -> bool {
    if DECORATION_RE.is_match(line) {
        return true;
    }

    let trimmed = line.trim();
    if trimmed.len() < 5 {
        return false;
    }

    // Check for Unicode box-drawing decorations: ═══, ───, ━━━
    let mut chars = trimmed.chars();
    if let Some(first) = chars.next() {
        if is_decoration_char(first) {
            return chars.all(|c| c == first || c.is_whitespace());
        }
    }
    false
}

/// Characters commonly used in decoration lines.
fn is_decoration_char(c: char) -> bool {
    matches!(
        c,
        '═' | '─' | '━' | '-' | '*' | '=' | '~' | '▔' | '▁' | '▂' | '▃'
    )
}

/// Extract percentage value from a line.
fn extract_percent(line: &str) -> Option<u32> {
    PERCENT_RE
        .captures(line)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1. Strip ANSI escape codes from colored text
    #[test]
    fn test_strip_ansi_escape_codes() {
        let input = "\x1b[32mSuccess\x1b[0m: build completed\n\x1b[31mError\x1b[0m: not found";
        let output = filter_ansi(input);
        assert!(output.contains("Success: build completed"));
        assert!(output.contains("Error: not found"));
        assert!(!output.contains("\x1b["));
    }

    // 2. Strip spinner characters, keep final checkmark line
    #[test]
    fn test_strip_spinner_keep_checkmark() {
        let input = "⠋ Installing packages...\n⠙ Installing packages...\n⠹ Installing packages...\n✓ Packages installed successfully";
        let output = filter_ansi(input);
        assert!(!output.contains("⠋"));
        assert!(!output.contains("⠙"));
        assert!(!output.contains("⠹"));
        assert!(output.contains("✓ Packages installed successfully"));
    }

    // 3. Strip progress bar intermediate states, keep final 100%
    #[test]
    fn test_strip_progress_keep_final() {
        let input = "████░░░░░░ 40%\n██████░░░░ 60%\n████████░░ 80%\n██████████ 100%";
        let output = filter_ansi(input);
        assert!(!output.contains("40%"));
        assert!(!output.contains("60%"));
        assert!(!output.contains("80%"));
        assert!(output.contains("100%"));
    }

    // 4. Strip decoration lines
    #[test]
    fn test_strip_decoration_lines() {
        let input = "Header\n═══════════════\nContent here\n───────────────\nMore content\n***************\nEnd";
        let output = filter_ansi(input);
        assert!(output.contains("Header"));
        assert!(output.contains("Content here"));
        assert!(output.contains("More content"));
        assert!(output.contains("End"));
        assert!(!output.contains("═══"));
        assert!(!output.contains("───"));
        assert!(!output.contains("***"));
    }

    // 5. Preserve error/warn/fail lines even if decorated
    #[test]
    fn test_preserve_error_warn_fail() {
        let input = "═══════════════\n\x1b[31mERROR: build failed\x1b[0m\nWarning: deprecated API\nTest failed at line 42\n═══════════════";
        let output = filter_ansi(input);
        assert!(output.contains("ERROR: build failed"));
        assert!(output.contains("Warning: deprecated API"));
        assert!(output.contains("Test failed at line 42"));
    }

    // 6. Preserve timestamp lines even if decorated
    #[test]
    fn test_preserve_timestamp_lines() {
        let input = "═══════════════\n2024-01-15T10:30:45 Starting build\n───────────────\n10:30:46 Build step 1\n[2024-01-15T10:30:47] Done";
        let output = filter_ansi(input);
        assert!(output.contains("2024-01-15T10:30:45 Starting build"));
        assert!(output.contains("10:30:46 Build step 1"));
        assert!(output.contains("[2024-01-15T10:30:47] Done"));
    }

    // 7. Handle carriage returns — keep only last state
    #[test]
    fn test_carriage_return_keep_last() {
        let input = "Downloading... 10%\rDownloading... 50%\rDownloading... 100%";
        let output = filter_ansi(input);
        assert!(output.contains("100%"));
        assert!(!output.contains("10%"));
    }

    // 8. Pass through normal text unchanged
    #[test]
    fn test_passthrough_normal_text() {
        let input = "src/main.rs\nsrc/lib.rs\nsrc/utils.rs";
        let output = filter_ansi(input);
        assert_eq!(output, input);
    }

    // 9. Handle empty input
    #[test]
    fn test_empty_input() {
        assert_eq!(filter_ansi(""), "");
    }

    // 10. Handle mixed content (ANSI + spinners + real content)
    #[test]
    fn test_mixed_content() {
        let input = "\x1b[32m✓\x1b[0m Build started\n\
                     ⠋ Compiling dependencies...\n\
                     ⠙ Compiling dependencies...\n\
                     ████░░░░ 50%\n\
                     ████████ 100%\n\
                     ═══════════════\n\
                     \x1b[32mBuild complete\x1b[0m\n\
                     3 modules compiled";
        let output = filter_ansi(input);
        assert!(output.contains("Build started"));
        assert!(!output.contains("⠋"));
        assert!(!output.contains("⠙"));
        assert!(output.contains("100%"));
        assert!(!output.contains("50%"));
        assert!(!output.contains("═══"));
        assert!(output.contains("Build complete"));
        assert!(output.contains("3 modules compiled"));
    }

    // 11. Decoration with equals signs
    #[test]
    fn test_decoration_equals() {
        let input = "Title\n==========\nBody";
        let output = filter_ansi(input);
        assert!(output.contains("Title"));
        assert!(output.contains("Body"));
        assert!(!output.contains("=========="));
    }

    // 12. ANSI codes in the middle of words
    #[test]
    fn test_ansi_mid_word() {
        let input = "buil\x1b[1md\x1b[0m complete";
        let output = filter_ansi(input);
        assert_eq!(output, "build complete");
    }

    // 13. Multiple ANSI codes stacked
    #[test]
    fn test_multiple_ansi_codes() {
        let input = "\x1b[1m\x1b[31m\x1b[4mBold Red Underline\x1b[0m";
        let output = filter_ansi(input);
        assert_eq!(output, "Bold Red Underline");
    }

    // 14. Short decoration lines (< 5 chars) should pass through
    #[test]
    fn test_short_decoration_passthrough() {
        let input = "---\nreal content\n===";
        let output = filter_ansi(input);
        assert!(output.contains("---"));
        assert!(output.contains("==="));
        assert!(output.contains("real content"));
    }

    // 15. Progress bar without percentage should be stripped
    #[test]
    fn test_progress_bar_no_percent() {
        let input = "██████████████████████████\nDone";
        let output = filter_ansi(input);
        assert!(!output.contains("████"));
        assert!(output.contains("Done"));
    }
}
