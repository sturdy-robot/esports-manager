//! The MOBA genre (ADR 0002): five-a-side, one map, roles defined by the
//! content package.

pub mod attributes;
pub mod content;
#[cfg(feature = "test-support")]
pub mod fixtures;
pub mod generation;
pub mod roles;
pub mod world;
