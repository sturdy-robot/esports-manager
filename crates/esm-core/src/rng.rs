use serde::{Deserialize, Serialize};

/// Deterministic seed-controlled PRNG using xorshift64.
///
/// Every game session stores its seed so that, given the same seed and the same
/// sequence of calls, every simulation produces identical results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRng {
    seed: u64,
    state: u64,
}

impl GameRng {
    pub fn from_seed(seed: u64) -> Self {
        // Ensure state is never zero (xorshift requirement)
        let state = if seed == 0 { 1 } else { seed };
        Self { seed, state }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Return the current internal state (for persistence).
    pub fn state(&self) -> u64 {
        self.state
    }

    /// Restore a GameRng from a previously saved seed and state.
    /// This allows resuming a deterministic sequence from exactly
    /// where it left off.
    pub fn from_state(seed: u64, state: u64) -> Self {
        let state = if state == 0 { 1 } else { state };
        Self { seed, state }
    }

    /// Advance the internal state and return the next raw u64.
    fn next_u64(&mut self) -> u64 {
        let mut s = self.state;
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        self.state = s;
        s
    }

    /// Return the next random u32.
    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Return a random u32 in `[min, max)`.
    ///
    /// # Panics
    /// Panics if `min >= max`.
    pub fn range_u32(&mut self, min: u32, max: u32) -> u32 {
        assert!(min < max, "range_u32: min ({min}) must be < max ({max})");
        let range = max - min;
        if range == 1 {
            return min;
        }
        min + (self.next_u32() % range)
    }

    /// Return a random f64 in `[0.0, 1.0)`.
    fn next_f64(&mut self) -> f64 {
        // Use upper 53 bits for a uniform double in [0, 1)
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Return a random f64 in `[min, max)`.
    pub fn range_f64(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.next_f64()
    }

    /// Return `true` with the given probability `[0.0, 1.0]`.
    pub fn check_probability(&mut self, probability: f64) -> bool {
        if probability <= 0.0 {
            return false;
        }
        if probability >= 1.0 {
            return true;
        }
        self.next_f64() < probability
    }

    /// Create a deterministic child RNG derived from the current state.
    /// This consumes two values from the parent to ensure parent and child
    /// have fully divergent sequences after forking.
    pub fn fork(&mut self) -> GameRng {
        let child_seed = self.next_u64() ^ self.next_u64();
        GameRng::from_seed(child_seed)
    }
}
