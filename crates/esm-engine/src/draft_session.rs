use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::draft::{Draft, DraftAction, DraftError, DraftFormat, DraftPhase};
use crate::match_sim::TeamSide;
use crate::patch::Patch;
use esm_ai::draft_ai::{ChampionEval, DraftAi};
use esm_core::rng::GameRng;
use esm_models::champion::MasteryLevel;

// ---------------------------------------------------------------------------
// DraftSession — orchestrates a draft with AI opponent support
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftSessionState {
    pub current_step: usize,
    pub total_steps: usize,
    pub current_phase: Option<DraftPhase>,
    pub current_team: Option<TeamSide>,
    pub blue_bans: Vec<String>,
    pub red_bans: Vec<String>,
    pub blue_picks: Vec<String>,
    pub red_picks: Vec<String>,
    pub active_hover: Option<String>,
    pub is_complete: bool,
    pub is_player_turn: bool,
    pub available_champions: Vec<String>,
    pub timer_seconds: u32,
}

const DEFAULT_TIMER_SECONDS: u32 = 30;

pub struct DraftSession {
    draft: Draft,
    player_team: TeamSide,
    champion_pool: Vec<String>,
    champion_evals: Vec<ChampionEval>,
    rng: GameRng,
    timer_seconds: u32,
}

/// Build default (uniform) ChampionEval entries for each champion name.
fn default_champion_evals(champions: &[String]) -> Vec<ChampionEval> {
    champions
        .iter()
        .map(|name| ChampionEval {
            champion_name: name.clone(),
            meta_strength: 1.0,
            player_mastery: MasteryLevel::Gold,
            composition_synergy: 0.0,
            counter_matchup: 0.0,
        })
        .collect()
}

/// Build ChampionEval entries using real patch data and player mastery info.
///
/// `patch` provides the meta tier (S/A/B/C/D) → meta_strength multiplier.
/// `mastery_map` maps champion name → best MasteryLevel among the AI team's
/// players for that champion. Champions not in the map default to Bronze.
pub fn build_champion_evals(
    champions: &[String],
    patch: &Patch,
    mastery_map: &HashMap<String, MasteryLevel>,
) -> Vec<ChampionEval> {
    champions
        .iter()
        .map(|name| {
            let meta_strength = patch.tier_for_or_default(name).multiplier();
            let player_mastery = mastery_map
                .get(name)
                .copied()
                .unwrap_or(MasteryLevel::Bronze);
            ChampionEval {
                champion_name: name.clone(),
                meta_strength,
                player_mastery,
                composition_synergy: 0.0,
                counter_matchup: 0.0,
            }
        })
        .collect()
}

impl DraftSession {
    pub fn new(format: DraftFormat, player_team: TeamSide, champion_pool: Vec<String>) -> Self {
        let evals = default_champion_evals(&champion_pool);
        Self {
            draft: Draft::new(format),
            player_team,
            champion_pool,
            champion_evals: evals,
            rng: GameRng::from_seed(42),
            timer_seconds: DEFAULT_TIMER_SECONDS,
        }
    }

    /// Create a session with custom RNG seed and champion evaluations.
    pub fn with_evals(
        format: DraftFormat,
        player_team: TeamSide,
        champion_pool: Vec<String>,
        champion_evals: Vec<ChampionEval>,
        rng: GameRng,
    ) -> Self {
        Self {
            draft: Draft::new(format),
            player_team,
            champion_pool,
            champion_evals,
            rng,
            timer_seconds: DEFAULT_TIMER_SECONDS,
        }
    }

    pub fn set_timer_seconds(&mut self, seconds: u32) {
        self.timer_seconds = seconds;
    }

    pub fn state(&self) -> DraftSessionState {
        let slot = self.draft.current_slot();
        let is_player_turn = slot.map_or(false, |s| s.team == self.player_team);
        let selected = self.draft.all_selected();
        let available: Vec<String> = self
            .champion_pool
            .iter()
            .filter(|c| !selected.contains(*c))
            .cloned()
            .collect();

        DraftSessionState {
            current_step: self.draft.current_step(),
            total_steps: self.draft.format().sequence().len(),
            current_phase: slot.map(|s| s.phase),
            current_team: slot.map(|s| s.team),
            blue_bans: self.draft.blue_bans().to_vec(),
            red_bans: self.draft.red_bans().to_vec(),
            blue_picks: self.draft.blue_picks().to_vec(),
            red_picks: self.draft.red_picks().to_vec(),
            active_hover: self.draft.active_hover().map(|s| s.to_string()),
            is_complete: self.draft.is_complete(),
            is_player_turn,
            available_champions: available,
            timer_seconds: self.timer_seconds,
        }
    }

    /// Player hovers a champion
    pub fn hover(&mut self, champion: String) -> Result<(), DraftError> {
        if !self.is_player_turn() {
            return Err(DraftError::DraftComplete); // reuse for "not your turn"
        }
        self.draft
            .apply_action(DraftAction::HoverChampion(champion))
    }

    /// Player locks in their current hover
    pub fn lock(&mut self) -> Result<(), DraftError> {
        if !self.is_player_turn() {
            return Err(DraftError::DraftComplete);
        }
        self.draft.apply_action(DraftAction::LockChampion)
    }

    /// Execute the AI's turn. Returns true if AI acted, false if it's not AI's turn.
    pub fn ai_act(&mut self) -> bool {
        if self.draft.is_complete() {
            return false;
        }
        let slot = match self.draft.current_slot() {
            Some(s) => *s,
            None => return false,
        };
        if slot.team == self.player_team {
            return false; // Player's turn, not AI
        }

        self.ai_pick()
    }

    /// Run all consecutive AI turns until it's the player's turn or draft is complete
    pub fn run_ai_turns(&mut self) -> usize {
        let mut count = 0;
        while self.ai_act() {
            count += 1;
        }
        count
    }

    /// Force AI to act for the current team regardless of player_team (used in spectate/auto-complete)
    pub fn force_ai_act(&mut self) -> bool {
        if self.draft.is_complete() {
            return false;
        }
        if self.draft.current_slot().is_none() {
            return false;
        }
        self.ai_pick()
    }

    /// Internal: use DraftAi to select a champion, then hover + lock it.
    fn ai_pick(&mut self) -> bool {
        let selected = self.draft.all_selected();
        let candidates: Vec<ChampionEval> = self
            .champion_evals
            .iter()
            .filter(|e| !selected.contains(&e.champion_name))
            .cloned()
            .collect();
        if candidates.is_empty() {
            return false;
        }
        let chosen = DraftAi::select_champion(&mut self.rng, &candidates);
        if self
            .draft
            .apply_action(DraftAction::HoverChampion(chosen))
            .is_err()
        {
            return false;
        }
        let _ = self.draft.apply_action(DraftAction::LockChampion);
        true
    }

    pub fn is_complete(&self) -> bool {
        self.draft.is_complete()
    }

    pub fn draft(&self) -> &Draft {
        &self.draft
    }

    fn is_player_turn(&self) -> bool {
        self.draft
            .current_slot()
            .map_or(false, |s| s.team == self.player_team)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_champions() -> Vec<String> {
        vec![
            "Orianna",
            "Azir",
            "Ahri",
            "Syndra",
            "Zed",
            "Malphite",
            "Ornn",
            "Gnar",
            "Fiora",
            "Jayce",
            "Lee Sin",
            "Viego",
            "Jarvan IV",
            "Jinx",
            "Kai'Sa",
            "Ezreal",
            "Aphelios",
            "Thresh",
            "Nautilus",
            "Lulu",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn new_session_starts_at_step_zero() {
        let session = DraftSession::new(DraftFormat::FiveBan, TeamSide::Blue, sample_champions());
        let state = session.state();
        assert_eq!(state.current_step, 0);
        assert!(!state.is_complete);
        assert_eq!(state.total_steps, 20);
    }

    #[test]
    fn state_includes_timer_seconds() {
        let session = DraftSession::new(DraftFormat::FiveBan, TeamSide::Blue, sample_champions());
        let state = session.state();
        assert_eq!(state.timer_seconds, 30);
    }

    #[test]
    fn custom_timer_seconds() {
        let mut session =
            DraftSession::new(DraftFormat::FiveBan, TeamSide::Blue, sample_champions());
        session.set_timer_seconds(45);
        assert_eq!(session.state().timer_seconds, 45);
    }

    #[test]
    fn player_on_blue_has_first_turn() {
        let session = DraftSession::new(DraftFormat::FiveBan, TeamSide::Blue, sample_champions());
        let state = session.state();
        assert!(state.is_player_turn);
        assert_eq!(state.current_team, Some(TeamSide::Blue));
        assert_eq!(state.current_phase, Some(DraftPhase::Ban));
    }

    #[test]
    fn player_on_red_does_not_have_first_turn() {
        let session = DraftSession::new(DraftFormat::FiveBan, TeamSide::Red, sample_champions());
        let state = session.state();
        assert!(!state.is_player_turn);
    }

    #[test]
    fn hover_and_lock_advances_step() {
        let mut session =
            DraftSession::new(DraftFormat::FiveBan, TeamSide::Blue, sample_champions());
        session.hover("Orianna".to_string()).unwrap();
        assert_eq!(session.state().active_hover, Some("Orianna".to_string()));

        session.lock().unwrap();
        assert_eq!(session.state().current_step, 1);
        assert_eq!(session.state().blue_bans, vec!["Orianna"]);
    }

    #[test]
    fn ai_acts_when_opponent_turn() {
        let mut session =
            DraftSession::new(DraftFormat::FiveBan, TeamSide::Red, sample_champions());
        // Blue goes first (AI for red player)
        assert!(!session.state().is_player_turn);

        let acted = session.ai_act();
        assert!(acted);
        assert_eq!(session.state().current_step, 1);
    }

    #[test]
    fn run_ai_turns_processes_all_consecutive_ai_slots() {
        let mut session =
            DraftSession::new(DraftFormat::FiveBan, TeamSide::Red, sample_champions());
        // Blue bans first 3 slots — all AI for red player
        let count = session.run_ai_turns();
        assert_eq!(count, 3); // 3 blue bans
        assert!(session.state().is_player_turn); // Now red's turn
        assert_eq!(session.state().current_phase, Some(DraftPhase::Ban));
    }

    #[test]
    fn available_champions_excludes_selected() {
        let mut session =
            DraftSession::new(DraftFormat::FiveBan, TeamSide::Blue, sample_champions());
        let initial_count = session.state().available_champions.len();

        session.hover("Orianna".to_string()).unwrap();
        session.lock().unwrap();

        assert_eq!(session.state().available_champions.len(), initial_count - 1);
        assert!(!session
            .state()
            .available_champions
            .contains(&"Orianna".to_string()));
    }

    #[test]
    fn full_draft_completes() {
        let mut session =
            DraftSession::new(DraftFormat::ThreeBan, TeamSide::Blue, sample_champions());
        // ThreeBan: 16 total steps
        let mut step = 0;
        while !session.is_complete() {
            if session.state().is_player_turn {
                let champ = session.state().available_champions[0].clone();
                session.hover(champ).unwrap();
                session.lock().unwrap();
            } else {
                session.ai_act();
            }
            step += 1;
            assert!(step <= 20, "Draft should complete within 20 iterations");
        }
        assert!(session.is_complete());
        assert_eq!(session.state().blue_bans.len(), 3);
        assert_eq!(session.state().red_bans.len(), 3);
        assert_eq!(session.state().blue_picks.len(), 5);
        assert_eq!(session.state().red_picks.len(), 5);
    }
}
