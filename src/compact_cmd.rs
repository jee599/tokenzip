//! `contextzip compact <session-id-or-path>` CLI.
//!
//! Wraps `jsonl_rewriter::compact_session_file`. Resolves a session-id like
//! `bfb59668-...` to its `.jsonl` under `~/.claude/projects/<project>/`,
//! produces a sidecar `.compressed` file, and prints a one-line summary.
//!
//! The original `.jsonl` is never modified. Rollback is `rm <sidecar>`. Apply
//! / expand commands will be added in a follow-up.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

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

/// Atomic swap: backup `<session>.jsonl` → `<session>.jsonl.bak`, then
/// promote `<session>.jsonl.compressed` to `<session>.jsonl`.
///
/// Refuses to run if the sidecar is missing, if a `.bak` already exists
/// (would lose the previous original), or if the original `.jsonl` is gone.
pub fn run_apply(target: &str, verbose: u8) -> Result<()> {
    let session_path = resolve_session_path(target)?;
    let sidecar = sidecar_path(&session_path);
    let backup = backup_path(&session_path);

    if !sidecar.is_file() {
        bail!(
            "No sidecar at {}. Run `contextzip compact {}` first.",
            sidecar.display(),
            target
        );
    }
    if backup.exists() {
        bail!(
            "Backup already exists at {}. Run `contextzip expand {}` first to roll back, or remove the backup manually.",
            backup.display(),
            target
        );
    }

    if verbose > 0 {
        eprintln!(
            "contextzip apply: {} → {} (backup at {})",
            sidecar.display(),
            session_path.display(),
            backup.display()
        );
    }

    std::fs::rename(&session_path, &backup).with_context(|| {
        format!(
            "Failed to back up original session to {}",
            backup.display()
        )
    })?;
    if let Err(e) = std::fs::rename(&sidecar, &session_path) {
        // Roll back the backup so we don't leave the user with no live session.
        let _ = std::fs::rename(&backup, &session_path);
        return Err(e).context("Failed to promote sidecar to live session; backup restored");
    }

    println!(
        "apply: {} now active; original preserved at {}",
        session_path.display(),
        backup.display()
    );
    Ok(())
}

/// Restore the original session by renaming `<session>.jsonl.bak` back to
/// `<session>.jsonl`. The current (compressed) `<session>.jsonl` is moved aside
/// to `<session>.jsonl.compressed` so a future `apply` can re-promote it.
///
/// If no `.bak` exists, errors with a clear instruction — there is nothing
/// to expand from. (A future v2 may reconstruct from `contextzip_compressed`
/// annotations alone.)
pub fn run_expand(target: &str, verbose: u8) -> Result<()> {
    let session_path = resolve_session_path(target)?;
    let backup = backup_path(&session_path);
    let sidecar = sidecar_path(&session_path);

    if !backup.is_file() {
        bail!(
            "No backup at {}. There is nothing to expand from. (`contextzip apply` writes the backup; without one we cannot restore.)",
            backup.display()
        );
    }

    if verbose > 0 {
        eprintln!(
            "contextzip expand: {} → {} (current → {})",
            backup.display(),
            session_path.display(),
            sidecar.display()
        );
    }

    // Move the current (compressed) file aside so we keep a working sidecar.
    if session_path.is_file() {
        std::fs::rename(&session_path, &sidecar).with_context(|| {
            format!(
                "Failed to move current session aside to {}",
                sidecar.display()
            )
        })?;
    }
    std::fs::rename(&backup, &session_path).with_context(|| {
        format!(
            "Failed to restore backup from {}",
            backup.display()
        )
    })?;

    println!(
        "expand: original restored at {} (compressed copy preserved at {})",
        session_path.display(),
        sidecar.display()
    );
    Ok(())
}

fn sidecar_path(session: &Path) -> PathBuf {
    let mut p = session.to_path_buf();
    let name = session
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("session.jsonl");
    p.set_file_name(format!("{}.compressed", name));
    p
}

fn backup_path(session: &Path) -> PathBuf {
    let mut p = session.to_path_buf();
    let name = session
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("session.jsonl");
    p.set_file_name(format!("{}.bak", name));
    p
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

    fn make_repeatable_session(dir: &Path) -> Result<PathBuf> {
        let session = dir.join("session.jsonl");
        let mut f = fs::File::create(&session)?;
        // Two reads of the same file → ReadDedup will trigger.
        writeln!(f, r#"{{"type":"assistant","uuid":"a1","message":{{"content":[{{"type":"tool_use","id":"r1","name":"Read","input":{{"file_path":"/tmp/x.rs"}}}}]}}}}"#)?;
        writeln!(f, r#"{{"type":"user","uuid":"u1","message":{{"content":[{{"type":"tool_result","tool_use_id":"r1","content":"fn main() {{}}"}}]}}}}"#)?;
        writeln!(f, r#"{{"type":"assistant","uuid":"a2","message":{{"content":[{{"type":"tool_use","id":"r2","name":"Read","input":{{"file_path":"/tmp/x.rs"}}}}]}}}}"#)?;
        writeln!(f, r#"{{"type":"user","uuid":"u2","message":{{"content":[{{"type":"tool_result","tool_use_id":"r2","content":"fn main() {{}}"}}]}}}}"#)?;
        Ok(session)
    }

    #[test]
    fn apply_swaps_sidecar_into_place_and_creates_backup() -> Result<()> {
        let dir = TempDir::new()?;
        let session = make_repeatable_session(dir.path())?;
        run(session.to_str().unwrap(), 0)?;
        let original_bytes = fs::read(&session)?;

        run_apply(session.to_str().unwrap(), 0)?;

        let backup = dir.path().join("session.jsonl.bak");
        let sidecar = dir.path().join("session.jsonl.compressed");
        assert!(backup.is_file(), "backup should exist after apply");
        assert!(!sidecar.exists(), "sidecar should be consumed by apply");
        assert_eq!(
            fs::read(&backup)?,
            original_bytes,
            "backup must equal pre-apply content"
        );
        Ok(())
    }

    #[test]
    fn apply_refuses_when_no_sidecar() -> Result<()> {
        let dir = TempDir::new()?;
        let session = dir.path().join("session.jsonl");
        fs::write(&session, "{}\n")?;
        let r = run_apply(session.to_str().unwrap(), 0);
        assert!(r.is_err());
        let msg = format!("{}", r.unwrap_err());
        assert!(msg.contains("No sidecar"), "wrong error: {}", msg);
        Ok(())
    }

    #[test]
    fn apply_refuses_when_backup_already_present() -> Result<()> {
        let dir = TempDir::new()?;
        let session = make_repeatable_session(dir.path())?;
        run(session.to_str().unwrap(), 0)?;
        // Pre-create a backup to simulate a prior apply.
        fs::write(dir.path().join("session.jsonl.bak"), "old backup")?;
        let r = run_apply(session.to_str().unwrap(), 0);
        assert!(r.is_err());
        assert!(format!("{}", r.unwrap_err()).contains("Backup already exists"));
        Ok(())
    }

    #[test]
    fn expand_restores_original_byte_for_byte() -> Result<()> {
        let dir = TempDir::new()?;
        let session = make_repeatable_session(dir.path())?;
        let original_bytes = fs::read(&session)?;

        run(session.to_str().unwrap(), 0)?;
        run_apply(session.to_str().unwrap(), 0)?;
        run_expand(session.to_str().unwrap(), 0)?;

        let restored = fs::read(&session)?;
        assert_eq!(
            restored, original_bytes,
            "expand must round-trip apply losslessly"
        );
        // The compressed copy is preserved on the side so apply can be redone.
        let sidecar = dir.path().join("session.jsonl.compressed");
        assert!(sidecar.is_file());
        Ok(())
    }

    #[test]
    fn expand_errors_clearly_when_no_backup() -> Result<()> {
        let dir = TempDir::new()?;
        let session = dir.path().join("session.jsonl");
        fs::write(&session, "{}\n")?;
        let r = run_expand(session.to_str().unwrap(), 0);
        assert!(r.is_err());
        assert!(format!("{}", r.unwrap_err()).contains("No backup"));
        Ok(())
    }
}
