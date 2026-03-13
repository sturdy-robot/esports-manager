use esm_core::rng::GameRng;

use super::event::MatchEvent;
use super::game_state::MatchGameState;

// ---------------------------------------------------------------------------
// SimEvent trait
// ---------------------------------------------------------------------------

/// Each simulation event is an independent object that knows:
/// - Whether it is enabled given the current game state.
/// - Its probability of occurring.
/// - How to process itself (mutate game state + return a MatchEvent).
pub trait SimEvent {
    /// A short identifier for logging/debugging.
    fn name(&self) -> &'static str;

    /// Is this event eligible to fire given the current game state?
    fn is_enabled(&self, state: &MatchGameState) -> bool;

    /// Relative weight of this event occurring (not necessarily 0..1).
    /// Higher weight = more likely to be picked among enabled events.
    fn weight(&self, state: &MatchGameState) -> f64;

    /// Execute the event: mutate game state, return the resulting MatchEvent.
    fn process(&self, state: &mut MatchGameState, rng: &mut GameRng) -> MatchEvent;
}
