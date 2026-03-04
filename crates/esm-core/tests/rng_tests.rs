use esm_core::rng::GameRng;

// ---------------------------------------------------------------------------
// Determinism: same seed produces same sequence
// ---------------------------------------------------------------------------

#[test]
fn same_seed_produces_same_u32_sequence() {
    let mut rng1 = GameRng::from_seed(42);
    let mut rng2 = GameRng::from_seed(42);

    let seq1: Vec<u32> = (0..100).map(|_| rng1.next_u32()).collect();
    let seq2: Vec<u32> = (0..100).map(|_| rng2.next_u32()).collect();
    assert_eq!(seq1, seq2);
}

#[test]
fn different_seeds_produce_different_sequences() {
    let mut rng1 = GameRng::from_seed(42);
    let mut rng2 = GameRng::from_seed(99);

    let seq1: Vec<u32> = (0..20).map(|_| rng1.next_u32()).collect();
    let seq2: Vec<u32> = (0..20).map(|_| rng2.next_u32()).collect();
    assert_ne!(seq1, seq2);
}

// ---------------------------------------------------------------------------
// Range generation
// ---------------------------------------------------------------------------

#[test]
fn range_u32_stays_within_bounds() {
    let mut rng = GameRng::from_seed(123);
    for _ in 0..1000 {
        let val = rng.range_u32(10, 50);
        assert!(val >= 10 && val < 50, "Got {val}, expected [10, 50)");
    }
}

#[test]
fn range_f64_stays_within_bounds() {
    let mut rng = GameRng::from_seed(456);
    for _ in 0..1000 {
        let val = rng.range_f64(0.0, 1.0);
        assert!((0.0..1.0).contains(&val), "Got {val}, expected [0.0, 1.0)");
    }
}

#[test]
fn range_u32_single_value_returns_that_value() {
    let mut rng = GameRng::from_seed(1);
    let val = rng.range_u32(5, 6);
    assert_eq!(val, 5);
}

// ---------------------------------------------------------------------------
// Probability check (weighted coin flip)
// ---------------------------------------------------------------------------

#[test]
fn check_probability_zero_always_false() {
    let mut rng = GameRng::from_seed(0);
    for _ in 0..100 {
        assert!(!rng.check_probability(0.0));
    }
}

#[test]
fn check_probability_one_always_true() {
    let mut rng = GameRng::from_seed(0);
    for _ in 0..100 {
        assert!(rng.check_probability(1.0));
    }
}

#[test]
fn check_probability_roughly_matches_expected_rate() {
    let mut rng = GameRng::from_seed(777);
    let trials = 10_000;
    let successes = (0..trials).filter(|_| rng.check_probability(0.5)).count();

    let rate = successes as f64 / trials as f64;
    assert!((0.45..=0.55).contains(&rate), "Expected ~0.50, got {rate}");
}

// ---------------------------------------------------------------------------
// Seed retrieval
// ---------------------------------------------------------------------------

#[test]
fn seed_is_retrievable() {
    let rng = GameRng::from_seed(42);
    assert_eq!(rng.seed(), 42);
}

// ---------------------------------------------------------------------------
// Fork: create a child RNG from current state
// ---------------------------------------------------------------------------

#[test]
fn fork_produces_deterministic_child() {
    let mut rng1 = GameRng::from_seed(42);
    let mut rng2 = GameRng::from_seed(42);

    let child1 = rng1.fork();
    let child2 = rng2.fork();

    let mut c1 = child1;
    let mut c2 = child2;
    let seq1: Vec<u32> = (0..50).map(|_| c1.next_u32()).collect();
    let seq2: Vec<u32> = (0..50).map(|_| c2.next_u32()).collect();
    assert_eq!(seq1, seq2);
}

#[test]
fn fork_does_not_share_state_with_parent() {
    let mut rng = GameRng::from_seed(42);
    let mut child = rng.fork();

    let parent_seq: Vec<u32> = (0..20).map(|_| rng.next_u32()).collect();
    let child_seq: Vec<u32> = (0..20).map(|_| child.next_u32()).collect();
    // Parent and child sequences must diverge
    assert_ne!(parent_seq, child_seq);
}
