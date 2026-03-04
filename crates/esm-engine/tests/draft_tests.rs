use esm_engine::draft::{Draft, DraftAction, DraftFormat, DraftPhase};
use esm_engine::match_sim::TeamSide;

// ---------------------------------------------------------------------------
// DraftFormat
// ---------------------------------------------------------------------------

#[test]
fn draft_format_variants_exist() {
    let _ = DraftFormat::ThreeBan;
    let _ = DraftFormat::FiveBan;
    let _ = DraftFormat::Fearless;
}

#[test]
fn three_ban_has_correct_sequence_length() {
    let format = DraftFormat::ThreeBan;
    assert_eq!(format.sequence().len(), 16); // 6 bans + 10 picks
}

#[test]
fn five_ban_has_correct_sequence_length() {
    let format = DraftFormat::FiveBan;
    assert_eq!(format.sequence().len(), 20); // 10 bans + 10 picks
}

// ---------------------------------------------------------------------------
// DraftPhase
// ---------------------------------------------------------------------------

#[test]
fn draft_phase_variants_exist() {
    let _ = DraftPhase::Ban;
    let _ = DraftPhase::Pick;
}

// ---------------------------------------------------------------------------
// Draft state machine
// ---------------------------------------------------------------------------

#[test]
fn draft_starts_at_step_zero() {
    let draft = Draft::new(DraftFormat::ThreeBan);
    assert_eq!(draft.current_step(), 0);
    assert!(!draft.is_complete());
}

#[test]
fn draft_current_slot_returns_first_action() {
    let draft = Draft::new(DraftFormat::ThreeBan);
    let slot = draft.current_slot().unwrap();
    assert_eq!(slot.phase, DraftPhase::Ban);
    assert_eq!(slot.team, TeamSide::Blue);
}

#[test]
fn draft_apply_action_advances_step() {
    let mut draft = Draft::new(DraftFormat::ThreeBan);
    let result = draft.apply_action(DraftAction::SelectChampion("Orianna".to_string()));
    assert!(result.is_ok());
    assert_eq!(draft.current_step(), 1);
}

#[test]
fn draft_apply_action_records_selection() {
    let mut draft = Draft::new(DraftFormat::ThreeBan);
    draft
        .apply_action(DraftAction::SelectChampion("Orianna".to_string()))
        .unwrap();
    assert!(draft.blue_bans().contains(&"Orianna".to_string()));
}

#[test]
fn draft_rejects_duplicate_champion() {
    let mut draft = Draft::new(DraftFormat::ThreeBan);
    draft
        .apply_action(DraftAction::SelectChampion("Orianna".to_string()))
        .unwrap();
    // Next slot is Red ban — try to ban the same champion
    let result = draft.apply_action(DraftAction::SelectChampion("Orianna".to_string()));
    assert!(result.is_err());
}

#[test]
fn draft_completes_after_all_steps() {
    let mut draft = Draft::new(DraftFormat::ThreeBan);
    let champs: Vec<String> = (0..16).map(|i| format!("Champ{i}")).collect();
    for name in &champs {
        draft
            .apply_action(DraftAction::SelectChampion(name.clone()))
            .unwrap();
    }
    assert!(draft.is_complete());
    assert!(draft.current_slot().is_none());
}

#[test]
fn draft_blue_picks_has_five_after_complete() {
    let mut draft = Draft::new(DraftFormat::ThreeBan);
    let champs: Vec<String> = (0..16).map(|i| format!("Champ{i}")).collect();
    for name in &champs {
        draft
            .apply_action(DraftAction::SelectChampion(name.clone()))
            .unwrap();
    }
    assert_eq!(draft.blue_picks().len(), 5);
    assert_eq!(draft.red_picks().len(), 5);
}

#[test]
fn draft_bans_are_recorded_correctly() {
    let mut draft = Draft::new(DraftFormat::ThreeBan);
    let champs: Vec<String> = (0..16).map(|i| format!("Champ{i}")).collect();
    for name in &champs {
        draft
            .apply_action(DraftAction::SelectChampion(name.clone()))
            .unwrap();
    }
    assert_eq!(draft.blue_bans().len(), 3);
    assert_eq!(draft.red_bans().len(), 3);
}

#[test]
fn draft_all_selected_contains_all_champions() {
    let mut draft = Draft::new(DraftFormat::ThreeBan);
    let champs: Vec<String> = (0..16).map(|i| format!("Champ{i}")).collect();
    for name in &champs {
        draft
            .apply_action(DraftAction::SelectChampion(name.clone()))
            .unwrap();
    }
    let all = draft.all_selected();
    assert_eq!(all.len(), 16);
    for name in &champs {
        assert!(all.contains(name));
    }
}

#[test]
fn draft_action_after_complete_fails() {
    let mut draft = Draft::new(DraftFormat::ThreeBan);
    let champs: Vec<String> = (0..16).map(|i| format!("Champ{i}")).collect();
    for name in &champs {
        draft
            .apply_action(DraftAction::SelectChampion(name.clone()))
            .unwrap();
    }
    let result = draft.apply_action(DraftAction::SelectChampion("Extra".to_string()));
    assert!(result.is_err());
}
