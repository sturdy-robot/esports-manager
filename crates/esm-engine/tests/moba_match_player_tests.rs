use esm_engine::moba_match::player_state::{MatchPlayerState, MultiKill};

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

// ---------------------------------------------------------------------------
// Multi-kill tracking
// ---------------------------------------------------------------------------

#[test]
fn no_multi_kill_after_single_fight_kill() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_fight_kill(300);
    assert_eq!(ps.current_multi_kill(), None);
}

#[test]
fn double_kill_after_two_fight_kills() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_fight_kill(300);
    ps.record_fight_kill(300);
    assert_eq!(ps.current_multi_kill(), Some(MultiKill::Double));
}

#[test]
fn triple_kill_after_three_fight_kills() {
    let mut ps = MatchPlayerState::new(0);
    for _ in 0..3 {
        ps.record_fight_kill(300);
    }
    assert_eq!(ps.current_multi_kill(), Some(MultiKill::Triple));
}

#[test]
fn quadra_kill_after_four_fight_kills() {
    let mut ps = MatchPlayerState::new(0);
    for _ in 0..4 {
        ps.record_fight_kill(300);
    }
    assert_eq!(ps.current_multi_kill(), Some(MultiKill::Quadra));
}

#[test]
fn penta_kill_after_five_fight_kills() {
    let mut ps = MatchPlayerState::new(0);
    for _ in 0..5 {
        ps.record_fight_kill(300);
    }
    assert_eq!(ps.current_multi_kill(), Some(MultiKill::Penta));
}

#[test]
fn penta_is_max_multi_kill() {
    let mut ps = MatchPlayerState::new(0);
    for _ in 0..7 {
        ps.record_fight_kill(300);
    }
    assert_eq!(ps.current_multi_kill(), Some(MultiKill::Penta));
}

#[test]
fn reset_fight_kills_clears_multi_kill() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_fight_kill(300);
    ps.record_fight_kill(300);
    assert_eq!(ps.current_multi_kill(), Some(MultiKill::Double));
    ps.reset_fight_kills();
    assert_eq!(ps.current_multi_kill(), None);
}

#[test]
fn fight_kill_also_increments_regular_kill_and_streak() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_fight_kill(300);
    assert_eq!(ps.kills(), 1);
    assert_eq!(ps.kill_streak(), 1);
}

// ---------------------------------------------------------------------------
// Killing spree labels
// ---------------------------------------------------------------------------

#[test]
fn killing_spree_at_three_kills() {
    let mut ps = MatchPlayerState::new(0);
    for _ in 0..3 {
        ps.record_kill(300);
    }
    assert_eq!(ps.spree_label(), Some("Killing Spree"));
}

#[test]
fn rampage_at_four_kills() {
    let mut ps = MatchPlayerState::new(0);
    for _ in 0..4 {
        ps.record_kill(300);
    }
    assert_eq!(ps.spree_label(), Some("Rampage"));
}

#[test]
fn unstoppable_at_five_kills() {
    let mut ps = MatchPlayerState::new(0);
    for _ in 0..5 {
        ps.record_kill(300);
    }
    assert_eq!(ps.spree_label(), Some("Unstoppable"));
}

#[test]
fn godlike_at_eight_kills() {
    let mut ps = MatchPlayerState::new(0);
    for _ in 0..8 {
        ps.record_kill(300);
    }
    assert_eq!(ps.spree_label(), Some("Godlike"));
}

#[test]
fn legendary_at_ten_kills() {
    let mut ps = MatchPlayerState::new(0);
    for _ in 0..10 {
        ps.record_kill(300);
    }
    assert_eq!(ps.spree_label(), Some("Legendary"));
}

#[test]
fn no_spree_label_below_three() {
    let mut ps = MatchPlayerState::new(0);
    ps.record_kill(300);
    ps.record_kill(300);
    assert_eq!(ps.spree_label(), None);
}
