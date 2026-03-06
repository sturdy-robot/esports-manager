use esm_data::datapack::{DataPack, PlayerData};
use esm_data::generator::generate_player;
use std::collections::HashMap;

#[test]
fn test_generate_player_assigns_names_and_attributes() {
    let json = r#"{
        "teams": [{"name": "T1", "tag": "T1", "budget": 1000000}],
        "players": [],
        "champions": [],
        "player_names": {
            "KR": {
                "first_names": ["Jeong"],
                "last_names": ["Ji-hoon"]
            }
        },
        "player_nicknames": ["Chovy"],
        "team_names": ["Gen.G"]
    }"#;
    let pack = DataPack::from_json(json).unwrap();
    
    // Attempt to generate a player with role "Mid" for team "T1"
    let player = generate_player(&pack, "Mid", "T1");
    
    // Verify random selection from pack
    assert_eq!(player.first_name, "Jeong");
    assert_eq!(player.last_name, "Ji-hoon");
    assert_eq!(player.nickname, "Chovy");
    
    assert_eq!(player.role, "Mid");
    assert_eq!(player.team, "T1");
    
    // Verify attributes are generated and within bounds
    assert!(player.endurance > 0 && player.endurance <= 100);
    assert!(player.mechanics > 0 && player.mechanics <= 100);
}
