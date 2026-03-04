use esm_core::game_state::GameState;
use esm_core::turn::TurnProcessor;
use esm_engine::match_sim::{MatchSimulator, TeamSide};
use esm_engine::tournament::{BracketKind, Tournament, TournamentFormat};
use esm_models::manager::{Manager, ManagerArchetype};
use esm_models::player::{
    BoundedAttribute, MentalAttributes, PhysicalAttributes, Player, PlayerAttributes, Role,
    TechnicalAttributes,
};
use esm_models::team::Team;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_player(nickname: &str, role: Role, skill: u8) -> Player {
    Player::new(
        nickname.to_string(),
        "First".to_string(),
        "Last".to_string(),
        role,
        PlayerAttributes {
            physical: PhysicalAttributes {
                endurance: BoundedAttribute::new(skill),
                reaction_time: BoundedAttribute::new(skill),
            },
            mental: MentalAttributes {
                decision_making: BoundedAttribute::new(skill),
                clutch: BoundedAttribute::new(skill),
                discipline: BoundedAttribute::new(skill),
                tilt_resistance: BoundedAttribute::new(skill),
            },
            technical: TechnicalAttributes {
                mechanics: BoundedAttribute::new(skill),
                vision_control: BoundedAttribute::new(skill),
                teamfighting: BoundedAttribute::new(skill),
            },
        },
    )
}

fn make_team(name: &str, skill: u8) -> Team {
    Team::new(
        name.to_string(),
        name[..2].to_uppercase(),
        vec![
            make_player(&format!("{name}_top"), Role::Top, skill),
            make_player(&format!("{name}_jg"), Role::Jungle, skill),
            make_player(&format!("{name}_mid"), Role::Mid, skill),
            make_player(&format!("{name}_bot"), Role::Bot, skill),
            make_player(&format!("{name}_sup"), Role::Support, skill),
        ],
    )
}

fn compute_team_power(team: &Team) -> u32 {
    let mut total: u32 = 0;
    for player in team.roster() {
        let attrs = player.attributes();
        total += attrs.physical.endurance.value() as u32;
        total += attrs.physical.reaction_time.value() as u32;
        total += attrs.mental.decision_making.value() as u32;
        total += attrs.technical.mechanics.value() as u32;
        total += attrs.technical.teamfighting.value() as u32;
    }
    total
}

struct GameSession {
    state: GameState,
    tournament: Tournament,
}

impl GameSession {
    fn new() -> Self {
        let teams = vec![
            make_team("Alpha", 70),
            make_team("Bravo", 60),
            make_team("Charlie", 80),
            make_team("Delta", 55),
        ];
        let team_names: Vec<String> = teams.iter().map(|t| t.name().to_string()).collect();

        let state = GameState::new(
            2025,
            42,
            Manager::new(
                "TestMgr".to_string(),
                "John".to_string(),
                "Doe".to_string(),
                "US".to_string(),
                ManagerArchetype::Balanced,
            ),
            0, // player manages Alpha
            teams,
        );

        let tournament = Tournament::new(
            "Test League".to_string(),
            team_names,
            TournamentFormat::RoundRobin,
            BracketKind::Bo1,
            3, // matches start on day 3
        );

        Self { state, tournament }
    }

    fn todays_matches(&self) -> Vec<&esm_engine::tournament::Match> {
        let day = self.state.calendar().days_elapsed();
        self.tournament.matches_today(day)
    }
}

// ---------------------------------------------------------------------------
// Integration: game loop pattern
// ---------------------------------------------------------------------------

#[test]
fn game_loop_no_matches_on_day_1() {
    let session = GameSession::new();
    assert!(session.todays_matches().is_empty());
}

#[test]
fn game_loop_advance_to_match_day() {
    let mut session = GameSession::new();
    // Advance to day 3 where matches are scheduled
    for _ in 0..3 {
        TurnProcessor::end_day(&mut session.state).unwrap();
    }
    let matches = session.todays_matches();
    assert!(
        !matches.is_empty(),
        "There should be matches scheduled on day 3"
    );
}

#[test]
fn game_loop_simulate_match_and_record_result() {
    let mut session = GameSession::new();
    // Advance to first match day
    for _ in 0..3 {
        TurnProcessor::end_day(&mut session.state).unwrap();
    }

    let todays: Vec<_> = session
        .todays_matches()
        .iter()
        .map(|m| (m.id(), m.blue_team_idx(), m.red_team_idx()))
        .collect();
    assert!(!todays.is_empty());

    let (match_id, blue_idx, red_idx) = todays[0];
    let blue_power = compute_team_power(&session.state.teams()[blue_idx]);
    let red_power = compute_team_power(&session.state.teams()[red_idx]);

    let result = MatchSimulator::simulate(session.state.rng_mut(), blue_power, red_power);

    let (blue_wins, red_wins) = match result.winner {
        TeamSide::Blue => (1, 0),
        TeamSide::Red => (0, 1),
    };
    session
        .tournament
        .record_result(match_id, blue_wins, red_wins);

    // Verify the result was recorded
    let standings = session.tournament.standings();
    let total_wins: u32 = standings.iter().map(|s| s.wins).sum();
    assert_eq!(total_wins, 1);
}

#[test]
fn game_loop_play_full_season() {
    let mut session = GameSession::new();

    // Simulate the entire season: advance days, play matches as they come
    for _ in 0..30 {
        TurnProcessor::end_day(&mut session.state).unwrap();

        let day = session.state.calendar().days_elapsed();
        let todays: Vec<_> = session
            .tournament
            .matches_today(day)
            .iter()
            .map(|m| (m.id(), m.blue_team_idx(), m.red_team_idx()))
            .collect();

        for (match_id, blue_idx, red_idx) in todays {
            let blue_power = compute_team_power(&session.state.teams()[blue_idx]);
            let red_power = compute_team_power(&session.state.teams()[red_idx]);

            let result = MatchSimulator::simulate(session.state.rng_mut(), blue_power, red_power);

            let (bw, rw) = match result.winner {
                TeamSide::Blue => (1, 0),
                TeamSide::Red => (0, 1),
            };
            session.tournament.record_result(match_id, bw, rw);
        }
    }

    // After 30 days with 4 teams (6 matches), tournament should be complete
    assert!(
        session.tournament.is_complete(),
        "Tournament should be complete after enough days"
    );

    let standings = session.tournament.standings();
    let total_wins: u32 = standings.iter().map(|s| s.wins).sum();
    let total_losses: u32 = standings.iter().map(|s| s.losses).sum();
    assert_eq!(total_wins, 6); // 6 matches = 6 winners
    assert_eq!(total_losses, 6);
}

#[test]
fn game_loop_deterministic_season() {
    fn play_season(seed: u64) -> Vec<(usize, u32, u32)> {
        let teams = vec![
            make_team("Alpha", 70),
            make_team("Bravo", 60),
            make_team("Charlie", 80),
            make_team("Delta", 55),
        ];
        let team_names: Vec<String> = teams.iter().map(|t| t.name().to_string()).collect();

        let mut state = GameState::new(
            2025,
            seed,
            Manager::new(
                "M".into(),
                "J".into(),
                "D".into(),
                "US".into(),
                ManagerArchetype::Balanced,
            ),
            0,
            teams,
        );
        let mut tournament = Tournament::new(
            "Test".into(),
            team_names,
            TournamentFormat::RoundRobin,
            BracketKind::Bo1,
            3,
        );

        for _ in 0..30 {
            TurnProcessor::end_day(&mut state).unwrap();
            let day = state.calendar().days_elapsed();
            let todays: Vec<_> = tournament
                .matches_today(day)
                .iter()
                .map(|m| (m.id(), m.blue_team_idx(), m.red_team_idx()))
                .collect();
            for (id, bi, ri) in todays {
                let bp = compute_team_power(&state.teams()[bi]);
                let rp = compute_team_power(&state.teams()[ri]);
                let res = MatchSimulator::simulate(state.rng_mut(), bp, rp);
                let (bw, rw) = match res.winner {
                    TeamSide::Blue => (1, 0),
                    TeamSide::Red => (0, 1),
                };
                tournament.record_result(id, bw, rw);
            }
        }

        tournament
            .standings()
            .iter()
            .map(|s| (s.team_idx, s.wins, s.losses))
            .collect()
    }

    let run1 = play_season(42);
    let run2 = play_season(42);
    assert_eq!(
        run1, run2,
        "Same seed must produce identical season results"
    );
}

#[test]
fn game_loop_stronger_team_tends_to_win_more() {
    let mut charlie_first_count = 0;
    let trials = 200;
    for seed in 0..trials {
        let teams = vec![
            make_team("Alpha", 50),
            make_team("Bravo", 50),
            make_team("Charlie", 95), // Much stronger
            make_team("Delta", 50),
        ];
        let team_names: Vec<String> = teams.iter().map(|t| t.name().to_string()).collect();

        let mut state = GameState::new(
            2025,
            seed,
            Manager::new(
                "M".into(),
                "J".into(),
                "D".into(),
                "US".into(),
                ManagerArchetype::Balanced,
            ),
            0,
            teams,
        );
        let mut tournament = Tournament::new(
            "T".into(),
            team_names,
            TournamentFormat::RoundRobin,
            BracketKind::Bo1,
            1,
        );

        for _ in 0..30 {
            TurnProcessor::end_day(&mut state).unwrap();
            let day = state.calendar().days_elapsed();
            let todays: Vec<_> = tournament
                .matches_today(day)
                .iter()
                .map(|m| (m.id(), m.blue_team_idx(), m.red_team_idx()))
                .collect();
            for (id, bi, ri) in todays {
                let bp = compute_team_power(&state.teams()[bi]);
                let rp = compute_team_power(&state.teams()[ri]);
                let res = MatchSimulator::simulate(state.rng_mut(), bp, rp);
                let (bw, rw) = match res.winner {
                    TeamSide::Blue => (1, 0),
                    TeamSide::Red => (0, 1),
                };
                tournament.record_result(id, bw, rw);
            }
        }

        let standings = tournament.standings();
        if standings[0].team_idx == 2 {
            charlie_first_count += 1;
        }
    }

    assert!(
        charlie_first_count > trials / 2,
        "Charlie (skill 95) should finish first in >50% of seasons, got {charlie_first_count}/{trials}"
    );
}
