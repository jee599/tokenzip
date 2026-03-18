use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Security keywords that must NEVER be removed
    static ref SECURITY_RE: Regex = Regex::new(
        r"(?i)(vulnerabilit|security|critical|breaking|CVE-|GHSA-)"
    ).unwrap();

    /// Deprecated warnings (non-security)
    static ref DEPRECATED_RE: Regex = Regex::new(
        r"(?i)(npm warn deprecated|WARN deprecated|deprecat)"
    ).unwrap();

    /// Funding messages
    static ref FUNDING_RE: Regex = Regex::new(
        r"(?i)(packages? (are|is) looking for funding|run .+fund)"
    ).unwrap();

    /// Dependency resolution / audit summary noise
    static ref RESOLUTION_RE: Regex = Regex::new(
        r"(?i)(added \d+ packages|audited \d+ packages|removed \d+ packages|changed \d+ packages)"
    ).unwrap();

    /// Progress bars
    static ref PROGRESS_RE: Regex = Regex::new(
        r"(\[#+\.+\]|\[#+\s*\]|\[\s*#+\])\s*\d+%"
    ).unwrap();

    /// pip "already satisfied"
    static ref ALREADY_SATISFIED_RE: Regex = Regex::new(
        r"(?i)already satisfied"
    ).unwrap();

    /// pip "Using cached"
    static ref USING_CACHED_RE: Regex = Regex::new(
        r"(?i)using cached"
    ).unwrap();

    /// npm install summary line: "added N packages in Ns"
    static ref NPM_SUMMARY_RE: Regex = Regex::new(
        r"added (\d+) packages.*?in (\d+)s"
    ).unwrap();

    /// Vulnerability summary: "N vulnerabilities (details)"
    static ref VULN_SUMMARY_RE: Regex = Regex::new(
        r"(\d+)\s+vulnerabilit(y|ies)\s*\(([^)]+)\)"
    ).unwrap();
}

/// Compress package install logs from npm/yarn/pnpm/pip/cargo.
///
/// Removes noise (deprecated warnings, funding, progress bars, etc.)
/// while preserving all security-related warnings.
pub fn compress_pkg_log(input: &str) -> String {
    // Quick check: if input has no package manager patterns, passthrough
    if !is_pkg_output(input) {
        return input.to_string();
    }

    let mut kept_lines: Vec<String> = Vec::new();
    let mut pkg_count: Option<(u64, u64)> = None; // (packages, seconds)
    let mut vuln_summary: Option<String> = None;

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        // ALWAYS keep lines with security keywords
        if SECURITY_RE.is_match(trimmed) {
            // Format security deprecated warnings specially
            if DEPRECATED_RE.is_match(trimmed) {
                kept_lines.push(format!("\u{26a0} {}", trimmed));
            } else if let Some(caps) = VULN_SUMMARY_RE.captures(trimmed) {
                // Extract vulnerability summary
                let count = caps.get(1).map(|m| m.as_str()).unwrap_or("?");
                let details = caps.get(3).map(|m| m.as_str()).unwrap_or("");
                vuln_summary = Some(format!("\u{26a0} {} vulnerabilities ({})", count, details));
            } else {
                kept_lines.push(format!("\u{26a0} {}", trimmed));
            }
            continue;
        }

        // Remove non-security deprecated warnings
        if DEPRECATED_RE.is_match(trimmed) {
            continue;
        }

        // Remove funding messages
        if FUNDING_RE.is_match(trimmed) {
            continue;
        }

        // Extract summary from resolution lines, then skip them
        if RESOLUTION_RE.is_match(trimmed) {
            if let Some(caps) = NPM_SUMMARY_RE.captures(trimmed) {
                let count: u64 = caps
                    .get(1)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(0);
                let secs: u64 = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(0);
                pkg_count = Some((count, secs));
            }
            continue;
        }

        // Remove progress bars
        if PROGRESS_RE.is_match(trimmed) {
            continue;
        }

        // Remove pip "already satisfied"
        if ALREADY_SATISFIED_RE.is_match(trimmed) {
            continue;
        }

        // Remove pip "Using cached"
        if USING_CACHED_RE.is_match(trimmed) {
            continue;
        }

        // Keep everything else
        kept_lines.push(line.to_string());
    }

    // Build compressed output
    let mut result = Vec::new();

    if let Some((count, secs)) = pkg_count {
        result.push(format!("\u{2713} {} packages ({}s)", count, secs));
    }

    if let Some(ref vuln) = vuln_summary {
        result.push(vuln.clone());
    }

    result.extend(kept_lines);

    if result.is_empty() {
        return input.to_string();
    }

    result.join("\n")
}

/// Detect if input looks like package manager output
fn is_pkg_output(input: &str) -> bool {
    let lower = input.to_lowercase();
    let indicators = [
        "npm warn",
        "added ",
        " packages",
        "looking for funding",
        "vulnerabilit",
        "already satisfied",
        "using cached",
        "downloading",
        "installing collected",
        "successfully installed",
        "compiling",
        "deprecated",
        "audited",
    ];
    indicators.iter().any(|ind| lower.contains(ind))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_tokens(s: &str) -> usize {
        s.split_whitespace().count()
    }

    #[test]
    fn test_npm_install_removes_deprecated_funding_keeps_summary() {
        let input = r#"npm warn deprecated rimraf@3.0.2: Rimraf versions prior to v4 are no longer supported
npm warn deprecated inflight@1.0.6: This module is not supported
npm warn deprecated glob@7.2.3: Glob versions prior to v9 are no longer supported

added 847 packages, and audited 848 packages in 32s

143 packages are looking for funding
  run `npm fund` for details

8 vulnerabilities (6 high, 2 moderate)
"#;
        let result = compress_pkg_log(input);

        // Deprecated warnings removed
        assert!(!result.contains("rimraf@3.0.2"));
        assert!(!result.contains("inflight@1.0.6"));
        assert!(!result.contains("glob@7.2.3"));

        // Funding removed
        assert!(!result.contains("looking for funding"));
        assert!(!result.contains("npm fund"));

        // Summary preserved
        assert!(result.contains("\u{2713} 847 packages (32s)"));

        // Vulnerability summary preserved
        assert!(result.contains("\u{26a0} 8 vulnerabilities (6 high, 2 moderate)"));
    }

    #[test]
    fn test_preserve_security_deprecated_cve() {
        let input = r#"npm warn deprecated bcrypt@3.0.0: security vulnerability (CVE-2023-31484)
npm warn deprecated rimraf@3.0.2: Rimraf versions prior to v4 are no longer supported
added 100 packages in 5s
"#;
        let result = compress_pkg_log(input);

        // Security deprecated kept
        assert!(result.contains("CVE-2023-31484"));
        assert!(result.contains("bcrypt@3.0.0"));

        // Non-security deprecated removed
        assert!(!result.contains("rimraf"));
    }

    #[test]
    fn test_preserve_vulnerability_warnings() {
        let input = r#"added 200 packages in 10s
6 vulnerabilities (2 critical, 3 high, 1 moderate)
"#;
        let result = compress_pkg_log(input);

        assert!(result.contains("\u{2713} 200 packages (10s)"));
        assert!(result.contains("vulnerabilities"));
        assert!(result.contains("2 critical, 3 high, 1 moderate"));
    }

    #[test]
    fn test_pip_install_removes_already_satisfied_and_cached() {
        let input = r#"Requirement already satisfied: requests in /usr/lib/python3/dist-packages (2.28.1)
Requirement already satisfied: urllib3<3,>=1.21.1 in /usr/lib/python3/dist-packages (from requests) (1.26.5)
Using cached certifi-2023.7.22-py3-none-any.whl (158 kB)
Using cached charset_normalizer-3.2.0-cp311-cp311-manylinux_2_17_x86_64.manylinux2014_x86_64.whl (197 kB)
Successfully installed flask-2.3.3
"#;
        let result = compress_pkg_log(input);

        // "already satisfied" removed
        assert!(!result.contains("already satisfied"));

        // "Using cached" removed
        assert!(!result.contains("Using cached"));

        // Real output preserved
        assert!(result.contains("Successfully installed flask-2.3.3"));
    }

    #[test]
    fn test_preserve_ghsa_warnings() {
        let input = r#"npm warn deprecated some-pkg@1.0.0: This package has a known security issue (GHSA-abcd-1234-efgh)
added 50 packages in 3s
"#;
        let result = compress_pkg_log(input);

        assert!(result.contains("GHSA-abcd-1234-efgh"));
        assert!(result.contains("some-pkg@1.0.0"));
    }

    #[test]
    fn test_preserve_critical_keyword_warnings() {
        let input = r#"npm warn deprecated crypto-lib@2.0.0: critical security flaw, upgrade immediately
npm warn deprecated old-util@1.0.0: Use new-util instead
added 30 packages in 2s
"#;
        let result = compress_pkg_log(input);

        // "critical" keyword preserved
        assert!(result.contains("crypto-lib@2.0.0"));
        assert!(result.contains("critical security flaw"));

        // Non-security deprecated removed
        assert!(!result.contains("old-util"));
    }

    #[test]
    fn test_normal_output_passthrough() {
        let input = "Hello world\nThis is normal output\nNo package patterns here\n";
        let result = compress_pkg_log(input);

        assert_eq!(result, input);
    }

    #[test]
    fn test_progress_bars_removed() {
        let input = r#"[####............] 34%
[########........] 50%
[################] 100%
added 100 packages in 5s
Successfully installed everything
"#;
        let result = compress_pkg_log(input);

        assert!(!result.contains("[####"));
        assert!(!result.contains("34%"));
        assert!(result.contains("\u{2713} 100 packages (5s)"));
    }

    #[test]
    fn test_token_savings() {
        let input = r#"npm warn deprecated rimraf@3.0.2: Rimraf versions prior to v4 are no longer supported
npm warn deprecated inflight@1.0.6: This module is not supported
npm warn deprecated glob@7.2.3: Glob versions prior to v9 are no longer supported
npm warn deprecated rimraf@2.7.1: Rimraf versions prior to v4 are no longer supported
npm warn deprecated source-map-resolve@0.6.0: See https://github.com/lydell/source-map-resolve#deprecated

added 847 packages, and audited 848 packages in 32s

143 packages are looking for funding
  run `npm fund` for details

8 vulnerabilities (6 high, 2 moderate)

To address all issues, run:
  npm audit fix
"#;
        let result = compress_pkg_log(input);
        let input_tokens = count_tokens(input);
        let output_tokens = count_tokens(&result);
        let savings = 100.0 - (output_tokens as f64 / input_tokens as f64 * 100.0);

        assert!(
            savings >= 60.0,
            "Expected >=60% savings, got {:.1}%",
            savings
        );
    }
}
