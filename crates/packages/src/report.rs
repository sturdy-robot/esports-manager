//! What the loader reports about a broken package (packages spec).
//!
//! Errors are data: a file, a path inside it and a typed problem. The UI and
//! the CLI render them through Fluent; the loader never produces prose. The
//! one exception is `InvalidValue`, which carries serde's own description of
//! a malformed value.

use domain::id::ContentId;
use esports::moba::roles::RoleSetError;

/// One thing wrong with a package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageError {
    /// The file, relative to the package root, with `/` separators.
    pub file: String,
    /// Where in the file: `[2].reputation`, or empty for the whole file.
    pub path: String,
    pub problem: Problem,
}

/// Which name pool of a nationality is empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameKind {
    Given,
    Family,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Problem {
    /// The package root can't be read.
    Unreadable {
        reason: String,
    },
    /// A required file, such as `manifest.json`, is missing.
    MissingFile,
    /// A top-level folder that names no content type.
    UnknownFolder,
    /// A file that doesn't belong where it is.
    UnexpectedFile,
    /// The file isn't valid JSON.
    Syntax {
        line: usize,
        column: usize,
    },
    /// A file's top level is neither an object nor a list.
    NotAnEntity,
    UnknownField {
        field: String,
    },
    MissingField {
        field: String,
    },
    InvalidValue {
        detail: String,
    },
    /// A field whose text doesn't have the required form (a date, a colour,
    /// a tag, a country code, a version).
    InvalidFormat,
    /// A number outside its allowed range.
    OutOfRange,
    InvalidId {
        id: String,
    },
    WrongKind {
        id: ContentId,
        expected: String,
    },
    DuplicateId {
        id: ContentId,
        other_file: String,
    },
    UnresolvedReference {
        field: String,
        id: String,
    },
    /// A type the package must define exactly once.
    WrongCount {
        kind: String,
        count: usize,
    },
    UnsupportedGenre {
        genre: String,
    },
    UnknownLocale {
        locale: String,
    },
    TextSyntax {
        line: usize,
    },
    MissingText {
        message: String,
        locale: String,
    },
    InvalidMix {
        region: ContentId,
    },
    EmptyNamePool {
        nationality: ContentId,
        names: NameKind,
    },
    RoleSet(RoleSetError),
    TooManySubs {
        subs: u8,
        roles: usize,
    },
    DuplicateNickname {
        nickname: String,
        other: ContentId,
    },
    NicknamePoolTooSmall {
        distinct: usize,
        needed: usize,
    },
    IncompletableRoster {
        team: ContentId,
        missing_roles: Vec<ContentId>,
    },
}

impl PackageError {
    pub(crate) fn new(file: impl Into<String>, path: impl Into<String>, problem: Problem) -> Self {
        Self {
            file: file.into(),
            path: path.into(),
            problem,
        }
    }
}
