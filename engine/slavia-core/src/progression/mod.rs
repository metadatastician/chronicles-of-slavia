//! Progression, save state, and durable world marking.
//!
//! Fail-forward (ADR-0005): no temporal rollback. The world only moves forward.
//! Place memory, moral/world marking, chronicle-level state.

pub mod chronicle;
pub mod marking;
pub mod memory;
