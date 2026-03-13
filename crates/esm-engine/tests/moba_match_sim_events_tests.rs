use esm_core::rng::GameRng;
use esm_engine::moba_match::event::MatchEventKind;
use esm_engine::moba_match::events::*;
use esm_engine::moba_match::game_state::{MatchGameState, MatchPlayerSimulationData};
use esm_engine::moba_match::state::TeamSide;

fn make_attrs() -> Vec<MatchPlayerSimulationData> {
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

fn make_state() -> MatchGameState {
    MatchGameState::new(make_attrs(), make_attrs(), 5)
}

fn make_state_at(minute: u32) -> MatchGameState {
    let mut s = make_state();
    for _ in 0..minute {
        s.tick_minute();
    }
    s
}

// ---------------------------------------------------------------------------
// SimEvent trait: FarmTick
// ---------------------------------------------------------------------------

#[test]
fn farm_tick_is_always_enabled() {
    let s = make_state();
    assert!(FarmTick.is_enabled(&s));
}

#[test]
fn farm_tick_has_positive_weight() {
    let s = make_state();
    assert!(FarmTick.weight(&s) > 0.0);
}

#[test]
fn farm_tick_advances_minute() {
    let mut s = make_state();
    let mut rng = GameRng::from_seed(42);
    let _ = FarmTick.process(&mut s, &mut rng);
    assert_eq!(s.minute, 1);
}

#[test]
fn farm_tick_gives_gold_and_cs() {
    let mut s = make_state();
    let mut rng = GameRng::from_seed(42);
    let starting_gold = s.blue_total_gold();
    let _ = FarmTick.process(&mut s, &mut rng);
    assert!(s.blue_total_gold() > starting_gold);
    assert!(s.players(TeamSide::Blue)[0].cs() > 0);
}

#[test]
fn farm_tick_returns_farm_tick_event() {
    let mut s = make_state();
    let mut rng = GameRng::from_seed(42);
    let event = FarmTick.process(&mut s, &mut rng);
    assert!(matches!(event.kind(), MatchEventKind::FarmTick));
}

// ---------------------------------------------------------------------------
// SimEvent trait: SoloKill
// ---------------------------------------------------------------------------

#[test]
fn solo_kill_is_enabled_when_both_sides_alive() {
    let s = make_state();
    assert!(SoloKill.is_enabled(&s));
}

#[test]
fn solo_kill_disabled_when_no_blue_alive() {
    let mut s = make_state();
    for p in s.blue_players.iter_mut() {
        p.record_death(999);
    }
    assert!(!SoloKill.is_enabled(&s));
}

#[test]
fn solo_kill_produces_correct_event() {
    let mut s = make_state_at(5);
    let mut rng = GameRng::from_seed(42);
    let event = SoloKill.process(&mut s, &mut rng);
    assert!(matches!(event.kind(), MatchEventKind::SoloKill { .. }));
    assert!(event.commentary().is_some());
}

#[test]
fn solo_kill_first_blood_sets_flag() {
    let mut s = make_state_at(3);
    assert!(!s.first_blood_claimed);
    let mut rng = GameRng::from_seed(42);
    let _ = SoloKill.process(&mut s, &mut rng);
    assert!(s.first_blood_claimed);
}

#[test]
fn solo_kill_updates_player_stats() {
    let mut s = make_state_at(5);
    let mut rng = GameRng::from_seed(42);
    let _ = SoloKill.process(&mut s, &mut rng);
    let total_kills: u32 = s
        .blue_players
        .iter()
        .chain(s.red_players.iter())
        .map(|p| p.kills())
        .sum();
    let total_deaths: u32 = s
        .blue_players
        .iter()
        .chain(s.red_players.iter())
        .map(|p| p.deaths())
        .sum();
    assert_eq!(total_kills, 1);
    assert_eq!(total_deaths, 1);
}

// ---------------------------------------------------------------------------
// SimEvent trait: Teamfight
// ---------------------------------------------------------------------------

#[test]
fn teamfight_enabled_when_both_sides_have_2_plus_alive() {
    let s = make_state();
    assert!(Teamfight.is_enabled(&s));
}

#[test]
fn teamfight_disabled_when_one_side_has_1_alive() {
    let mut s = make_state();
    for p in s.blue_players.iter_mut().skip(1) {
        p.record_death(999);
    }
    assert!(!Teamfight.is_enabled(&s));
}

#[test]
fn teamfight_produces_correct_event() {
    let mut s = make_state_at(15);
    let mut rng = GameRng::from_seed(42);
    let event = Teamfight.process(&mut s, &mut rng);
    assert!(matches!(event.kind(), MatchEventKind::Teamfight { .. }));
    assert!(event.commentary().is_some());
}

#[test]
fn teamfight_causes_kills_and_deaths() {
    let mut s = make_state_at(15);
    let mut rng = GameRng::from_seed(42);
    let _ = Teamfight.process(&mut s, &mut rng);
    let total_kills: u32 = s
        .blue_players
        .iter()
        .chain(s.red_players.iter())
        .map(|p| p.kills())
        .sum();
    let total_deaths: u32 = s
        .blue_players
        .iter()
        .chain(s.red_players.iter())
        .map(|p| p.deaths())
        .sum();
    assert!(total_kills > 0);
    assert!(total_deaths > 0);
}

// ---------------------------------------------------------------------------
// SimEvent trait: TowerSiege
// ---------------------------------------------------------------------------

#[test]
fn tower_siege_enabled_when_towers_standing() {
    let s = make_state_at(5);
    assert!(TowerSiege.is_enabled(&s));
}

#[test]
fn tower_siege_produces_tower_destroyed_event() {
    let mut s = make_state_at(10);
    let mut rng = GameRng::from_seed(42);
    let event = TowerSiege.process(&mut s, &mut rng);
    assert!(matches!(
        event.kind(),
        MatchEventKind::TowerDestroyed { .. }
    ));
}

#[test]
fn tower_siege_grants_gold() {
    let mut s = make_state_at(10);
    let total_before = s.blue_total_gold() + s.red_total_gold();
    let mut rng = GameRng::from_seed(42);
    let _ = TowerSiege.process(&mut s, &mut rng);
    let total_after = s.blue_total_gold() + s.red_total_gold();
    assert!(total_after > total_before);
}

// ---------------------------------------------------------------------------
// SimEvent trait: DragonFight
// ---------------------------------------------------------------------------

#[test]
fn dragon_fight_disabled_before_minute_5() {
    let s = make_state();
    assert!(!DragonFight.is_enabled(&s));
}

#[test]
fn dragon_fight_enabled_at_minute_5() {
    let s = make_state_at(5);
    assert!(DragonFight.is_enabled(&s));
}

#[test]
fn dragon_fight_disabled_during_cooldown() {
    let mut s = make_state_at(10);
    s.dragon_timer = 3;
    assert!(!DragonFight.is_enabled(&s));
}

#[test]
fn dragon_fight_produces_dragon_kill_event() {
    let mut s = make_state_at(6);
    let mut rng = GameRng::from_seed(42);
    let event = DragonFight.process(&mut s, &mut rng);
    assert!(matches!(event.kind(), MatchEventKind::DragonKill { .. }));
}

#[test]
fn dragon_fight_sets_cooldown_timer() {
    let mut s = make_state_at(6);
    let mut rng = GameRng::from_seed(42);
    let _ = DragonFight.process(&mut s, &mut rng);
    assert!(s.dragon_timer > 0);
}

#[test]
fn dragon_fight_increments_dragon_count() {
    let mut s = make_state_at(6);
    let mut rng = GameRng::from_seed(42);
    let _ = DragonFight.process(&mut s, &mut rng);
    let total = s.map.objectives().dragon_count(TeamSide::Blue)
        + s.map.objectives().dragon_count(TeamSide::Red);
    assert_eq!(total, 1);
}

// ---------------------------------------------------------------------------
// SimEvent trait: HeraldFight
// ---------------------------------------------------------------------------

#[test]
fn herald_fight_disabled_before_minute_8() {
    let s = make_state_at(7);
    assert!(!HeraldFight.is_enabled(&s));
}

#[test]
fn herald_fight_enabled_at_minute_8() {
    let s = make_state_at(8);
    assert!(HeraldFight.is_enabled(&s));
}

#[test]
fn herald_fight_disabled_after_minute_20() {
    let s = make_state_at(20);
    assert!(!HeraldFight.is_enabled(&s));
}

#[test]
fn herald_fight_produces_herald_kill_event() {
    let mut s = make_state_at(10);
    let mut rng = GameRng::from_seed(42);
    let event = HeraldFight.process(&mut s, &mut rng);
    assert!(matches!(event.kind(), MatchEventKind::HeraldKill { .. }));
}

#[test]
fn herald_fight_consumes_herald() {
    let mut s = make_state_at(10);
    let mut rng = GameRng::from_seed(42);
    let _ = HeraldFight.process(&mut s, &mut rng);
    assert!(!s.map.objectives().herald_available());
}

#[test]
fn herald_fight_disabled_after_consumed() {
    let mut s = make_state_at(10);
    let mut rng = GameRng::from_seed(42);
    let _ = HeraldFight.process(&mut s, &mut rng);
    assert!(!HeraldFight.is_enabled(&s));
}

// ---------------------------------------------------------------------------
// SimEvent trait: BaronFight
// ---------------------------------------------------------------------------

#[test]
fn baron_fight_disabled_before_spawn() {
    let s = make_state_at(19);
    assert!(!BaronFight.is_enabled(&s));
}

#[test]
fn baron_fight_enabled_when_baron_alive_after_20() {
    let mut s = make_state_at(20);
    s.map.objectives_mut().spawn_baron();
    assert!(BaronFight.is_enabled(&s));
}

#[test]
fn baron_fight_disabled_when_baron_dead() {
    let s = make_state_at(25);
    // Baron not spawned
    assert!(!BaronFight.is_enabled(&s));
}

#[test]
fn baron_fight_produces_baron_kill_event() {
    let mut s = make_state_at(25);
    s.map.objectives_mut().spawn_baron();
    let mut rng = GameRng::from_seed(42);
    let event = BaronFight.process(&mut s, &mut rng);
    assert!(matches!(event.kind(), MatchEventKind::BaronKill { .. }));
}

#[test]
fn baron_fight_sets_respawn_timer() {
    let mut s = make_state_at(25);
    s.map.objectives_mut().spawn_baron();
    let mut rng = GameRng::from_seed(42);
    let _ = BaronFight.process(&mut s, &mut rng);
    assert!(s.baron_spawn_timer > 0);
}

#[test]
fn baron_fight_kills_baron() {
    let mut s = make_state_at(25);
    s.map.objectives_mut().spawn_baron();
    let mut rng = GameRng::from_seed(42);
    let _ = BaronFight.process(&mut s, &mut rng);
    assert!(!s.map.objectives().baron_alive());
}

#[test]
fn baron_cannot_be_taken_twice_in_a_row_without_respawn() {
    let mut s = make_state_at(25);
    s.map.objectives_mut().spawn_baron();
    let mut rng = GameRng::from_seed(42);
    let _ = BaronFight.process(&mut s, &mut rng);
    // Baron is dead and respawn timer is set — should not be enabled
    assert!(!BaronFight.is_enabled(&s));
}

// ---------------------------------------------------------------------------
// SimEvent trait: InhibSiege
// ---------------------------------------------------------------------------

#[test]
fn inhib_siege_disabled_when_no_inhibitors_vulnerable() {
    let s = make_state();
    assert!(!InhibSiege.is_enabled(&s));
}

#[test]
fn inhib_siege_enabled_when_inhibitor_vulnerable() {
    let mut s = make_state_at(15);
    // Destroy all towers in top lane on blue side to make inhib vulnerable
    s.map.destroy_tower(
        TeamSide::Blue,
        esm_engine::moba_match::map::Lane::Top,
        esm_engine::moba_match::map::TowerTier::Outer,
    );
    s.map.destroy_tower(
        TeamSide::Blue,
        esm_engine::moba_match::map::Lane::Top,
        esm_engine::moba_match::map::TowerTier::Inner,
    );
    s.map.destroy_tower(
        TeamSide::Blue,
        esm_engine::moba_match::map::Lane::Top,
        esm_engine::moba_match::map::TowerTier::Inhibitor,
    );
    assert!(InhibSiege.is_enabled(&s));
}

// ---------------------------------------------------------------------------
// SimEvent trait: NexusSiege
// ---------------------------------------------------------------------------

#[test]
fn nexus_siege_disabled_when_all_inhibitors_standing() {
    let s = make_state();
    assert!(!NexusSiege.is_enabled(&s));
}

#[test]
fn nexus_siege_enabled_when_inhibitor_down() {
    let mut s = make_state_at(20);
    // Open a path to nexus on blue side
    s.map.destroy_tower(
        TeamSide::Blue,
        esm_engine::moba_match::map::Lane::Top,
        esm_engine::moba_match::map::TowerTier::Outer,
    );
    s.map.destroy_tower(
        TeamSide::Blue,
        esm_engine::moba_match::map::Lane::Top,
        esm_engine::moba_match::map::TowerTier::Inner,
    );
    s.map.destroy_tower(
        TeamSide::Blue,
        esm_engine::moba_match::map::Lane::Top,
        esm_engine::moba_match::map::TowerTier::Inhibitor,
    );
    s.map
        .destroy_inhibitor(TeamSide::Blue, esm_engine::moba_match::map::Lane::Top, 300);
    assert!(NexusSiege.is_enabled(&s));
}

#[test]
fn nexus_siege_weight_increases_over_time() {
    let mut s1 = make_state_at(25);
    #[allow(unused_mut)]
    let mut s2 = make_state_at(40);
    // Set up nexus vulnerability for weight calculation
    for s in [&mut s1, &mut s2] {
        s.map.destroy_tower(
            TeamSide::Blue,
            esm_engine::moba_match::map::Lane::Top,
            esm_engine::moba_match::map::TowerTier::Outer,
        );
        s.map.destroy_tower(
            TeamSide::Blue,
            esm_engine::moba_match::map::Lane::Top,
            esm_engine::moba_match::map::TowerTier::Inner,
        );
        s.map.destroy_tower(
            TeamSide::Blue,
            esm_engine::moba_match::map::Lane::Top,
            esm_engine::moba_match::map::TowerTier::Inhibitor,
        );
        s.map
            .destroy_inhibitor(TeamSide::Blue, esm_engine::moba_match::map::Lane::Top, 300);
    }
    assert!(NexusSiege.weight(&s2) > NexusSiege.weight(&s1));
}

// ---------------------------------------------------------------------------
// Event registry and selection
// ---------------------------------------------------------------------------

#[test]
fn all_events_returns_nine_events() {
    let events = all_events();
    assert_eq!(events.len(), 9);
}

#[test]
fn enabled_events_includes_farm_tick() {
    let s = make_state();
    let events = all_events();
    let enabled = enabled_events(&s, &events);
    assert!(!enabled.is_empty());
    // Farm tick (index 0) should be enabled
    assert!(enabled.iter().any(|(i, _)| *i == 0));
}

#[test]
fn enabled_events_filters_unavailable() {
    let s = make_state();
    let events = all_events();
    let enabled = enabled_events(&s, &events);
    // At minute 0, dragon/herald/baron should NOT be enabled
    // DragonFight is index 4, HeraldFight index 5, BaronFight index 6
    assert!(!enabled.iter().any(|(i, _)| *i == 4)); // DragonFight
    assert!(!enabled.iter().any(|(i, _)| *i == 5)); // HeraldFight
    assert!(!enabled.iter().any(|(i, _)| *i == 6)); // BaronFight
}

#[test]
fn pick_event_returns_valid_index() {
    let s = make_state();
    let events = all_events();
    let enabled = enabled_events(&s, &events);
    let mut rng = GameRng::from_seed(42);
    let idx = pick_event(&mut rng, &enabled);
    assert!(enabled.iter().any(|(i, _)| *i == idx));
}

#[test]
fn pick_event_is_deterministic() {
    let s = make_state();
    let events = all_events();
    let enabled = enabled_events(&s, &events);
    let mut rng1 = GameRng::from_seed(42);
    let mut rng2 = GameRng::from_seed(42);
    assert_eq!(
        pick_event(&mut rng1, &enabled),
        pick_event(&mut rng2, &enabled)
    );
}

// ---------------------------------------------------------------------------
// Integration: no consecutive barons
// ---------------------------------------------------------------------------

#[test]
fn baron_cannot_appear_consecutively_in_event_selection() {
    let mut s = make_state_at(25);
    s.map.objectives_mut().spawn_baron();
    let mut rng = GameRng::from_seed(42);

    // Take baron
    let _ = BaronFight.process(&mut s, &mut rng);

    // Now baron should be disabled
    let events = all_events();
    let enabled = enabled_events(&s, &events);
    // BaronFight is index 6
    assert!(
        !enabled.iter().any(|(i, _)| *i == 6),
        "Baron should not be enabled right after being taken"
    );
}

// ---------------------------------------------------------------------------
// Integration: game ends via NexusSiege
// ---------------------------------------------------------------------------

#[test]
fn nexus_siege_can_end_game() {
    let mut s = make_state_at(35);
    // Open nexus on blue side
    for lane in [
        esm_engine::moba_match::map::Lane::Top,
        esm_engine::moba_match::map::Lane::Mid,
        esm_engine::moba_match::map::Lane::Bot,
    ] {
        s.map.destroy_tower(
            TeamSide::Blue,
            lane,
            esm_engine::moba_match::map::TowerTier::Outer,
        );
        s.map.destroy_tower(
            TeamSide::Blue,
            lane,
            esm_engine::moba_match::map::TowerTier::Inner,
        );
        s.map.destroy_tower(
            TeamSide::Blue,
            lane,
            esm_engine::moba_match::map::TowerTier::Inhibitor,
        );
        s.map.destroy_inhibitor(TeamSide::Blue, lane, 300);
    }
    // Give red a massive gold lead to ensure they win the push
    for p in s.red_players.iter_mut() {
        p.add_gold(50000);
    }

    // Try nexus siege many times — it should eventually succeed
    let mut game_ended = false;
    for seed in 0..200 {
        let mut test_state = s.clone();
        let mut rng = GameRng::from_seed(seed);
        let event = NexusSiege.process(&mut test_state, &mut rng);
        if matches!(event.kind(), MatchEventKind::NexusDestroyed { .. }) {
            game_ended = true;
            assert!(test_state.is_over());
            break;
        }
    }
    assert!(game_ended, "NexusSiege should be able to end the game");
}
