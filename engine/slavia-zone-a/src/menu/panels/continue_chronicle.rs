// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! "Continue the Chronicle" — real once a save exists: shows where the last
//! session left off (`SaveData::summary`) and resumes it exactly, restoring
//! both girls' positions and the world's transition state, rather than
//! starting fresh. The nav button that reaches this panel is only enabled
//! when a save exists (`menu::shell::spawn_shell`), so the `None` branch
//! below is a defensive fallback, not a normally-reachable path.

use super::{body, heading};
use crate::menu::fonts::MenuFont;
use crate::menu::theme;
use crate::save::SaveData;
use bevy::prelude::*;

/// The "Resume" button — read by `continue_chronicle` in `panels::mod`.
#[derive(Component)]
pub struct ContinueButton;

pub fn build(p: &mut ChildBuilder, font: &MenuFont, save: Option<&SaveData>) {
    heading(p, font, "Continue the Chronicle");
    let Some(data) = save else {
        body(
            p,
            font,
            "No saved chronicle yet. Saving isn't built - begin a new one below.",
        );
        return;
    };
    body(p, font, &data.summary);
    p.spawn((
        Button,
        ContinueButton,
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
            Text::new("Resume"),
            TextFont {
                font: font.bold.clone(),
                font_size: 15.0,
                ..default()
            },
            TextColor(theme::gold()),
        ));
    });
}
