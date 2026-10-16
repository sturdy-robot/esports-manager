//! Headless adapter: play, script and benchmark the game without the UI
//! (ADR 0001). It translates arguments, calls use cases in `app`, and prints.

/// The line printed by `esm --version`.
#[must_use]
pub fn version_line() -> String {
    format!("esm {}", env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_line_names_the_binary_and_version() {
        assert_eq!(version_line(), format!("esm {}", env!("CARGO_PKG_VERSION")));
    }
}
