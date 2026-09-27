//! Per-girl emotional state — moment-to-moment mood.
//!
//! Anya: stressed → volatile world; confident → smooth world.
//! Donna: anxious → brittle world; secure → stable world.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalEmotionalState {
    pub stress: f32,
    pub confidence: f32,
    pub joy: f32,
    pub fear: f32,
}
