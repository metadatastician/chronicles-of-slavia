// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! "Begin a New Chronicle" — the only panel with a real, working action.
//! Lists all three of the mock-up's modes, but only Story Chronicle exists;
//! the other two get an honest "Not implemented" tag rather than the
//! mock-up's misleading "Reflective"/"Exploratory" labels.

use super::{body, card, heading};
use crate::menu::theme;
use bevy::prelude::*;

/// The "Begin New Chronicle" button — the one real transition in the whole
/// menu. Read by `begin_chronicle` in `menu/mod.rs`.
#[derive(Component)]
pub struct BeginChronicleButton;

pub fn build(p: &mut ChildBuilder) {
    heading(p, "Begin a New Chronicle");
    card(
        p,
        "Story Chronicle",
        "The authored journey of Anya, Donna and the first tear.",
    );
    card(p, "Memory Walk - Not implemented", "");
    card(p, "Living World Study - Not implemented", "");

    p.spawn((
        Button,
        BeginChronicleButton,
        Node {
            margin: UiRect::top(Val::Px(16.0)),
            padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BorderColor(theme::gold()),
        BackgroundColor(Color::NONE),
    ))
    .with_children(|b| {
        b.spawn((
            Text::new("Begin New Chronicle"),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextColor(theme::gold()),
        ));
    });

    body(p, "Zone A: Border Path. Shrine, birds, unstable bridge.");
}
