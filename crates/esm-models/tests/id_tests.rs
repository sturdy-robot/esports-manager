use esm_models::id::{EntityId, IdGenerator};

// ---------------------------------------------------------------------------
// EntityId
// ---------------------------------------------------------------------------

#[test]
fn entity_id_stores_value() {
    let id = EntityId::new(42);
    assert_eq!(id.value(), 42);
}

#[test]
fn entity_id_equality() {
    let a = EntityId::new(1);
    let b = EntityId::new(1);
    let c = EntityId::new(2);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn entity_id_display() {
    let id = EntityId::new(123);
    assert_eq!(format!("{id}"), "123");
}

#[test]
fn entity_id_copy_semantics() {
    let id = EntityId::new(5);
    let id2 = id;
    assert_eq!(id, id2);
}

// ---------------------------------------------------------------------------
// IdGenerator
// ---------------------------------------------------------------------------

#[test]
fn id_generator_starts_at_one() {
    let mut gen = IdGenerator::new();
    let id = gen.next();
    assert_eq!(id.value(), 1);
}

#[test]
fn id_generator_increments_sequentially() {
    let mut gen = IdGenerator::new();
    let a = gen.next();
    let b = gen.next();
    let c = gen.next();
    assert_eq!(a.value(), 1);
    assert_eq!(b.value(), 2);
    assert_eq!(c.value(), 3);
}

#[test]
fn id_generator_never_produces_duplicates() {
    let mut gen = IdGenerator::new();
    let ids: Vec<EntityId> = (0..1000).map(|_| gen.next()).collect();
    let unique: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(unique.len(), 1000);
}

#[test]
fn id_generator_from_offset() {
    let mut gen = IdGenerator::from_offset(100);
    let id = gen.next();
    assert_eq!(id.value(), 101);
}

#[test]
fn id_generator_deterministic() {
    let mut gen1 = IdGenerator::new();
    let mut gen2 = IdGenerator::new();

    let seq1: Vec<u64> = (0..50).map(|_| gen1.next().value()).collect();
    let seq2: Vec<u64> = (0..50).map(|_| gen2.next().value()).collect();
    assert_eq!(seq1, seq2);
}

#[test]
fn id_generator_current_returns_last_issued() {
    let mut gen = IdGenerator::new();
    assert_eq!(gen.current(), 0);
    gen.next();
    assert_eq!(gen.current(), 1);
    gen.next();
    gen.next();
    assert_eq!(gen.current(), 3);
}
