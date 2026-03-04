use std::fs;
use std::process;

use esm_core::game_state::GameState;
use esm_core::turn::TurnProcessor;
use esm_data::datapack::DataPack;
use esm_engine::moba_match::engine::{MobaMatchConfig, MobaMatchEngine};
use esm_engine::moba_match::state::TeamSide;
use esm_engine::tournament::{BracketKind, Tournament, TournamentFormat};
use esm_models::manager::{Manager, ManagerArchetype};
use esm_models::moba::player::{MobaPlayer, MobaPlayerAttributes, MobaRole, RoleAssignment};
use esm_models::moba::team::MobaTeam;
use esm_models::player::BoundedAttribute;
use esm_models::team::Team;

fn extract_team_attrs(moba_team: &MobaTeam) -> Vec<[u8; 9]> {
    moba_team
        .roster()
        .iter()
        .map(|p| {
            let a = p.attributes();
            [
                a.endurance.value(),
                a.reaction_time.value(),
                a.decision_making.value(),
                a.clutch.value(),
                a.discipline.value(),
                a.tilt_resistance.value(),
                a.mechanics.value(),
                a.vision_control.value(),
                a.teamfighting.value(),
            ]
        })
        .collect()
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

    // Build MOBA teams from data pack
    let moba_teams = build_moba_teams_from_pack(&pack);
    let team_names: Vec<String> = moba_teams.iter().map(|t| t.name().to_string()).collect();

    // Build legacy teams for GameState (still needed for calendar/turn processing)
    let legacy_teams: Vec<Team> = moba_teams
        .iter()
        .map(|mt| Team::new(mt.name().to_string(), mt.tag().to_string(), vec![]))
        .collect();

    let manager = Manager::new(
        "Player".to_string(),
        "Human".to_string(),
        "Manager".to_string(),
        "KR".to_string(),
        ManagerArchetype::Balanced,
    );

    let mut state = GameState::new(2025, seed, manager, 0, legacy_teams);

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

    let config = MobaMatchConfig::default();
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
            let blue_name = moba_teams[blue_idx].name().to_string();
            let red_name = moba_teams[red_idx].name().to_string();
            let blue_attrs = extract_team_attrs(&moba_teams[blue_idx]);
            let red_attrs = extract_team_attrs(&moba_teams[red_idx]);

            let result =
                MobaMatchEngine::simulate(state.rng_mut(), &blue_attrs, &red_attrs, &config);

            let (bw, rw, winner_name) = match result.winner {
                TeamSide::Blue => (1u32, 0u32, blue_name.clone()),
                TeamSide::Red => (0u32, 1u32, red_name.clone()),
            };

            tournament.record_result(match_id, bw, rw);
            matches_played += 1;

            println!(
                "  Day {:>3} | {} vs {} — {} wins ({} min, {} events, B:{} R:{} gold)",
                day,
                blue_name,
                red_name,
                winner_name,
                result.duration_minutes,
                result.events.len(),
                result.blue_team_gold,
                result.red_team_gold,
            );

            // Print key commentary moments
            for event in &result.events {
                if let Some(commentary) = event.commentary() {
                    let text = commentary.text();
                    if text.contains("FIRST BLOOD")
                        || text.contains("BARON")
                        || text.contains("Dragon Soul")
                        || text.contains("ACE")
                        || text.contains("Nexus")
                    {
                        println!("           🎙️ {text}");
                    }
                }
            }

            // Print top performer
            let all_players: Vec<_> = result
                .blue_players
                .iter()
                .chain(result.red_players.iter())
                .collect();
            if let Some(mvp) = all_players.iter().max_by(|a, b| {
                a.kda()
                    .partial_cmp(&b.kda())
                    .unwrap_or(std::cmp::Ordering::Equal)
            }) {
                let side = if mvp.player_index() < 5 {
                    &blue_name
                } else {
                    &red_name
                };
                println!(
                    "           ⭐ MVP: {} Player{} — {:.1} KDA ({}/{}/{}), {} CS, {} gold",
                    side,
                    mvp.player_index() % 5 + 1,
                    mvp.kda(),
                    mvp.kills(),
                    mvp.deaths(),
                    mvp.assists(),
                    mvp.cs(),
                    mvp.gold(),
                );
            }
            println!();
        }

        if tournament.is_complete() {
            break;
        }
    }

    // Print results
    println!("=== Season Complete ===");
    println!();
    println!("Days played: {}", state.calendar().days_elapsed());
    println!("Matches played: {matches_played}");
    println!();
    println!("--- Final Standings ---");
    println!();
    println!(
        "  {:<4} {:<16} {:>4} {:>4} {:>6}",
        "Rank", "Team", "W", "L", "Win%"
    );
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
    println!(
        "Your team ({}) finished at rank {}.",
        moba_teams[state.player_team_index()].name(),
        tournament
            .standings()
            .iter()
            .position(|s| s.team_idx == state.player_team_index())
            .map(|p| p + 1)
            .unwrap_or(0),
    );
}

fn parse_role(s: &str) -> MobaRole {
    match s {
        "Top" => MobaRole::Top,
        "Jungle" => MobaRole::Jungle,
        "Mid" => MobaRole::Mid,
        "Bot" => MobaRole::Bot,
        "Support" => MobaRole::Support,
        _ => MobaRole::Mid,
    }
}

fn build_moba_teams_from_pack(pack: &DataPack) -> Vec<MobaTeam> {
    let mut teams = Vec::new();

    for team_data in &pack.teams {
        let players: Vec<MobaPlayer> = pack
            .players
            .iter()
            .filter(|p| p.team == team_data.name)
            .map(|p| {
                let primary = parse_role(&p.role);
                let secondary: Vec<MobaRole> =
                    p.secondary_roles.iter().map(|r| parse_role(r)).collect();

                MobaPlayer::new(
                    p.nickname.clone(),
                    p.first_name.clone(),
                    p.last_name.clone(),
                    RoleAssignment::new(primary, secondary),
                    MobaPlayerAttributes {
                        endurance: BoundedAttribute::new(p.endurance),
                        reaction_time: BoundedAttribute::new(p.reaction_time),
                        decision_making: BoundedAttribute::new(p.decision_making),
                        clutch: BoundedAttribute::new(p.clutch),
                        discipline: BoundedAttribute::new(p.discipline),
                        tilt_resistance: BoundedAttribute::new(p.tilt_resistance),
                        mechanics: BoundedAttribute::new(p.mechanics),
                        vision_control: BoundedAttribute::new(p.vision_control),
                        teamfighting: BoundedAttribute::new(p.teamfighting),
                    },
                )
            })
            .collect();

        teams.push(MobaTeam::new(
            team_data.name.clone(),
            team_data.tag.clone(),
            players,
        ));
    }

    teams
}
