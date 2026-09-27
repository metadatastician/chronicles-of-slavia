//! Aura mechanics — the field that emanates from each girl.
//!
//! An aura has radius, strength, and control. Early on it is leaky and
//! unbidden; with mastery it can be aimed, contained, or withheld.

use serde::{Deserialize, Serialize};

/// The state of a girl's aura at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraState {
    /// How far the aura reaches (in world units).
    pub radius: f32,
    /// How strongly the aura affects things within range (0.0–1.0).
    pub strength: f32,
    /// How selectively the aura acts (0.0 = leaky, 1.0 = precise).
    pub control: f32,
    /// Whether the aura is actively suppressed (Zetsu-like hiding).
    pub suppressed: bool,
}

impl Default for AuraState {
    fn default() -> Self {
        Self {
            radius: 3.0,
            strength: 0.5,
            control: 0.2,
            suppressed: false,
        }
    }
}
