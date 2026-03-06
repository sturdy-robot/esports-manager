use esm_engine::tournament::{
    BracketKind, Match, MatchStatus, Schedule, Tournament, TournamentFormat,
};

// ---------------------------------------------------------------------------
// TournamentFormat
// ---------------------------------------------------------------------------

#[test]
fn tournament_format_variants_exist() {
    let _ = TournamentFormat::RoundRobin;
    let _ = TournamentFormat::DoubleRoundRobin;
}

// ---------------------------------------------------------------------------
// BracketKind
// ---------------------------------------------------------------------------

#[test]
fn bracket_kind_variants_exist() {
    let _ = BracketKind::Bo1;
    let _ = BracketKind::Bo3;
    let _ = BracketKind::Bo5;
}

#[test]
fn bracket_kind_wins_needed() {
    assert_eq!(BracketKind::Bo1.wins_needed(), 1);
    assert_eq!(BracketKind::Bo3.wins_needed(), 2);
    assert_eq!(BracketKind::Bo5.wins_needed(), 3);
}

// ---------------------------------------------------------------------------
// Match
// ---------------------------------------------------------------------------

#[test]
fn match_creation() {
    let m = Match::new(1, 0, 1, 10, BracketKind::Bo1);
    assert_eq!(m.id(), 1);
    assert_eq!(m.blue_team_idx(), 0);
    assert_eq!(m.red_team_idx(), 1);
    assert_eq!(m.scheduled_day(), 10);
    assert_eq!(m.status(), MatchStatus::Pending);
}

#[test]
fn match_set_result() {
    let mut m = Match::new(1, 0, 1, 10, BracketKind::Bo3);
    m.set_result(2, 1);
    assert_eq!(m.status(), MatchStatus::Completed);
    assert_eq!(m.blue_wins(), 2);
    assert_eq!(m.red_wins(), 1);
    assert_eq!(m.winner_team_idx(), Some(0)); // blue won
}

#[test]
fn match_red_wins() {
    let mut m = Match::new(1, 0, 1, 10, BracketKind::Bo3);
    m.set_result(0, 2);
    assert_eq!(m.winner_team_idx(), Some(1)); // red won
}

// ---------------------------------------------------------------------------
// Schedule generation — round robin
// ---------------------------------------------------------------------------

#[test]
fn schedule_round_robin_generates_correct_match_count() {
    // 4 teams, round robin: n*(n-1)/2 = 6 matches
    let schedule = Schedule::round_robin(4, BracketKind::Bo1, 1);
    assert_eq!(schedule.matches().len(), 6);
}

#[test]
fn schedule_round_robin_3_teams() {
    let schedule = Schedule::round_robin(3, BracketKind::Bo1, 1);
    // 3 teams: 3 matches
    assert_eq!(schedule.matches().len(), 3);
}

#[test]
fn schedule_round_robin_every_team_plays_every_other() {
    let schedule = Schedule::round_robin(4, BracketKind::Bo1, 1);
    // Team 0 should play teams 1, 2, 3
    let team0_matches: Vec<_> = schedule
        .matches()
        .iter()
        .filter(|m| m.blue_team_idx() == 0 || m.red_team_idx() == 0)
        .collect();
    assert_eq!(team0_matches.len(), 3);
}

#[test]
fn schedule_double_round_robin_doubles_match_count() {
    let single = Schedule::round_robin(4, BracketKind::Bo1, 1);
    let double = Schedule::double_round_robin(4, BracketKind::Bo1, 1);
    assert_eq!(double.matches().len(), single.matches().len() * 2);
}

#[test]
fn schedule_matches_have_sequential_days() {
    let schedule = Schedule::round_robin(4, BracketKind::Bo1, 1);
    let days: Vec<u32> = schedule
        .matches()
        .iter()
        .map(|m| m.scheduled_day())
        .collect();
    // Days should be non-decreasing
    for window in days.windows(2) {
        assert!(window[0] <= window[1]);
    }
}

#[test]
fn schedule_matches_for_day() {
    let schedule = Schedule::round_robin(4, BracketKind::Bo1, 1);
    let day1 = schedule.matches_for_day(1);
    assert!(!day1.is_empty());
}

// ---------------------------------------------------------------------------
// Tournament
// ---------------------------------------------------------------------------

#[test]
fn tournament_creation() {
    let team_names = vec!["T1".to_string(), "Gen.G".to_string(), "DRX".to_string()];
    let t = Tournament::new(
        "LCK Spring".to_string(),
        team_names,
        TournamentFormat::RoundRobin,
        BracketKind::Bo3,
        1,
    );
    assert_eq!(t.name(), "LCK Spring");
    assert_eq!(t.team_count(), 3);
    assert!(!t.schedule().matches().is_empty());
}

#[test]
fn tournament_standings_start_at_zero() {
    let team_names = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let t = Tournament::new(
        "Test".to_string(),
        team_names,
        TournamentFormat::RoundRobin,
        BracketKind::Bo1,
        1,
    );
    let standings = t.standings();
    assert_eq!(standings.len(), 3);
    for s in &standings {
        assert_eq!(s.wins, 0);
        assert_eq!(s.losses, 0);
    }
}

#[test]
fn tournament_record_result_updates_standings() {
    let team_names = vec!["A".to_string(), "B".to_string()];
    let mut t = Tournament::new(
        "Test".to_string(),
        team_names,
        TournamentFormat::RoundRobin,
        BracketKind::Bo1,
        1,
    );

    let match_id = t.schedule().matches()[0].id();
    t.record_result(match_id, 1, 0); // A wins

    let standings = t.standings();
    let a = standings.iter().find(|s| s.team_idx == 0).unwrap();
    let b = standings.iter().find(|s| s.team_idx == 1).unwrap();
    assert_eq!(a.wins, 1);
    assert_eq!(a.losses, 0);
    assert_eq!(b.wins, 0);
    assert_eq!(b.losses, 1);
}

#[test]
fn tournament_standings_sorted_by_wins_descending() {
    let team_names = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let mut t = Tournament::new(
        "Test".to_string(),
        team_names,
        TournamentFormat::RoundRobin,
        BracketKind::Bo1,
        1,
    );

    // Record: B beats A, C beats A → B=1W, C=1W, A=0W2L
    let matches: Vec<_> = t.schedule().matches().to_vec();
    // Find A vs B match
    let ab = matches
        .iter()
        .find(|m| {
            (m.blue_team_idx() == 0 && m.red_team_idx() == 1)
                || (m.blue_team_idx() == 1 && m.red_team_idx() == 0)
        })
        .unwrap();
    // If A is blue, red (B) wins
    if ab.blue_team_idx() == 0 {
        t.record_result(ab.id(), 0, 1);
    } else {
        t.record_result(ab.id(), 1, 0);
    }

    let ac = matches
        .iter()
        .find(|m| {
            (m.blue_team_idx() == 0 && m.red_team_idx() == 2)
                || (m.blue_team_idx() == 2 && m.red_team_idx() == 0)
        })
        .unwrap();
    if ac.blue_team_idx() == 0 {
        t.record_result(ac.id(), 0, 1);
    } else {
        t.record_result(ac.id(), 1, 0);
    }

    let standings = t.standings();
    // First two should have 1 win each, A should be last with 0 wins
    assert_eq!(standings[2].team_idx, 0); // A is last
    assert_eq!(standings[2].wins, 0);
}

#[test]
fn tournament_is_complete_when_all_matches_played() {
    let team_names = vec!["A".to_string(), "B".to_string()];
    let mut t = Tournament::new(
        "Test".to_string(),
        team_names,
        TournamentFormat::RoundRobin,
        BracketKind::Bo1,
        1,
    );
    assert!(!t.is_complete());

    let match_id = t.schedule().matches()[0].id();
    t.record_result(match_id, 1, 0);
    assert!(t.is_complete());
}

// ---------------------------------------------------------------------------
// Match::add_game_win / bracket()
// ---------------------------------------------------------------------------

#[test]
fn match_bracket_accessor() {
    let m = Match::new(1, 0, 1, 10, BracketKind::Bo3);
    assert_eq!(m.bracket(), BracketKind::Bo3);
}

#[test]
fn match_add_game_win_increments_blue() {
    let mut m = Match::new(1, 0, 1, 10, BracketKind::Bo3);
    m.add_game_win(true);
    assert_eq!(m.blue_wins(), 1);
    assert_eq!(m.red_wins(), 0);
    assert_eq!(m.status(), MatchStatus::Pending);
}

#[test]
fn match_add_game_win_completes_bo3_at_two_wins() {
    let mut m = Match::new(1, 0, 1, 10, BracketKind::Bo3);
    m.add_game_win(true);
    m.add_game_win(false);
    assert_eq!(m.status(), MatchStatus::Pending);

    m.add_game_win(true); // blue reaches 2 wins
    assert_eq!(m.status(), MatchStatus::Completed);
    assert_eq!(m.blue_wins(), 2);
    assert_eq!(m.red_wins(), 1);
    assert_eq!(m.winner_team_idx(), Some(0));
}

#[test]
fn match_add_game_win_red_wins_bo3() {
    let mut m = Match::new(1, 0, 1, 10, BracketKind::Bo3);
    m.add_game_win(false);
    m.add_game_win(false); // red reaches 2 wins
    assert_eq!(m.status(), MatchStatus::Completed);
    assert_eq!(m.winner_team_idx(), Some(1));
}

// ---------------------------------------------------------------------------
// Tournament::add_game_win
// ---------------------------------------------------------------------------

#[test]
fn tournament_add_game_win_tracks_incremental_wins() {
    let team_names = vec!["A".to_string(), "B".to_string()];
    let mut t = Tournament::new(
        "Test".to_string(),
        team_names,
        TournamentFormat::RoundRobin,
        BracketKind::Bo3,
        1,
    );

    let match_id = t.schedule().matches()[0].id();
    assert!(!t.add_game_win(match_id, true)); // 1-0, not done
    assert!(!t.is_complete());

    assert!(t.add_game_win(match_id, true)); // 2-0, series complete
    assert!(t.is_complete());

    let standings = t.standings();
    let a = standings.iter().find(|s| s.team_idx == 0).unwrap();
    assert_eq!(a.wins, 1);
}
