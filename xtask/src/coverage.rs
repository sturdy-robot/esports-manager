//! Per-crate line coverage gate (ADR 0008), reading a `cargo llvm-cov --json`
//! report.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::files;

#[derive(Debug, Deserialize)]
struct Report {
    data: Vec<Export>,
}

#[derive(Debug, Deserialize)]
struct Export {
    files: Vec<FileEntry>,
}

#[derive(Debug, Deserialize)]
struct FileEntry {
    filename: String,
    summary: Summary,
}

#[derive(Debug, Deserialize)]
struct Summary {
    lines: Lines,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
pub struct Lines {
    pub count: u64,
    pub covered: u64,
}

pub fn run(root: &Path, thresholds: &BTreeMap<String, u32>, report: &Path) -> Result<Vec<String>> {
    let report: Report =
        serde_json::from_str(&files::read(report)?).context("parsing the llvm-cov JSON report")?;
    let entries = report
        .data
        .iter()
        .flat_map(|export| &export.files)
        .map(|file| (file.filename.as_str(), file.summary.lines));
    let totals = per_crate(root, entries);
    let (lines, violations) = check(thresholds, &totals);
    for line in lines {
        println!("{line}");
    }
    Ok(violations)
}

/// Sums line counts per crate for files under `<root>/crates/<crate>/`.
pub fn per_crate<'a>(
    root: &Path,
    files: impl IntoIterator<Item = (&'a str, Lines)>,
) -> BTreeMap<String, Lines> {
    let crates_dir = root.join("crates");
    let mut totals: BTreeMap<String, Lines> = BTreeMap::new();
    for (filename, lines) in files {
        let Ok(relative) = Path::new(filename).strip_prefix(&crates_dir) else {
            continue;
        };
        let Some(crate_name) = relative.components().next() else {
            continue;
        };
        let total = totals
            .entry(crate_name.as_os_str().to_string_lossy().into_owned())
            .or_default();
        total.count += lines.count;
        total.covered += lines.covered;
    }
    totals
}

/// Returns a report line per crate and the violations.
pub fn check(
    thresholds: &BTreeMap<String, u32>,
    totals: &BTreeMap<String, Lines>,
) -> (Vec<String>, Vec<String>) {
    let mut report = Vec::new();
    let mut violations = Vec::new();

    for (name, &minimum) in thresholds {
        let lines = totals.get(name).copied().unwrap_or_default();
        if lines.count == 0 {
            report.push(format!("{name}: no code yet (minimum {minimum}%)"));
            continue;
        }
        let percent = lines.covered * 100 / lines.count;
        report.push(format!(
            "{name}: {percent}% of {} lines (minimum {minimum}%)",
            lines.count
        ));
        if lines.covered * 100 < u64::from(minimum) * lines.count {
            violations.push(format!(
                "crate `{name}` has {percent}% line coverage, below its {minimum}% minimum"
            ));
        }
    }
    for name in totals.keys() {
        if !thresholds.contains_key(name) {
            violations.push(format!(
                "crate `{name}` has no coverage minimum in quality.toml [coverage]"
            ));
        }
    }

    (report, violations)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(count: u64, covered: u64) -> Lines {
        Lines { count, covered }
    }

    #[test]
    fn sums_files_per_crate_and_ignores_files_elsewhere() {
        let root = Path::new("/repo");
        let totals = per_crate(
            root,
            [
                ("/repo/crates/domain/src/a.rs", lines(10, 9)),
                ("/repo/crates/domain/src/b.rs", lines(10, 10)),
                ("/repo/crates/app/src/lib.rs", lines(4, 1)),
                ("/repo/xtask/src/main.rs", lines(50, 0)),
                ("/home/user/.cargo/registry/x.rs", lines(5, 0)),
            ],
        );
        assert_eq!(
            totals,
            BTreeMap::from([
                ("app".to_owned(), lines(4, 1)),
                ("domain".to_owned(), lines(20, 19)),
            ])
        );
    }

    #[test]
    fn coverage_at_the_minimum_passes() {
        let thresholds = BTreeMap::from([("domain".to_owned(), 90)]);
        let totals = BTreeMap::from([("domain".to_owned(), lines(10, 9))]);
        let (report, violations) = check(&thresholds, &totals);
        assert_eq!(report, ["domain: 90% of 10 lines (minimum 90%)"]);
        assert_eq!(violations, Vec::<String>::new());
    }

    #[test]
    fn coverage_below_the_minimum_fails_even_by_a_fraction() {
        let thresholds = BTreeMap::from([("domain".to_owned(), 90)]);
        let totals = BTreeMap::from([("domain".to_owned(), lines(1000, 899))]);
        let (_, violations) = check(&thresholds, &totals);
        assert_eq!(
            violations,
            ["crate `domain` has 89% line coverage, below its 90% minimum"]
        );
    }

    #[test]
    fn crates_without_code_are_skipped() {
        let thresholds = BTreeMap::from([("db".to_owned(), 80)]);
        let (report, violations) = check(&thresholds, &BTreeMap::new());
        assert_eq!(report, ["db: no code yet (minimum 80%)"]);
        assert_eq!(violations, Vec::<String>::new());
    }

    #[test]
    fn crates_without_a_minimum_fail() {
        let totals = BTreeMap::from([("rogue".to_owned(), lines(1, 1))]);
        let (_, violations) = check(&BTreeMap::new(), &totals);
        assert_eq!(
            violations,
            ["crate `rogue` has no coverage minimum in quality.toml [coverage]"]
        );
    }

    #[test]
    fn reads_an_llvm_cov_report() {
        let dir = std::env::temp_dir().join(format!("xtask-coverage-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let report = dir.join("report.json");
        std::fs::write(
            &report,
            r#"{"data":[{"files":[
                {"filename":"/repo/crates/domain/src/lib.rs","summary":{"lines":{"count":10,"covered":5,"percent":50.0}}}
            ]}]}"#,
        )
        .unwrap();
        let thresholds = BTreeMap::from([("domain".to_owned(), 90)]);

        let violations = run(Path::new("/repo"), &thresholds, &report).unwrap();

        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(
            violations,
            ["crate `domain` has 50% line coverage, below its 90% minimum"]
        );
    }
}
