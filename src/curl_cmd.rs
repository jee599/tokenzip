use crate::json_cmd;
use crate::tracking;
use crate::utils::{resolved_command, truncate};
use anyhow::{Context, Result};

pub fn run(args: &[String], verbose: u8) -> Result<()> {
    let timer = tracking::TimedExecution::start();
    let mut cmd = resolved_command("curl");
    cmd.arg("-s"); // Silent mode (no progress bar)

    for arg in args {
        cmd.arg(arg);
    }

    if verbose > 0 {
        eprintln!("Running: curl -s {}", args.join(" "));
    }

    let output = cmd.output().context("Failed to run curl")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        let msg = if stderr.trim().is_empty() {
            stdout.trim().to_string()
        } else {
            stderr.trim().to_string()
        };
        eprintln!("FAILED: curl {}", msg);
        std::process::exit(output.status.code().unwrap_or(1));
    }

    let raw = stdout.to_string();

    // Internal/localhost requests (health checks, dev servers) usually want
    // the raw response — running them through JSON-schema collapse breaks
    // status checks and assertion-based tests. Pass through unchanged.
    let filtered = if targets_localhost(args) {
        raw.clone()
    } else {
        filter_curl_output(&stdout)
    };
    println!("{}", filtered);

    timer.track(
        &format!("curl {}", args.join(" ")),
        &format!("contextzip curl {}", args.join(" ")),
        &raw,
        &filtered,
    );

    Ok(())
}

fn targets_localhost(args: &[String]) -> bool {
    args.iter().any(|a| {
        let lower = a.to_lowercase();
        is_loopback_host(&lower, "localhost")
            || is_loopback_host(&lower, "127.0.0.1")
            || is_loopback_host(&lower, "[::1]")
            || is_loopback_host(&lower, "0.0.0.0")
            || has_internal_tld(&lower)
    })
}

/// True if `url` has `//<host>` followed immediately by `:`, `/`, `?`, `#`, or end-of-string —
/// i.e. matches the host exactly, not a prefix like `localhost.example.com`.
fn is_loopback_host(url: &str, host: &str) -> bool {
    let needle = format!("//{}", host);
    let Some(idx) = url.find(&needle) else {
        return false;
    };
    let rest = &url[idx + needle.len()..];
    rest.is_empty()
        || rest.starts_with(':')
        || rest.starts_with('/')
        || rest.starts_with('?')
        || rest.starts_with('#')
}

fn has_internal_tld(url: &str) -> bool {
    // Looking for hostname ending in `.internal` or `.local` *as a TLD*, e.g.
    // `https://service.internal/x` or `https://api.svc.local`.
    // Skip the scheme then check hostname boundary.
    let after_scheme = url.split("://").nth(1).unwrap_or(url);
    let host = after_scheme
        .split(['/', '?', '#', ':'])
        .next()
        .unwrap_or(after_scheme);
    host.ends_with(".internal") || host.ends_with(".local")
}

fn filter_curl_output(output: &str) -> String {
    let trimmed = output.trim();

    // Try JSON detection: starts with { or [
    if (trimmed.starts_with('{') || trimmed.starts_with('['))
        && (trimmed.ends_with('}') || trimmed.ends_with(']'))
    {
        if let Ok(schema) = json_cmd::filter_json_string(trimmed, 5) {
            // Only use schema if it's actually shorter than the original (#297)
            if schema.len() <= trimmed.len() {
                return schema;
            }
        }
    }

    // Not JSON: truncate long output
    let lines: Vec<&str> = trimmed.lines().collect();
    if lines.len() > 30 {
        let mut result: Vec<&str> = lines[..30].to_vec();
        result.push("");
        let msg = format!(
            "... ({} more lines, {} bytes total)",
            lines.len() - 30,
            trimmed.len()
        );
        return format!("{}\n{}", result.join("\n"), msg);
    }

    // Short output: return as-is but truncate long lines
    lines
        .iter()
        .map(|l| truncate(l, 200))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_curl_json() {
        // Large JSON where schema is shorter than original — schema should be returned
        let output = r#"{"name": "a very long user name here", "count": 42, "items": [1, 2, 3], "description": "a very long description that takes up many characters in the original JSON payload", "status": "active", "url": "https://example.com/api/v1/users/123"}"#;
        let result = filter_curl_output(output);
        assert!(result.contains("name"));
        assert!(result.contains("string"));
        assert!(result.contains("int"));
    }

    #[test]
    fn test_filter_curl_json_array() {
        let output = r#"[{"id": 1}, {"id": 2}]"#;
        let result = filter_curl_output(output);
        assert!(result.contains("id"));
    }

    #[test]
    fn test_filter_curl_non_json() {
        let output = "Hello, World!\nThis is plain text.";
        let result = filter_curl_output(output);
        assert!(result.contains("Hello, World!"));
        assert!(result.contains("plain text"));
    }

    #[test]
    fn test_filter_curl_json_small_returns_original() {
        // Small JSON where schema would be larger than original (issue #297)
        let output = r#"{"r2Ready":true,"status":"ok"}"#;
        let result = filter_curl_output(output);
        // Schema would be "{\n  r2Ready: bool,\n  status: string\n}" which is longer
        // Should return the original JSON unchanged
        assert_eq!(result.trim(), output.trim());
    }

    #[test]
    fn test_filter_curl_long_output() {
        let lines: Vec<String> = (0..50).map(|i| format!("Line {}", i)).collect();
        let output = lines.join("\n");
        let result = filter_curl_output(&output);
        assert!(result.contains("Line 0"));
        assert!(result.contains("Line 29"));
        assert!(result.contains("more lines"));
    }

    #[test]
    fn targets_localhost_recognises_loopback_hosts() {
        for url in [
            "http://localhost:3000/api",
            "http://127.0.0.1/health",
            "https://[::1]/x",
            "http://0.0.0.0:8080",
            "https://api.svc.local/v1",
            "https://service.internal/check",
        ] {
            assert!(
                targets_localhost(&[url.to_string()]),
                "expected localhost match for {}",
                url
            );
        }
    }

    #[test]
    fn targets_localhost_does_not_match_public_urls() {
        for url in [
            "https://example.com/api",
            "https://github.com/jee599/contextzip",
            "https://localhost.example.com/x",
        ] {
            assert!(
                !targets_localhost(&[url.to_string()]),
                "did not expect localhost match for {}",
                url
            );
        }
    }
}
