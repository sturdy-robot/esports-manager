use esm_data::datapack::{ChampionData, DataPack, PlayerData, TeamData};

// ---------------------------------------------------------------------------
// TeamData deserialization
// ---------------------------------------------------------------------------

#[test]
fn team_data_from_json() {
    let json = r#"{"name": "T1", "tag": "T1", "budget": 1000000}"#;
    let team: TeamData = serde_json::from_str(json).unwrap();
    assert_eq!(team.name, "T1");
    assert_eq!(team.tag, "T1");
    assert_eq!(team.budget, 1_000_000);
}

#[test]
fn team_data_missing_field_errors() {
    let json = r#"{"name": "T1"}"#;
    let result: Result<TeamData, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// PlayerData deserialization
// ---------------------------------------------------------------------------

#[test]
fn player_data_from_json() {
    let json = r#"{
        "nickname": "Faker",
        "first_name": "Lee",
        "last_name": "Sang-hyeok",
        "role": "Mid",
        "team": "T1",
        "endurance": 70,
        "reaction_time": 90,
        "decision_making": 95,
        "clutch": 99,
        "discipline": 85,
        "tilt_resistance": 80,
        "mechanics": 97,
        "vision_control": 88,
        "teamfighting": 92
    }"#;
    let player: PlayerData = serde_json::from_str(json).unwrap();
    assert_eq!(player.nickname, "Faker");
    assert_eq!(player.role, "Mid");
    assert_eq!(player.mechanics, 97);
    assert_eq!(player.team, "T1");
}

// ---------------------------------------------------------------------------
// ChampionData deserialization
// ---------------------------------------------------------------------------

#[test]
fn champion_data_from_json() {
    let json = r#"{
        "name": "Orianna",
        "class": "Mage",
        "scaling": "Mid",
        "tags": ["Poke", "Waveclear", "Peel"]
    }"#;
    let champ: ChampionData = serde_json::from_str(json).unwrap();
    assert_eq!(champ.name, "Orianna");
    assert_eq!(champ.class, "Mage");
    assert_eq!(champ.tags.len(), 3);
}

// ---------------------------------------------------------------------------
// DataPack loading from JSON string
// ---------------------------------------------------------------------------

fn sample_datapack_json() -> String {
    r#"{
        "teams": [
            {"name": "T1", "tag": "T1", "budget": 1000000},
            {"name": "Gen.G", "tag": "GEN", "budget": 900000}
        ],
        "players": [
            {
                "nickname": "Faker", "first_name": "Lee", "last_name": "SH",
                "role": "Mid", "team": "T1",
                "endurance": 70, "reaction_time": 90, "decision_making": 95,
                "clutch": 99, "discipline": 85, "tilt_resistance": 80,
                "mechanics": 97, "vision_control": 88, "teamfighting": 92
            },
            {
                "nickname": "Chovy", "first_name": "Jeong", "last_name": "JH",
                "role": "Mid", "team": "Gen.G",
                "endurance": 65, "reaction_time": 88, "decision_making": 90,
                "clutch": 85, "discipline": 80, "tilt_resistance": 75,
                "mechanics": 96, "vision_control": 82, "teamfighting": 88
            }
        ],
        "champions": [
            {"name": "Orianna", "class": "Mage", "scaling": "Mid", "tags": ["Poke", "Waveclear"]},
            {"name": "Malphite", "class": "Tank", "scaling": "Late", "tags": ["Knockup", "Engage"]}
        ]
    }"#
    .to_string()
}

#[test]
fn datapack_from_json_string() {
    let pack = DataPack::from_json(&sample_datapack_json()).unwrap();
    assert_eq!(pack.teams.len(), 2);
    assert_eq!(pack.players.len(), 2);
    assert_eq!(pack.champions.len(), 2);
}

#[test]
fn datapack_invalid_json_returns_error() {
    let result = DataPack::from_json("not valid json{{{");
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// DataPack validation
// ---------------------------------------------------------------------------

#[test]
fn datapack_validate_valid_pack_succeeds() {
    let pack = DataPack::from_json(&sample_datapack_json()).unwrap();
    let result = pack.validate();
    assert!(result.is_ok(), "Validation failed: {:?}", result.err());
}

#[test]
fn datapack_validate_rejects_attribute_over_100() {
    let json = r#"{
        "teams": [{"name": "T1", "tag": "T1", "budget": 100}],
        "players": [{
            "nickname": "Bad", "first_name": "A", "last_name": "B",
            "role": "Mid", "team": "T1",
            "endurance": 150, "reaction_time": 50, "decision_making": 50,
            "clutch": 50, "discipline": 50, "tilt_resistance": 50,
            "mechanics": 50, "vision_control": 50, "teamfighting": 50
        }],
        "champions": []
    }"#;
    let pack = DataPack::from_json(json).unwrap();
    let result = pack.validate();
    assert!(result.is_err());
}

#[test]
fn datapack_validate_rejects_invalid_role() {
    let json = r#"{
        "teams": [{"name": "T1", "tag": "T1", "budget": 100}],
        "players": [{
            "nickname": "Bad", "first_name": "A", "last_name": "B",
            "role": "InvalidRole", "team": "T1",
            "endurance": 50, "reaction_time": 50, "decision_making": 50,
            "clutch": 50, "discipline": 50, "tilt_resistance": 50,
            "mechanics": 50, "vision_control": 50, "teamfighting": 50
        }],
        "champions": []
    }"#;
    let pack = DataPack::from_json(json).unwrap();
    let result = pack.validate();
    assert!(result.is_err());
}

#[test]
fn datapack_validate_rejects_player_referencing_unknown_team() {
    let json = r#"{
        "teams": [{"name": "T1", "tag": "T1", "budget": 100}],
        "players": [{
            "nickname": "Lost", "first_name": "A", "last_name": "B",
            "role": "Mid", "team": "NonExistentTeam",
            "endurance": 50, "reaction_time": 50, "decision_making": 50,
            "clutch": 50, "discipline": 50, "tilt_resistance": 50,
            "mechanics": 50, "vision_control": 50, "teamfighting": 50
        }],
        "champions": []
    }"#;
    let pack = DataPack::from_json(json).unwrap();
    let result = pack.validate();
    assert!(result.is_err());
}

#[test]
fn datapack_validate_rejects_invalid_champion_class() {
    let json = r#"{
        "teams": [],
        "players": [],
        "champions": [{"name": "X", "class": "Wizard", "scaling": "Mid", "tags": []}]
    }"#;
    let pack = DataPack::from_json(json).unwrap();
    let result = pack.validate();
    assert!(result.is_err());
}

#[test]
fn datapack_validate_rejects_duplicate_team_names() {
    let json = r#"{
        "teams": [
            {"name": "T1", "tag": "T1", "budget": 100},
            {"name": "T1", "tag": "T2", "budget": 200}
        ],
        "players": [],
        "champions": []
    }"#;
    let pack = DataPack::from_json(json).unwrap();
    let result = pack.validate();
    assert!(result.is_err());
}
