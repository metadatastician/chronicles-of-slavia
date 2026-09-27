//! Shared rendering infrastructure for Chronicles of Slavia.
//!
//! This crate provides common rendering systems used across all zone crates:
//! - Camera follow with smoothing (CODE-1 fix)
//! - Room/area management (CODE-2 fix)
//! - Sprite rendering and animation
//! - Parallax backgrounds
//! - Particle effects
//! - Manpu overlays (ESM legibility)
//! - Lighting and mood
//! - Room/zone transitions

pub mod animation;
pub mod camera;
pub mod lighting;
pub mod manpu;
pub mod parallax;
pub mod particles;
pub mod room;
pub mod sprite;
pub mod transitions;
