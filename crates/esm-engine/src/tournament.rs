use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TournamentFormat {
    RoundRobin,
    DoubleRoundRobin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BracketKind {
    Bo1,
    Bo3,
    Bo5,
}

impl BracketKind {
    pub fn wins_needed(self) -> u32 {
        match self {
            BracketKind::Bo1 => 1,
            BracketKind::Bo3 => 2,
            BracketKind::Bo5 => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchStatus {
    Pending,
    Completed,
}

// ---------------------------------------------------------------------------
// Match
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    id: u32,
    blue_team_idx: usize,
    red_team_idx: usize,
    scheduled_day: u32,
    bracket: BracketKind,
    status: MatchStatus,
    blue_wins: u32,
    red_wins: u32,
}

impl Match {
    pub fn new(
        id: u32,
        blue_team_idx: usize,
        red_team_idx: usize,
        scheduled_day: u32,
        bracket: BracketKind,
    ) -> Self {
        Self {
            id,
            blue_team_idx,
            red_team_idx,
            scheduled_day,
            bracket,
            status: MatchStatus::Pending,
            blue_wins: 0,
            red_wins: 0,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn blue_team_idx(&self) -> usize {
        self.blue_team_idx
    }

    pub fn red_team_idx(&self) -> usize {
        self.red_team_idx
    }

    pub fn scheduled_day(&self) -> u32 {
        self.scheduled_day
    }

    pub fn status(&self) -> MatchStatus {
        self.status
    }

    pub fn blue_wins(&self) -> u32 {
        self.blue_wins
    }

    pub fn red_wins(&self) -> u32 {
        self.red_wins
    }

    pub fn set_result(&mut self, blue_wins: u32, red_wins: u32) {
        self.blue_wins = blue_wins;
        self.red_wins = red_wins;
        self.status = MatchStatus::Completed;
    }

    pub fn winner_team_idx(&self) -> Option<usize> {
        if self.status != MatchStatus::Completed {
            return None;
        }
        if self.blue_wins > self.red_wins {
            Some(self.blue_team_idx)
        } else {
            Some(self.red_team_idx)
        }
    }
}

// ---------------------------------------------------------------------------
// Standing
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Standing {
    pub team_idx: usize,
    pub team_name: String,
    pub wins: u32,
    pub losses: u32,
}

// ---------------------------------------------------------------------------
// Schedule
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    matches: Vec<Match>,
}

impl Schedule {
    /// Generate a single round-robin schedule for `num_teams` teams.
    /// Each pair plays once. Matches are spread across days starting at `start_day`.
    pub fn round_robin(num_teams: usize, bracket: BracketKind, start_day: u32) -> Self {
        let mut matches = Vec::new();
        let mut id = 1u32;
        let mut day = start_day;
        let matches_per_day = (num_teams / 2).max(1);
        let mut count_on_day = 0;

        for i in 0..num_teams {
            for j in (i + 1)..num_teams {
                matches.push(Match::new(id, i, j, day, bracket));
                id += 1;
                count_on_day += 1;
                if count_on_day >= matches_per_day {
                    count_on_day = 0;
                    day += 1;
                }
            }
        }

        Self { matches }
    }

    /// Generate a double round-robin (each pair plays twice, home and away).
    pub fn double_round_robin(num_teams: usize, bracket: BracketKind, start_day: u32) -> Self {
        let first_half = Self::round_robin(num_teams, bracket, start_day);
        let max_day = first_half
            .matches
            .iter()
            .map(|m| m.scheduled_day)
            .max()
            .unwrap_or(start_day);

        let mut all_matches = first_half.matches;
        let mut id = all_matches.len() as u32 + 1;
        let mut day = max_day + 1;
        let matches_per_day = (num_teams / 2).max(1);
        let mut count_on_day = 0;

        // Second round: swap sides
        for i in 0..num_teams {
            for j in (i + 1)..num_teams {
                all_matches.push(Match::new(id, j, i, day, bracket));
                id += 1;
                count_on_day += 1;
                if count_on_day >= matches_per_day {
                    count_on_day = 0;
                    day += 1;
                }
            }
        }

        Self {
            matches: all_matches,
        }
    }

    pub fn matches(&self) -> &[Match] {
        &self.matches
    }

    pub fn matches_mut(&mut self) -> &mut [Match] {
        &mut self.matches
    }

    pub fn matches_for_day(&self, day: u32) -> Vec<&Match> {
        self.matches
            .iter()
            .filter(|m| m.scheduled_day() == day)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Tournament
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    name: String,
    team_names: Vec<String>,
    format: TournamentFormat,
    schedule: Schedule,
}

impl Tournament {
    pub fn new(
        name: String,
        team_names: Vec<String>,
        format: TournamentFormat,
        bracket: BracketKind,
        start_day: u32,
    ) -> Self {
        let num_teams = team_names.len();
        let schedule = match format {
            TournamentFormat::RoundRobin => Schedule::round_robin(num_teams, bracket, start_day),
            TournamentFormat::DoubleRoundRobin => {
                Schedule::double_round_robin(num_teams, bracket, start_day)
            }
        };

        Self {
            name,
            team_names,
            format,
            schedule,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn team_count(&self) -> usize {
        self.team_names.len()
    }

    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    pub fn record_result(&mut self, match_id: u32, blue_wins: u32, red_wins: u32) {
        if let Some(m) = self.schedule.matches_mut().iter_mut().find(|m| m.id() == match_id) {
            m.set_result(blue_wins, red_wins);
        }
    }

    pub fn standings(&self) -> Vec<Standing> {
        let mut standings: Vec<Standing> = self
            .team_names
            .iter()
            .enumerate()
            .map(|(idx, name)| Standing {
                team_idx: idx,
                team_name: name.clone(),
                wins: 0,
                losses: 0,
            })
            .collect();

        for m in self.schedule.matches() {
            if m.status() != MatchStatus::Completed {
                continue;
            }
            if let Some(winner_idx) = m.winner_team_idx() {
                let loser_idx = if winner_idx == m.blue_team_idx() {
                    m.red_team_idx()
                } else {
                    m.blue_team_idx()
                };
                standings[winner_idx].wins += 1;
                standings[loser_idx].losses += 1;
            }
        }

        standings.sort_by(|a, b| b.wins.cmp(&a.wins).then(a.losses.cmp(&b.losses)));
        standings
    }

    pub fn is_complete(&self) -> bool {
        self.schedule
            .matches()
            .iter()
            .all(|m| m.status() == MatchStatus::Completed)
    }
}
