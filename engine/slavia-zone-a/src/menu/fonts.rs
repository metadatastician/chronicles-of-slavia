// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! PT Serif (`assets/fonts/PT-Serif/`), copyright ParaType Ltd. under the
//! SIL Open Font License 1.1 (`assets/fonts/PT-Serif/OFL.txt`) — see
//! `LICENSE-INTENT.md`'s "OFL-1.1 font assets" section. Replaces Bevy's
//! bundled `default_font` fallback, whose glyph coverage doesn't include
//! curly quotes, middot, or section-sign (found by screenshot, not assumed —
//! see the menu's own commit history). PT Serif also has real Cyrillic
//! coverage, a deliberate choice for a Slavic-folklore game over an
//! arbitrary Western serif.

use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct MenuFont {
    pub regular: Handle<Font>,
    pub bold: Handle<Font>,
}

/// Called directly from `MenuPlugin::build`, not scheduled as a `Startup`
/// system: `OnEnter(AppState::Opening)` (the default state) turned out to
/// fire *before* `Startup` systems run in this Bevy version — confirmed by
/// an actual panic (`Res<MenuFont>` missing), not assumed from how other
/// engine versions typically order this. Loading synchronously at plugin-
/// build time, while `AssetServer` already exists (`DefaultPlugins` builds
/// before `MenuPlugin` in `main.rs`'s registration order) sidesteps the
/// race entirely instead of chasing schedule ordering.
pub fn load(assets: &AssetServer) -> MenuFont {
    MenuFont {
        regular: assets.load("fonts/PT-Serif/PT_Serif-Regular.ttf"),
        bold: assets.load("fonts/PT-Serif/PT_Serif-Bold.ttf"),
    }
}
