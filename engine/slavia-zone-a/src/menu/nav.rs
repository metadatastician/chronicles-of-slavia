// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Menu navigation: which panel is showing, and how it changes — a click on
//! a [`NavButton`], or its keyboard shortcut, writes [`CurrentPanel`]; the
//! panel-dispatch system in `panels::mod` reacts to that resource changing.
//! Ported from the mock-up's own `menu` array and `keys` map
//! (`docs/design/chronicles-landing-page.html`).

use crate::menu::theme;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuView {
    Continue,
    NewChronicle,
    Chapters,
    WorldBook,
    Bond,
    Ums,
    Settings,
    Credits,
}

/// Which panel the centre pane currently shows. Defaults to `NewChronicle`,
/// not the mock-up's `Continue` default — there is no save system, so
/// defaulting to a disabled panel would just be a dead-end first screen.
#[derive(Resource, Clone, Copy)]
pub struct CurrentPanel(pub MenuView);

impl Default for CurrentPanel {
    fn default() -> Self {
        CurrentPanel(MenuView::NewChronicle)
    }
}

/// Real toggle state for the Settings panel — genuinely flips on click, but
/// (honestly) wired to nothing downstream yet; nothing else in this codebase
/// has motion/contrast/subtitle behavior to wire it to.
#[derive(Resource, Default, Clone, Copy)]
pub struct UiSettings {
    pub reduced_motion: bool,
    pub high_contrast: bool,
    pub large_text: bool,
    pub captions: bool,
}

#[derive(Component)]
pub struct NavButton {
    pub view: MenuView,
    pub enabled: bool,
}

pub fn label(view: MenuView) -> (&'static str, &'static str, char) {
    match view {
        MenuView::Continue => (
            "Continue the Chronicle",
            "Return to the unstable bridge",
            'C',
        ),
        MenuView::NewChronicle => (
            "Begin a New Chronicle",
            "Start again at the first tear",
            'N',
        ),
        MenuView::Chapters => ("Chapter Map", "Places, memories and unfinished paths", 'M'),
        MenuView::WorldBook => (
            "The World Book",
            "Folklore, remedies and remembered choices",
            'W',
        ),
        MenuView::Bond => ("Anya & Donna", "The bond that lets the world answer", 'B'),
        MenuView::Ums => (
            "Universal Modding Studio",
            "Open this world for authoring",
            'U',
        ),
        MenuView::Settings => (
            "Settings & Accessibility",
            "Motion, reading, sound and assistance",
            'S',
        ),
        MenuView::Credits => ("Credits & Provenance", "People, sources and licences", 'I'),
    }
}

pub fn nav_click(
    mut q: Query<(&Interaction, &NavButton, &mut BackgroundColor), Changed<Interaction>>,
    mut current: ResMut<CurrentPanel>,
) {
    for (interaction, nav, mut bg) in &mut q {
        if !nav.enabled {
            continue;
        }
        match *interaction {
            Interaction::Pressed => current.0 = nav.view,
            Interaction::Hovered => *bg = theme::panel_soft().into(),
            Interaction::None => *bg = Color::NONE.into(),
        }
    }
}

/// Highlights whichever nav button matches the current panel, mirroring the
/// mock-up's `.menu-btn.active` gold left-bar treatment (here: background).
pub fn nav_highlight(current: Res<CurrentPanel>, mut q: Query<(&NavButton, &mut BorderColor)>) {
    if !current.is_changed() {
        return;
    }
    for (nav, mut border) in &mut q {
        *border = if nav.view == current.0 {
            theme::gold().into()
        } else {
            Color::NONE.into()
        };
    }
}

pub fn keyboard_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    mut current: ResMut<CurrentPanel>,
    q: Query<&NavButton>,
) {
    for nav in &q {
        if !nav.enabled {
            continue;
        }
        let (_, _, key) = label(nav.view);
        let code = match key {
            'C' => KeyCode::KeyC,
            'N' => KeyCode::KeyN,
            'M' => KeyCode::KeyM,
            'W' => KeyCode::KeyW,
            'B' => KeyCode::KeyB,
            'U' => KeyCode::KeyU,
            'S' => KeyCode::KeyS,
            'I' => KeyCode::KeyI,
            _ => continue,
        };
        if keys.just_pressed(code) {
            current.0 = nav.view;
        }
    }
}
