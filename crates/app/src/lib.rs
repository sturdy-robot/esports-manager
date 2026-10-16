//! Application layer: use cases, ports and view DTOs (ADR 0001).
//!
//! Adapters (the CLI, the desktop app) call use cases here and nothing else.

mod about;
pub mod rng;

pub use about::{About, about};
