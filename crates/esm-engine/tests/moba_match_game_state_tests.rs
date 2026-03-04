use esm_core::rng::GameRng;
use esm_engine::moba_match::event::MobaMatchPhase;
use esm_engine::moba_match::game_state::MatchGameState;
use esm_engine::moba_match::state::TeamSide;

fn make_attrs() -> Vec<[u8; 9]> {
    vec![
        [70, 85, 80, 78, 70, 65, 90, 72, 85],
        [80, 82, 85, 75, 78, 70, 88, 80, 82],
        [70, 90, 95, 99, 85, 80, 97, 88, 92],
        [72, 88, 78, 82, 68, 60, 93, 70, 85],
        [78, 86, 90, 80, 82, 75, 85, 92, 88],
    ]
}

fn make_state() -> MatchGameState {
    MatchGameState::new(make_attrs(), make_attrs(), 5)
}

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

#[test]
fn game_state_starts_at_minute_zero() {
    let s = make_state();
    assert_eq!(s.minute, 0);
}

#[test]
fn game_state_starts_with_no_winner() {
    let s = make_state();
    assert!(!s.is_over());
    assert!(s.winner.is_none());
}

#[test]
fn game_state_has_correct_player_counts() {
    let s = make_state();
    assert_eq!(s.blue_players.len(), 5);
    assert_eq!(s.red_players.len(), 5);
}

#[test]
fn game_state_first_blood_not_claimed() {
    let s = make_state();
    assert!(!s.first_blood_claimed);
}

// ---------------------------------------------------------------------------
// Phase
// ---------------------------------------------------------------------------

#[test]
fn game_state_phase_early() {
    let s = make_state();
    assert_eq!(s.phase(), MobaMatchPhase::Early);
}

#[test]
fn game_state_phase_mid() {
    let mut s = make_state();
    s.minute = 15;
    assert_eq!(s.phase(), MobaMatchPhase::Mid);
}

#[test]
fn game_state_phase_late() {
    let mut s = make_state();
    s.minute = 25;
    assert_eq!(s.phase(), MobaMatchPhase::Late);
}

// ---------------------------------------------------------------------------
// Gold
// ---------------------------------------------------------------------------

#[test]
fn game_state_starting_gold_is_symmetric() {
    let s = make_state();
    assert_eq!(s.blue_total_gold(), s.red_total_gold());
    assert_eq!(s.gold_delta(), 0);
}

#[test]
fn game_state_gold_delta_updates_after_earning() {
    let mut s = make_state();
    s.blue_players[0].add_gold(1000);
    assert_eq!(s.gold_delta(), 1000);
}

// ---------------------------------------------------------------------------
// Team power
// ---------------------------------------------------------------------------

#[test]
fn game_state_team_power_is_sum_of_all_attrs() {
    let s = make_state();
    let expected: f64 = make_attrs()
        .iter()
        .map(|a| a.iter().map(|&v| v as f64).sum::<f64>())
        .sum();
    assert!((s.team_power(TeamSide::Blue) - expected).abs() < 0.001);
}

#[test]
fn game_state_symmetric_teams_have_equal_power() {
    let s = make_state();
    assert!((s.team_power(TeamSide::Blue) - s.team_power(TeamSide::Red)).abs() < 0.001);
}

// ---------------------------------------------------------------------------
// Alive count
// ---------------------------------------------------------------------------

#[test]
fn game_state_all_players_alive_initially() {
    let s = make_state();
    assert_eq!(s.alive_count(TeamSide::Blue), 5);
    assert_eq!(s.alive_count(TeamSide::Red), 5);
}

#[test]
fn game_state_alive_count_decreases_on_death() {
    let mut s = make_state();
    s.blue_players[0].record_death(60);
    assert_eq!(s.alive_count(TeamSide::Blue), 4);
}

// ---------------------------------------------------------------------------
// Player access
// ---------------------------------------------------------------------------

#[test]
fn game_state_players_returns_correct_side() {
    let s = make_state();
    assert_eq!(s.players(TeamSide::Blue).len(), 5);
    assert_eq!(s.players(TeamSide::Red).len(), 5);
}

#[test]
fn game_state_players_mut_can_modify() {
    let mut s = make_state();
    s.players_mut(TeamSide::Blue)[0].add_gold(999);
    assert_eq!(s.players(TeamSide::Blue)[0].gold(), 500 + 999);
}

// ---------------------------------------------------------------------------
// Pick alive player
// ---------------------------------------------------------------------------

#[test]
fn game_state_pick_alive_player_returns_some() {
    let s = make_state();
    let mut rng = GameRng::from_seed(42);
    assert!(s.pick_alive_player(&mut rng, TeamSide::Blue).is_some());
}

#[test]
fn game_state_pick_alive_player_returns_none_when_all_dead() {
    let mut s = make_state();
    for p in s.blue_players.iter_mut() {
        p.record_death(999);
    }
    let mut rng = GameRng::from_seed(42);
    assert!(s.pick_alive_player(&mut rng, TeamSide::Blue).is_none());
}

// ---------------------------------------------------------------------------
// Resolve winner
// ---------------------------------------------------------------------------

#[test]
fn game_state_resolve_winner_returns_a_side() {
    let s = make_state();
    let mut rng = GameRng::from_seed(42);
    let winner = s.resolve_winner(&mut rng);
    assert!(winner == TeamSide::Blue || winner == TeamSide::Red);
}

#[test]
fn game_state_resolve_winner_favors_gold_leader() {
    let mut s = make_state();
    // Give blue a huge gold lead
    for p in s.blue_players.iter_mut() {
        p.add_gold(10000);
    }
    let mut blue_wins = 0;
    for seed in 0..100 {
        let mut rng = GameRng::from_seed(seed);
        if s.resolve_winner(&mut rng) == TeamSide::Blue {
            blue_wins += 1;
        }
    }
    assert!(
        blue_wins > 60,
        "Blue with gold lead should win more, got {blue_wins}/100"
    );
}

// ---------------------------------------------------------------------------
// Tick minute
// ---------------------------------------------------------------------------

#[test]
fn game_state_tick_minute_advances_clock() {
    let mut s = make_state();
    s.tick_minute();
    assert_eq!(s.minute, 1);
}

#[test]
fn game_state_tick_minute_reduces_death_timers() {
    let mut s = make_state();
    s.blue_players[0].record_death(120);
    assert!(s.blue_players[0].is_dead());
    s.tick_minute(); // ticks 60 seconds
    assert!(s.blue_players[0].is_dead()); // 120-60=60, still dead
    s.tick_minute(); // ticks another 60 seconds
    assert!(!s.blue_players[0].is_dead()); // 60-60=0, alive
}

// ---------------------------------------------------------------------------
// Objective availability
// ---------------------------------------------------------------------------

#[test]
fn game_state_dragon_not_available_before_minute_5() {
    let s = make_state();
    assert!(!s.dragon_available());
}

#[test]
fn game_state_dragon_available_at_minute_5() {
    let mut s = make_state();
    s.minute = 5;
    assert!(s.dragon_available());
}

#[test]
fn game_state_dragon_not_available_during_cooldown() {
    let mut s = make_state();
    s.minute = 10;
    s.dragon_timer = 3;
    assert!(!s.dragon_available());
}

#[test]
fn game_state_herald_not_available_before_minute_8() {
    let mut s = make_state();
    s.minute = 7;
    assert!(!s.herald_available());
}

#[test]
fn game_state_herald_available_at_minute_8() {
    let mut s = make_state();
    s.minute = 8;
    assert!(s.herald_available());
}

#[test]
fn game_state_herald_not_available_after_minute_20() {
    let mut s = make_state();
    s.minute = 20;
    assert!(!s.herald_available());
}

#[test]
fn game_state_baron_not_available_before_minute_20() {
    let mut s = make_state();
    s.minute = 19;
    assert!(!s.baron_available());
}

#[test]
fn game_state_baron_available_when_spawned_after_minute_20() {
    let mut s = make_state();
    s.minute = 20;
    s.map.objectives_mut().spawn_baron();
    assert!(s.baron_available());
}

// ---------------------------------------------------------------------------
// Tower / inhibitor vulnerability
// ---------------------------------------------------------------------------

#[test]
fn game_state_next_vulnerable_tower_starts_with_outer() {
    let s = make_state();
    let (_, tier) = s.next_vulnerable_tower(TeamSide::Blue).unwrap();
    assert_eq!(tier, esm_engine::moba_match::map::TowerTier::Outer);
}

#[test]
fn game_state_vulnerable_inhibitor_none_initially() {
    let s = make_state();
    assert!(s.vulnerable_inhibitor_lane(TeamSide::Blue).is_none());
}

// ---------------------------------------------------------------------------
// Record event
// ---------------------------------------------------------------------------

#[test]
fn game_state_record_event_adds_to_history() {
    let mut s = make_state();
    let event = esm_engine::moba_match::event::MatchEvent::new(
        1,
        MobaMatchPhase::Early,
        esm_engine::moba_match::event::MatchEventKind::FarmTick,
    );
    s.record_event(event);
    assert_eq!(s.events.len(), 1);
}

// ---------------------------------------------------------------------------
// Is over
// ---------------------------------------------------------------------------

#[test]
fn game_state_is_over_when_winner_set() {
    let mut s = make_state();
    s.winner = Some(TeamSide::Blue);
    assert!(s.is_over());
}

// ---------------------------------------------------------------------------
// Baron spawn timer
// ---------------------------------------------------------------------------

#[test]
fn game_state_baron_spawn_timer_ticks_down() {
    let mut s = make_state();
    s.minute = 25;
    s.baron_spawn_timer = 3;
    s.tick_minute();
    assert_eq!(s.baron_spawn_timer, 2);
}

#[test]
fn game_state_baron_respawns_when_timer_reaches_zero() {
    let mut s = make_state();
    s.minute = 25;
    s.baron_spawn_timer = 1;
    s.tick_minute();
    assert_eq!(s.baron_spawn_timer, 0);
    assert!(s.map.objectives().baron_alive());
}
