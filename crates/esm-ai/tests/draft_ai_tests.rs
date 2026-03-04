use esm_ai::draft_ai::{ChampionEval, DraftAi};
use esm_core::rng::GameRng;
use esm_models::champion::MasteryLevel;

// ---------------------------------------------------------------------------
// ChampionEval — scoring inputs for a single champion candidate
// ---------------------------------------------------------------------------

fn make_eval(meta: f64, mastery: MasteryLevel, synergy: f64, counter: f64) -> ChampionEval {
    ChampionEval {
        champion_name: "TestChamp".to_string(),
        meta_strength: meta,
        player_mastery: mastery,
        composition_synergy: synergy,
        counter_matchup: counter,
    }
}

#[test]
fn champion_eval_score_combines_all_factors() {
    let eval = make_eval(1.5, MasteryLevel::Challenger, 20.0, 30.0);
    let score = eval.score();
    // meta_strength * 100 * mastery_multiplier + synergy + counter
    // 1.5 * 100 * 1.5 + 20 + 30 = 225 + 50 = 275
    assert!((score - 275.0).abs() < f64::EPSILON);
}

#[test]
fn champion_eval_low_mastery_reduces_score() {
    let high = make_eval(1.0, MasteryLevel::Challenger, 0.0, 0.0);
    let low = make_eval(1.0, MasteryLevel::Bronze, 0.0, 0.0);
    assert!(high.score() > low.score());
}

#[test]
fn champion_eval_negative_counter_reduces_score() {
    let favorable = make_eval(1.0, MasteryLevel::Platinum, 0.0, 30.0);
    let unfavorable = make_eval(1.0, MasteryLevel::Platinum, 0.0, -30.0);
    assert!(favorable.score() > unfavorable.score());
}

#[test]
fn champion_eval_synergy_bonus_adds_to_score() {
    let with_synergy = make_eval(1.0, MasteryLevel::Platinum, 20.0, 0.0);
    let without = make_eval(1.0, MasteryLevel::Platinum, 0.0, 0.0);
    assert!((with_synergy.score() - without.score() - 20.0).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// DraftAi — pick selection
// ---------------------------------------------------------------------------

#[test]
fn draft_ai_selects_highest_scoring_champion() {
    let mut rng = GameRng::from_seed(42);
    let candidates = vec![
        ChampionEval {
            champion_name: "Weak".to_string(),
            meta_strength: 0.8,
            player_mastery: MasteryLevel::Bronze,
            composition_synergy: 0.0,
            counter_matchup: 0.0,
        },
        ChampionEval {
            champion_name: "Strong".to_string(),
            meta_strength: 1.5,
            player_mastery: MasteryLevel::Challenger,
            composition_synergy: 20.0,
            counter_matchup: 30.0,
        },
        ChampionEval {
            champion_name: "Medium".to_string(),
            meta_strength: 1.0,
            player_mastery: MasteryLevel::Gold,
            composition_synergy: 10.0,
            counter_matchup: 0.0,
        },
    ];

    let pick = DraftAi::select_champion(&mut rng, &candidates);
    // "Strong" has the highest score by far, should be selected
    assert_eq!(pick, "Strong");
}

#[test]
fn draft_ai_is_deterministic_with_same_seed() {
    let candidates = vec![
        ChampionEval {
            champion_name: "A".to_string(),
            meta_strength: 1.0,
            player_mastery: MasteryLevel::Platinum,
            composition_synergy: 10.0,
            counter_matchup: 5.0,
        },
        ChampionEval {
            champion_name: "B".to_string(),
            meta_strength: 1.0,
            player_mastery: MasteryLevel::Platinum,
            composition_synergy: 10.0,
            counter_matchup: 4.0,
        },
    ];

    let mut rng1 = GameRng::from_seed(42);
    let mut rng2 = GameRng::from_seed(42);

    let pick1 = DraftAi::select_champion(&mut rng1, &candidates);
    let pick2 = DraftAi::select_champion(&mut rng2, &candidates);
    assert_eq!(pick1, pick2);
}

#[test]
fn draft_ai_handles_single_candidate() {
    let mut rng = GameRng::from_seed(1);
    let candidates = vec![ChampionEval {
        champion_name: "Only".to_string(),
        meta_strength: 1.0,
        player_mastery: MasteryLevel::Gold,
        composition_synergy: 0.0,
        counter_matchup: 0.0,
    }];

    let pick = DraftAi::select_champion(&mut rng, &candidates);
    assert_eq!(pick, "Only");
}

#[test]
fn draft_ai_with_randomization_still_picks_strong_most_often() {
    let candidates = vec![
        ChampionEval {
            champion_name: "Strong".to_string(),
            meta_strength: 1.5,
            player_mastery: MasteryLevel::Challenger,
            composition_synergy: 20.0,
            counter_matchup: 30.0,
        },
        ChampionEval {
            champion_name: "Weak".to_string(),
            meta_strength: 0.8,
            player_mastery: MasteryLevel::Bronze,
            composition_synergy: 0.0,
            counter_matchup: -30.0,
        },
    ];

    let mut strong_picks = 0;
    for seed in 0..100 {
        let mut rng = GameRng::from_seed(seed);
        let pick = DraftAi::select_champion(&mut rng, &candidates);
        if pick == "Strong" {
            strong_picks += 1;
        }
    }
    assert!(
        strong_picks > 80,
        "Strong should be picked >80% of the time, got {strong_picks}/100"
    );
}
