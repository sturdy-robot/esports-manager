use esm_core::rng::GameRng;
use esm_engine::moba_match::engine::{MobaMatchConfig, MobaMatchEngine};
use esm_engine::moba_match::event::MatchEventKind;
use esm_engine::moba_match::game_state::MatchPlayerSimulationData;
use esm_engine::moba_match::state::TeamSide;

// ---------------------------------------------------------------------------
// MobaMatchConfig
// ---------------------------------------------------------------------------

#[test]
fn config_default_has_5_players_per_side() {
    let cfg = MobaMatchConfig::default();
    assert_eq!(cfg.players_per_side, 5);
}

// ---------------------------------------------------------------------------
// MobaMatchEngine: basic simulation
// ---------------------------------------------------------------------------

fn make_team_attrs() -> Vec<MatchPlayerSimulationData> {
    // 5 players, each with 9 attributes
    vec![
        [70, 85, 80, 78, 70, 65, 90, 72, 85], // Top
        [80, 82, 85, 75, 78, 70, 88, 80, 82], // Jungle
        [70, 90, 95, 99, 85, 80, 97, 88, 92], // Mid (star player)
        [72, 88, 78, 82, 68, 60, 93, 70, 85], // Bot
        [78, 86, 90, 80, 82, 75, 85, 92, 88], // Support
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

fn make_weaker_team_attrs() -> Vec<MatchPlayerSimulationData> {
    vec![
        [55, 60, 58, 55, 58, 52, 62, 56, 60],
        [58, 62, 60, 58, 60, 55, 65, 60, 62],
        [56, 64, 62, 60, 60, 56, 68, 58, 64],
        [57, 63, 58, 56, 56, 54, 66, 55, 62],
        [60, 60, 64, 56, 62, 58, 60, 66, 62],
    ]
    .into_iter()
    .map(|attributes| MatchPlayerSimulationData {
        attributes,
        stamina: 80,
        morale: 40,
        mastery_multiplier: 0.9,
    })
    .collect()
}

#[test]
fn engine_produces_a_result() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    assert!(result.winner == TeamSide::Blue || result.winner == TeamSide::Red);
    assert!(!result.events.is_empty());
    assert!(result.duration_minutes > 0);
}

#[test]
fn engine_result_has_player_stats() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    assert_eq!(result.blue_players.len(), 5);
    assert_eq!(result.red_players.len(), 5);
}

#[test]
fn engine_result_events_have_commentary() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    let with_commentary = result
        .events
        .iter()
        .filter(|e| e.commentary().is_some())
        .count();
    assert!(
        with_commentary > 0,
        "Some events should have commentary, got 0 out of {}",
        result.events.len()
    );
}

#[test]
fn engine_result_ends_with_nexus_destroyed() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    let last = result.events.last().unwrap();
    assert!(
        matches!(
            last.kind(),
            esm_engine::moba_match::event::MatchEventKind::NexusDestroyed { .. }
        ),
        "Last event should be NexusDestroyed"
    );
}

#[test]
fn engine_is_deterministic() {
    let blue = make_team_attrs();
    let red = make_weaker_team_attrs();
    let cfg = MobaMatchConfig::default();

    let mut rng1 = GameRng::from_seed(42);
    let mut rng2 = GameRng::from_seed(42);

    let r1 = MobaMatchEngine::simulate(&mut rng1, &blue, &red, &cfg);
    let r2 = MobaMatchEngine::simulate(&mut rng2, &blue, &red, &cfg);

    assert_eq!(r1.winner, r2.winner);
    assert_eq!(r1.duration_minutes, r2.duration_minutes);
    assert_eq!(r1.events.len(), r2.events.len());
}

#[test]
fn engine_stronger_team_wins_more_often() {
    let strong = make_team_attrs();
    let weak = make_weaker_team_attrs();
    let cfg = MobaMatchConfig::default();

    let mut blue_wins = 0;
    for seed in 0..100 {
        let mut rng = GameRng::from_seed(seed);
        let result = MobaMatchEngine::simulate(&mut rng, &strong, &weak, &cfg);
        if result.winner == TeamSide::Blue {
            blue_wins += 1;
        }
    }
    assert!(
        blue_wins > 55,
        "Stronger team should win >55%, got {blue_wins}/100"
    );
}

#[test]
fn engine_result_has_tower_data() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    // At least one tower should be destroyed in most games
    let tower_events = result
        .events
        .iter()
        .filter(|e| {
            matches!(
                e.kind(),
                esm_engine::moba_match::event::MatchEventKind::TowerDestroyed { .. }
            )
        })
        .count();
    assert!(tower_events > 0, "Should have tower events in a full game");
}

#[test]
fn engine_result_has_objective_data() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    let obj_events = result
        .events
        .iter()
        .filter(|e| {
            matches!(
                e.kind(),
                esm_engine::moba_match::event::MatchEventKind::DragonKill { .. }
                    | esm_engine::moba_match::event::MatchEventKind::BaronKill { .. }
                    | esm_engine::moba_match::event::MatchEventKind::HeraldKill { .. }
            )
        })
        .count();
    assert!(
        obj_events > 0,
        "Should have objective events in a full game"
    );
}

#[test]
fn engine_players_accumulate_gold_and_cs() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    for ps in &result.blue_players {
        assert!(
            ps.gold() > 500,
            "Player should earn gold above starting 500, got {}",
            ps.gold()
        );
    }
}

#[test]
fn engine_match_ends_within_reasonable_time() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    assert!(
        result.duration_minutes <= 60,
        "Match should end within 60 minutes, got {}",
        result.duration_minutes
    );
}

// ---------------------------------------------------------------------------
// Monte Carlo Statistical Distribution Tests
// ---------------------------------------------------------------------------

#[test]
fn engine_monte_carlo_stamina_impact() {
    let base_blue = make_team_attrs();
    let mut exhausted_red = make_team_attrs();

    // Red team is completely exhausted
    for p in &mut exhausted_red {
        p.stamina = 10;
        p.morale = 50;
    }

    let cfg = MobaMatchConfig::default();
    let mut blue_wins = 0;
    for seed in 0..100 {
        let mut rng = GameRng::from_seed(seed);
        let result = MobaMatchEngine::simulate(&mut rng, &base_blue, &exhausted_red, &cfg);
        if result.winner == TeamSide::Blue {
            blue_wins += 1;
        }
    }

    // Blue should win overwhelmingly against an exhausted team with identical base stats
    assert!(
        blue_wins > 80,
        "Blue with full stamina should win >80% vs exhausted Red, got {blue_wins}/100"
    );
}

#[test]
fn engine_events_carry_game_snapshot() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    // Every non-FarmTick event should have a snapshot
    let non_farm: Vec<_> = result
        .events
        .iter()
        .filter(|e| !matches!(e.kind(), MatchEventKind::FarmTick))
        .collect();
    assert!(!non_farm.is_empty());
    for ev in &non_farm {
        let snap = ev
            .snapshot()
            .expect("Non-farm event should have a snapshot");
        assert_eq!(snap.blue_players.len(), 5);
        assert_eq!(snap.red_players.len(), 5);
    }
}

#[test]
fn engine_snapshot_gold_increases_over_time() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    let snapshots: Vec<_> = result.events.iter().filter_map(|e| e.snapshot()).collect();
    assert!(snapshots.len() >= 2);
    let first = &snapshots[0];
    let last = &snapshots[snapshots.len() - 1];
    assert!(
        last.blue_team_gold > first.blue_team_gold,
        "Blue gold should increase: first={}, last={}",
        first.blue_team_gold,
        last.blue_team_gold
    );
}

#[test]
fn engine_snapshot_has_objective_state() {
    let mut rng = GameRng::from_seed(42);
    let result = MobaMatchEngine::simulate(
        &mut rng,
        &make_team_attrs(),
        &make_team_attrs(),
        &MobaMatchConfig::default(),
    );
    // Find a dragon event and check its snapshot tracks dragon count
    let dragon_event = result
        .events
        .iter()
        .find(|e| matches!(e.kind(), MatchEventKind::DragonKill { .. }));
    if let Some(ev) = dragon_event {
        let snap = ev.snapshot().expect("Dragon event should have snapshot");
        assert!(
            snap.dragons_blue + snap.dragons_red >= 1,
            "After a dragon kill, total dragons should be >= 1"
        );
    }
}

#[test]
fn engine_monte_carlo_mastery_impact() {
    let base_blue = make_team_attrs();
    let mut high_mastery_red = make_team_attrs();

    // Red team has massive champion mastery advantage
    for p in &mut high_mastery_red {
        p.mastery_multiplier = 1.5; // 50% boost
    }

    let cfg = MobaMatchConfig::default();
    let mut red_wins = 0;
    for seed in 0..100 {
        let mut rng = GameRng::from_seed(seed + 100); // offset seeds
        let result = MobaMatchEngine::simulate(&mut rng, &base_blue, &high_mastery_red, &cfg);
        if result.winner == TeamSide::Red {
            red_wins += 1;
        }
    }

    // Red should win overwhelmingly due to mastery advantage
    assert!(
        red_wins >= 70,
        "Red with high mastery should still win decisively against base Blue, got {red_wins}/100"
    );
}
