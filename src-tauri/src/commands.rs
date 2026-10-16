//! Tauri commands. Commands live in a module, not at the crate root, because
//! the command macros re-export helper items that clash at the root. Each one
//! locks the desktop state and makes one call into it.

#![allow(
    clippy::needless_pass_by_value,
    reason = "Tauri hands command arguments over by value"
)]

use std::sync::{Mutex, MutexGuard};

use app::messages::Message;
use app::saves::{ContentText, Timestamp};
use app::views::{Failure, NewGameOptions, PlayerProfile, RosterView, SaveSummary};
use tauri::State;

use crate::desktop::{Desktop, GameOpened, NewGameInput};

type DesktopState<'a> = State<'a, Mutex<Desktop>>;

fn lock<'a>(state: &'a DesktopState<'_>) -> Result<MutexGuard<'a, Desktop>, Failure> {
    state.lock().map_err(|_| Failure {
        messages: vec![Message::new("error-storage").text("detail", "the game state is poisoned")],
    })
}

/// Real time, for save timestamps. Adapters are the only place it is read
/// (ADR 0003).
fn now() -> Timestamp {
    #[allow(
        clippy::disallowed_methods,
        reason = "save timestamps are real time, read only by adapters (ADR 0003)"
    )]
    let elapsed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    Timestamp(i64::try_from(elapsed).unwrap_or(i64::MAX))
}

/// A seed from OS entropy, used when New Game doesn't give one.
fn random_seed() -> Result<u64, Failure> {
    getrandom::u64().map_err(|e| Failure {
        messages: vec![Message::new("error-storage").text("detail", e.to_string())],
    })
}

#[tauri::command]
#[specta::specta]
pub fn get_about() -> app::About {
    app::about()
}

#[tauri::command]
#[specta::specta]
pub fn get_new_game_options(state: DesktopState<'_>) -> Result<NewGameOptions, Failure> {
    lock(&state)?.new_game_options()
}

#[tauri::command]
#[specta::specta]
pub fn get_content_text(state: DesktopState<'_>) -> Result<Vec<ContentText>, Failure> {
    lock(&state)?.content_text()
}

#[tauri::command]
#[specta::specta]
pub fn start_new_game(state: DesktopState<'_>, input: NewGameInput) -> Result<GameOpened, Failure> {
    lock(&state)?.start_new_game(input, random_seed()?, now())
}

#[tauri::command]
#[specta::specta]
pub fn list_saves(state: DesktopState<'_>) -> Result<Vec<SaveSummary>, Failure> {
    lock(&state)?.list_saves()
}

#[tauri::command]
#[specta::specta]
pub fn open_save(state: DesktopState<'_>, file: String) -> Result<GameOpened, Failure> {
    lock(&state)?.open_save(&file, now())
}

#[tauri::command]
#[specta::specta]
pub fn get_roster(state: DesktopState<'_>, team: Option<String>) -> Result<RosterView, Failure> {
    lock(&state)?.roster(team.as_deref())
}

#[tauri::command]
#[specta::specta]
pub fn get_player(state: DesktopState<'_>, id: String) -> Result<PlayerProfile, Failure> {
    lock(&state)?.player(&id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_about_returns_the_app_layer_answer() {
        assert_eq!(get_about(), app::about());
    }

    #[test]
    fn now_is_after_2026() {
        assert!(now().0 > 1_767_225_600);
    }

    #[test]
    fn random_seeds_differ() {
        assert_ne!(random_seed().unwrap(), random_seed().unwrap());
    }
}
