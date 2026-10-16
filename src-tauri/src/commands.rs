//! Tauri commands. Commands live in a module, not at the crate root, because
//! the command macros re-export helper items that clash at the root. Each one
//! locks the desktop state and makes one call into it.

#![allow(
    clippy::needless_pass_by_value,
    reason = "Tauri hands command arguments over by value"
)]

use std::sync::{Mutex, MutexGuard};

use app::content::ContentText;
use app::messages::Message;
use app::views::{Failure, GameOpened, NewGameOptions, PlayerProfile, RosterView, SaveSummary};
use tauri::State;

use crate::desktop::{Desktop, NewGameInput};

type DesktopState<'a> = State<'a, Mutex<Desktop>>;

/// The desktop state. A command that panicked while holding it can't have
/// left it half-changed (every change is one assignment), so the lock is
/// taken back rather than failing every later command.
fn lock<'a>(state: &'a DesktopState<'_>) -> MutexGuard<'a, Desktop> {
    state
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// A seed from OS entropy, used when New Game doesn't give one.
fn random_seed() -> Result<u64, Failure> {
    db::environment::random_seed().map_err(|e| Failure {
        messages: vec![Message::new("error-entropy").text("detail", e.to_string())],
    })
}

#[tauri::command]
#[specta::specta]
pub fn get_about() -> app::About {
    app::about()
}

// `async` runs it on Tauri's thread pool instead of the UI thread, so
// generating a world doesn't freeze the window.
#[tauri::command(async)]
#[specta::specta]
pub fn get_new_game_options(state: DesktopState<'_>) -> Result<NewGameOptions, Failure> {
    lock(&state).new_game_options()
}

// `async` runs it on Tauri's thread pool instead of the UI thread, so
// generating a world doesn't freeze the window.
#[tauri::command(async)]
#[specta::specta]
pub fn get_content_text(state: DesktopState<'_>) -> Result<Vec<ContentText>, Failure> {
    lock(&state).content_text()
}

// `async` runs it on Tauri's thread pool instead of the UI thread, so
// generating a world doesn't freeze the window.
#[tauri::command(async)]
#[specta::specta]
pub fn start_new_game(state: DesktopState<'_>, input: NewGameInput) -> Result<GameOpened, Failure> {
    lock(&state).start_new_game(input, random_seed()?, db::environment::now())
}

// `async` runs it on Tauri's thread pool instead of the UI thread, so
// generating a world doesn't freeze the window.
#[tauri::command(async)]
#[specta::specta]
pub fn list_saves(state: DesktopState<'_>) -> Result<Vec<SaveSummary>, Failure> {
    lock(&state).list_saves()
}

// `async` runs it on Tauri's thread pool instead of the UI thread, so
// generating a world doesn't freeze the window.
#[tauri::command(async)]
#[specta::specta]
pub fn open_save(state: DesktopState<'_>, file: String) -> Result<GameOpened, Failure> {
    lock(&state).open_save(&file, db::environment::now())
}

// `async` runs it on Tauri's thread pool instead of the UI thread, so
// generating a world doesn't freeze the window.
#[tauri::command(async)]
#[specta::specta]
pub fn get_roster(state: DesktopState<'_>, team: Option<String>) -> Result<RosterView, Failure> {
    lock(&state).roster(team.as_deref())
}

// `async` runs it on Tauri's thread pool instead of the UI thread, so
// generating a world doesn't freeze the window.
#[tauri::command(async)]
#[specta::specta]
pub fn get_player(state: DesktopState<'_>, id: String) -> Result<PlayerProfile, Failure> {
    lock(&state).player(&id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_about_returns_the_app_layer_answer() {
        assert_eq!(get_about(), app::about());
    }

    #[test]
    fn random_seeds_differ() {
        assert_ne!(random_seed().unwrap(), random_seed().unwrap());
    }
}
