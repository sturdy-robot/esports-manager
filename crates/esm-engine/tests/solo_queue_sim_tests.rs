use esm_core::rng::GameRng;
use esm_engine::schedule::solo_queue::{SoloQueueResult, SoloQueueSimulator};
use esm_engine::schedule::SoloQueueFocus;

// ---------------------------------------------------------------------------
// SoloQueueSimulator generates pseudo-game results
// ---------------------------------------------------------------------------

#[test]
fn simulator_generates_result_with_correct_game_count() {
    let mut rng = GameRng::from_seed(42);
    let skill = 60; // player overall skill 0-100
    let result = SoloQueueSimulator::generate_result(&mut rng, skill, SoloQueueFocus::Mechanics);
    assert!(result.games_played() >= 3);
    assert!(result.games_played() <= 5);
}

#[test]
fn simulator_wins_plus_losses_equals_games_played() {
    let mut rng = GameRng::from_seed(123);
    let result = SoloQueueSimulator::generate_result(&mut rng, 50, SoloQueueFocus::Champions);
    assert_eq!(result.wins() + result.losses(), result.games_played());
}

#[test]
fn simulator_high_skill_wins_more_often() {
    // Run many simulations; a high-skill player should average more wins
    let mut high_wins_total = 0u32;
    let mut low_wins_total = 0u32;
    let iterations = 100;

    for i in 0..iterations {
        let mut rng_high = GameRng::from_seed(1000 + i);
        let mut rng_low = GameRng::from_seed(1000 + i);
        let high = SoloQueueSimulator::generate_result(&mut rng_high, 90, SoloQueueFocus::Mechanics);
        let low = SoloQueueSimulator::generate_result(&mut rng_low, 20, SoloQueueFocus::Mechanics);
        high_wins_total += high.wins();
        low_wins_total += low.wins();
    }

    assert!(
        high_wins_total > low_wins_total,
        "High skill ({high_wins_total} wins) should beat low skill ({low_wins_total} wins) over {iterations} iterations"
    );
}

#[test]
fn simulator_experience_is_positive() {
    let mut rng = GameRng::from_seed(77);
    let result = SoloQueueSimulator::generate_result(&mut rng, 50, SoloQueueFocus::Tactics);
    assert!(result.experience_gained() > 0);
}

#[test]
fn simulator_is_deterministic() {
    let mut rng1 = GameRng::from_seed(42);
    let mut rng2 = GameRng::from_seed(42);
    let r1 = SoloQueueSimulator::generate_result(&mut rng1, 60, SoloQueueFocus::Champions);
    let r2 = SoloQueueSimulator::generate_result(&mut rng2, 60, SoloQueueFocus::Champions);
    assert_eq!(r1.wins(), r2.wins());
    assert_eq!(r1.losses(), r2.losses());
    assert_eq!(r1.experience_gained(), r2.experience_gained());
}

#[test]
fn simulator_experience_scales_with_games_played() {
    // More games should mean more experience (within same focus)
    let mut rng = GameRng::from_seed(42);
    let result = SoloQueueSimulator::generate_result(&mut rng, 50, SoloQueueFocus::Mechanics);
    // Experience should be at least 1 per game
    assert!(result.experience_gained() >= result.games_played());
}
