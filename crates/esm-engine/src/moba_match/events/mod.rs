use esm_core::rng::GameRng;

use super::game_state::MatchGameState;
pub use super::sim_event::SimEvent;
use super::tactics::MatchTactics;

mod baron_fight;
mod dragon_fight;
mod farm_tick;
mod herald_fight;
mod inhib_siege;
mod nexus_siege;
mod solo_kill;
mod teamfight;
mod tower_siege;

pub use baron_fight::BaronFight;
pub use dragon_fight::DragonFight;
pub use farm_tick::FarmTick;
pub use herald_fight::HeraldFight;
pub use inhib_siege::InhibSiege;
pub use nexus_siege::NexusSiege;
pub use solo_kill::SoloKill;
pub use teamfight::Teamfight;
pub use tower_siege::TowerSiege;

// ---------------------------------------------------------------------------
// Event registry: all events in one place
// ---------------------------------------------------------------------------

/// Returns a Vec of all simulation events.
pub fn all_events() -> Vec<Box<dyn SimEvent>> {
    vec![
        Box::new(FarmTick),
        Box::new(SoloKill),
        Box::new(Teamfight),
        Box::new(TowerSiege),
        Box::new(DragonFight),
        Box::new(HeraldFight),
        Box::new(BaronFight),
        Box::new(InhibSiege),
        Box::new(NexusSiege),
    ]
}

/// Given the current game state, return enabled events with their weights.
pub fn enabled_events(state: &MatchGameState, events: &[Box<dyn SimEvent>]) -> Vec<(usize, f64)> {
    events
        .iter()
        .enumerate()
        .filter(|(_, e)| e.is_enabled(state))
        .map(|(i, e)| (i, e.weight(state)))
        .filter(|(_, w)| *w > 0.0)
        .collect()
}

/// Return the tactical weight multiplier for a given event name.
fn tactical_mult(name: &str, tactics: &MatchTactics) -> f64 {
    match name {
        "FarmTick" => tactics.farm_mult(),
        "SoloKill" => tactics.solo_kill_mult(),
        "Teamfight" => tactics.teamfight_mult(),
        "TowerSiege" | "InhibSiege" | "NexusSiege" => tactics.tower_mult(),
        "DragonFight" | "HeraldFight" | "BaronFight" => tactics.objective_mult(),
        _ => 1.0,
    }
}

/// Like `enabled_events` but applies tactical weight multipliers.
pub fn enabled_events_with_tactics(
    state: &MatchGameState,
    events: &[Box<dyn SimEvent>],
    tactics: &MatchTactics,
) -> Vec<(usize, f64)> {
    events
        .iter()
        .enumerate()
        .filter(|(_, e)| e.is_enabled(state))
        .map(|(i, e)| {
            let base = e.weight(state);
            let mult = tactical_mult(e.name(), tactics);
            (i, base * mult)
        })
        .filter(|(_, w)| *w > 0.0)
        .collect()
}

/// Pick an event index from the enabled events using weighted random selection.
pub fn pick_event(rng: &mut GameRng, enabled: &[(usize, f64)]) -> usize {
    let total_weight: f64 = enabled.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return enabled[0].0;
    }

    let roll = rng.range_u32(0, 10000) as f64 / 10000.0 * total_weight;
    let mut cumulative = 0.0;
    for &(idx, weight) in enabled {
        cumulative += weight;
        if roll <= cumulative {
            return idx;
        }
    }
    // Fallback to last
    enabled.last().unwrap().0
}
