mod core;
mod draft;
mod matchplay;
mod schedule;
mod team;

pub use core::{
    advance_turn, delete_save, get_game_info, get_inbox, get_roster, get_schedule, get_standings,
    greet, list_saves, load_datapack, load_save, new_game, play_match_delegate, resolve_message,
    save_game,
};
pub(crate) use core::advance_after_match_slot;
pub use draft::{
    auto_draft_complete, draft_hover, draft_lock, draft_swap_picks, get_draft_state, start_draft,
};
pub use matchplay::{get_series_info, simulate_match};
pub use schedule::{
    cancel_scrim, clear_schedule_slot, get_scrims_list, get_team_schedule, schedule_rest,
    schedule_scrim, schedule_solo_queue,
};
pub use team::{apply_player_talk, get_roster_state, get_tactics, set_tactics};
