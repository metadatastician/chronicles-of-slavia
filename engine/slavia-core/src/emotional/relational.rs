//! The bond between Anya and Donna — trust, conflict, guilt, reunion.
//!
//! The heart of the emotional system: how they are with each other
//! shapes the world more than individual mood.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondState {
    pub trust: f32,
    pub conflict: f32,
    pub guilt: f32,
    pub protectiveness: f32,
    pub reunion_joy: f32,
}
