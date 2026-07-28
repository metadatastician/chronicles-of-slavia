// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The app-level screen state — shared by [`crate::menu`] and [`crate::render`]
//! so neither owns the other. The menu drives transitions (`NextState`); Zone A
//! only reacts to being in [`AppState::Playing`] via `run_if(in_state(...))`.

use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum AppState {
    /// The title card ("Enter the Chronicle").
    #[default]
    Opening,
    /// The main menu shell (Continue / New Chronicle / Chapter Map / ...).
    MenuShell,
    /// Real Zone A gameplay — the only state [`crate::render::ZoneAPlugin`] runs in.
    Playing,
}
