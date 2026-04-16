//! `contextzip compact <session-id-or-path>` CLI.
//!
//! Wraps `jsonl_rewriter::compact_session_file`. Resolves a session-id like
//! `bfb59668-...` to its `.jsonl` under `~/.claude/projects/<project>/`,
//! produces a sidecar `.compressed` file, and prints a one-line summary.
//!
//! The original `.jsonl` is never modified. Rollback is `rm <sidecar>`. Apply
//! / expand commands will be added in a follow-up.

use anyhow::{bail, Context, Result};
use std::path::PathBuf;

use crate::jsonl_rewriter;

pub fn run(target: &str, verbose: u8) -> Result<()> {
    let session_path = resolve_session_path(target)?;

    if verbose > 0 {
        eprintln!("contextzip compact: {}", session_path.display());
    }

    let (sidecar, stats) = jsonl_rewriter::compact_session_file(&session_path)
        .with_context(|| format!("Failed to compact session: {}", session_path.display()))?;

    println!(
        "compact: {} → {}\n  records: {}, bytes: {} → {} ({:.1}% saved)\n  axes: ReadDedup={}, BashHistoryCompact={}",
        session_path.display(),
        sidecar.display(),
        stats.records_written,
        stats.bytes_in,
        stats.bytes_out,
        stats.percent_saved(),
        stats.read_results_deduped,
        stats.bash_results_recompressed,
    );

    Ok(())
}

/// Accept either an absolute/relative path to a `.jsonl` file or a Claude Code
/// session-id (resolved against `~/.claude/projects/`).
fn resolve_session_path(target: &str) -> Result<PathBuf> {
    let direct = PathBuf::from(target);
    if direct.is_file() {
        return Ok(direct);
    }

    let projects_root = projects_root()?;
    let mut hits: Vec<PathBuf> = Vec::new();
    for project_dir in std::fs::read_dir(&projects_root).with_context(|| {
        format!(
            "Failed to read Claude Code projects directory: {}",
            projects_root.display()
        )
    })? {
        let Ok(project_dir) = project_dir else { continue };
        if !project_dir.path().is_dir() {
            continue;
        }
        let candidate = project_dir.path().join(format!("{}.jsonl", target));
        if candidate.is_file() {
            hits.push(candidate);
        }
    }

    match hits.len() {
        0 => bail!(
            "No session found. Tried: {} (file) and `{}.jsonl` under {}",
            target,
            target,
            projects_root.display()
        ),
        1 => Ok(hits.into_iter().next().unwrap()),
        n => bail!(
            "Session id `{}` matched {} files across multiple projects. Pass the full path instead.",
            target,
            n
        ),
    }
}

fn projects_root() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    Ok(home.join(".claude").join("projects"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn resolve_session_path_accepts_existing_file() {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("session.jsonl");
        let mut f = fs::File::create(&file).unwrap();
        writeln!(f, "{{\"type\":\"user\"}}").unwrap();
        let resolved = resolve_session_path(file.to_str().unwrap()).unwrap();
        assert_eq!(resolved, file);
    }

    #[test]
    fn resolve_session_path_errors_clearly_when_missing() {
        let r = resolve_session_path("__definitely_no_such_session_id_zzzz__");
        assert!(r.is_err());
        let msg = format!("{}", r.unwrap_err());
        assert!(
            msg.contains("No session found"),
            "expected 'No session found' in: {}",
            msg
        );
    }

    #[test]
    fn run_compacts_a_real_jsonl_file() -> Result<()> {
        let dir = TempDir::new()?;
        let session = dir.path().join("session.jsonl");
        let mut f = fs::File::create(&session)?;
        // A trivial Bash result that should pass through (too small to compress).
        writeln!(f, r#"{{"type":"assistant","uuid":"a1","message":{{"content":[{{"type":"tool_use","id":"b1","name":"Bash","input":{{"command":"true"}}}}]}}}}"#)?;
        writeln!(f, r#"{{"type":"user","uuid":"u1","message":{{"content":[{{"type":"tool_result","tool_use_id":"b1","content":"ok\n"}}]}}}}"#)?;
        drop(f);

        // Should not panic, should produce a sidecar.
        run(session.to_str().unwrap(), 0)?;
        let sidecar = dir.path().join("session.jsonl.compressed");
        assert!(sidecar.exists(), "expected sidecar at {}", sidecar.display());
        Ok(())
    }
}
