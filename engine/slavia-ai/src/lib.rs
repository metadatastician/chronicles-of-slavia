//! Game AI for Chronicles of Slavia.
//!
//! This crate provides the AI systems that drive NPC behaviour,
//! animal responses, and the AI director. It bridges the ESM (mind)
//! in slavia-core with the FSM (body) in the Bevy layer.
//!
//! ## Enaction Engine integration
//!
//! The `enaction` module provides the integration point with the
//! Enaction Engine (metadatastician/enaction-engine) for neutral
//! deterministic primitives: events, appraisal domains, relationships,
//! influence, and place memory. Slavia retains all narrative vocabulary
//! and response rules (ADR-0007).

pub mod animal_ai;
pub mod director;
pub mod enaction;
pub mod npc_brain;
pub mod pathfinding;
