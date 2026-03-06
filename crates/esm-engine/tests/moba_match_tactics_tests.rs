use esm_core::rng::GameRng;
use esm_engine::moba_match::engine::{MobaMatchConfig, MobaMatchEngine};
use esm_engine::moba_match::event::MatchEventKind;
use esm_engine::moba_match::game_state::MatchPlayerSimulationData;
use esm_engine::moba_match::tactics::{Focus, MatchTactics, Playstyle};

fn make_team() -> Vec<MatchPlayerSimulationData> {
    vec![
        [70, 85, 80, 78, 70, 65, 90, 72, 85],
        [80, 82, 85, 75, 78, 70, 88, 80, 82],
        [70, 90, 95, 99, 85, 80, 97, 88, 92],
        [72, 88, 78, 82, 68, 60, 93, 70, 85],
        [78, 86, 90, 80, 82, 75, 85, 92, 88],
    ]
    .into_iter()
    .map(|attributes| MatchPlayerSimulationData {
        attributes,
        stamina: 100,
        morale: 50,
        mastery_multiplier: 1.0,
    })
    .collect()
}

// ---------------------------------------------------------------------------
// MatchTactics defaults
// ---------------------------------------------------------------------------

#[test]
fn tactics_default_is_balanced_teamfight() {
    let t = MatchTactics::default();
    assert_eq!(t.playstyle, Playstyle::Balanced);
    assert_eq!(t.focus, Focus::Teamfight);
}

#[test]
fn tactics_balanced_multipliers_are_near_one() {
    let t = MatchTactics::default();
    assert!((t.solo_kill_mult() - 1.0).abs() < 0.01);
    assert!((t.farm_mult() - 1.0).abs() < 0.01);
}

// ---------------------------------------------------------------------------
// Multiplier ranges
// ---------------------------------------------------------------------------

#[test]
fn aggressive_increases_solo_kill_weight() {
    let t = MatchTactics { playstyle: Playstyle::Aggressive, focus: Focus::Teamfight };
    assert!(t.solo_kill_mult() > 1.0);
}

#[test]
fn defensive_decreases_solo_kill_weight() {
    let t = MatchTactics { playstyle: Playstyle::Defensive, focus: Focus::Teamfight };
    assert!(t.solo_kill_mult() < 1.0);
}

#[test]
fn objective_focus_increases_objective_weight() {
    let t = MatchTactics { playstyle: Playstyle::Balanced, focus: Focus::Objective };
    assert!(t.objective_mult() > 1.0);
}

#[test]
fn splitpush_focus_increases_tower_weight() {
    let t = MatchTactics { playstyle: Playstyle::Balanced, focus: Focus::Splitpush };
    assert!(t.tower_mult() > 1.0);
}

// ---------------------------------------------------------------------------
// Tactics affect simulation outcome distribution
// ---------------------------------------------------------------------------

fn count_events(seed_range: std::ops::Range<u64>, tactics: &MatchTactics) -> (usize, usize, usize) {
    let blue = make_team();
    let red = make_team();
    let cfg = MobaMatchConfig::default();
    let mut solo_kills = 0;
    let mut teamfights = 0;
    let mut objectives = 0;

    for seed in seed_range {
        let mut rng = GameRng::from_seed(seed);
        let result = MobaMatchEngine::simulate_with_tactics(
            &mut rng, &blue, &red, &cfg, tactics,
        );
        for ev in &result.events {
            match ev.kind() {
                MatchEventKind::SoloKill { .. } => solo_kills += 1,
                MatchEventKind::Teamfight { .. } => teamfights += 1,
                MatchEventKind::DragonKill { .. }
                | MatchEventKind::BaronKill { .. }
                | MatchEventKind::HeraldKill { .. } => objectives += 1,
                _ => {}
            }
        }
    }
    (solo_kills, teamfights, objectives)
}

#[test]
fn aggressive_tactics_produce_more_solo_kills_than_defensive() {
    let aggressive = MatchTactics { playstyle: Playstyle::Aggressive, focus: Focus::Teamfight };
    let defensive = MatchTactics { playstyle: Playstyle::Defensive, focus: Focus::Teamfight };

    let (agg_sk, _, _) = count_events(0..30, &aggressive);
    let (def_sk, _, _) = count_events(0..30, &defensive);

    assert!(
        agg_sk > def_sk,
        "Aggressive should produce more solo kills: aggressive={agg_sk}, defensive={def_sk}"
    );
}

#[test]
fn objective_focus_produces_more_objectives_than_teamfight() {
    let obj = MatchTactics { playstyle: Playstyle::Balanced, focus: Focus::Objective };
    let tf = MatchTactics { playstyle: Playstyle::Balanced, focus: Focus::Teamfight };

    let (_, _, obj_count) = count_events(0..30, &obj);
    let (_, _, tf_count) = count_events(0..30, &tf);

    assert!(
        obj_count > tf_count,
        "Objective focus should produce more objectives: objective={obj_count}, teamfight={tf_count}"
    );
}
