mod app_state;
mod commands;
mod helpers;
mod types;

use app_state::AppState;
use commands::{
    advance_turn, apply_player_talk, auto_draft_complete, cancel_scrim, clear_schedule_slot,
    delete_save, draft_hover, draft_lock, draft_swap_picks, get_draft_state, get_game_info,
    get_inbox, get_roster, get_roster_state, get_schedule, get_scrims_list, get_series_info,
    get_standings, get_tactics, get_team_schedule, greet, list_saves, load_datapack, load_save,
    new_game, play_match_delegate, resolve_message, save_game, schedule_rest, schedule_scrim,
    schedule_solo_queue, set_tactics, simulate_match, start_draft,
};
use helpers::default_saves_dir;

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Workaround for WebKitGTK DMABuf rendering issues on Wayland (Linux)
    #[cfg(target_os = "linux")]
    {
        if std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
    }
    let saves_dir = default_saves_dir();

    tauri::Builder::default()
        .manage(AppState::new(saves_dir))
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
        .invoke_handler(tauri::generate_handler![
            greet,
            load_datapack,
            list_saves,
            new_game,
            load_save,
            delete_save,
            save_game,
            advance_turn,
            play_match_delegate,
            get_roster,
            get_inbox,
            resolve_message,
            get_game_info,
            get_standings,
            get_schedule,
            start_draft,
            draft_hover,
            draft_lock,
            get_draft_state,
            simulate_match,
            get_series_info,
            set_tactics,
            get_tactics,
            get_roster_state,
            apply_player_talk,
            get_team_schedule,
            schedule_scrim,
            cancel_scrim,
            schedule_solo_queue,
            schedule_rest,
            clear_schedule_slot,
            get_scrims_list,
            auto_draft_complete,
            draft_swap_picks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
