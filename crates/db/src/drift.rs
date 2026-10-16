//! `schema.rs` must match the migrations (ADR 0004). Diesel's CLI writes it
//! (`just db-schema`); this test compares the committed copy with a database
//! migrated from scratch, so CI needs no CLI.

use std::collections::BTreeMap;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::MigrationHarness;

use crate::store::MIGRATIONS;

/// Table → its columns as `(name, Diesel type)`, in order.
type Tables = BTreeMap<String, Vec<(String, String)>>;

/// Reads the `diesel::table!` blocks of `schema.rs`.
fn committed(source: &str) -> Tables {
    let mut tables = Tables::new();
    let mut current: Option<String> = None;
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(name) = current.as_ref() {
            if trimmed == "}" {
                current = None;
            } else if let Some((column, kind)) = trimmed.split_once(" -> ") {
                let entry = tables.entry(name.clone()).or_default();
                entry.push((column.to_owned(), kind.trim_end_matches(',').to_owned()));
            }
        } else if line.starts_with("    ")
            && trimmed.ends_with('{')
            && !trimmed.starts_with("diesel")
        {
            current = trimmed.split_whitespace().next().map(str::to_owned);
        }
    }
    tables
}

#[derive(QueryableByName)]
struct Column {
    #[diesel(sql_type = diesel::sql_types::Text)]
    name: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    #[diesel(column_name = type_)]
    kind: String,
    #[diesel(sql_type = diesel::sql_types::Integer)]
    notnull: i32,
}

#[derive(QueryableByName)]
struct Table {
    #[diesel(sql_type = diesel::sql_types::Text)]
    name: String,
}

/// The tables of a database migrated from nothing, with SQL types mapped as
/// Diesel's CLI maps them for SQLite.
fn migrated() -> Tables {
    let mut connection = SqliteConnection::establish(":memory:").unwrap();
    connection.run_pending_migrations(MIGRATIONS).unwrap();
    let names: Vec<Table> = diesel::sql_query(
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' \
         AND name != '__diesel_schema_migrations' ORDER BY name",
    )
    .load(&mut connection)
    .unwrap();
    let mut tables = Tables::new();
    for Table { name } in names {
        let columns: Vec<Column> = diesel::sql_query(format!(
            "SELECT name, type AS type_, \"notnull\" FROM pragma_table_info('{name}') ORDER BY cid"
        ))
        .load(&mut connection)
        .unwrap();
        let columns = columns
            .into_iter()
            .map(|column| {
                let kind = match column.kind.as_str() {
                    "INTEGER" => "Integer",
                    "BIGINT" => "BigInt",
                    "TEXT" => "Text",
                    "BLOB" => "Binary",
                    "BOOLEAN" => "Bool",
                    other => panic!("no Diesel mapping for SQL type {other}"),
                };
                let kind = if column.notnull == 1 {
                    kind.to_owned()
                } else {
                    format!("Nullable<{kind}>")
                };
                (column.name, kind)
            })
            .collect();
        tables.insert(name, columns);
    }
    tables
}

#[test]
fn schema_rs_matches_the_migrations() {
    let source = include_str!("schema.rs");
    assert_eq!(
        committed(source),
        migrated(),
        "crates/db/src/schema.rs is out of date; run `just db-schema` and commit the result"
    );
}

#[test]
fn the_reader_understands_table_blocks() {
    let source = "diesel::table! {\n    a (id) {\n        id -> Text,\n        n -> Nullable<Integer>,\n    }\n}\n";
    let tables = committed(source);
    assert_eq!(
        tables["a"],
        [
            ("id".to_owned(), "Text".to_owned()),
            ("n".to_owned(), "Nullable<Integer>".to_owned())
        ]
    );
}
