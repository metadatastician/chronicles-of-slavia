// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Settings & Accessibility — real, working toggles (genuinely flip
//! `UiSettings` on click) but honestly not wired to any actual behavior:
//! nothing else in this codebase has motion/contrast/text-size/caption
//! systems yet to wire them to. Not persisted (no save system).

use crate::menu::fonts::MenuFont;
use crate::menu::nav::UiSettings;
use crate::menu::theme;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Setting {
    ReducedMotion,
    HighContrast,
    LargeText,
    Captions,
}

#[derive(Component)]
pub struct SettingToggle(pub Setting);

fn is_on(s: &UiSettings, which: Setting) -> bool {
    match which {
        Setting::ReducedMotion => s.reduced_motion,
        Setting::HighContrast => s.high_contrast,
        Setting::LargeText => s.large_text,
        Setting::Captions => s.captions,
    }
}

fn flip(s: &mut UiSettings, which: Setting) {
    let target = match which {
        Setting::ReducedMotion => &mut s.reduced_motion,
        Setting::HighContrast => &mut s.high_contrast,
        Setting::LargeText => &mut s.large_text,
        Setting::Captions => &mut s.captions,
    };
    *target = !*target;
}

fn row(p: &mut ChildBuilder, font: &MenuFont, settings: &UiSettings, which: Setting, label: &str) {
    let on = is_on(settings, which);
    let regular = font.regular.clone();
    p.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(10.0),
        margin: UiRect::top(Val::Px(8.0)),
        ..default()
    })
    .with_children(|row| {
        row.spawn((
            Button,
            SettingToggle(which),
            Node {
                width: Val::Px(44.0),
                height: Val::Px(24.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderColor(theme::gold()),
            BackgroundColor(if on { theme::gold() } else { Color::NONE }),
        ));
        row.spawn((
            Text::new(label.to_string()),
            TextFont {
                font: regular,
                font_size: 14.0,
                ..default()
            },
            TextColor(theme::ink()),
        ));
    });
}

pub fn build(p: &mut ChildBuilder, font: &MenuFont, settings: &UiSettings) {
    super::heading(p, font, "Settings & Accessibility");
    super::body(
        p,
        font,
        "These toggles work, but nothing downstream reads them yet - no \
         motion, contrast, text-size or caption system exists in this build.",
    );
    row(p, font, settings, Setting::ReducedMotion, "Reduced motion");
    row(p, font, settings, Setting::HighContrast, "High contrast");
    row(p, font, settings, Setting::LargeText, "Large text");
    row(p, font, settings, Setting::Captions, "Captions");
}

/// Flips the backing `UiSettings` bool and repaints the toggle pill.
/// Standalone from `panels::dispatch` because this panel doesn't get
/// rebuilt on every click — only nav changes trigger a rebuild.
pub fn toggle_click(
    mut q: Query<(&Interaction, &SettingToggle, &mut BackgroundColor), Changed<Interaction>>,
    mut settings: ResMut<UiSettings>,
) {
    for (interaction, toggle, mut bg) in &mut q {
        if *interaction == Interaction::Pressed {
            flip(&mut settings, toggle.0);
            *bg = if is_on(&settings, toggle.0) {
                theme::gold().into()
            } else {
                Color::NONE.into()
            };
        }
    }
}
