//! Headless adapter: play, script and benchmark the game without the UI
//! (ADR 0001). It translates arguments, calls use cases in `app`, and prints.
//!
//! Exit codes: 0 on success, 1 when a use case fails, 2 on bad arguments
//! (clap's own).

mod text;

use std::io::Write;
use std::path::PathBuf;

use app::game::{self, NewGameRequest};
use app::messages::{Message, package_error};
use app::saves::{SaveStore, Timestamp};
use app::views::{DamageView, RosterPlayer, SaveSummary};
use clap::{Parser, Subcommand};
use db::SqliteSaveStore;
use packages::PackageDir;
use serde::Serialize;

pub use text::Text;

/// The line `esm --version` prints.
#[must_use]
pub fn version_line() -> String {
    format!("esm {}", app::about().version)
}

#[derive(Debug, Parser)]
// clap's help is for developers and stays in English; game output goes
// through Fluent. The version is the workspace's, as `app::about` reports.
#[command(name = "esm", version, about = "eSports Manager, without the UI")]
pub struct Cli {
    /// Keep saves in this folder instead of the usual one.
    #[arg(long, global = true)]
    pub data_dir: Option<PathBuf>,
    /// The package to build new worlds from.
    #[arg(long, global = true, default_value = "content/core")]
    pub content: PathBuf,
    /// The language of text output: en or pt-BR.
    #[arg(long, global = true, default_value = "en")]
    pub locale: String,
    /// Print the view as JSON instead of text.
    #[arg(long, global = true)]
    pub json: bool,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List the teams New Game offers, by region.
    Teams,
    /// Start a new game and save it.
    NewGame {
        #[arg(long)]
        manager: String,
        /// A team id, such as `team:azure_tigers` (see `esm teams`).
        #[arg(long)]
        team: String,
        #[arg(long)]
        save: String,
        /// Build the world from this seed instead of a random one.
        #[arg(long)]
        seed: Option<u64>,
    },
    /// List the saves.
    Saves,
    /// Show a team's roster.
    Roster {
        /// The save's name or file.
        #[arg(long)]
        save: String,
        /// Another team than your own.
        #[arg(long)]
        team: Option<String>,
    },
    /// Show one player.
    Player {
        #[arg(long)]
        save: String,
        /// A player id, such as `player:kairos`.
        player: String,
    },
}

/// A use case failed; the messages say why.
struct Failed(Vec<Message>);

impl From<Message> for Failed {
    fn from(message: Message) -> Self {
        Self(vec![message])
    }
}

/// Runs one command, printing to `out` and errors to `err`. Returns the exit
/// code.
pub fn run(cli: &Cli, out: &mut impl Write, err: &mut impl Write) -> u8 {
    match execute(cli, out) {
        Ok(()) => 0,
        Err(Failed(messages)) => {
            let text = Text::new(&cli.locale, &[]);
            for message in &messages {
                let _ = writeln!(err, "{}", text.render(message));
            }
            1
        }
    }
}

fn execute(cli: &Cli, out: &mut impl Write) -> Result<(), Failed> {
    let content = PackageDir(cli.content.clone());
    match &cli.command {
        Command::Teams => teams(cli, out, &content),
        Command::NewGame {
            manager,
            team,
            save,
            seed,
        } => new_game(cli, out, &content, (manager, team, save, *seed)),
        Command::Saves => saves(cli, out),
        Command::Roster { save, team } => roster(cli, out, save, team.as_deref()),
        Command::Player { save, player: id } => player(cli, out, save, id),
    }
}

fn teams(cli: &Cli, out: &mut impl Write, content: &PackageDir) -> Result<(), Failed> {
    let options = game::new_game_options(content).map_err(package_failure)?;
    if cli.json {
        return print_json(out, &options);
    }
    let package = packages::load(&cli.content).map_err(package_failure)?;
    let text = Text::new(&cli.locale, &package.texts);
    for region in &options.regions {
        let mut line = Message::new("cli-region").message("region", region.name_message.clone());
        line = match &region.league_message {
            Some(league) => line.message("league", league.clone()),
            None => line.text("league", ""),
        };
        print(out, &text.render(&line))?;
        for team in &region.teams {
            let row = Message::new("cli-team")
                .text("tag", team.crest.tag.clone())
                .message("name", team.name_message.clone())
                .number("stars", i64::from(team.strength))
                .text("id", team.id.clone());
            print(out, &format!("  {}", text.render(&row)))?;
        }
    }
    Ok(())
}

fn new_game(
    cli: &Cli,
    out: &mut impl Write,
    content: &PackageDir,
    (manager, team, save, seed): (&str, &str, &str, Option<u64>),
) -> Result<(), Failed> {
    let store = store(cli)?;
    let request = NewGameRequest {
        manager_name: manager.to_owned(),
        team: team.to_owned(),
        save_name: save.to_owned(),
        seed: match seed {
            Some(seed) => seed,
            None => random_seed()?,
        },
        now: now(),
    };
    let created = game::new_game(content, &store, &request)
        .map_err(|error| Failed(game::new_game_messages(&error)))?;
    if cli.json {
        #[derive(Serialize)]
        struct Created<'a> {
            file: &'a str,
            team: String,
        }
        return print_json(
            out,
            &Created {
                file: &created.file,
                team: created.team.to_string(),
            },
        );
    }
    let package = packages::load(&cli.content).map_err(package_failure)?;
    let line = Message::new("cli-new-game-created")
        .text("file", created.file.clone())
        .message("team", created.team.message_id());
    print(out, &Text::new(&cli.locale, &package.texts).render(&line))
}

fn saves(cli: &Cli, out: &mut impl Write) -> Result<(), Failed> {
    let saves = game::list_saves(&store(cli)?).map_err(|e| Failed::from(game::save_message(&e)))?;
    if cli.json {
        return print_json(out, &saves);
    }
    print_saves(cli, out, &saves)
}

fn roster(cli: &Cli, out: &mut impl Write, save: &str, team: Option<&str>) -> Result<(), Failed> {
    let store = store(cli)?;
    let mut open = open(&store, save)?;
    let view = game::roster(&mut open, team).map_err(|e| Failed::from(game::query_message(&e)))?;
    if cli.json {
        return print_json(out, &view);
    }
    let texts =
        app::saves::OpenSave::texts(&mut open).map_err(|e| Failed::from(game::save_message(&e)))?;
    let text = Text::new(&cli.locale, &texts);
    let mut title =
        Message::new("cli-roster-title").message("team", view.team.name_message.clone());
    title = match &view.team.league_message {
        Some(league) => title.message("league", league.clone()),
        None => title.message("league", view.team.region_message.clone()),
    };
    print(out, &text.render(&title))?;
    for (heading, players) in [
        ("roster-starters", &view.starters),
        ("roster-subs", &view.subs),
    ] {
        print(out, "")?;
        print(out, &text.render(&Message::new(heading)))?;
        print_table(out, &text, players)?;
    }
    Ok(())
}

fn player(cli: &Cli, out: &mut impl Write, save: &str, player: &str) -> Result<(), Failed> {
    let store = store(cli)?;
    let mut open = open(&store, save)?;
    let profile = game::player_profile(&mut open, player)
        .map_err(|e| Failed::from(game::query_message(&e)))?;
    if cli.json {
        return print_json(out, &profile);
    }
    let texts =
        app::saves::OpenSave::texts(&mut open).map_err(|e| Failed::from(game::save_message(&e)))?;
    let text = Text::new(&cli.locale, &texts);
    let p = &profile.player;
    let title = Message::new("cli-player-title")
        .text("nickname", p.nickname.clone())
        .text("name", full_name(&text, p))
        .message("team", profile.team.name_message.clone());
    print(out, &text.render(&title))?;
    print(
        out,
        &text.render(&Message::new("profile-overall").number("overall", i64::from(p.overall))),
    )?;
    print(
        out,
        &text.render(&Message::new("profile-born").text("date", profile.birth_date.clone())),
    )?;
    let unknown = text.render(&Message::new("profile-unknown"));
    for (heading, attributes) in [
        ("profile-skills", &profile.skills),
        ("profile-traits", &profile.traits),
        ("profile-hidden", &profile.hidden),
    ] {
        print(out, "")?;
        print(out, &text.render(&Message::new(heading)))?;
        let rows: Vec<(String, String)> = attributes
            .iter()
            .map(|a| {
                let value = a.value.map_or_else(|| unknown.clone(), |v| v.to_string());
                (text.render(&Message::content(&a.name_message)), value)
            })
            .collect();
        let width = rows
            .iter()
            .map(|(name, _)| name.chars().count())
            .max()
            .unwrap_or(0);
        for (name, value) in rows {
            print(out, &format!("  {}  {value:>2}", pad(&name, width)))?;
        }
    }
    Ok(())
}

fn print_saves(cli: &Cli, out: &mut impl Write, saves: &[SaveSummary]) -> Result<(), Failed> {
    // Team names come from the package's text; without it, the ids show.
    let texts = packages::load(&cli.content)
        .map(|p| p.texts)
        .unwrap_or_default();
    let text = Text::new(&cli.locale, &texts);
    if saves.is_empty() {
        return print(out, &text.render(&Message::new("cli-saves-empty")));
    }
    for save in saves {
        let line = match save {
            SaveSummary::Ready {
                file,
                in_use,
                name,
                manager_name,
                team_message,
                game_date,
                last_played_at: _,
            } => Message::new("cli-save")
                .text("name", name.clone())
                .text("manager", manager_name.clone())
                .message("team", team_message.clone())
                .text("date", game_date.clone())
                .text("file", file.clone())
                .text("inUse", if *in_use { "yes" } else { "no" }),
            SaveSummary::Damaged { file, damage } => Message::new("cli-save-damaged")
                .text("file", file.clone())
                .message("damage", damage_message(*damage)),
        };
        print(out, &text.render(&line))?;
    }
    Ok(())
}

const fn damage_message(damage: DamageView) -> &'static str {
    match damage {
        DamageView::NotASave => "save-damage-not-a-save",
        DamageView::NewerVersion => "save-damage-newer-version",
    }
}

fn full_name(text: &Text, player: &RosterPlayer) -> String {
    let order = match player.name_order {
        app::views::NameOrderView::GivenFirst => "given-first",
        app::views::NameOrderView::FamilyFirst => "family-first",
    };
    text.render(
        &Message::new("person-full-name")
            .text("order", order)
            .text("given", player.given_name.clone())
            .text("family", player.family_name.clone()),
    )
}

fn print_table(out: &mut impl Write, text: &Text, players: &[RosterPlayer]) -> Result<(), Failed> {
    let header = [
        "roster-column-role",
        "roster-column-nickname",
        "roster-column-name",
        "roster-column-nationality",
        "roster-column-age",
        "roster-column-overall",
    ]
    .map(|id| text.render(&Message::new(id)));
    let mut rows = vec![header.to_vec()];
    for p in players {
        rows.push(vec![
            text.render(&Message::content(&p.role_message)),
            p.nickname.clone(),
            full_name(text, p),
            p.nationality.code.clone(),
            p.age.to_string(),
            p.overall.to_string(),
        ]);
    }
    let widths: Vec<usize> = (0..6)
        .map(|column| {
            rows.iter()
                .map(|row| row[column].chars().count())
                .max()
                .unwrap_or(0)
        })
        .collect();
    for row in rows {
        let cells: Vec<String> = row
            .iter()
            .zip(&widths)
            .map(|(cell, &width)| pad(cell, width))
            .collect();
        print(out, &format!("  {}", cells.join("  ").trim_end()))?;
    }
    Ok(())
}

fn pad(text: &str, width: usize) -> String {
    let fill = width.saturating_sub(text.chars().count());
    format!("{text}{}", " ".repeat(fill))
}

fn print(out: &mut impl Write, line: &str) -> Result<(), Failed> {
    writeln!(out, "{line}")
        .map_err(|e| Failed::from(Message::new("error-storage").text("detail", e.to_string())))
}

fn print_json(out: &mut impl Write, value: &impl Serialize) -> Result<(), Failed> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|e| Failed::from(Message::new("error-storage").text("detail", e.to_string())))?;
    print(out, &json)
}

#[expect(clippy::needless_pass_by_value, reason = "passed to `map_err` by name")]
fn package_failure(errors: Vec<app::content::PackageError>) -> Failed {
    Failed(errors.iter().map(package_error).collect())
}

fn store(cli: &Cli) -> Result<SqliteSaveStore, Failed> {
    let locations = db::environment::locations(cli.data_dir.clone())
        .map_err(|e| Failed::from(Message::new("error-storage").text("detail", e.to_string())))?;
    db::environment::prepare(&locations).map_err(|(dir, error)| {
        Failed::from(
            Message::new("error-storage").text("detail", format!("{}: {error}", dir.display())),
        )
    })?;
    Ok(SqliteSaveStore::new(locations.saves()))
}

/// The save a `--save` argument names: a save's name (ignoring case) or its
/// file.
fn open(store: &SqliteSaveStore, wanted: &str) -> Result<db::SqliteSave, Failed> {
    let listings = store
        .list()
        .map_err(|e| Failed::from(game::save_message(&e)))?;
    let lowered = wanted.to_lowercase();
    let file = listings
        .iter()
        .find(|l| {
            l.file == wanted
                || l.save
                    .as_ref()
                    .is_ok_and(|meta| meta.name.to_lowercase() == lowered)
        })
        .map_or_else(|| wanted.to_owned(), |l| l.file.clone());
    store
        .open(&file, now())
        .map_err(|e| Failed::from(game::save_message(&e)))
}

fn random_seed() -> Result<u64, Failed> {
    getrandom::u64()
        .map_err(|e| Failed::from(Message::new("error-storage").text("detail", e.to_string())))
}

/// Real time, for save timestamps. Adapters are the only place it is read
/// (ADR 0003).
fn now() -> Timestamp {
    #[allow(
        clippy::disallowed_methods,
        reason = "save timestamps are real time, read only by adapters (ADR 0003)"
    )]
    let elapsed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    Timestamp(i64::try_from(elapsed).unwrap_or(i64::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_line_names_the_binary_and_version() {
        // The workspace shares one version, so the CLI crate's own is the oracle.
        assert_eq!(version_line(), format!("esm {}", env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn padding_counts_characters_not_bytes() {
        assert_eq!(pad("Onça", 6), "Onça  ");
        assert_eq!(pad("long", 2), "long");
    }

    #[test]
    fn every_damage_has_a_message() {
        let ids = [DamageView::NotASave, DamageView::NewerVersion].map(damage_message);
        assert_eq!(ids, ["save-damage-not-a-save", "save-damage-newer-version"]);
    }

    #[test]
    fn now_is_after_2026() {
        assert!(now().0 > 1_767_225_600);
    }
}
