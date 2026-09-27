//! Mastery progression — strength and control of the attunement gift.
//!
//! Mastery has two axes: strength (how far/powerful) and control
//! (how selective/precise). Early game is low control; late game
//! allows aimed, contained, persistent effects.

use serde::{Deserialize, Serialize};

/// A girl's current mastery level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryLevel {
    /// 0–10 scale for aura strength.
    pub strength: u8,
    /// 0–10 scale for aura control.
    pub control: u8,
    /// Whether distance effects are unlocked.
    pub can_act_at_distance: bool,
    /// Whether persistent effects are unlocked.
    pub can_leave_running: bool,
}
