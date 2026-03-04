use esm_models::staff::{Staff, StaffRole};

/// Computes staff influence modifiers for simulation and progression systems.
///
/// Per DESIGN.md §8, staff members actively hook into the simulation:
/// - Assistant Coach → delegation/spectate AI quality
/// - Positional Coach → training attribute growth multiplier
/// - Sports Psychologist → morale loss buffer + stamina penalty reduction
/// - Scout → scouting accuracy for hidden attributes
pub struct StaffInfluence;

impl StaffInfluence {
    /// Assistant Coach delegation quality multiplier.
    /// Skill 0 → 0.5x, Skill 100 → 1.5x. Non-coaches return 1.0.
    pub fn delegation_quality(staff: &Staff) -> f64 {
        if staff.role() != StaffRole::AssistantCoach {
            return 1.0;
        }
        0.5 + (staff.skill().value() as f64 / 100.0)
    }

    /// Positional Coach training growth multiplier.
    /// Skill 0 → 1.0x, Skill 100 → 1.5x. Non-positional coaches return 1.0.
    pub fn training_growth_multiplier(staff: &Staff) -> f64 {
        if staff.role() != StaffRole::PositionalCoach {
            return 1.0;
        }
        1.0 + (staff.skill().value() as f64 / 100.0) * 0.5
    }

    /// Sports Psychologist morale loss buffer (fraction of morale loss absorbed).
    /// Skill 0 → 0%, Skill 100 → 50%. Non-psychologists return 0.
    pub fn morale_loss_buffer(staff: &Staff) -> f64 {
        if staff.role() != StaffRole::SportsPsychologist {
            return 0.0;
        }
        (staff.skill().value() as f64 / 100.0) * 0.5
    }

    /// Sports Psychologist stamina penalty reduction (fraction of penalty absorbed).
    /// Skill 0 → 0%, Skill 100 → 30%. Non-psychologists return 0.
    pub fn stamina_penalty_reduction(staff: &Staff) -> f64 {
        if staff.role() != StaffRole::SportsPsychologist {
            return 0.0;
        }
        (staff.skill().value() as f64 / 100.0) * 0.3
    }

    /// Scout scouting accuracy (fraction of hidden attributes revealed).
    /// Skill 0 → 20%, Skill 100 → 100%. Non-scouts return 0.2.
    pub fn scouting_accuracy(staff: &Staff) -> f64 {
        if staff.role() != StaffRole::Scout {
            return 0.2;
        }
        0.2 + (staff.skill().value() as f64 / 100.0) * 0.8
    }
}
