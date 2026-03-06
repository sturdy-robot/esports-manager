use esm_engine::draft::{Draft, DraftAction, DraftFormat};
use esm_engine::draft_ai::AutoDraftAI;
use esm_engine::match_sim::TeamSide;

#[test]
fn draft_ai_returns_none_when_not_turn() {
    let draft = Draft::new(DraftFormat::ThreeBan); // Blue ban first
    let ai = AutoDraftAI::new();
    let champs = vec!["Ahri".to_string(), "Zed".to_string()];
    
    // Red team tries to act, but it's Blue's turn
    let action = ai.decide_action(&draft, TeamSide::Red, &champs);
    assert!(action.is_none());
}

#[test]
fn draft_ai_picks_first_available_champion() {
    let draft = Draft::new(DraftFormat::ThreeBan); // Blue ban first
    let ai = AutoDraftAI::new();
    let champs = vec!["Ahri".to_string(), "Zed".to_string()];
    
    let action = ai.decide_action(&draft, TeamSide::Blue, &champs).unwrap();
    match action {
        DraftAction::SelectChampion(name) => assert_eq!(name, "Ahri"),
        _ => panic!("Expected SelectChampion action"),
    }
}

#[test]
fn draft_ai_skips_selected_champions() {
    let mut draft = Draft::new(DraftFormat::ThreeBan); // Blue ban first
    draft.apply_action(DraftAction::SelectChampion("Ahri".to_string())).unwrap();
    
    // Now it's Blue's turn again (second ban)
    let ai = AutoDraftAI::new();
    let champs = vec!["Ahri".to_string(), "Zed".to_string()];
    
    let action = ai.decide_action(&draft, TeamSide::Blue, &champs).unwrap();
    match action {
        DraftAction::SelectChampion(name) => assert_eq!(name, "Zed"), // Skipped Ahri
        _ => panic!("Expected SelectChampion action"),
    }
}

#[test]
fn draft_ai_returns_none_when_draft_complete() {
    let mut draft = Draft::new(DraftFormat::ThreeBan);
    let champs: Vec<String> = (0..16).map(|i| format!("Champ{i}")).collect();
    for champ in &champs {
        draft.apply_action(DraftAction::SelectChampion(champ.clone())).unwrap();
    }
    
    let ai = AutoDraftAI::new();
    // It's nobody's turn since draft is complete
    let action = ai.decide_action(&draft, TeamSide::Blue, &champs);
    assert!(action.is_none());
}
