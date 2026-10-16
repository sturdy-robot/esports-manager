//! One process at a time per save (saves spec), through an OS lock on a file
//! next to it: `tigers.esmsave.lock`.
//!
//! The lock lives in its own file, never on the database: on Windows a lock
//! on the `.esmsave` would collide with SQLite's own byte-range locks. The OS
//! releases it when the process ends, so a crash never leaves a save locked.
//! A lock file is never deleted while its save exists: another process may
//! have it open, and deleting it would let a third lock a fresh one.

use std::fs::{File, TryLockError};
use std::path::{Path, PathBuf};

use app::saves::SaveError;

/// The lock file of `path`.
pub(super) fn lock_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".lock");
    PathBuf::from(name)
}

/// Takes the lock of `path`, or fails with `InUse` if another holder has it.
/// The lock lasts as long as the returned file.
pub(super) fn acquire(path: &Path) -> Result<File, SaveError> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path(path))
        .map_err(|e| SaveError::Storage(e.to_string()))?;
    match file.try_lock() {
        Ok(()) => Ok(file),
        Err(TryLockError::WouldBlock) => Err(SaveError::InUse),
        Err(TryLockError::Error(error)) => Err(SaveError::Storage(error.to_string())),
    }
}

/// Whether someone holds the lock of `path`, this process included. A save
/// without a lock file was never opened, so it isn't in use.
pub(super) fn is_held(path: &Path) -> bool {
    let Ok(file) = File::options().read(true).write(true).open(lock_path(path)) else {
        return false;
    };
    matches!(file.try_lock(), Err(TryLockError::WouldBlock))
}
