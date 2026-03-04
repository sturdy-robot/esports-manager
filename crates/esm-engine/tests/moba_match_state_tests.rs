use esm_engine::moba_match::map::{
    Inhibitor, Lane, MapState, ObjectiveState, Tower, TowerTier,
};
use esm_engine::moba_match::state::TeamSide;

// ---------------------------------------------------------------------------
// Lane
// ---------------------------------------------------------------------------

#[test]
fn lane_variants_exist() {
    let _ = Lane::Top;
    let _ = Lane::Mid;
    let _ = Lane::Bot;
}

// ---------------------------------------------------------------------------
// TowerTier
// ---------------------------------------------------------------------------

#[test]
fn tower_tier_variants_exist() {
    let _ = TowerTier::Outer;
    let _ = TowerTier::Inner;
    let _ = TowerTier::Inhibitor;
    let _ = TowerTier::Nexus;
}

// ---------------------------------------------------------------------------
// Tower
// ---------------------------------------------------------------------------

#[test]
fn tower_starts_standing() {
    let t = Tower::new(Lane::Mid, TowerTier::Outer);
    assert!(t.is_standing());
    assert!(!t.is_destroyed());
}

#[test]
fn tower_can_be_destroyed() {
    let mut t = Tower::new(Lane::Mid, TowerTier::Outer);
    t.destroy();
    assert!(t.is_destroyed());
    assert!(!t.is_standing());
}

#[test]
fn tower_stores_lane_and_tier() {
    let t = Tower::new(Lane::Top, TowerTier::Inner);
    assert_eq!(t.lane(), Lane::Top);
    assert_eq!(t.tier(), TowerTier::Inner);
}

// ---------------------------------------------------------------------------
// Inhibitor
// ---------------------------------------------------------------------------

#[test]
fn inhibitor_starts_standing() {
    let inh = Inhibitor::new(Lane::Bot);
    assert!(inh.is_standing());
    assert!(!inh.is_destroyed());
}

#[test]
fn inhibitor_can_be_destroyed_with_respawn_timer() {
    let mut inh = Inhibitor::new(Lane::Bot);
    inh.destroy(300); // 5 minutes respawn
    assert!(inh.is_destroyed());
    assert_eq!(inh.respawn_timer(), Some(300));
}

#[test]
fn inhibitor_respawn_timer_ticks_down() {
    let mut inh = Inhibitor::new(Lane::Mid);
    inh.destroy(60);
    inh.tick(30);
    assert_eq!(inh.respawn_timer(), Some(30));
    assert!(inh.is_destroyed());
}

#[test]
fn inhibitor_respawns_when_timer_reaches_zero() {
    let mut inh = Inhibitor::new(Lane::Mid);
    inh.destroy(60);
    inh.tick(60);
    assert!(inh.is_standing());
    assert_eq!(inh.respawn_timer(), None);
}

// ---------------------------------------------------------------------------
// MapState — initial state
// ---------------------------------------------------------------------------

#[test]
fn map_state_starts_with_all_towers_standing() {
    let map = MapState::new();
    // Each side has 3 lanes × 3 tiers (Outer, Inner, Inhib) + 2 nexus towers = 11
    assert_eq!(map.towers(TeamSide::Blue).len(), 11);
    assert_eq!(map.towers(TeamSide::Red).len(), 11);
    for t in map.towers(TeamSide::Blue) {
        assert!(t.is_standing());
    }
}

#[test]
fn map_state_starts_with_all_inhibitors_standing() {
    let map = MapState::new();
    assert_eq!(map.inhibitors(TeamSide::Blue).len(), 3);
    assert_eq!(map.inhibitors(TeamSide::Red).len(), 3);
    for inh in map.inhibitors(TeamSide::Blue) {
        assert!(inh.is_standing());
    }
}

// ---------------------------------------------------------------------------
// MapState — tower vulnerability (must destroy outer before inner, etc)
// ---------------------------------------------------------------------------

#[test]
fn map_state_outer_tower_is_vulnerable_initially() {
    let map = MapState::new();
    assert!(map.is_tower_vulnerable(TeamSide::Blue, Lane::Mid, TowerTier::Outer));
}

#[test]
fn map_state_inner_tower_not_vulnerable_while_outer_stands() {
    let map = MapState::new();
    assert!(!map.is_tower_vulnerable(TeamSide::Blue, Lane::Mid, TowerTier::Inner));
}

#[test]
fn map_state_inner_tower_becomes_vulnerable_after_outer_destroyed() {
    let mut map = MapState::new();
    map.destroy_tower(TeamSide::Blue, Lane::Mid, TowerTier::Outer);
    assert!(map.is_tower_vulnerable(TeamSide::Blue, Lane::Mid, TowerTier::Inner));
}

#[test]
fn map_state_inhibitor_tower_vulnerable_after_inner_destroyed() {
    let mut map = MapState::new();
    map.destroy_tower(TeamSide::Blue, Lane::Mid, TowerTier::Outer);
    map.destroy_tower(TeamSide::Blue, Lane::Mid, TowerTier::Inner);
    assert!(map.is_tower_vulnerable(TeamSide::Blue, Lane::Mid, TowerTier::Inhibitor));
}

// ---------------------------------------------------------------------------
// MapState — inhibitor vulnerability
// ---------------------------------------------------------------------------

#[test]
fn map_state_inhibitor_not_vulnerable_while_inhib_tower_stands() {
    let map = MapState::new();
    assert!(!map.is_inhibitor_vulnerable(TeamSide::Blue, Lane::Mid));
}

#[test]
fn map_state_inhibitor_vulnerable_after_inhib_tower_destroyed() {
    let mut map = MapState::new();
    map.destroy_tower(TeamSide::Blue, Lane::Mid, TowerTier::Outer);
    map.destroy_tower(TeamSide::Blue, Lane::Mid, TowerTier::Inner);
    map.destroy_tower(TeamSide::Blue, Lane::Mid, TowerTier::Inhibitor);
    assert!(map.is_inhibitor_vulnerable(TeamSide::Blue, Lane::Mid));
}

// ---------------------------------------------------------------------------
// MapState — nexus vulnerability
// ---------------------------------------------------------------------------

#[test]
fn map_state_nexus_not_vulnerable_while_all_inhibitors_stand() {
    let map = MapState::new();
    assert!(!map.is_nexus_vulnerable(TeamSide::Blue));
}

#[test]
fn map_state_nexus_vulnerable_when_any_inhibitor_destroyed() {
    let mut map = MapState::new();
    // Destroy full mid lane
    map.destroy_tower(TeamSide::Blue, Lane::Mid, TowerTier::Outer);
    map.destroy_tower(TeamSide::Blue, Lane::Mid, TowerTier::Inner);
    map.destroy_tower(TeamSide::Blue, Lane::Mid, TowerTier::Inhibitor);
    map.destroy_inhibitor(TeamSide::Blue, Lane::Mid, 300);
    assert!(map.is_nexus_vulnerable(TeamSide::Blue));
}

// ---------------------------------------------------------------------------
// ObjectiveState
// ---------------------------------------------------------------------------

#[test]
fn objective_state_initial() {
    let obj = ObjectiveState::new();
    assert_eq!(obj.dragon_count(TeamSide::Blue), 0);
    assert_eq!(obj.dragon_count(TeamSide::Red), 0);
    assert!(!obj.baron_alive());
    assert!(obj.herald_available()); // Herald spawns at 8 min, not tracked by time here
    assert!(!obj.elder_available());
}

#[test]
fn objective_state_add_dragon() {
    let mut obj = ObjectiveState::new();
    obj.add_dragon(TeamSide::Blue);
    assert_eq!(obj.dragon_count(TeamSide::Blue), 1);
    assert_eq!(obj.dragon_count(TeamSide::Red), 0);
}

#[test]
fn objective_state_dragon_soul_at_4() {
    let mut obj = ObjectiveState::new();
    for _ in 0..4 {
        obj.add_dragon(TeamSide::Blue);
    }
    assert!(obj.has_dragon_soul(TeamSide::Blue));
    assert!(!obj.has_dragon_soul(TeamSide::Red));
}

#[test]
fn objective_state_elder_available_after_soul() {
    let mut obj = ObjectiveState::new();
    for _ in 0..4 {
        obj.add_dragon(TeamSide::Blue);
    }
    assert!(obj.elder_available());
}

#[test]
fn objective_state_baron_spawn_and_kill() {
    let mut obj = ObjectiveState::new();
    obj.spawn_baron();
    assert!(obj.baron_alive());
    obj.kill_baron(TeamSide::Red);
    assert!(!obj.baron_alive());
    assert!(obj.baron_buff(TeamSide::Red));
}

#[test]
fn objective_state_herald_consumed_after_use() {
    let mut obj = ObjectiveState::new();
    assert!(obj.herald_available());
    obj.consume_herald();
    assert!(!obj.herald_available());
}
