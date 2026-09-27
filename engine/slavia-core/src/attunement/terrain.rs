//! Terrain affinity — how the land favours its own.
//!
//! Each girl is of a place: Anya of the forest, Donna of the mountains.
//! Native ground amplifies; complement ground drains.
//! See `docs/design/06-attunement-and-modifiers.md` § Terrain.

use serde::{Deserialize, Serialize};

/// Terrain types in Slavia.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Terrain {
    /// Ukrainian birch forest — Anya's native ground.
    Forest,
    /// Bulgarian mountains — Donna's native ground.
    Mountain,
    /// Tundra — complement to Anya (drains her).
    Tundra,
    /// Swampland — complement to Donna (drains her).
    Swamp,
    /// Neutral ground (the bridge, the border).
    Neutral,
}

/// How a terrain modifies a girl's effectiveness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainModifier {
    pub aura_multiplier: f32,
    pub potentiality_bonus: f32,
    pub damage_modifier: f32,
    pub heals_over_time: bool,
}
