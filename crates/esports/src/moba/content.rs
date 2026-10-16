//! MOBA content a package defines beyond the shared world structure: pinned
//! players, the roster shape and the generator's constants.

use domain::date::GameDate;
use domain::id::ContentId;
use domain::rating::Rating;

/// Attributes a package pins on a player. Every `None` is generated.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PinnedAttributes {
    pub mechanics: Option<Rating>,
    pub laning: Option<Rating>,
    pub game_sense: Option<Rating>,
    pub awareness: Option<Rating>,
    pub positioning: Option<Rating>,
    pub champion_pool_breadth: Option<Rating>,
    pub shotcalling: Option<Rating>,
    pub teamwork: Option<Rating>,
    pub aggression: Option<Rating>,
    pub composure: Option<Rating>,
    pub work_ethic: Option<Rating>,
    pub consistency: Option<Rating>,
    pub potential: Option<Rating>,
}

/// A player a package defines by hand. Every field left out is generated as
/// if the player were generated for their team.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinnedPlayer {
    pub id: ContentId,
    pub team: ContentId,
    pub role: ContentId,
    pub nickname: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub nationality: Option<ContentId>,
    pub birth_date: Option<GameDate>,
    pub attributes: PinnedAttributes,
}

/// How many players a roster has beyond one per role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RosterRules {
    pub subs: u8,
}

/// The generator's constants (world generation spec). Starting values, to be
/// tuned in sim-bench.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GenerationParams {
    /// The team mean is `mu_base` + `mu_per_reputation` × reputation + region level.
    pub mu_base: f64,
    pub mu_per_reputation: f64,
    /// The team spread is max(`sigma_min`, `sigma_base` − `sigma_per_reputation` × reputation).
    pub sigma_base: f64,
    pub sigma_per_reputation: f64,
    pub sigma_min: f64,
    /// Spread of each attribute around the player's base.
    pub attribute_spread: f64,
    /// Scale of the role bias.
    pub bias_scale: f64,
    /// Aggression and Work Ethic.
    pub style_mean: f64,
    pub style_spread: f64,
    pub consistency_mean: f64,
    pub consistency_spread: f64,
    /// Potential gains `potential_per_year` for each year under this age.
    pub potential_peak_age: f64,
    pub potential_per_year: f64,
    pub potential_spread: f64,
    pub age_mean: f64,
    pub age_spread: f64,
    pub age_min: u8,
    pub age_max: u8,
    /// The nickname pool needs this many distinct entries per generated player.
    pub nickname_pool_factor: u32,
}
