// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The persistent menu scaffold: topbar, left nav, and footer — spawned once
//! on entering [`crate::state::AppState::MenuShell`], torn down on leaving
//! it. Ported from `docs/design/chronicles-landing-page.html`'s `.topbar`/
//! `.menu-panel`/`.footer`.

use crate::menu::fonts::MenuFont;
use crate::menu::nav::{label, CurrentPanel, MenuView, NavButton};
use crate::menu::{theme, SaveSlot};
use bevy::prelude::*;

/// Everything `spawn_shell` creates — despawned wholesale on `OnExit`.
#[derive(Component)]
pub struct MenuShellRoot;

/// The empty container panel content mounts into (`panels::mod`'s dispatch
/// system finds this by marker and spawns/despawns its children).
#[derive(Component)]
pub struct ContextPanelRoot;

/// An empty UI mount point — reserves layout space over the world-space
/// centre column so the nav/context "glass" panels either side don't
/// visually overlap where `crate::menu::heroine` draws Anya & Donna
/// underneath. Carries no content of its own.
#[derive(Component)]
pub struct CentreStageRoot;

const NAV_ORDER: [MenuView; 8] = [
    MenuView::Continue,
    MenuView::NewChronicle,
    MenuView::Chapters,
    MenuView::WorldBook,
    MenuView::Bond,
    MenuView::Ums,
    MenuView::Settings,
    MenuView::Credits,
];

pub fn spawn_shell(
    mut commands: Commands,
    mut current: ResMut<CurrentPanel>,
    font: Res<MenuFont>,
    save_slot: Res<SaveSlot>,
) {
    let regular = font.regular.clone();
    let bold = font.bold.clone();
    // Force a change signal even if the panel selection is unchanged from
    // last time — `panels::dispatch` only (re)populates `ContextPanelRoot`
    // on `CurrentPanel::is_changed()`, and re-entering the menu (e.g. Esc
    // from gameplay) respawns an empty `ContextPanelRoot` that otherwise
    // wouldn't get filled again.
    current.set_changed();
    commands
        .spawn((
            MenuShellRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            // Transparent: the world layer (`crate::menu::background`/
            // `heroine`) shows through behind the UI; only the topbar/nav/
            // context "glass" panels below keep their own backgrounds.
            BackgroundColor(Color::NONE),
        ))
        .with_children(|root| {
            // topbar
            root.spawn(Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            })
            .insert(BorderColor(theme::line()))
            .with_children(|top| {
                top.spawn((
                    Text::new("Chronicles of Slavia"),
                    TextFont {
                        font: bold.clone(),
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(theme::ink()),
                ));
                top.spawn((
                    Text::new("The First Tear - Border Path"),
                    TextFont {
                        font: regular.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(theme::muted()),
                ));
            });

            // body: nav aside + panel-mount container
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                ..default()
            })
            .with_children(|body| {
                // nav
                body.spawn(Node {
                    width: Val::Px(300.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    border: UiRect::right(Val::Px(1.0)),
                    ..default()
                })
                .insert(BorderColor(theme::line()))
                .with_children(|nav| {
                    for view in NAV_ORDER {
                        let enabled = match view {
                            MenuView::Continue => save_slot.0.is_some(),
                            _ => true,
                        };
                        let (title, sub, key) = label(view);
                        let border = if current.0 == view {
                            theme::gold()
                        } else {
                            Color::NONE
                        };
                        nav.spawn((
                            Button,
                            NavButton { view, enabled },
                            Node {
                                padding: UiRect::all(Val::Px(10.0)),
                                flex_direction: FlexDirection::Column,
                                border: UiRect::left(Val::Px(3.0)),
                                ..default()
                            },
                            BorderColor(border),
                            BackgroundColor(Color::NONE),
                        ))
                        .with_children(|b| {
                            let title_color = if enabled {
                                theme::ink()
                            } else {
                                theme::faint()
                            };
                            b.spawn((
                                Text::new(format!("{title}  [{key}]")),
                                TextFont {
                                    font: bold.clone(),
                                    font_size: 15.0,
                                    ..default()
                                },
                                TextColor(title_color),
                            ));
                            b.spawn((
                                Text::new(if !enabled {
                                    format!("{sub} - no saved chronicle yet")
                                } else if view == MenuView::Continue {
                                    save_slot
                                        .0
                                        .as_ref()
                                        .map(|d| d.summary.clone())
                                        .unwrap_or_else(|| sub.to_string())
                                } else {
                                    sub.to_string()
                                }),
                                TextFont {
                                    font: regular.clone(),
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(theme::faint()),
                            ));
                        });
                    }
                });

                // centre stage: reserves layout space over the world-space
                // heroine figures (`crate::menu::heroine`) — no content of
                // its own, so nothing renders here but the world behind it.
                body.spawn((
                    CentreStageRoot,
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                ));

                // panel mount point
                body.spawn((
                    ContextPanelRoot,
                    Node {
                        width: Val::Px(360.0),
                        padding: UiRect::all(Val::Px(24.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(theme::panel()),
                ));
            });

            // footer
            root.spawn(Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::top(Val::Px(1.0)),
                ..default()
            })
            .insert(BorderColor(theme::line()))
            .with_children(|footer| {
                footer.spawn((
                    Text::new("Slavia is memory, medicine, folklore and moral choice."),
                    TextFont {
                        font: regular.clone(),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(theme::muted()),
                ));
            });
        });
}

pub fn despawn_shell(mut commands: Commands, q: Query<Entity, With<MenuShellRoot>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}
