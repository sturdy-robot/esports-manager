//! Tauri commands. Commands live in a module, not at the crate root, because
//! the command macros re-export helper items that clash at the root.

#[tauri::command]
#[specta::specta]
pub fn get_about() -> app::About {
    app::about()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_about_returns_the_app_layer_answer() {
        assert_eq!(get_about(), app::about());
    }
}
