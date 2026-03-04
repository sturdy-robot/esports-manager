use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult, Row};

// ---------------------------------------------------------------------------
// TeamRow
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TeamRow {
    pub id: i64,
    pub name: String,
    pub tag: String,
    pub synergy: i32,
    pub reputation: i32,
    pub budget: i64,
}

impl TeamRow {
    fn from_row(row: &Row) -> SqlResult<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            tag: row.get("tag")?,
            synergy: row.get("synergy")?,
            reputation: row.get("reputation")?,
            budget: row.get("budget")?,
        })
    }

    pub fn insert(conn: &Connection, team: &TeamRow) -> SqlResult<()> {
        conn.execute(
            "INSERT INTO teams (id, name, tag, synergy, reputation, budget)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![team.id, team.name, team.tag, team.synergy, team.reputation, team.budget],
        )?;
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqlResult<Option<TeamRow>> {
        conn.query_row("SELECT * FROM teams WHERE id = ?1", [id], Self::from_row)
            .optional()
    }

    pub fn list_all(conn: &Connection) -> SqlResult<Vec<TeamRow>> {
        let mut stmt = conn.prepare("SELECT * FROM teams ORDER BY id")?;
        let rows = stmt.query_map([], Self::from_row)?;
        rows.collect()
    }

    pub fn update(conn: &Connection, team: &TeamRow) -> SqlResult<()> {
        conn.execute(
            "UPDATE teams SET name = ?1, tag = ?2, synergy = ?3, reputation = ?4, budget = ?5
             WHERE id = ?6",
            params![team.name, team.tag, team.synergy, team.reputation, team.budget, team.id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqlResult<()> {
        conn.execute("DELETE FROM teams WHERE id = ?1", [id])?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// PlayerRow
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PlayerRow {
    pub id: i64,
    pub nickname: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub endurance: i32,
    pub reaction_time: i32,
    pub decision_making: i32,
    pub clutch: i32,
    pub discipline: i32,
    pub tilt_resistance: i32,
    pub mechanics: i32,
    pub vision_control: i32,
    pub teamfighting: i32,
    pub stamina: i32,
    pub morale: i32,
    pub confidence: String,
    pub team_id: Option<i64>,
}

impl PlayerRow {
    fn from_row(row: &Row) -> SqlResult<Self> {
        Ok(Self {
            id: row.get("id")?,
            nickname: row.get("nickname")?,
            first_name: row.get("first_name")?,
            last_name: row.get("last_name")?,
            role: row.get("role")?,
            endurance: row.get("endurance")?,
            reaction_time: row.get("reaction_time")?,
            decision_making: row.get("decision_making")?,
            clutch: row.get("clutch")?,
            discipline: row.get("discipline")?,
            tilt_resistance: row.get("tilt_resistance")?,
            mechanics: row.get("mechanics")?,
            vision_control: row.get("vision_control")?,
            teamfighting: row.get("teamfighting")?,
            stamina: row.get("stamina")?,
            morale: row.get("morale")?,
            confidence: row.get("confidence")?,
            team_id: row.get("team_id")?,
        })
    }

    pub fn insert(conn: &Connection, p: &PlayerRow) -> SqlResult<()> {
        conn.execute(
            "INSERT INTO players (id, nickname, first_name, last_name, role,
             endurance, reaction_time, decision_making, clutch, discipline,
             tilt_resistance, mechanics, vision_control, teamfighting,
             stamina, morale, confidence, team_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            params![
                p.id, p.nickname, p.first_name, p.last_name, p.role,
                p.endurance, p.reaction_time, p.decision_making, p.clutch, p.discipline,
                p.tilt_resistance, p.mechanics, p.vision_control, p.teamfighting,
                p.stamina, p.morale, p.confidence, p.team_id,
            ],
        )?;
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> SqlResult<Option<PlayerRow>> {
        conn.query_row("SELECT * FROM players WHERE id = ?1", [id], Self::from_row)
            .optional()
    }

    pub fn list_by_team(conn: &Connection, team_id: i64) -> SqlResult<Vec<PlayerRow>> {
        let mut stmt = conn.prepare("SELECT * FROM players WHERE team_id = ?1 ORDER BY id")?;
        let rows = stmt.query_map([team_id], Self::from_row)?;
        rows.collect()
    }

    pub fn update(conn: &Connection, p: &PlayerRow) -> SqlResult<()> {
        conn.execute(
            "UPDATE players SET nickname = ?1, first_name = ?2, last_name = ?3, role = ?4,
             endurance = ?5, reaction_time = ?6, decision_making = ?7, clutch = ?8,
             discipline = ?9, tilt_resistance = ?10, mechanics = ?11, vision_control = ?12,
             teamfighting = ?13, stamina = ?14, morale = ?15, confidence = ?16, team_id = ?17
             WHERE id = ?18",
            params![
                p.nickname, p.first_name, p.last_name, p.role,
                p.endurance, p.reaction_time, p.decision_making, p.clutch,
                p.discipline, p.tilt_resistance, p.mechanics, p.vision_control,
                p.teamfighting, p.stamina, p.morale, p.confidence, p.team_id,
                p.id,
            ],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> SqlResult<()> {
        conn.execute("DELETE FROM players WHERE id = ?1", [id])?;
        Ok(())
    }
}
