//! How the world reacts to emotional state.
//!
//! Maps emotional state to environmental effects: platform stability,
//! hazard speed, creature behaviour, colour shifts.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEmotionalReaction {
    pub platform_stability_modifier: f32,
    pub hazard_speed_modifier: f32,
    pub creature_aggression_modifier: f32,
    pub colour_shift: f32,
}
