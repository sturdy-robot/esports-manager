use esm_core::rng::GameRng;
use esm_models::champion::MasteryLevel;

/// Evaluation data for a single champion candidate during drafting.
///
/// Per DESIGN.md §3, the AI assigns a priority score based on:
/// - Base meta strength (S-tier = 1.5, C-tier = 0.8)
/// - Player mastery (Bronze 0.5 to Challenger 1.5)
/// - Composition synergy bonus (e.g., +20 for matching tags)
/// - Counter-matchup bonus (+30 favorable, -30 unfavorable)
#[derive(Debug, Clone)]
pub struct ChampionEval {
    pub champion_name: String,
    pub meta_strength: f64,
    pub player_mastery: MasteryLevel,
    pub composition_synergy: f64,
    pub counter_matchup: f64,
}

impl ChampionEval {
    /// Compute the composite score for this champion candidate.
    ///
    /// Formula: `meta_strength * 100 * mastery_multiplier + synergy + counter`
    pub fn score(&self) -> f64 {
        let base = self.meta_strength * 100.0 * self.player_mastery.multiplier();
        base + self.composition_synergy + self.counter_matchup
    }
}

/// AI logic for selecting champions during the draft phase.
pub struct DraftAi;

impl DraftAi {
    /// Select the best champion from the candidate list.
    ///
    /// Applies a slight randomization to the top scores (via RNG) to prevent
    /// 100% predictable drafts, but strongly favors the highest-scoring champion.
    pub fn select_champion(rng: &mut GameRng, candidates: &[ChampionEval]) -> String {
        assert!(!candidates.is_empty(), "candidates must not be empty");

        if candidates.len() == 1 {
            return candidates[0].champion_name.clone();
        }

        // Score all candidates
        let mut scored: Vec<(usize, f64)> = candidates
            .iter()
            .enumerate()
            .map(|(i, c)| {
                // Add a small random perturbation (±5% of score) for variety
                let base_score = c.score();
                let noise = rng.range_f64(-0.05, 0.05) * base_score.abs().max(1.0);
                (i, base_score + noise)
            })
            .collect();

        // Sort descending by score
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        candidates[scored[0].0].champion_name.clone()
    }
}
