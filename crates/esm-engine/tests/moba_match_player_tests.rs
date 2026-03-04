use esm_engine::moba_match::player_state::MatchPlayerState;

// ---------------------------------------------------------------------------
// Initial state
// ---------------------------------------------------------------------------

#[test]
fn player_state_starts_with_zero_stats() {
    let ps = MatchPlayerState::new(0);
    assert_eq!(ps.player_index(), 0);
    assert_eq!(ps.gold(), 500); // starting gold
    assert_eq!(ps.cs(), 0);
    assert_eq!(ps.kills(), 0);
    assert_eq!(ps.deaths(), 0);
    assert_eq!(ps.assists(), 0);
    assert_eq!(ps.kill_streak(), 0);
    assert_eq!(ps.death_streak(), 0);
    assert!(!ps.is_dead());
    assert_eq!(ps.death_timer(), 0);
}

// ---------------------------------------------------------------------------
// Gold & farm
// ---------------------------------------------------------------------------

#[test]
fn player_state_add_gold() {
    let mut ps = MatchPlayerState::new(0);
    ps.add_gold(300);
    assert_eq!(ps.gold(), 800);
}

#[test]
fn player_state_add_cs() {
    let mut ps = MatchPlayerState::new(0);
    ps.add_cs(10);
    assert_eq!(ps.cs(), 10);
    ps.add_cs(5);
    assert_eq!(ps.cs(), 15);
}

// ---------------------------------------------------------------------------
// Kills, deaths, assists
// ---------------------------------------------------------------------------

#[test]
fn player_state_record_kill() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_kill(300);
    assert_eq!(ps.kills(), 1);
    assert_eq!(ps.gold(), 800);
    assert_eq!(ps.kill_streak(), 1);
    assert_eq!(ps.death_streak(), 0);
}

#[test]
fn player_state_kill_streak_increments() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_kill(300);
    ps.record_kill(300);
    ps.record_kill(300);
    assert_eq!(ps.kill_streak(), 3);
}

#[test]
fn player_state_record_death() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_kill(300); // 1 kill streak
    ps.record_death(20); // 20 second death timer
    assert_eq!(ps.deaths(), 1);
    assert_eq!(ps.kill_streak(), 0); // reset
    assert_eq!(ps.death_streak(), 1);
    assert!(ps.is_dead());
    assert_eq!(ps.death_timer(), 20);
}

#[test]
fn player_state_death_streak_increments() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_death(10);
    ps.tick_death_timer(10); // respawn
    ps.record_death(10);
    assert_eq!(ps.death_streak(), 2);
}

#[test]
fn player_state_record_assist() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_assist(150);
    assert_eq!(ps.assists(), 1);
    assert_eq!(ps.gold(), 650);
}

// ---------------------------------------------------------------------------
// Death timer
// ---------------------------------------------------------------------------

#[test]
fn player_state_death_timer_ticks_down() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_death(30);
    ps.tick_death_timer(10);
    assert_eq!(ps.death_timer(), 20);
    assert!(ps.is_dead());
}

#[test]
fn player_state_respawns_when_timer_reaches_zero() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_death(20);
    ps.tick_death_timer(20);
    assert_eq!(ps.death_timer(), 0);
    assert!(!ps.is_dead());
}

#[test]
fn player_state_death_timer_does_not_underflow() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_death(10);
    ps.tick_death_timer(50);
    assert_eq!(ps.death_timer(), 0);
    assert!(!ps.is_dead());
}

// ---------------------------------------------------------------------------
// KDA calculation
// ---------------------------------------------------------------------------

#[test]
fn player_state_kda_with_no_deaths() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_kill(300);
    ps.record_kill(300);
    ps.record_assist(150);
    // KDA with 0 deaths is perfect → (K+A)/1
    assert!((ps.kda() - 3.0).abs() < f64::EPSILON);
}

#[test]
fn player_state_kda_with_deaths() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_kill(300);
    ps.record_kill(300);
    ps.record_assist(150);
    ps.record_death(10);
    ps.tick_death_timer(10);
    ps.record_death(10);
    // KDA = (2+1)/2 = 1.5
    assert!((ps.kda() - 1.5).abs() < f64::EPSILON);
}

#[test]
fn player_state_kda_zero_everything() {
    let ps = MatchPlayerState::new(0);
    assert!((ps.kda() - 0.0).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// Bounty (based on kill streak)
// ---------------------------------------------------------------------------

#[test]
fn player_state_bounty_base_with_no_streak() {
    let ps = MatchPlayerState::new(0);
    assert_eq!(ps.bounty(), 300);
}

#[test]
fn player_state_bounty_increases_with_kill_streak() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_kill(300);
    ps.record_kill(300);
    ps.record_kill(300);
    assert!(ps.bounty() > 300);
}
