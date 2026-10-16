//! Reading a package directory into memory.
//!
//! Everything after this works on [`PackageFiles`], so the loader's rules are
//! tested without touching the filesystem.

use std::collections::BTreeMap;
use std::path::Path;

use sha2::{Digest, Sha256};

/// A package's files, keyed by their path relative to the package root with
/// `/` separators. Iteration is in sorted path order, the order the loader
/// reads files in on every OS.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageFiles(BTreeMap<String, Vec<u8>>);

impl PackageFiles {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces a file.
    pub fn insert(&mut self, path: impl Into<String>, contents: impl Into<Vec<u8>>) {
        self.0.insert(path.into(), contents.into());
    }

    /// Reads every file under `root`.
    ///
    /// # Errors
    ///
    /// When a directory or file can't be read, or a path isn't UTF-8.
    pub fn read_dir(root: &Path) -> std::io::Result<Self> {
        let mut files = Self::new();
        read_into(root, "", &mut files)?;
        Ok(files)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &[u8])> {
        self.0
            .iter()
            .map(|(path, contents)| (path.as_str(), contents.as_slice()))
    }

    /// SHA-256 over every path and its contents, in sorted order. Two packages
    /// with the same hash hold the same files.
    #[must_use]
    pub fn content_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for (path, contents) in &self.0 {
            hasher.update((path.len() as u64).to_le_bytes());
            hasher.update(path.as_bytes());
            hasher.update((contents.len() as u64).to_le_bytes());
            hasher.update(contents);
        }
        hasher.finalize().into()
    }
}

fn read_into(dir: &Path, prefix: &str, files: &mut PackageFiles) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().into_string().map_err(|name| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{} is not a UTF-8 file name", name.to_string_lossy()),
            )
        })?;
        let relative = format!("{prefix}{name}");
        if entry.file_type()?.is_dir() {
            read_into(&entry.path(), &format!("{relative}/"), files)?;
        } else {
            files.insert(relative, std::fs::read(entry.path())?);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_nested_files_with_forward_slashes() {
        let root = std::env::temp_dir().join(format!("esm-package-files-{}", std::process::id()));
        std::fs::create_dir_all(root.join("players/kr")).unwrap();
        std::fs::write(root.join("manifest.json"), "{}").unwrap();
        std::fs::write(root.join("players/kr/kairos.json"), "[]").unwrap();

        let files = PackageFiles::read_dir(&root).unwrap();
        std::fs::remove_dir_all(&root).unwrap();

        let paths: Vec<&str> = files.iter().map(|(path, _)| path).collect();
        assert_eq!(paths, ["manifest.json", "players/kr/kairos.json"]);
    }

    #[test]
    fn a_missing_directory_is_an_error() {
        assert!(PackageFiles::read_dir(Path::new("/definitely/not/here")).is_err());
    }

    #[test]
    fn files_iterate_in_sorted_order() {
        let mut files = PackageFiles::new();
        files.insert("teams/b.json", "1");
        files.insert("teams/a.json", "2");
        files.insert("manifest.json", "3");
        let paths: Vec<&str> = files.iter().map(|(path, _)| path).collect();
        assert_eq!(paths, ["manifest.json", "teams/a.json", "teams/b.json"]);
    }

    #[test]
    fn the_hash_covers_paths_and_contents() {
        let mut one = PackageFiles::new();
        one.insert("a.json", "x");
        let mut renamed = PackageFiles::new();
        renamed.insert("b.json", "x");
        let mut edited = PackageFiles::new();
        edited.insert("a.json", "y");
        let mut split = PackageFiles::new();
        split.insert("a.jso", "nx");

        let hashes = [&one, &renamed, &edited, &split].map(PackageFiles::content_hash);
        for (i, a) in hashes.iter().enumerate() {
            for b in &hashes[i + 1..] {
                assert_ne!(a, b);
            }
        }
        assert_eq!(one.content_hash(), one.clone().content_hash());
    }

    /// Pins the hash so a change to it is deliberate: saves record it.
    #[test]
    fn the_hash_is_pinned() {
        let mut files = PackageFiles::new();
        files.insert("manifest.json", "{}");
        let hex = files
            .content_hash()
            .iter()
            .fold(String::new(), |mut hex, byte| {
                use std::fmt::Write;
                let _ = write!(hex, "{byte:02x}");
                hex
            });
        assert_eq!(
            hex,
            "1403ca0b13a95fe4c2d9fbb85ea908c4c61c18856539735b9673ffeffa47df6d"
        );
    }
}
