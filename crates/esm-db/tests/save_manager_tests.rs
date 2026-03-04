use std::fs;

use esm_core::game_state::GameState;
use esm_db::save_manager::SaveManager;
use esm_models::esport_type::EsportType;
use esm_models::manager::{Manager, ManagerArchetype};
use esm_models::player::{
    BoundedAttribute, MentalAttributes, PhysicalAttributes, Player, PlayerAttributes, Role,
    TechnicalAttributes,
};
use esm_models::team::Team;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_player(nickname: &str, role: Role) -> Player {
    Player::new(
        nickname.to_string(),
        "First".to_string(),
        "Last".to_string(),
        role,
        PlayerAttributes {
            physical: PhysicalAttributes {
                endurance: BoundedAttribute::new(50),
                reaction_time: BoundedAttribute::new(50),
            },
            mental: MentalAttributes {
                decision_making: BoundedAttribute::new(50),
                clutch: BoundedAttribute::new(50),
                discipline: BoundedAttribute::new(50),
                tilt_resistance: BoundedAttribute::new(50),
            },
            technical: TechnicalAttributes {
                mechanics: BoundedAttribute::new(50),
                vision_control: BoundedAttribute::new(50),
                teamfighting: BoundedAttribute::new(50),
            },
        },
    )
}

fn make_team(name: &str) -> Team {
    Team::new(
        name.to_string(),
        name[..2].to_uppercase(),
        vec![
            make_player(&format!("{name}_top"), Role::Top),
            make_player(&format!("{name}_jg"), Role::Jungle),
            make_player(&format!("{name}_mid"), Role::Mid),
            make_player(&format!("{name}_bot"), Role::Bot),
            make_player(&format!("{name}_sup"), Role::Support),
        ],
    )
}

fn make_game_state() -> GameState {
    let teams = vec![make_team("Alpha"), make_team("Bravo"), make_team("Charlie")];
    GameState::new(
        2025,
        42,
        EsportType::Moba,
        Manager::new(
            "kkOma".to_string(),
            "Kim".to_string(),
            "Jeong-gyun".to_string(),
            "KR".to_string(),
            ManagerArchetype::TacticalGenius,
        ),
        0,
        teams,
    )
}

/// Create a temporary directory for saves that is cleaned up on drop.
fn temp_saves_dir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

// ---------------------------------------------------------------------------
// SaveManager: create and list
// ---------------------------------------------------------------------------

#[test]
fn create_save_produces_db_file_and_index() {
    let dir = temp_saves_dir();
    let gs = make_game_state();

    SaveManager::create_save(dir.path(), "test_save", &gs).unwrap();

    // DB file exists
    assert!(dir.path().join("test_save.db").exists());
    // Index file exists
    assert!(dir.path().join("saves.json").exists());
}

#[test]
fn list_saves_returns_created_save() {
    let dir = temp_saves_dir();
    let gs = make_game_state();

    SaveManager::create_save(dir.path(), "my_save", &gs).unwrap();

    let saves = SaveManager::list_saves(dir.path()).unwrap();
    assert_eq!(saves.len(), 1);
    assert_eq!(saves[0].name, "my_save");
    assert!(!saves[0].checksum.is_empty());
}

#[test]
fn list_saves_returns_empty_when_no_saves() {
    let dir = temp_saves_dir();
    let saves = SaveManager::list_saves(dir.path()).unwrap();
    assert!(saves.is_empty());
}

#[test]
fn create_multiple_saves_lists_all() {
    let dir = temp_saves_dir();
    let gs = make_game_state();

    SaveManager::create_save(dir.path(), "save_a", &gs).unwrap();
    SaveManager::create_save(dir.path(), "save_b", &gs).unwrap();

    let saves = SaveManager::list_saves(dir.path()).unwrap();
    assert_eq!(saves.len(), 2);
    let names: Vec<&str> = saves.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"save_a"));
    assert!(names.contains(&"save_b"));
}

// ---------------------------------------------------------------------------
// SaveManager: load
// ---------------------------------------------------------------------------

#[test]
fn load_save_roundtrip() {
    let dir = temp_saves_dir();
    let gs = make_game_state();

    SaveManager::create_save(dir.path(), "roundtrip", &gs).unwrap();
    let loaded = SaveManager::load_save(dir.path(), "roundtrip").unwrap();

    assert_eq!(loaded.calendar().year(), 2025);
    assert_eq!(loaded.esport_type(), EsportType::Moba);
    assert_eq!(loaded.manager().nickname(), "kkOma");
    assert_eq!(loaded.teams().len(), 3);
}

// ---------------------------------------------------------------------------
// SaveManager: delete
// ---------------------------------------------------------------------------

#[test]
fn delete_save_removes_file_and_index_entry() {
    let dir = temp_saves_dir();
    let gs = make_game_state();

    SaveManager::create_save(dir.path(), "to_delete", &gs).unwrap();
    assert!(dir.path().join("to_delete.db").exists());

    SaveManager::delete_save(dir.path(), "to_delete").unwrap();

    assert!(!dir.path().join("to_delete.db").exists());
    let saves = SaveManager::list_saves(dir.path()).unwrap();
    assert!(saves.is_empty());
}

// ---------------------------------------------------------------------------
// SaveManager: checksum validation
// ---------------------------------------------------------------------------

#[test]
fn load_save_detects_corrupted_file() {
    let dir = temp_saves_dir();
    let gs = make_game_state();

    SaveManager::create_save(dir.path(), "corrupt", &gs).unwrap();

    // Corrupt the file by overwriting with garbage
    let db_path = dir.path().join("corrupt.db");
    fs::write(&db_path, b"this is not a valid sqlite db").unwrap();

    let result = SaveManager::load_save(dir.path(), "corrupt");
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// SaveManager: overwrite existing save
// ---------------------------------------------------------------------------

#[test]
fn create_save_overwrites_existing() {
    let dir = temp_saves_dir();
    let gs = make_game_state();

    SaveManager::create_save(dir.path(), "overwrite", &gs).unwrap();
    let saves1 = SaveManager::list_saves(dir.path()).unwrap();
    let checksum1 = saves1[0].checksum.clone();

    // Advance and re-save
    let mut gs2 = gs;
    gs2.advance_day();
    SaveManager::create_save(dir.path(), "overwrite", &gs2).unwrap();

    let saves2 = SaveManager::list_saves(dir.path()).unwrap();
    assert_eq!(saves2.len(), 1);
    // Checksum should differ since state changed
    assert_ne!(saves2[0].checksum, checksum1);

    let loaded = SaveManager::load_save(dir.path(), "overwrite").unwrap();
    assert_eq!(loaded.calendar().day(), 2);
}
