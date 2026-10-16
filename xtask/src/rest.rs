//! No rest patterns in package conversion modules (ADR 0005).
//!
//! The conversion from package types to domain types destructures every
//! definition exhaustively, so a field can't be validated and then ignored.
//! A rest pattern (`let TeamDef { id, .. } = def;`) would silently let a new
//! field through, so it is banned in the files `quality.toml` lists.

use std::path::Path;

use anyhow::Result;

use crate::files;

pub fn run(root: &Path, conversion_files: &[String]) -> Result<Vec<String>> {
    let mut violations = Vec::new();
    for file in conversion_files {
        let text = files::read(&root.join(file))?;
        for line in find_rest_patterns(&text) {
            violations.push(format!(
                "{file}:{line}: rest pattern `..` in a package conversion module; bind every field (ADR 0005)"
            ));
        }
    }
    Ok(violations)
}

/// Line numbers where `..` is followed, possibly on a later line, by `}`,
/// `)`, `]` or `,`: a rest pattern. Ranges (`0..2`, `..=`) and struct update
/// syntax are followed by something else. Comments and string literals are
/// ignored.
pub fn find_rest_patterns(text: &str) -> Vec<usize> {
    let lines: Vec<String> = text.lines().map(code_only).collect();
    let mut found = Vec::new();
    for (number, line) in lines.iter().enumerate() {
        let is_rest = line.match_indices("..").any(|(index, _)| {
            let after = std::iter::once(&line[index + 2..])
                .chain(lines[number + 1..].iter().map(String::as_str));
            after
                .flat_map(str::chars)
                .find(|c| !c.is_whitespace())
                .is_some_and(|next| matches!(next, '}' | ')' | ']' | ','))
        });
        if is_rest {
            found.push(number + 1);
        }
    }
    found
}

/// The line without its `//` comment and with string literals emptied.
fn code_only(line: &str) -> String {
    let mut code = String::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
                code.push('"');
            }
            continue;
        }
        match c {
            '"' => {
                in_string = true;
                code.push('"');
            }
            '/' if chars.peek() == Some(&'/') => break,
            _ => code.push(c),
        }
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_struct_tuple_and_slice_rest_patterns() {
        let text = "let TeamDef { id, .. } = def;\nlet Pair(a, ..) = pair;\nlet [first, ..] = list;\nlet (a, .., z) = t;\n";
        assert_eq!(find_rest_patterns(text), [1, 2, 3, 4]);
    }

    #[test]
    fn finds_a_rest_pattern_on_its_own_line() {
        let text = "let TeamDef {\n    id,\n    ..\n} = def;\n";
        assert_eq!(find_rest_patterns(text), [3]);
        let text = "let TeamDef {\n    id, ..\n} = def;\n";
        assert_eq!(find_rest_patterns(text), [2]);
    }

    #[test]
    fn ranges_and_struct_updates_are_not_rest_patterns() {
        let text = "for i in 0..2 {}\nlet r = 1..=3;\nlet s = &x[2..4];\nlet t = Foo { a, ..Foo::default() };\n";
        assert_eq!(find_rest_patterns(text), Vec::<usize>::new());
    }

    #[test]
    fn comments_and_strings_are_ignored() {
        let text = "// let A { .. } = a;\nlet s = \"{ .. }\"; // ..}\nlet t = \"a\\\" .. }\";\n";
        assert_eq!(find_rest_patterns(text), Vec::<usize>::new());
    }

    #[test]
    fn the_repository_conversion_module_is_clean() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let violations = run(&root, &["crates/packages/src/convert.rs".to_owned()]).unwrap();
        assert_eq!(violations, Vec::<String>::new());
    }
}
