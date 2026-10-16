//! The `esm` binary end to end: the backend's end-to-end tests (ADR 0001).
#![allow(clippy::unwrap_used, reason = "integration test helpers")]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn core() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/core")
}

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("esm-cli-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn esm(data: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_esm"))
        .arg("--data-dir")
        .arg(data)
        .arg("--content")
        .arg(core())
        .args(args)
        .output()
        .unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

fn json(output: &Output) -> serde_json::Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn new_game(data: &Path, save: &str, seed: Option<&str>) -> Output {
    let mut args = vec![
        "new-game",
        "--manager",
        "Ana",
        "--team",
        "team:azure_tigers",
        "--save",
        save,
    ];
    if let Some(seed) = seed {
        args.extend(["--seed", seed]);
    }
    esm(data, &args)
}

#[test]
fn a_given_seed_reproduces_the_world() {
    // spec: NEWG-004
    // Given
    let data = folder("seed");
    assert!(new_game(&data, "First", Some("42")).status.success());
    // When
    assert!(new_game(&data, "Second", Some("42")).status.success());
    // Then
    let first = stdout(&esm(&data, &["--json", "roster", "--save", "First"]));
    let second = stdout(&esm(&data, &["--json", "roster", "--save", "Second"]));
    assert!(first.contains("\"starters\""));
    assert_eq!(first, second);
}

#[test]
fn without_a_seed_each_game_gets_its_own_world() {
    // spec: NEWG-005
    // Given
    let data = folder("no-seed");
    // When
    assert!(new_game(&data, "One", None).status.success());
    assert!(new_game(&data, "Two", None).status.success());
    // Then
    let one = stdout(&esm(&data, &["--json", "roster", "--save", "One"]));
    let two = stdout(&esm(&data, &["--json", "roster", "--save", "Two"]));
    assert_ne!(one, two);
}

#[test]
fn esm_teams_lists_the_ids_new_game_accepts() {
    // spec: NEWG-012
    // Given
    let data = folder("teams");
    // When
    let options = json(&esm(&data, &["--json", "teams"]));
    // Then
    let ids: Vec<String> = options["regions"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|region| {
            region["teams"]
                .as_array()
                .unwrap()
                .iter()
                .map(|t| t["id"].as_str().unwrap().to_owned())
        })
        .collect();
    assert_eq!(ids.len(), 18);
    for (n, id) in ids.iter().enumerate() {
        let output = esm(
            &data,
            &[
                "new-game",
                "--manager",
                "Ana",
                "--team",
                id,
                "--save",
                &format!("Save {n}"),
                "--seed",
                "1",
            ],
        );
        assert!(
            output.status.success(),
            "{id}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn the_core_world_offers_both_regions() {
    let data = folder("regions");
    let options = json(&esm(&data, &["--json", "teams"]));
    let regions: Vec<(String, String, usize)> = options["regions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["nameMessage"].as_str().unwrap().to_owned(),
                r["leagueMessage"].as_str().unwrap().to_owned(),
                r["teams"].as_array().unwrap().len(),
            )
        })
        .collect();
    assert_eq!(
        regions,
        [
            (
                "region-pindorama".to_owned(),
                "competition-lpd".to_owned(),
                8
            ),
            (
                "region-seorabeol".to_owned(),
                "competition-spl".to_owned(),
                10
            )
        ]
    );
}

#[test]
fn text_output_is_localized() {
    let data = folder("text");
    let created = new_game(&data, "Ana's Tigers", Some("42"));
    assert_eq!(
        stdout(&created),
        "Created ana-s-tigers.esmsave with Azure Tigers.\n"
    );
    let roster = stdout(&esm(
        &data,
        &["--locale", "pt-BR", "roster", "--save", "Ana's Tigers"],
    ));
    assert!(
        roster.starts_with("Azure Tigers · Liga Profissional Seorabeol\n"),
        "{roster}"
    );
    assert!(
        roster.contains("Titulares") && roster.contains("Caçador") && roster.contains("Han Do-yun"),
        "{roster}"
    );
    let saves = stdout(&esm(&data, &["saves"]));
    assert_eq!(
        saves,
        "Ana's Tigers · Ana · Azure Tigers · 2027-01-01 (ana-s-tigers.esmsave)\n"
    );
    let player = stdout(&esm(
        &data,
        &["player", "--save", "ana-s-tigers.esmsave", "player:kairos"],
    ));
    assert!(
        player.starts_with("Kairos · Han Do-yun · Azure Tigers\n"),
        "{player}"
    );
    assert!(
        player.contains("Mechanics") && player.contains("Potential") && player.contains('?'),
        "{player}"
    );
}

#[test]
fn failures_print_their_messages_and_exit_1() {
    let data = folder("failures");
    let output = esm(
        &data,
        &[
            "new-game",
            "--manager",
            " ",
            "--team",
            "team:nowhere",
            "--save",
            "x",
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "Enter a manager name.\nThere is no team \"team:nowhere\".\n"
    );
    let missing = esm(&data, &["roster", "--save", "Nothing"]);
    assert_eq!(missing.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&missing.stderr),
        "There is no save \"Nothing\".\n"
    );
    let empty = stdout(&esm(&data, &["saves"]));
    assert_eq!(empty, "No saves yet.\n");
}

#[test]
fn bad_arguments_exit_2() {
    let data = folder("arguments");
    assert_eq!(esm(&data, &["new-game"]).status.code(), Some(2));
    assert_eq!(esm(&data, &["dance"]).status.code(), Some(2));
}

#[test]
fn a_broken_package_reports_its_problems() {
    let data = folder("broken");
    let output = Command::new(env!("CARGO_BIN_EXE_esm"))
        .args(["--data-dir"])
        .arg(&data)
        .args(["--content", "/definitely/not/a/package", "teams"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).starts_with("The package can't be read"));
}

#[test]
fn json_views_are_the_use_case_dtos() {
    let data = folder("json");
    let created = json(&esm(
        &data,
        &[
            "--json",
            "new-game",
            "--manager",
            "Ana",
            "--team",
            "team:azure_tigers",
            "--save",
            "Json",
            "--seed",
            "7",
        ],
    ));
    assert_eq!(created["file"], "json.esmsave");
    let saves = json(&esm(&data, &["--json", "saves"]));
    assert_eq!(saves[0]["status"], "ready");
    let roster = json(&esm(
        &data,
        &[
            "--json",
            "roster",
            "--save",
            "Json",
            "--team",
            "team:onca_gaming",
        ],
    ));
    assert_eq!(roster["team"]["crest"]["tag"], "ONC");
    let profile = json(&esm(
        &data,
        &["--json", "player", "--save", "Json", "player:faisca"],
    ));
    assert_eq!(profile["player"]["age"], 17);
    assert_eq!(profile["hidden"][1]["value"], serde_json::Value::Null);
}
