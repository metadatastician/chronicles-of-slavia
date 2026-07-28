// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Panel dispatch: when [`CurrentPanel`] changes, despawn whatever was
//! mounted in [`ContextPanelRoot`] and spawn the new panel's content. Also
//! houses `continue`/`ums`, each under 15 lines of static text — not worth
//! their own file.

mod bond;
mod chapters;
mod continue_chronicle;
mod credits;
mod new_chronicle;
pub mod settings;
mod world_book;

use crate::menu::fonts::MenuFont;
use crate::menu::nav::{CurrentPanel, MenuView, UiSettings};
use crate::menu::shell::ContextPanelRoot;
use crate::menu::{theme, LaunchMode, SaveSlot};
use crate::state::AppState;
use bevy::prelude::*;

/// Shared small heading widget every panel opens with, so the eight panel
/// modules don't each re-derive the same `Text`/`TextFont`/`TextColor` bundle.
pub fn heading(p: &mut ChildBuilder, font: &MenuFont, title: &str) {
    p.spawn((
        Text::new(title.to_string()),
        TextFont {
            font: font.bold.clone(),
            font_size: 22.0,
            ..default()
        },
        TextColor(theme::ink()),
    ));
}

/// Shared body-text widget for a panel's ordinary paragraphs.
pub fn body(p: &mut ChildBuilder, font: &MenuFont, text: &str) {
    p.spawn((
        Text::new(text.to_string()),
        TextFont {
            font: font.regular.clone(),
            font_size: 14.0,
            ..default()
        },
        TextColor(theme::muted()),
    ));
}

/// One "pillar"/"tag" style small card: a bold label + a short description,
/// the shape the mock-up reuses across the World Book, Chapter Map, and
/// New Chronicle panels.
pub fn card(p: &mut ChildBuilder, font: &MenuFont, label: &str, desc: &str) {
    p.spawn(Node {
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(10.0)),
        margin: UiRect::top(Val::Px(6.0)),
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    })
    .insert(BorderColor(theme::line()))
    .with_children(|c| {
        c.spawn((
            Text::new(label.to_string()),
            TextFont {
                font: font.bold.clone(),
                font_size: 15.0,
                ..default()
            },
            TextColor(theme::gold()),
        ));
        c.spawn((
            Text::new(desc.to_string()),
            TextFont {
                font: font.regular.clone(),
                font_size: 13.0,
                ..default()
            },
            TextColor(theme::muted()),
        ));
    });
}

pub fn dispatch(
    mut commands: Commands,
    current: Res<CurrentPanel>,
    settings: Res<UiSettings>,
    save_slot: Res<SaveSlot>,
    font: Res<MenuFont>,
    root: Query<Entity, With<ContextPanelRoot>>,
    children: Query<&Children>,
) {
    if !current.is_changed() {
        return;
    }
    let Some(root_entity) = root.iter().next() else {
        return;
    };
    if let Ok(kids) = children.get(root_entity) {
        for kid in kids.iter() {
            commands.entity(*kid).despawn_recursive();
        }
    }
    let view = current.0;
    let settings = *settings;
    let font = font.clone();
    commands.entity(root_entity).with_children(|p| match view {
        MenuView::Continue => continue_chronicle::build(p, &font, save_slot.0.as_ref()),
        MenuView::NewChronicle => new_chronicle::build(p, &font),
        MenuView::Chapters => chapters::build(p, &font),
        MenuView::WorldBook => world_book::build(p, &font),
        MenuView::Bond => bond::build(p, &font),
        MenuView::Ums => ums_panel(p, &font),
        MenuView::Settings => settings::build(p, &font, &settings),
        MenuView::Credits => credits::build(p, &font),
    });
}

/// Static stub only. `docs/design/20-startup-interface-mockup.md` is explicit
/// that a real implementation "must not move Slavia-specific ontology into
/// Universal Modding Studio Core" — UMS is a separate external platform
/// (`ECOSYSTEM.a2ml` lists it as a prospective-consumer), not integrated here.
fn ums_panel(p: &mut ChildBuilder, font: &MenuFont) {
    heading(p, font, "Universal Modding Studio");
    body(
        p,
        font,
        "Chronicles of Slavia is authored as a profile within the separate \
         Universal Modding Studio platform. That authoring surface isn't part \
         of this build - this panel is a placeholder for the portal, not the \
         portal itself.",
    );
}

/// "Begin New Chronicle" -> [`AppState::Playing`] with a fresh
/// [`crate::session::Session`] (`crate::render::setup` reads [`LaunchMode`]
/// to decide `Session::new()` vs restoring a save).
pub fn begin_chronicle(
    q: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<new_chronicle::BeginChronicleButton>,
        ),
    >,
    mut launch: ResMut<LaunchMode>,
    mut next: ResMut<NextState<AppState>>,
) {
    for interaction in &q {
        if *interaction == Interaction::Pressed {
            *launch = LaunchMode::New;
            next.set(AppState::Playing);
        }
    }
}

/// "Continue the Chronicle" -> [`AppState::Playing`], restoring the last
/// save (`crate::render::setup` does the actual restore, reading
/// [`LaunchMode`] and [`SaveSlot`]).
pub fn continue_chronicle(
    q: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<continue_chronicle::ContinueButton>,
        ),
    >,
    mut launch: ResMut<LaunchMode>,
    mut next: ResMut<NextState<AppState>>,
) {
    for interaction in &q {
        if *interaction == Interaction::Pressed {
            *launch = LaunchMode::Continue;
            next.set(AppState::Playing);
        }
    }
}
