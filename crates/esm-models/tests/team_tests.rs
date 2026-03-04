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

fn make_full_roster() -> Vec<Player> {
    vec![
        make_player("TopLaner", Role::Top),
        make_player("Jungler", Role::Jungle),
        make_player("MidLaner", Role::Mid),
        make_player("BotLaner", Role::Bot),
        make_player("Support", Role::Support),
    ]
}

fn make_test_team() -> Team {
    Team::new(
        "T1".to_string(),
        "T1".to_string(),
        make_full_roster(),
    )
}

// ---------------------------------------------------------------------------
// Team creation
// ---------------------------------------------------------------------------

#[test]
fn team_creation_stores_identity() {
    let team = make_test_team();
    assert_eq!(team.name(), "T1");
    assert_eq!(team.tag(), "T1");
}

#[test]
fn team_has_roster_of_five() {
    let team = make_test_team();
    assert_eq!(team.roster().len(), 5);
}

#[test]
fn team_starts_with_default_synergy() {
    let team = make_test_team();
    assert_eq!(team.synergy().value(), 0);
}

#[test]
fn team_starts_with_default_reputation() {
    let team = make_test_team();
    assert_eq!(team.reputation().value(), 50);
}

// ---------------------------------------------------------------------------
// Roster queries
// ---------------------------------------------------------------------------

#[test]
fn team_find_player_by_role() {
    let team = make_test_team();
    let mid = team.player_by_role(Role::Mid);
    assert!(mid.is_some());
    assert_eq!(mid.unwrap().nickname(), "MidLaner");
}

#[test]
fn team_find_player_by_role_returns_none_if_missing() {
    let team = Team::new(
        "Duo".to_string(),
        "DUO".to_string(),
        vec![
            make_player("TopOnly", Role::Top),
            make_player("JgOnly", Role::Jungle),
        ],
    );
    assert!(team.player_by_role(Role::Mid).is_none());
}

// ---------------------------------------------------------------------------
// Roster mutation
// ---------------------------------------------------------------------------

#[test]
fn team_player_by_role_mut_allows_state_change() {
    let mut team = make_test_team();
    let mid = team.player_by_role_mut(Role::Mid).unwrap();
    mid.state_mut().stamina.decrease(30);

    let mid = team.player_by_role(Role::Mid).unwrap();
    assert_eq!(mid.state().stamina.value(), 70);
}

// ---------------------------------------------------------------------------
// Synergy mutation
// ---------------------------------------------------------------------------

#[test]
fn team_synergy_can_increase() {
    let mut team = make_test_team();
    team.synergy_mut().increase(15);
    assert_eq!(team.synergy().value(), 15);
}

#[test]
fn team_synergy_clamps_at_100() {
    let mut team = make_test_team();
    team.synergy_mut().increase(110);
    assert_eq!(team.synergy().value(), 100);
}
