use esm_models::activity::Activity;
use esm_models::player::Player;
use esm_models::time::TimeSlot;

use super::{ScheduleEntry, TeamDailySchedule};

// ---------------------------------------------------------------------------
// ScheduleProcessor: applies team-level daily schedule effects to players
// ---------------------------------------------------------------------------

pub struct ScheduleProcessor;

impl ScheduleProcessor {
    /// Apply all slot effects from a team's daily schedule to the roster.
    ///
    /// For each time slot:
    /// - **Scrim**: all 5 players pay the ScrimBlock stamina cost
    /// - **SoloQueue**: only assigned players pay the SoloQueue stamina cost
    /// - **Rest**: all 5 players gain the RestDay stamina recovery
    /// - **Free (None)**: no effect
    ///
    /// If any player's stamina hits 0 after processing, they lose morale.
    pub fn apply_daily_effects(day: &TeamDailySchedule, roster: &mut [Player]) {
        // Track which players hit zero stamina during processing
        let mut depleted: Vec<bool> = vec![false; roster.len()];

        for slot in TimeSlot::ALL {
            match day.get(slot) {
                Some(ScheduleEntry::Scrim { .. }) => {
                    let cost = Activity::ScrimBlock.effect().stamina_cost;
                    for (i, player) in roster.iter_mut().enumerate() {
                        player.state_mut().stamina.decrease(cost);
                        if player.state().stamina.value() == 0 {
                            depleted[i] = true;
                        }
                    }
                }
                Some(ScheduleEntry::SoloQueue { players, .. }) => {
                    let cost = Activity::SoloQueue.effect().stamina_cost;
                    for &idx in players {
                        if idx < roster.len() {
                            roster[idx].state_mut().stamina.decrease(cost);
                            if roster[idx].state().stamina.value() == 0 {
                                depleted[idx] = true;
                            }
                        }
                    }
                }
                Some(ScheduleEntry::Rest) => {
                    let recovery = Activity::RestDay.effect().stamina_recovery;
                    for player in roster.iter_mut() {
                        player.state_mut().stamina.increase(recovery);
                    }
                }
                None => {}
            }
        }

        // Penalty: players whose stamina hit 0 lose morale and satisfaction
        for (i, player) in roster.iter_mut().enumerate() {
            if depleted[i] {
                player.state_mut().morale.decrease(5);
                player.state_mut().satisfaction.decrease(2);
            }
        }
    }
}
