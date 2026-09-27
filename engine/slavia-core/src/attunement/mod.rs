//! Attunement and Living Taxis system.
//!
//! The girls' signature ability: affecting the rhythm of living things
//! by their presence. Anya raises taxis (quickens, emboldens); Donna
//! lowers taxis (calms, settles). See `docs/design/06-attunement-and-modifiers.md`.
//!
//! ## Architecture
//!
//! The attunement system is the heart of Slavia's gameplay. It is:
//! - **Passive**: emanates as an aura, not a button press
//! - **Nature-gated**: the animal's own essence determines the result
//! - **Mastery-scaled**: strength and control grow over the game
//! - **Terrain-modulated**: native ground amplifies, complement ground drains

pub mod aura;
pub mod mastery;
pub mod taxis;
pub mod terrain;
