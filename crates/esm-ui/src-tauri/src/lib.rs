use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TeamInfo {
    pub name: String,
    pub tag: String,
    pub player_count: usize,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to eSports Manager.", name)
}

#[tauri::command]
fn load_datapack(path: String) -> Result<Vec<TeamInfo>, String> {
    let json = std::fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {e}"))?;
    let pack =
        esm_data::datapack::DataPack::from_json(&json).map_err(|e| format!("Parse error: {e}"))?;
    pack.validate()
        .map_err(|e| format!("Validation error: {e}"))?;

    let teams: Vec<TeamInfo> = pack
        .teams
        .iter()
        .map(|t| {
            let player_count = pack.players.iter().filter(|p| p.team == t.name).count();
            TeamInfo {
                name: t.name.clone(),
                tag: t.tag.clone(),
                player_count,
            }
        })
        .collect();

    Ok(teams)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, load_datapack])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
