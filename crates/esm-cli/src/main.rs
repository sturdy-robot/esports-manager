use std::fs;
use std::process;

use esm_core::game_state::GameState;
use esm_core::turn::TurnProcessor;
use esm_data::datapack::DataPack;
use esm_engine::match_sim::{MatchSimulator, TeamSide};
use esm_engine::tournament::{BracketKind, Tournament, TournamentFormat};
use esm_models::manager::{Manager, ManagerArchetype};
use esm_models::team::Team;

fn compute_team_power(team: &Team) -> u32 {
    let mut total: u32 = 0;
    for player in team.roster() {
        let a = player.attributes();
        total += a.physical.endurance.value() as u32;
        total += a.physical.reaction_time.value() as u32;
        total += a.mental.decision_making.value() as u32;
        total += a.mental.clutch.value() as u32;
        total += a.technical.mechanics.value() as u32;
        total += a.technical.teamfighting.value() as u32;
    }
    total
}

fn main() {
    let datapack_path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: esm <datapack.json> [seed]");
        eprintln!("  Simulates a full LCK-style season from a JSON data pack.");
        process::exit(1);
    });

    let seed: u64 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);

    // Load and validate data pack
    let json = fs::read_to_string(&datapack_path).unwrap_or_else(|e| {
        eprintln!("Error reading {datapack_path}: {e}");
        process::exit(1);
    });

    let pack = DataPack::from_json(&json).unwrap_or_else(|e| {
        eprintln!("Error parsing data pack: {e}");
        process::exit(1);
    });

    if let Err(e) = pack.validate() {
        eprintln!("Data pack validation failed: {e}");
        process::exit(1);
    }

    println!("=== eSports Manager — Season Simulator ===");
    println!();
    println!("Data pack: {datapack_path}");
    println!("Teams: {}", pack.teams.len());
    println!("Players: {}", pack.players.len());
    println!("Champions: {}", pack.champions.len());
    println!("Seed: {seed}");
    println!();

    // Build teams from data pack
    let teams = build_teams_from_pack(&pack);
    let team_names: Vec<String> = teams.iter().map(|t| t.name().to_string()).collect();

    // Create game state (player manages first team)
    let manager = Manager::new(
        "Player".to_string(),
        "Human".to_string(),
        "Manager".to_string(),
        "KR".to_string(),
        ManagerArchetype::Balanced,
    );

    let mut state = GameState::new(2025, seed, manager, 0, teams);

    // Create double round-robin tournament (Bo1) starting on day 3
    let mut tournament = Tournament::new(
        "LCK Spring 2025".to_string(),
        team_names,
        TournamentFormat::DoubleRoundRobin,
        BracketKind::Bo1,
        3,
    );

    let total_matches = tournament.schedule().matches().len();
    println!(
        "Tournament: {} (Double Round Robin, Bo1)",
        tournament.name()
    );
    println!("Total matches: {total_matches}");
    println!();
    println!("--- Simulating season... ---");
    println!();

    // Run the season
    let max_days = 120;
    let mut matches_played = 0;

    for _ in 0..max_days {
        TurnProcessor::end_day(&mut state).unwrap();
        let day = state.calendar().days_elapsed();

        let todays: Vec<_> = tournament
            .matches_today(day)
            .iter()
            .map(|m| (m.id(), m.blue_team_idx(), m.red_team_idx()))
            .collect();

        for (match_id, blue_idx, red_idx) in todays {
            let blue_name = state.teams()[blue_idx].name().to_string();
            let red_name = state.teams()[red_idx].name().to_string();
            let blue_power = compute_team_power(&state.teams()[blue_idx]);
            let red_power = compute_team_power(&state.teams()[red_idx]);

            let result = MatchSimulator::simulate(state.rng_mut(), blue_power, red_power);

            let (bw, rw, winner_name) = match result.winner {
                TeamSide::Blue => (1u32, 0u32, &blue_name),
                TeamSide::Red => (0u32, 1u32, &red_name),
            };

            tournament.record_result(match_id, bw, rw);
            matches_played += 1;

            println!(
                "  Day {:>3} | {} vs {} — {} wins ({} min, {} events)",
                day,
                blue_name,
                red_name,
                winner_name,
                result.duration_minutes,
                result.events.len()
            );
        }

        if tournament.is_complete() {
            break;
        }
    }

    // Print results
    println!();
    println!("=== Season Complete ===");
    println!();
    println!("Days played: {}", state.calendar().days_elapsed());
    println!("Matches played: {matches_played}");
    println!();
    println!("--- Final Standings ---");
    println!();
    println!("  {:<4} {:<16} {:>4} {:>4} {:>6}", "Rank", "Team", "W", "L", "Win%");
    println!("  {}", "-".repeat(38));

    for (i, standing) in tournament.standings().iter().enumerate() {
        let total = standing.wins + standing.losses;
        let win_pct = if total > 0 {
            standing.wins as f64 / total as f64 * 100.0
        } else {
            0.0
        };
        println!(
            "  {:<4} {:<16} {:>4} {:>4} {:>5.1}%",
            i + 1,
            standing.team_name,
            standing.wins,
            standing.losses,
            win_pct,
        );
    }

    println!();
    println!("Your team ({}) finished at rank {}.",
        state.player_team().name(),
        tournament.standings().iter().position(|s| s.team_idx == state.player_team_index()).map(|p| p + 1).unwrap_or(0),
    );
}

fn build_teams_from_pack(pack: &DataPack) -> Vec<Team> {
    use esm_models::player::{
        BoundedAttribute, MentalAttributes, PhysicalAttributes, Player, PlayerAttributes, Role,
        TechnicalAttributes,
    };

    let mut teams = Vec::new();

    for team_data in &pack.teams {
        let players: Vec<Player> = pack
            .players
            .iter()
            .filter(|p| p.team == team_data.name)
            .map(|p| {
                let role = match p.role.as_str() {
                    "Top" => Role::Top,
                    "Jungle" => Role::Jungle,
                    "Mid" => Role::Mid,
                    "Bot" => Role::Bot,
                    "Support" => Role::Support,
                    _ => Role::Mid,
                };

                Player::new(
                    p.nickname.clone(),
                    p.first_name.clone(),
                    p.last_name.clone(),
                    role,
                    PlayerAttributes {
                        physical: PhysicalAttributes {
                            endurance: BoundedAttribute::new(p.endurance),
                            reaction_time: BoundedAttribute::new(p.reaction_time),
                        },
                        mental: MentalAttributes {
                            decision_making: BoundedAttribute::new(p.decision_making),
                            clutch: BoundedAttribute::new(p.clutch),
                            discipline: BoundedAttribute::new(p.discipline),
                            tilt_resistance: BoundedAttribute::new(p.tilt_resistance),
                        },
                        technical: TechnicalAttributes {
                            mechanics: BoundedAttribute::new(p.mechanics),
                            vision_control: BoundedAttribute::new(p.vision_control),
                            teamfighting: BoundedAttribute::new(p.teamfighting),
                        },
                    },
                )
            })
            .collect();

        teams.push(Team::new(
            team_data.name.clone(),
            team_data.tag.clone(),
            players,
        ));
    }

    teams
}
