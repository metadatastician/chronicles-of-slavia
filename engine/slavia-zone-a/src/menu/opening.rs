// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The title card: "Invisible Door presents / Chronicles of Slavia / [motto]
//! / Enter the Chronicle". Ported from the mock-up's `#opening` section.

use crate::menu::theme;
use crate::state::AppState;
use bevy::prelude::*;

#[derive(Component)]
pub struct OpeningRoot;

#[derive(Component)]
pub(super) struct EnterButton;

pub fn spawn_opening(mut commands: Commands) {
    commands
        .spawn((
            OpeningRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(14.0),
                ..default()
            },
            BackgroundColor(theme::night()),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Invisible Door presents"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme::muted()),
            ));
            p.spawn((
                Text::new("Chronicles of Slavia"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(theme::ink()),
            ));
            p.spawn((
                Text::new("\"What do you become when the world breaks?\""),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme::gold_soft()),
            ));
            p.spawn((
                Button,
                EnterButton,
                Node {
                    margin: UiRect::top(Val::Px(24.0)),
                    padding: UiRect::axes(Val::Px(28.0), Val::Px(12.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor(theme::gold()),
                BackgroundColor(Color::NONE),
            ))
            .with_children(|b| {
                b.spawn((
                    Text::new("Enter the Chronicle"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(theme::gold()),
                ));
            });
        });
}

pub fn despawn_opening(mut commands: Commands, q: Query<Entity, With<OpeningRoot>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

pub fn enter_button(
    q: Query<&Interaction, (Changed<Interaction>, With<EnterButton>)>,
    mut next: ResMut<NextState<AppState>>,
) {
    for interaction in &q {
        if *interaction == Interaction::Pressed {
            next.set(AppState::MenuShell);
        }
    }
}
