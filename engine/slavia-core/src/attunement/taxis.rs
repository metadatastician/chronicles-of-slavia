//! Living Taxis — how the girls' aura moves living things.
//!
//! Taxis is "the rate and direction of a living thing's motion."
//! Anya raises it; Donna lowers it. The creature's own nature gates the result.
//! See `docs/design/03-living-taxis.md`.

use serde::{Deserialize, Serialize};

/// The direction of taxis modulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxisDirection {
    /// Anya: quicken, embolden, stir, excite.
    Raise,
    /// Donna: calm, settle, steady, quieten.
    Lower,
}

/// The result of a taxis interaction, gated by the creature's nature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxisResponse {
    /// What the creature actually does in response.
    pub behaviour: String,
    /// Whether the response is natural (matches the creature's essence).
    pub is_natural: bool,
}
