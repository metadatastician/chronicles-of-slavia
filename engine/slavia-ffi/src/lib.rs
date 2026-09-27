//! Cleave-aligned integration surface for Chronicles of Slavia.
//!
//! This crate is a placeholder for a future FFI seam, if one is needed.
//! The game is currently pure Rust with no FFI requirement.
//!
//! When a seam is needed, it follows the cleave spectrum:
//! - **Raw FFI-ABI**: direct, fast, unsafe
//! - **SNIF**: sandboxed (native → WASM → wasmtime), crash-isolated
//! - **Unified protocol**: feels like an internal API call
//!
//! See the estate's cleave project for the transmutable integration surface.

// Empty — no FFI needed today.
