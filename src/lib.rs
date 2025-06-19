//! Git-ignore generator: Rust core exposed to Node via napi-rs.
use std::collections::HashSet;
use napi_derive::napi;

#[napi(object)]
pub struct GenConfig {
    /// Directories to ignore, relative to repo root.
    pub ignore_dirs: Vec<String>,
    /// Single directory to focus on (track only this dir). Optional.
    pub focus_dir: Option<String>,
}

#[napi]
pub fn generate_gitignore(base_rules: String, cfg: GenConfig) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    // ---------- (A) only-focus mode ----------
    if let Some(focus) = cfg.focus_dir.filter(|s| !s.trim().is_empty()) {
        let focus_clean = normalize_dir(&focus);
        // 1. ignore everything
        push_unique(&mut lines, &mut seen, "/*");
        // 2. keep the .gitignore itself
        push_unique(&mut lines, &mut seen, "!.gitignore");

        // 3. un-ignore every parent path
        let mut cur = String::new();
        for part in focus_clean.split('/') {
            if !cur.is_empty() {
                cur.push('/');
            }
            cur.push_str(part);
            push_unique(&mut lines, &mut seen, &format!("!/{}/", cur));
        }
        // 4. finally, un-ignore all descendants
        push_unique(
            &mut lines,
            &mut seen,
            &format!("!/{}/**", focus_clean.trim_end_matches('/')),
        );
        // empty line separator
        lines.push(String::new());
    }

    // ---------- (B) extra ignore-dirs ----------
    for dir in cfg.ignore_dirs.iter().filter(|d| !d.trim().is_empty()) {
        let d = format!("/{}/", normalize_dir(dir));
        push_unique(&mut lines, &mut seen, &d);
    }
    if !cfg.ignore_dirs.is_empty() {
        lines.push(String::new());
    }

    // ---------- (C) append base template ----------
    lines.push(base_rules.trim_matches('\n').to_owned());

    lines.join("\n") + "\n"
}

/// Ensure path uses forward slashes and no leading/trailing spaces.
fn normalize_dir(path: &str) -> String {
    path.trim()
        .trim_matches('/')
        .replace('\\', "/")
        .to_string()
}

/// Push line if not seen; preserve insertion order.
fn push_unique(lines: &mut Vec<String>, seen: &mut HashSet<String>, line: &str) {
    if seen.insert(line.to_string()) {
        lines.push(line.to_string());
    }
}

