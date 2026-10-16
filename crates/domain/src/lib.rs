//! Genre-agnostic core of eSports Manager.
//!
//! Pure Rust: no IO, no wall-clock, no global randomness (ADR 0001, ADR 0003).
//! Concepts specific to one esport belong in the `esports` crate (ADR 0002).

pub mod rng;
