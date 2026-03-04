use serde::{Deserialize, Serialize};

use super::state::TeamSide;

// ---------------------------------------------------------------------------
// Lane
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Lane {
    Top,
    Mid,
    Bot,
}

const ALL_LANES: [Lane; 3] = [Lane::Top, Lane::Mid, Lane::Bot];

// ---------------------------------------------------------------------------
// TowerTier
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TowerTier {
    Outer,
    Inner,
    Inhibitor,
    Nexus,
}

impl TowerTier {
    /// Return the tier that must be destroyed before this one is vulnerable.
    fn prerequisite(self) -> Option<TowerTier> {
        match self {
            TowerTier::Outer => None,
            TowerTier::Inner => Some(TowerTier::Outer),
            TowerTier::Inhibitor => Some(TowerTier::Inner),
            TowerTier::Nexus => None, // Nexus towers require an inhibitor down, handled separately
        }
    }
}

// ---------------------------------------------------------------------------
// Tower
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tower {
    lane: Lane,
    tier: TowerTier,
    standing: bool,
}

impl Tower {
    pub fn new(lane: Lane, tier: TowerTier) -> Self {
        Self {
            lane,
            tier,
            standing: true,
        }
    }

    pub fn lane(&self) -> Lane {
        self.lane
    }

    pub fn tier(&self) -> TowerTier {
        self.tier
    }

    pub fn is_standing(&self) -> bool {
        self.standing
    }

    pub fn is_destroyed(&self) -> bool {
        !self.standing
    }

    pub fn destroy(&mut self) {
        self.standing = false;
    }
}

// ---------------------------------------------------------------------------
// Inhibitor
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inhibitor {
    lane: Lane,
    standing: bool,
    respawn_timer: Option<u32>,
}

impl Inhibitor {
    pub fn new(lane: Lane) -> Self {
        Self {
            lane,
            standing: true,
            respawn_timer: None,
        }
    }

    pub fn lane(&self) -> Lane {
        self.lane
    }

    pub fn is_standing(&self) -> bool {
        self.standing
    }

    pub fn is_destroyed(&self) -> bool {
        !self.standing
    }

    pub fn respawn_timer(&self) -> Option<u32> {
        self.respawn_timer
    }

    pub fn destroy(&mut self, respawn_seconds: u32) {
        self.standing = false;
        self.respawn_timer = Some(respawn_seconds);
    }

    /// Tick the respawn timer by `seconds`. If timer reaches zero, respawn.
    pub fn tick(&mut self, seconds: u32) {
        if let Some(timer) = self.respawn_timer {
            let remaining = timer.saturating_sub(seconds);
            if remaining == 0 {
                self.standing = true;
                self.respawn_timer = None;
            } else {
                self.respawn_timer = Some(remaining);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ObjectiveState
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveState {
    blue_dragons: u32,
    red_dragons: u32,
    baron_alive: bool,
    baron_buff_blue: bool,
    baron_buff_red: bool,
    herald_available: bool,
}

impl ObjectiveState {
    pub fn new() -> Self {
        Self {
            blue_dragons: 0,
            red_dragons: 0,
            baron_alive: false,
            baron_buff_blue: false,
            baron_buff_red: false,
            herald_available: true,
        }
    }

    pub fn dragon_count(&self, side: TeamSide) -> u32 {
        match side {
            TeamSide::Blue => self.blue_dragons,
            TeamSide::Red => self.red_dragons,
        }
    }

    pub fn add_dragon(&mut self, side: TeamSide) {
        match side {
            TeamSide::Blue => self.blue_dragons += 1,
            TeamSide::Red => self.red_dragons += 1,
        }
    }

    pub fn has_dragon_soul(&self, side: TeamSide) -> bool {
        self.dragon_count(side) >= 4
    }

    pub fn elder_available(&self) -> bool {
        self.has_dragon_soul(TeamSide::Blue) || self.has_dragon_soul(TeamSide::Red)
    }

    pub fn baron_alive(&self) -> bool {
        self.baron_alive
    }

    pub fn spawn_baron(&mut self) {
        self.baron_alive = true;
    }

    pub fn kill_baron(&mut self, killer: TeamSide) {
        self.baron_alive = false;
        match killer {
            TeamSide::Blue => self.baron_buff_blue = true,
            TeamSide::Red => self.baron_buff_red = true,
        }
    }

    pub fn baron_buff(&self, side: TeamSide) -> bool {
        match side {
            TeamSide::Blue => self.baron_buff_blue,
            TeamSide::Red => self.baron_buff_red,
        }
    }

    pub fn herald_available(&self) -> bool {
        self.herald_available
    }

    pub fn consume_herald(&mut self) {
        self.herald_available = false;
    }
}

impl Default for ObjectiveState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// SideMap — towers + inhibitors for one side
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SideMap {
    towers: Vec<Tower>,
    inhibitors: Vec<Inhibitor>,
}

impl SideMap {
    fn new() -> Self {
        let mut towers = Vec::with_capacity(11);

        // 3 lanes × 3 tiers (Outer, Inner, Inhibitor)
        for lane in ALL_LANES {
            towers.push(Tower::new(lane, TowerTier::Outer));
            towers.push(Tower::new(lane, TowerTier::Inner));
            towers.push(Tower::new(lane, TowerTier::Inhibitor));
        }

        // 2 Nexus towers (use Mid lane as placeholder)
        towers.push(Tower::new(Lane::Mid, TowerTier::Nexus));
        towers.push(Tower::new(Lane::Mid, TowerTier::Nexus));

        let inhibitors = ALL_LANES.iter().map(|&l| Inhibitor::new(l)).collect();

        Self { towers, inhibitors }
    }

    fn tower(&self, lane: Lane, tier: TowerTier) -> Option<&Tower> {
        self.towers
            .iter()
            .find(|t| t.lane() == lane && t.tier() == tier && t.tier() != TowerTier::Nexus)
    }

    fn tower_mut(&mut self, lane: Lane, tier: TowerTier) -> Option<&mut Tower> {
        self.towers
            .iter_mut()
            .find(|t| t.lane() == lane && t.tier() == tier && t.tier() != TowerTier::Nexus)
    }

    fn inhibitor(&self, lane: Lane) -> Option<&Inhibitor> {
        self.inhibitors.iter().find(|i| i.lane() == lane)
    }

    fn inhibitor_mut(&mut self, lane: Lane) -> Option<&mut Inhibitor> {
        self.inhibitors.iter_mut().find(|i| i.lane() == lane)
    }
}

// ---------------------------------------------------------------------------
// MapState
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapState {
    blue: SideMap,
    red: SideMap,
    objectives: ObjectiveState,
}

impl MapState {
    pub fn new() -> Self {
        Self {
            blue: SideMap::new(),
            red: SideMap::new(),
            objectives: ObjectiveState::new(),
        }
    }

    fn side(&self, side: TeamSide) -> &SideMap {
        match side {
            TeamSide::Blue => &self.blue,
            TeamSide::Red => &self.red,
        }
    }

    fn side_mut(&mut self, side: TeamSide) -> &mut SideMap {
        match side {
            TeamSide::Blue => &mut self.blue,
            TeamSide::Red => &mut self.red,
        }
    }

    pub fn towers(&self, side: TeamSide) -> &[Tower] {
        &self.side(side).towers
    }

    pub fn inhibitors(&self, side: TeamSide) -> &[Inhibitor] {
        &self.side(side).inhibitors
    }

    pub fn objectives(&self) -> &ObjectiveState {
        &self.objectives
    }

    pub fn objectives_mut(&mut self) -> &mut ObjectiveState {
        &mut self.objectives
    }

    /// Check if a specific tower is vulnerable (predecessor destroyed).
    pub fn is_tower_vulnerable(&self, side: TeamSide, lane: Lane, tier: TowerTier) -> bool {
        let s = self.side(side);

        // Check the tower itself is still standing
        if let Some(tower) = s.tower(lane, tier) {
            if tower.is_destroyed() {
                return false;
            }
        } else {
            return false;
        }

        // Check prerequisite tier is destroyed
        if let Some(prereq) = tier.prerequisite() {
            if let Some(prereq_tower) = s.tower(lane, prereq) {
                return prereq_tower.is_destroyed();
            }
        }

        true // Outer towers are always vulnerable if standing
    }

    /// Destroy a specific tower.
    pub fn destroy_tower(&mut self, side: TeamSide, lane: Lane, tier: TowerTier) {
        if let Some(tower) = self.side_mut(side).tower_mut(lane, tier) {
            tower.destroy();
        }
    }

    /// Check if an inhibitor is vulnerable (inhibitor tower destroyed).
    pub fn is_inhibitor_vulnerable(&self, side: TeamSide, lane: Lane) -> bool {
        let s = self.side(side);
        // Inhibitor tower must be destroyed
        if let Some(inhib_tower) = s.tower(lane, TowerTier::Inhibitor) {
            if inhib_tower.is_standing() {
                return false;
            }
        }
        // And the inhibitor itself must still be standing
        if let Some(inh) = s.inhibitor(lane) {
            return inh.is_standing();
        }
        false
    }

    /// Destroy an inhibitor with a respawn timer.
    pub fn destroy_inhibitor(&mut self, side: TeamSide, lane: Lane, respawn_seconds: u32) {
        if let Some(inh) = self.side_mut(side).inhibitor_mut(lane) {
            inh.destroy(respawn_seconds);
        }
    }

    /// Check if the nexus is vulnerable (any inhibitor destroyed).
    pub fn is_nexus_vulnerable(&self, side: TeamSide) -> bool {
        self.side(side)
            .inhibitors
            .iter()
            .any(|inh| inh.is_destroyed())
    }

    /// Tick all inhibitor respawn timers.
    pub fn tick_inhibitors(&mut self, seconds: u32) {
        for inh in &mut self.blue.inhibitors {
            inh.tick(seconds);
        }
        for inh in &mut self.red.inhibitors {
            inh.tick(seconds);
        }
    }
}

impl Default for MapState {
    fn default() -> Self {
        Self::new()
    }
}
