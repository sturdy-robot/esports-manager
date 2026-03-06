use crate::draft::{Draft, DraftAction, DraftPhase};
use crate::match_sim::TeamSide;

pub struct AutoDraftAI;

impl AutoDraftAI {
    pub fn new() -> Self {
        Self
    }

    /// Evaluates the current state of a draft and produces a Selection Action if it is our turn.
    /// Returns None if it is not our turn or if no valid champions remain.
    pub fn decide_action(
        &self,
        draft: &Draft,
        my_team: TeamSide,
        available_champions: &[String],
    ) -> Option<DraftAction> {
        let slot = draft.current_slot()?;
        
        if slot.team != my_team {
            return None; // Not our turn
        }

        let selected = draft.all_selected();
        for champ in available_champions {
            if !selected.contains(champ) {
                return Some(DraftAction::SelectChampion(champ.clone()));
            }
        }

        None
    }
}

impl Default for AutoDraftAI {
    fn default() -> Self {
        Self::new()
    }
}
