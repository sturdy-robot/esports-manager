//! The browser E2E tests answer with views the real backend produced:
//! `src/test/fixtures/*.json`, made by this CLI from the core package and
//! seed 42. This test fails when a view changes and the fixtures don't. Set
//! `UPDATE_FIXTURES=1` to rewrite them (`just e2e-fixtures`).
#![allow(clippy::unwrap_used, reason = "integration test helpers")]

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn esm(data: &Path, args: &[&str]) -> serde_json::Value {
    let output = Command::new(env!("CARGO_BIN_EXE_esm"))
        .arg("--data-dir")
        .arg(data)
        .arg("--content")
        .arg(root().join("content/core"))
        .arg("--json")
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn e2e_fixtures_are_up_to_date() {
    let data = std::env::temp_dir().join(format!("esm-e2e-fixtures-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&data);
    let options = esm(&data, &["teams"]);
    esm(
        &data,
        &[
            "new-game",
            "--manager",
            "Ana",
            "--team",
            "team:azure_tigers",
            "--save",
            "Ana's Tigers",
            "--seed",
            "42",
        ],
    );
    let roster = esm(&data, &["roster", "--save", "Ana's Tigers"]);
    let player = esm(
        &data,
        &["player", "--save", "Ana's Tigers", "player:kairos"],
    );
    let mut saves = esm(&data, &["saves"]);
    // Real time differs on every run; the fixture keeps a fixed moment.
    saves[0]["lastPlayedAt"] = serde_json::json!(1_792_000_000);

    let fixtures = root().join("src/test/fixtures");
    let mut stale = Vec::new();
    for (name, value) in [
        ("new-game-options.json", options),
        ("roster.json", roster),
        ("player.json", player),
        ("saves.json", saves),
    ] {
        let path = fixtures.join(name);
        let text = format!("{}\n", serde_json::to_string_pretty(&value).unwrap());
        if std::env::var_os("UPDATE_FIXTURES").is_some() {
            std::fs::write(&path, &text).unwrap();
        }
        if std::fs::read_to_string(&path).unwrap_or_default() != text {
            stale.push(name);
        }
    }
    assert!(
        stale.is_empty(),
        "src/test/fixtures is out of date ({stale:?}); run `just e2e-fixtures`"
    );
}
