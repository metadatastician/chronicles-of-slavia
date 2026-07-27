// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The startup interface — a real, working implementation of the mock-up at
//! `docs/design/chronicles-landing-page.html` (documented in `docs/design/
//! 20-startup-interface-mockup.md`), as a genuine Bevy in-game menu rather
//! than a second (Elixir/web) tech stack, since it needs to read real save
//! state and hand off directly into actual gameplay.
//!
//! **No save system exists anywhere in this codebase.** "Continue the
//! Chronicle" is therefore honestly disabled rather than faked — see
//! `panels::mod`'s `continue_panel`. Building persistence is a separate,
//! substantial feature, not smuggled into this one.
//!
//! Structure: [`opening`] (title card) -> [`shell`] (nav + [`panels`]) ->
//! [`crate::render::ZoneAPlugin`]'s `AppState::Playing`, driven entirely by
//! `crate::state::AppState` — this plugin never touches `render.rs` directly.

pub mod nav;
mod opening;
mod panels;
mod shell;
pub mod theme;

use crate::state::AppState;
use bevy::prelude::*;
use nav::{CurrentPanel, UiSettings};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentPanel>()
            .init_resource::<UiSettings>()
            .add_systems(OnEnter(AppState::Opening), opening::spawn_opening)
            .add_systems(OnExit(AppState::Opening), opening::despawn_opening)
            .add_systems(
                Update,
                opening::enter_button.run_if(in_state(AppState::Opening)),
            )
            .add_systems(OnEnter(AppState::MenuShell), shell::spawn_shell)
            .add_systems(OnExit(AppState::MenuShell), shell::despawn_shell)
            .add_systems(
                Update,
                (
                    nav::nav_click,
                    nav::nav_highlight,
                    nav::keyboard_shortcuts,
                    panels::dispatch,
                    panels::settings::toggle_click,
                    panels::begin_chronicle,
                )
                    .chain()
                    .run_if(in_state(AppState::MenuShell)),
            );
    }
}
