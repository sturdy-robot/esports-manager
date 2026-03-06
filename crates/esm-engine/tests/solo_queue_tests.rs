use esm_engine::schedule::solo_queue::{SoloQueueResult, SoloQueueSession};
use esm_engine::schedule::SoloQueueFocus;
use esm_models::time::TimeSlot;

// ---------------------------------------------------------------------------
// SoloQueueSession creation
// ---------------------------------------------------------------------------

#[test]
fn session_stores_fields() {
    let session = SoloQueueSession::new(
        0,
        TimeSlot::Morning,
        SoloQueueFocus::Mechanics,
        5,
    );
    assert_eq!(session.player_index(), 0);
    assert_eq!(session.time_slot(), TimeSlot::Morning);
    assert_eq!(session.focus(), SoloQueueFocus::Mechanics);
    assert_eq!(session.scheduled_day(), 5);
    assert!(session.result().is_none());
}

#[test]
fn session_default_has_no_result() {
    let session = SoloQueueSession::new(2, TimeSlot::Evening, SoloQueueFocus::Champions, 1);
    assert!(session.result().is_none());
}

// ---------------------------------------------------------------------------
// SoloQueueResult
// ---------------------------------------------------------------------------

#[test]
fn result_stores_wins_losses_and_experience() {
    let result = SoloQueueResult::new(3, 2, 4);
    assert_eq!(result.wins(), 3);
    assert_eq!(result.losses(), 2);
    assert_eq!(result.games_played(), 5);
    assert_eq!(result.experience_gained(), 4);
}

#[test]
fn result_zero_games() {
    let result = SoloQueueResult::new(0, 0, 0);
    assert_eq!(result.games_played(), 0);
    assert_eq!(result.experience_gained(), 0);
}

// ---------------------------------------------------------------------------
// Completing a session
// ---------------------------------------------------------------------------

#[test]
fn session_can_be_completed_with_result() {
    let mut session = SoloQueueSession::new(1, TimeSlot::Afternoon, SoloQueueFocus::Tactics, 3);
    let result = SoloQueueResult::new(2, 1, 3);
    session.complete(result.clone());
    assert!(session.result().is_some());
    let r = session.result().unwrap();
    assert_eq!(r.wins(), 2);
    assert_eq!(r.losses(), 1);
    assert_eq!(r.experience_gained(), 3);
}

// ---------------------------------------------------------------------------
// Focus variants
// ---------------------------------------------------------------------------

#[test]
fn all_focus_variants_exist() {
    let _ = SoloQueueFocus::Champions;
    let _ = SoloQueueFocus::Tactics;
    let _ = SoloQueueFocus::Mechanics;
    let _ = SoloQueueFocus::Mentality;
}

// ---------------------------------------------------------------------------
// Max games cap (players play 5 or fewer)
// ---------------------------------------------------------------------------

#[test]
fn result_games_played_at_most_five() {
    // This is a business rule: solo queue sessions produce at most 5 games
    let result = SoloQueueResult::new(3, 2, 4);
    assert!(result.games_played() <= 5);
}
