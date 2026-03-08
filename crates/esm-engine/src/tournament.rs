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

    pub fn bracket(&self) -> BracketKind {
        self.bracket
    }

    pub fn set_result(&mut self, blue_wins: u32, red_wins: u32) {
        self.blue_wins = blue_wins;
        self.red_wins = red_wins;
        self.status = MatchStatus::Completed;
    }

    /// Record a single game win for the given side.
    /// Automatically marks the match as Completed once a side reaches `wins_needed`.
    pub fn add_game_win(&mut self, blue_won: bool) {
        if blue_won {
            self.blue_wins += 1;
        } else {
            self.red_wins += 1;
        }
        let needed = self.bracket.wins_needed();
        if self.blue_wins >= needed || self.red_wins >= needed {
            self.status = MatchStatus::Completed;
        }
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

const MAX_SERIES_PER_DAY: usize = 2;

fn round_robin_rounds(num_teams: usize) -> Vec<Vec<(usize, usize)>> {
    if num_teams < 2 {
        return Vec::new();
    }

    let has_bye = num_teams % 2 == 1;
    let total_slots = if has_bye { num_teams + 1 } else { num_teams };
    let bye_team = num_teams;
    let mut rotation: Vec<usize> = (0..total_slots).collect();
    let mut rounds = Vec::with_capacity(total_slots - 1);

    for round_idx in 0..(total_slots - 1) {
        let mut pairings = Vec::with_capacity(total_slots / 2);

        for i in 0..(total_slots / 2) {
            let left = rotation[i];
            let right = rotation[total_slots - 1 - i];

            if has_bye && (left == bye_team || right == bye_team) {
                continue;
            }

            if round_idx % 2 == 0 {
                pairings.push((left, right));
            } else {
                pairings.push((right, left));
            }
        }

        rounds.push(pairings);

        if total_slots > 2 {
            let fixed = rotation[0];
            let mut rotating = rotation[1..].to_vec();
            rotating.rotate_right(1);
            rotation = std::iter::once(fixed).chain(rotating.into_iter()).collect();
        }
    }

    rounds
}

fn can_schedule_on_day(
    day_matches: &[(usize, usize)],
    blue_team_idx: usize,
    red_team_idx: usize,
) -> bool {
    day_matches.len() < MAX_SERIES_PER_DAY
        && day_matches.iter().all(|(scheduled_blue, scheduled_red)| {
            *scheduled_blue != blue_team_idx
                && *scheduled_blue != red_team_idx
                && *scheduled_red != blue_team_idx
                && *scheduled_red != red_team_idx
        })
}

fn assign_rounds_to_days(
    rounds: Vec<Vec<(usize, usize)>>,
    start_day: u32,
) -> Vec<(u32, usize, usize)> {
    let mut scheduled_matches = Vec::new();
    let mut current_day = start_day;
    let mut current_day_matches: Vec<(usize, usize)> = Vec::new();

    for round in rounds {
        for (blue_team_idx, red_team_idx) in round {
            if !can_schedule_on_day(&current_day_matches, blue_team_idx, red_team_idx) {
                current_day += 1;
                current_day_matches.clear();
            }

            current_day_matches.push((blue_team_idx, red_team_idx));
            scheduled_matches.push((current_day, blue_team_idx, red_team_idx));
        }
    }

    scheduled_matches
}

impl Schedule {
    /// Generate a single round-robin schedule for `num_teams` teams.
    /// Each pair plays once. Matches are spread across days starting at `start_day`.
    pub fn round_robin(num_teams: usize, bracket: BracketKind, start_day: u32) -> Self {
        let rounds = round_robin_rounds(num_teams);
        let mut matches = Vec::new();
        let mut id = 1u32;

        for (day, blue_team_idx, red_team_idx) in assign_rounds_to_days(rounds, start_day) {
            matches.push(Match::new(id, blue_team_idx, red_team_idx, day, bracket));
            id += 1;
        }

        Self { matches }
    }

    /// Generate a double round-robin (each pair plays twice, home and away).
    pub fn double_round_robin(num_teams: usize, bracket: BracketKind, start_day: u32) -> Self {
        let rounds = round_robin_rounds(num_teams);
        let mut all_matches = Vec::new();
        let mut id = 1u32;

        let first_half_matches = assign_rounds_to_days(rounds.clone(), start_day);
        for (day, blue_team_idx, red_team_idx) in &first_half_matches {
            all_matches.push(Match::new(id, *blue_team_idx, *red_team_idx, *day, bracket));
            id += 1;
        }

        let second_half_start_day = first_half_matches
            .last()
            .map(|(day, _, _)| day + 1)
            .unwrap_or(start_day);
        let mirrored_rounds: Vec<Vec<(usize, usize)>> = rounds
            .into_iter()
            .map(|round| {
                round
                    .into_iter()
                    .map(|(blue_team_idx, red_team_idx)| (red_team_idx, blue_team_idx))
                    .collect()
            })
            .collect();
        for (day, blue_team_idx, red_team_idx) in
            assign_rounds_to_days(mirrored_rounds, second_half_start_day)
        {
            all_matches.push(Match::new(id, blue_team_idx, red_team_idx, day, bracket));
            id += 1;
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

    pub fn team_name(&self, idx: usize) -> Option<&str> {
        self.team_names.get(idx).map(|s| s.as_str())
    }

    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    pub fn record_result(&mut self, match_id: u32, blue_wins: u32, red_wins: u32) {
        if let Some(m) = self
            .schedule
            .matches_mut()
            .iter_mut()
            .find(|m| m.id() == match_id)
        {
            m.set_result(blue_wins, red_wins);
        }
    }

    /// Record a single game win within a series for the given match.
    /// Returns true if the series is now complete.
    pub fn add_game_win(&mut self, match_id: u32, blue_won: bool) -> bool {
        if let Some(m) = self
            .schedule
            .matches_mut()
            .iter_mut()
            .find(|m| m.id() == match_id)
        {
            m.add_game_win(blue_won);
            m.status() == MatchStatus::Completed
        } else {
            false
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

    /// Return pending matches scheduled for the given day (by days_elapsed).
    pub fn matches_today(&self, day: u32) -> Vec<&Match> {
        self.schedule
            .matches_for_day(day)
            .into_iter()
            .filter(|m| m.status() == MatchStatus::Pending)
            .collect()
    }

    pub fn is_complete(&self) -> bool {
        self.schedule
            .matches()
            .iter()
            .all(|m| m.status() == MatchStatus::Completed)
    }
}
