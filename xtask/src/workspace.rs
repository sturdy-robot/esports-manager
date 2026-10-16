//! The workspace as `cargo metadata` describes it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand};

/// Workspace members that are tooling, not part of the game's architecture.
pub const TOOLING: [&str; 1] = ["xtask"];

pub fn metadata(root: &Path) -> Result<Metadata> {
    MetadataCommand::new()
        .manifest_path(root.join("Cargo.toml"))
        .other_options(vec!["--locked".to_owned()])
        .exec()
        .context("running cargo metadata")
}

/// The directory of every workspace member, by package name.
pub fn member_dirs(metadata: &Metadata) -> BTreeMap<String, PathBuf> {
    metadata
        .workspace_packages()
        .into_iter()
        .filter_map(|package| {
            let dir = package.manifest_path.parent()?;
            Some((package.name.to_string(), dir.as_std_path().to_path_buf()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_dirs_lists_every_member_with_its_directory() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let dirs = member_dirs(&metadata(&root).unwrap());
        let root = root.canonicalize().unwrap();
        assert_eq!(dirs["domain"], root.join("crates/domain"));
        assert_eq!(dirs["xtask"], root.join("xtask"));
    }
}
