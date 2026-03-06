use crate::datapack::{DataPack, PlayerData};
use rand::RngExt;
use rand::seq::IteratorRandom;

pub fn generate_player(pack: &DataPack, role: &str, team: &str) -> PlayerData {
    let mut rng = rand::rng();

    // Default names if no data available
    let mut first_name = String::from("Generated");
    let mut last_name = String::from("Player");
    let mut nickname = String::from("Player");

    if !pack.player_names.is_empty() {
        if let Some((_nat, names)) = pack.player_names.iter().choose(&mut rng) {
            if !names.first_names.is_empty() {
                first_name = names.first_names.iter().choose(&mut rng).unwrap().clone();
            }
            if !names.last_names.is_empty() {
                last_name = names.last_names.iter().choose(&mut rng).unwrap().clone();
            }
        }
    }

    if !pack.player_nicknames.is_empty() {
        nickname = pack.player_nicknames.iter().choose(&mut rng).unwrap().clone();
    }

    let mut gen_attr = || -> u8 {
        rng.random_range(30..=95)
    };

    PlayerData {
        nickname,
        first_name,
        last_name,
        role: role.to_string(),
        secondary_roles: vec![],
        team: team.to_string(),
        endurance: gen_attr(),
        reaction_time: gen_attr(),
        decision_making: gen_attr(),
        clutch: gen_attr(),
        discipline: gen_attr(),
        tilt_resistance: gen_attr(),
        mechanics: gen_attr(),
        vision_control: gen_attr(),
        teamfighting: gen_attr(),
    }
}
