// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The startup interface — a real, working implementation of the mock-up at
//! `docs/design/chronicles-landing-page.html` (documented in `docs/design/
//! 20-startup-interface-mockup.md`), as a genuine Bevy in-game menu rather
//! than a second (Elixir/web) tech stack, since it needs to read real save
//! state and hand off directly into actual gameplay.
//!
//! "Continue the Chronicle" reads a real save, written by `crate::render`'s
//! `teardown`/`announce_progress` — see `panels::continue_chronicle`.
//!
//! Structure: [`opening`] (title card) -> [`shell`] (nav + [`panels`]) ->
//! [`crate::render::ZoneAPlugin`]'s `AppState::Playing`, driven entirely by
//! `crate::state::AppState` — this plugin never touches `render.rs` directly.

mod background;
pub mod fonts;
mod heroine;
pub mod nav;
mod opening;
mod panels;
mod shell;
pub mod theme;

use crate::save::SaveData;
use crate::state::AppState;
use bevy::prelude::*;
pub use heroine::BondFocus;
use nav::{CurrentPanel, UiSettings};

/// The last save on disk, read once at startup and refreshed by
/// `render::teardown` every time gameplay is left. `None` means no save
/// exists yet — "Continue the Chronicle" stays disabled.
#[derive(Resource)]
pub struct SaveSlot(pub Option<SaveData>);

/// Which session `render::setup` should build on the next
/// `OnEnter(AppState::Playing)` — set by whichever menu button makes the
/// transition into gameplay.
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
pub enum LaunchMode {
    #[default]
    New,
    Continue,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // Loaded synchronously here, not as a `Startup` system — see
        // `fonts::load`'s doc comment for why.
        let font = {
            let assets = app.world().resource::<AssetServer>();
            fonts::load(assets)
        };
        app.insert_resource(font)
            .insert_resource(SaveSlot(crate::save::read()))
            .init_resource::<LaunchMode>()
            .init_resource::<CurrentPanel>()
            .init_resource::<UiSettings>()
            .init_resource::<BondFocus>()
            .add_systems(
                OnEnter(AppState::Opening),
                (
                    background::spawn.run_if(background::absent),
                    opening::spawn_opening,
                ),
            )
            .add_systems(OnExit(AppState::Opening), opening::despawn_opening)
            .add_systems(
                Update,
                opening::enter_button.run_if(in_state(AppState::Opening)),
            )
            .add_systems(
                OnEnter(AppState::MenuShell),
                (
                    background::spawn.run_if(background::absent),
                    heroine::spawn.run_if(heroine::absent),
                    shell::spawn_shell,
                ),
            )
            .add_systems(
                OnExit(AppState::MenuShell),
                (background::despawn, heroine::despawn, shell::despawn_shell),
            )
            .add_systems(
                Update,
                (
                    nav::nav_click,
                    nav::nav_highlight,
                    nav::keyboard_shortcuts,
                    panels::dispatch,
                    panels::settings::toggle_click,
                    panels::begin_chronicle,
                    panels::continue_chronicle,
                )
                    .chain()
                    .run_if(in_state(AppState::MenuShell)),
            )
            .add_systems(
                Update,
                (
                    background::animate_rift,
                    background::animate_birds,
                    background::animate_mist,
                    background::animate_fireflies,
                    heroine::apply_focus,
                )
                    .run_if(not(in_state(AppState::Playing))),
            );
    }
}
