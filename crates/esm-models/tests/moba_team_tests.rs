use esm_models::moba::player::{
    MobaPlayer, MobaPlayerAttributes, MobaRole, RoleAssignment,
};
use esm_models::moba::team::MobaTeam;
use esm_models::player::BoundedAttribute;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_player(nickname: &str, primary: MobaRole, skill: u8) -> MobaPlayer {
    MobaPlayer::new(
        nickname.to_string(),
        "First".to_string(),
        "Last".to_string(),
        RoleAssignment::new(primary, vec![]),
        MobaPlayerAttributes {
            endurance: BoundedAttribute::new(skill),
            reaction_time: BoundedAttribute::new(skill),
            decision_making: BoundedAttribute::new(skill),
            clutch: BoundedAttribute::new(skill),
            discipline: BoundedAttribute::new(skill),
            tilt_resistance: BoundedAttribute::new(skill),
            mechanics: BoundedAttribute::new(skill),
            vision_control: BoundedAttribute::new(skill),
            teamfighting: BoundedAttribute::new(skill),
        },
    )
}

fn make_full_roster() -> Vec<MobaPlayer> {
    vec![
        make_player("TopLaner", MobaRole::Top, 70),
        make_player("Jungler", MobaRole::Jungle, 65),
        make_player("MidLaner", MobaRole::Mid, 80),
        make_player("BotLaner", MobaRole::Bot, 75),
        make_player("Support", MobaRole::Support, 60),
    ]
}

fn make_test_team() -> MobaTeam {
    MobaTeam::new("T1".to_string(), "T1".to_string(), make_full_roster())
}

// ---------------------------------------------------------------------------
// MobaTeam creation
// ---------------------------------------------------------------------------

#[test]
fn moba_team_stores_identity() {
    let team = make_test_team();
    assert_eq!(team.name(), "T1");
    assert_eq!(team.tag(), "T1");
}

#[test]
fn moba_team_has_roster() {
    let team = make_test_team();
    assert_eq!(team.roster().len(), 5);
}

#[test]
fn moba_team_starts_with_default_synergy() {
    let team = make_test_team();
    assert_eq!(team.synergy().value(), 0);
}

#[test]
fn moba_team_starts_with_default_reputation() {
    let team = make_test_team();
    assert_eq!(team.reputation().value(), 50);
}

// ---------------------------------------------------------------------------
// Roster queries
// ---------------------------------------------------------------------------

#[test]
fn moba_team_find_player_by_primary_role() {
    let team = make_test_team();
    let mid = team.player_by_primary_role(MobaRole::Mid);
    assert!(mid.is_some());
    assert_eq!(mid.unwrap().nickname(), "MidLaner");
}

#[test]
fn moba_team_find_player_by_primary_role_returns_none_if_missing() {
    let team = MobaTeam::new(
        "Duo".to_string(),
        "DUO".to_string(),
        vec![make_player("Top", MobaRole::Top, 50)],
    );
    assert!(team.player_by_primary_role(MobaRole::Mid).is_none());
}

// ---------------------------------------------------------------------------
// Roster mutation
// ---------------------------------------------------------------------------

#[test]
fn moba_team_player_by_primary_role_mut_allows_state_change() {
    let mut team = make_test_team();
    let mid = team.player_by_primary_role_mut(MobaRole::Mid).unwrap();
    mid.state_mut().stamina.decrease(30);

    let mid = team.player_by_primary_role(MobaRole::Mid).unwrap();
    assert_eq!(mid.state().stamina.value(), 70);
}

// ---------------------------------------------------------------------------
// Synergy mutation
// ---------------------------------------------------------------------------

#[test]
fn moba_team_synergy_can_increase() {
    let mut team = make_test_team();
    team.synergy_mut().increase(15);
    assert_eq!(team.synergy().value(), 15);
}

// ---------------------------------------------------------------------------
// Team power calculation
// ---------------------------------------------------------------------------

#[test]
fn moba_team_compute_power_sums_key_attributes() {
    let team = make_test_team();
    let power = team.compute_power();
    assert!(power > 0);
}

#[test]
fn moba_team_higher_skill_gives_higher_power() {
    let weak = MobaTeam::new(
        "Weak".to_string(),
        "W".to_string(),
        vec![make_player("P1", MobaRole::Mid, 30)],
    );
    let strong = MobaTeam::new(
        "Strong".to_string(),
        "S".to_string(),
        vec![make_player("P1", MobaRole::Mid, 90)],
    );
    assert!(strong.compute_power() > weak.compute_power());
}
