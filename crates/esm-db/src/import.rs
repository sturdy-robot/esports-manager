use std::collections::HashMap;

use esm_data::datapack::DataPack;
use esm_models::moba::player::{MobaPlayer, MobaPlayerAttributes, MobaRole, RoleAssignment};
use esm_models::moba::team::MobaTeam;
use esm_models::player::{
    BoundedAttribute, MentalAttributes, PhysicalAttributes, Player, PlayerAttributes, Role,
    TechnicalAttributes,
};
use esm_models::team::Team;
use rusqlite::{params, Connection};

/// Build domain `Team` objects (with full `Player` rosters) from a validated
/// `DataPack`. This is used by the new-game flow to create a `GameState`
/// without touching the database.
pub fn build_teams_from_datapack(pack: &DataPack) -> Vec<Team> {
    let mut teams: Vec<Team> = Vec::new();

    for td in &pack.teams {
        let roster: Vec<Player> = pack
            .players
            .iter()
            .filter(|p| p.team == td.name)
            .map(|p| {
                let role = parse_role(&p.role);
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

        teams.push(Team::new(td.name.clone(), td.tag.clone(), roster));
    }

    teams
}

/// Build MOBA-specific `MobaTeam` objects from a validated `DataPack`.
/// These contain the full attribute sets needed for match simulation.
pub fn build_moba_teams_from_datapack(pack: &DataPack) -> Vec<MobaTeam> {
    let mut teams = Vec::new();

    for td in &pack.teams {
        let players: Vec<MobaPlayer> = pack
            .players
            .iter()
            .filter(|p| p.team == td.name)
            .map(|p| {
                let primary = parse_moba_role(&p.role);
                let secondary: Vec<MobaRole> = p
                    .secondary_roles
                    .iter()
                    .map(|r| parse_moba_role(r))
                    .collect();

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

        teams.push(MobaTeam::new(td.name.clone(), td.tag.clone(), players));
    }

    teams
}

fn parse_moba_role(s: &str) -> MobaRole {
    match s {
        "Top" => MobaRole::Top,
        "Jungle" => MobaRole::Jungle,
        "Mid" => MobaRole::Mid,
        "Bot" => MobaRole::Bot,
        "Support" => MobaRole::Support,
        _ => MobaRole::Mid,
    }
}

fn parse_role(s: &str) -> Role {
    match s.to_lowercase().as_str() {
        "top" => Role::Top,
        "jungle" | "jg" => Role::Jungle,
        "mid" | "middle" => Role::Mid,
        "bot" | "adc" => Role::Bot,
        "support" | "sup" => Role::Support,
        _ => Role::Mid, // fallback
    }
}

/// Imports a validated DataPack into the database.
pub struct DataPackImporter;

impl DataPackImporter {
    /// Validate and import a full data pack into the database.
    ///
    /// Inserts teams first (to get their IDs), then players referencing
    /// those teams, then champions and their tags. All operations run
    /// inside a transaction.
    pub fn import(conn: &Connection, pack: &DataPack) -> Result<(), Box<dyn std::error::Error>> {
        pack.validate()?;

        let tx = conn.unchecked_transaction()?;

        // Insert teams and build a name→id lookup
        let team_ids = Self::insert_teams(&tx, pack)?;

        // Insert players with team_id references
        Self::insert_players(&tx, pack, &team_ids)?;

        // Insert champions and their tags
        Self::insert_champions(&tx, pack)?;

        tx.commit()?;
        Ok(())
    }

    fn insert_teams(
        conn: &Connection,
        pack: &DataPack,
    ) -> Result<HashMap<String, i64>, rusqlite::Error> {
        let mut map = HashMap::new();
        let mut next_id: i64 = 1;

        for team in &pack.teams {
            conn.execute(
                "INSERT INTO moba_teams (id, name, tag, budget) VALUES (?1, ?2, ?3, ?4)",
                params![next_id, team.name, team.tag, team.budget],
            )?;
            map.insert(team.name.clone(), next_id);
            next_id += 1;
        }

        Ok(map)
    }

    fn insert_players(
        conn: &Connection,
        pack: &DataPack,
        team_ids: &HashMap<String, i64>,
    ) -> Result<(), rusqlite::Error> {
        let mut next_id: i64 = 1;

        for player in &pack.players {
            let team_id = team_ids.get(&player.team).copied();

            conn.execute(
                "INSERT INTO moba_players (id, nickname, first_name, last_name, primary_role,
                 endurance, reaction_time, decision_making, clutch, discipline,
                 tilt_resistance, mechanics, vision_control, teamfighting, team_id)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                params![
                    next_id,
                    player.nickname,
                    player.first_name,
                    player.last_name,
                    player.role,
                    player.endurance,
                    player.reaction_time,
                    player.decision_making,
                    player.clutch,
                    player.discipline,
                    player.tilt_resistance,
                    player.mechanics,
                    player.vision_control,
                    player.teamfighting,
                    team_id,
                ],
            )?;
            next_id += 1;
        }

        Ok(())
    }

    fn insert_champions(conn: &Connection, pack: &DataPack) -> Result<(), rusqlite::Error> {
        let mut next_id: i64 = 1;

        for champ in &pack.champions {
            conn.execute(
                "INSERT INTO moba_champions (id, name, class, scaling) VALUES (?1, ?2, ?3, ?4)",
                params![next_id, champ.name, champ.class, champ.scaling],
            )?;

            for tag in &champ.tags {
                conn.execute(
                    "INSERT INTO moba_champion_tags (champion_id, tag) VALUES (?1, ?2)",
                    params![next_id, tag],
                )?;
            }

            next_id += 1;
        }

        Ok(())
    }
}
