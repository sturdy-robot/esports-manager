use esm_data::datapack::DataPack;
use std::fs;
use tempfile::tempdir;

fn write_file(dir: &std::path::Path, filename: &str, content: &str) {
    let path = dir.join(filename);
    fs::write(path, content).unwrap();
}

#[test]
fn datapack_load_from_dir_success() {
    let dir = tempdir().unwrap();

    let teams_json = r#"[
        {"name": "T1", "tag": "T1", "budget": 1000000},
        {"name": "Gen.G", "tag": "GEN", "budget": 900000}
    ]"#;
    write_file(dir.path(), "teams.json", teams_json);

    let players_json = r#"[
        {
            "nickname": "Faker", "first_name": "Lee", "last_name": "SH",
            "role": "Mid", "team": "T1",
            "endurance": 70, "reaction_time": 90, "decision_making": 95,
            "clutch": 99, "discipline": 85, "tilt_resistance": 80,
            "mechanics": 97, "vision_control": 88, "teamfighting": 92
        }
    ]"#;
    write_file(dir.path(), "players.json", players_json);

    let champions_json = r#"[
        {"name": "Orianna", "class": "Mage", "scaling": "Mid", "tags": ["Poke", "Waveclear"]}
    ]"#;
    write_file(dir.path(), "champions.json", champions_json);

    let player_names_json = r#"{
        "KR": {
            "first_names": ["Jeong", "Lee"],
            "last_names": ["Sang-hyeok", "Ji-hoon"]
        }
    }"#;
    write_file(dir.path(), "player_names.json", player_names_json);

    let player_nicknames_json = r#"[
        "Chovy", "ShowMaker"
    ]"#;
    write_file(dir.path(), "player_nicknames.json", player_nicknames_json);

    let team_names_json = r#"[
        "DRX", "Hanwha Life Esports"
    ]"#;
    write_file(dir.path(), "team_names.json", team_names_json);

    let pack = DataPack::load_from_dir(dir.path()).unwrap();
    
    assert_eq!(pack.teams.len(), 2);
    assert_eq!(pack.players.len(), 1);
    assert_eq!(pack.champions.len(), 1);
    assert_eq!(pack.player_names.len(), 1);
    assert_eq!(pack.player_nicknames.len(), 2);
    assert_eq!(pack.team_names.len(), 2);
}
