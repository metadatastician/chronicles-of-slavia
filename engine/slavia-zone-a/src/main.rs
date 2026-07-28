// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Playable Zone A (milestone M1) — the Bevy renderer over `slavia-core`.
//!
//! Run it:            `cargo run -p slavia-zone-a`
//! Verify logic only: `cargo test -p slavia-zone-a --no-default-features`

// `Session` is the sole bridge between renderer and rules core, in both the
// headless build (below) and the M2 Bevy renderer (`render.rs`/`menu/`).
mod session;
// Save/load — engine-agnostic like `session`, so it's available (if unused)
// in the headless build too, matching how `slavia-core` itself pulls in
// serde/toml unconditionally for its own SGS loader.
mod save;

#[cfg(feature = "render")]
mod menu;
#[cfg(feature = "render")]
mod render;
#[cfg(feature = "render")]
mod state;

fn main() {
    #[cfg(feature = "render")]
    run();

    #[cfg(not(feature = "render"))]
    {
        // Built without the renderer (no Bevy). Prove the session still drives.
        let mut s = session::Session::new();
        println!("slavia-zone-a built headless (no `render` feature).");
        println!("Active girl: {}", s.active_name());
        let r = s.approach_birds();
        println!("Anya reaches out to the birds -> {r:?}");
    }
}

/// Builds the one `App` for the whole game. Owns the window and the single
/// persistent `Camera2d` — both the menu (`menu::MenuPlugin`) and real
/// gameplay (`render::ZoneAPlugin`) render through it, so it's spawned once
/// here rather than by either plugin, and survives every state transition.
#[cfg(feature = "render")]
fn run() {
    use bevy::prelude::*;
    use state::AppState;

    App::new()
        .insert_resource(ClearColor(menu::theme::night()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chronicles of Slavia".into(),
                resolution: (1180.0_f32, 640.0_f32).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .add_plugins((menu::MenuPlugin, render::ZoneAPlugin))
        .run();
}
