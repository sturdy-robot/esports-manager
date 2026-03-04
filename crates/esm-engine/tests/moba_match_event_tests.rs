use esm_engine::moba_match::event::{
    Commentary, MatchEvent, MatchEventKind, MobaMatchPhase,
};
use esm_engine::moba_match::map::Lane;
use esm_engine::moba_match::state::TeamSide;

// ---------------------------------------------------------------------------
// MobaMatchPhase
// ---------------------------------------------------------------------------

#[test]
fn moba_match_phase_from_minute() {
    assert_eq!(MobaMatchPhase::from_minute(0), MobaMatchPhase::Early);
    assert_eq!(MobaMatchPhase::from_minute(14), MobaMatchPhase::Early);
    assert_eq!(MobaMatchPhase::from_minute(15), MobaMatchPhase::Mid);
    assert_eq!(MobaMatchPhase::from_minute(24), MobaMatchPhase::Mid);
    assert_eq!(MobaMatchPhase::from_minute(25), MobaMatchPhase::Late);
    assert_eq!(MobaMatchPhase::from_minute(50), MobaMatchPhase::Late);
}

// ---------------------------------------------------------------------------
// MatchEventKind
// ---------------------------------------------------------------------------

#[test]
fn event_kind_variants_exist() {
    let _ = MatchEventKind::SoloKill {
        killer_idx: 0,
        victim_idx: 5,
        lane: Lane::Mid,
    };
    let _ = MatchEventKind::Teamfight {
        winning_side: TeamSide::Blue,
        kills_blue: 3,
        kills_red: 1,
    };
    let _ = MatchEventKind::TowerDestroyed {
        attacker: TeamSide::Blue,
        lane: Lane::Top,
    };
    let _ = MatchEventKind::DragonKill {
        killer: TeamSide::Blue,
    };
    let _ = MatchEventKind::HeraldKill {
        killer: TeamSide::Red,
    };
    let _ = MatchEventKind::BaronKill {
        killer: TeamSide::Blue,
    };
    let _ = MatchEventKind::InhibitorDestroyed {
        attacker: TeamSide::Red,
        lane: Lane::Mid,
    };
    let _ = MatchEventKind::NexusDestroyed {
        winner: TeamSide::Blue,
    };
}

// ---------------------------------------------------------------------------
// MatchEvent
// ---------------------------------------------------------------------------

#[test]
fn match_event_stores_data() {
    let event = MatchEvent::new(
        10,
        MobaMatchPhase::Mid,
        MatchEventKind::DragonKill {
            killer: TeamSide::Blue,
        },
    );
    assert_eq!(event.minute(), 10);
    assert_eq!(event.phase(), MobaMatchPhase::Mid);
    assert!(matches!(event.kind(), MatchEventKind::DragonKill { .. }));
}

#[test]
fn match_event_has_commentary() {
    let event = MatchEvent::new(
        5,
        MobaMatchPhase::Early,
        MatchEventKind::SoloKill {
            killer_idx: 2,
            victim_idx: 7,
            lane: Lane::Mid,
        },
    );
    assert!(event.commentary().is_none()); // Commentary is set separately
}

#[test]
fn match_event_commentary_can_be_set() {
    let mut event = MatchEvent::new(
        5,
        MobaMatchPhase::Early,
        MatchEventKind::SoloKill {
            killer_idx: 2,
            victim_idx: 7,
            lane: Lane::Mid,
        },
    );
    event.set_commentary(Commentary::new("FIRST BLOOD! What an outplay in the mid lane!".to_string()));
    assert!(event.commentary().is_some());
    assert!(event.commentary().unwrap().text().contains("FIRST BLOOD"));
}

// ---------------------------------------------------------------------------
// Commentary
// ---------------------------------------------------------------------------

#[test]
fn commentary_stores_text() {
    let c = Commentary::new("Baron secured by the blue side!".to_string());
    assert_eq!(c.text(), "Baron secured by the blue side!");
}

// ---------------------------------------------------------------------------
// Commentary generation
// ---------------------------------------------------------------------------

#[test]
fn generate_solo_kill_commentary_contains_lane() {
    let commentary = Commentary::for_solo_kill("Faker", "Chovy", Lane::Mid, true);
    assert!(commentary.text().to_lowercase().contains("mid"));
}

#[test]
fn generate_first_blood_commentary_is_special() {
    let commentary = Commentary::for_solo_kill("Faker", "Chovy", Lane::Mid, true);
    assert!(
        commentary.text().contains("First Blood")
            || commentary.text().contains("first blood")
            || commentary.text().contains("FIRST BLOOD"),
        "First blood commentary should mention 'first blood', got: {}",
        commentary.text()
    );
}

#[test]
fn generate_dragon_commentary_mentions_dragon() {
    let commentary = Commentary::for_dragon("T1", 2);
    assert!(commentary.text().to_lowercase().contains("dragon"));
}

#[test]
fn generate_baron_commentary_mentions_baron() {
    let commentary = Commentary::for_baron("Gen.G");
    assert!(commentary.text().to_lowercase().contains("baron"));
}

#[test]
fn generate_tower_commentary_mentions_tower() {
    let commentary = Commentary::for_tower_destroy("T1", Lane::Bot);
    assert!(commentary.text().to_lowercase().contains("tower") || commentary.text().to_lowercase().contains("turret"));
}

#[test]
fn generate_nexus_commentary_mentions_victory() {
    let commentary = Commentary::for_nexus_destroy("T1");
    assert!(
        commentary.text().to_lowercase().contains("win")
            || commentary.text().to_lowercase().contains("victory")
            || commentary.text().to_lowercase().contains("nexus")
            || commentary.text().to_lowercase().contains("game"),
        "Nexus commentary should mention victory/win/nexus, got: {}",
        commentary.text()
    );
}

#[test]
fn generate_teamfight_commentary_mentions_fight() {
    let commentary = Commentary::for_teamfight("T1", 3, 1);
    let text = commentary.text().to_lowercase();
    assert!(
        text.contains("fight") || text.contains("team") || text.contains("skirmish"),
        "Teamfight commentary should mention fight, got: {}",
        commentary.text()
    );
}
