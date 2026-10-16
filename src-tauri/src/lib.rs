//! Desktop adapter (ADR 0001). Each command translates its input, calls one
//! use case in `app`, and returns a view DTO. No game rules live here.

mod commands;

/// The command set, also used to generate the TypeScript bindings (ADR 0006).
fn specta_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![commands::get_about])
}

/// Starts the desktop app.
///
/// # Errors
///
/// Returns an error if Tauri fails to start the window or event loop.
// `generate_context!` expands to code using `HashMap`; it is Tauri's own
// asset table, not game state, so ADR 0003 doesn't apply.
#[allow(clippy::disallowed_types)]
pub fn run() -> tauri::Result<()> {
    let builder = specta_builder();
    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// The committed bindings must match the commands (ADR 0006). Set
    /// `UPDATE_BINDINGS=1` to rewrite them (`just bindings`).
    #[test]
    fn typescript_bindings_are_up_to_date() {
        let committed = Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/bindings.ts");
        let generated =
            std::env::temp_dir().join(format!("esm-bindings-{}.ts", std::process::id()));
        super::specta_builder()
            .export(specta_typescript::Typescript::default(), &generated)
            .unwrap();
        let generated_text = std::fs::read_to_string(&generated).unwrap();
        std::fs::remove_file(&generated).unwrap();

        if std::env::var_os("UPDATE_BINDINGS").is_some() {
            std::fs::write(&committed, &generated_text).unwrap();
        }
        let committed_text = std::fs::read_to_string(&committed).unwrap_or_default();
        assert!(
            committed_text == generated_text,
            "src/bindings.ts is out of date; run `just bindings` and commit the result"
        );
    }
}
