use esm_engine::draft::DraftFormat;
use esm_engine::draft_session::DraftSession;
use esm_engine::match_sim::TeamSide;

fn sample_champions() -> Vec<String> {
    (0..20).map(|i| format!("Champ{i}")).collect()
}

#[test]
fn ai_does_not_act_on_player_turn() {
    let mut session = DraftSession::new(DraftFormat::ThreeBan, TeamSide::Blue, sample_champions());
    // Blue goes first, so AI (opponent) should not act
    let acted = session.ai_act();
    assert!(!acted);
}

#[test]
fn ai_acts_on_opponent_turn() {
    let mut session = DraftSession::new(DraftFormat::ThreeBan, TeamSide::Red, sample_champions());
    // Blue goes first; player is Red, so AI should act for Blue
    let acted = session.ai_act();
    assert!(acted);
    assert_eq!(session.state().current_step, 1);
}

#[test]
fn ai_skips_already_selected_champions() {
    let mut session = DraftSession::new(DraftFormat::ThreeBan, TeamSide::Red, sample_champions());
    // Run 3 AI bans for Blue
    let count = session.run_ai_turns();
    assert_eq!(count, 3);
    // All 3 bans should be different champions
    let bans = &session.state().blue_bans;
    assert_eq!(bans.len(), 3);
    assert_ne!(bans[0], bans[1]);
    assert_ne!(bans[1], bans[2]);
    assert_ne!(bans[0], bans[2]);
}

#[test]
fn ai_does_not_act_when_draft_complete() {
    let mut session = DraftSession::new(DraftFormat::ThreeBan, TeamSide::Blue, sample_champions());
    // Complete the entire draft
    while !session.is_complete() {
        if session.state().is_player_turn {
            let champ = session.state().available_champions[0].clone();
            session.hover(champ).unwrap();
            session.lock().unwrap();
        } else {
            session.ai_act();
        }
    }
    let acted = session.ai_act();
    assert!(!acted);
}
