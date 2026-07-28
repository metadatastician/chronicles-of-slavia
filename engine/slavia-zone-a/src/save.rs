// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Save/load — a TOML snapshot of everything [`crate::session::Session`]
//! needs to resume exactly where a play session left off. Engine-agnostic
//! (no Bevy types), like `session.rs` itself.

use crate::session::{Beats, BirdState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};

/// Everything needed to reconstruct a [`crate::session::Session`]. Does not
/// save `Spec` — it's reconstructed fresh from `zone_a()` on every load, the
/// same as a new session does.
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub active_id: String,
    pub rift_active: bool,
    pub bridge_stable: bool,
    pub crossed: bool,
    pub birds: BirdState,
    pub beats: Beats,
    pub pos: HashMap<String, f32>,
    pub revealed: Vec<bool>,
    /// A human-readable line describing where this save left off (e.g.
    /// "Anya, at the grove"), computed once at save time so the menu never
    /// needs its own beat-lookup logic to describe a save.
    pub summary: String,
}

/// Where the save file lives: next to the running executable, falling back
/// to the current directory. A proper OS-appropriate save directory (e.g.
/// via a `dirs`-style crate) is the correct long-term answer but isn't
/// worth a new dependency yet — deliberate v1 simplification.
fn save_path() -> PathBuf {
    let dir = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();
    dir.join("chronicles-of-slavia-save.toml")
}

pub fn write(data: &SaveData) -> io::Result<()> {
    let text = toml::to_string(data).map_err(io::Error::other)?;
    fs::write(save_path(), text)
}

/// Reads back the last save, if any. A missing file and an unparseable one
/// are both treated as "no save" — not worth distinguishing yet.
pub fn read() -> Option<SaveData> {
    let text = fs::read_to_string(save_path()).ok()?;
    toml::from_str(&text).ok()
}
