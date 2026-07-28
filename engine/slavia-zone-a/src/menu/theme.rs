// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The menu's palette — ported directly from `docs/design/
//! chronicles-landing-page.html`'s `:root` CSS custom properties, not
//! invented fresh. `Color::srgb_u8`/`srgba_u8` aren't `const fn` in this
//! Bevy version, so these are functions, matching `render.rs`'s own
//! color-helper convention rather than introducing a different pattern.

use bevy::prelude::*;

pub fn night() -> Color {
    Color::srgb_u8(0x13, 0x0f, 0x1c)
}
pub fn ink() -> Color {
    Color::srgb_u8(0xf5, 0xef, 0xe4)
}
pub fn muted() -> Color {
    Color::srgb_u8(0xc2, 0xb6, 0xaa)
}
pub fn faint() -> Color {
    Color::srgb_u8(0x88, 0x7c, 0x78)
}
/// `--panel: rgba(23, 17, 31, .90)`
pub fn panel() -> Color {
    Color::srgba_u8(23, 17, 31, 230)
}
/// `--panel-soft: rgba(37, 26, 43, .84)`
pub fn panel_soft() -> Color {
    Color::srgba_u8(37, 26, 43, 214)
}
/// `--line: rgba(239, 218, 178, .22)`
pub fn line() -> Color {
    Color::srgba_u8(239, 218, 178, 56)
}
pub fn gold() -> Color {
    Color::srgb_u8(0xe4, 0xbd, 0x72)
}
pub fn gold_soft() -> Color {
    Color::srgb_u8(0x9c, 0x79, 0x48)
}
/// The Rift's seam — pulses between these two in `background::animate_rift`.
pub fn rift() -> Color {
    Color::srgb_u8(0xad, 0x70, 0xe7)
}
pub fn rift_hot() -> Color {
    Color::srgb_u8(0xe8, 0x8b, 0xd7)
}
/// Heroine costume colors, shared between `background`/`heroine` (the
/// centre-stage figures) — one-off world/scene colors (ridge, moon, birds,
/// bridge, ...) live as local helpers in `background.rs`/`heroine.rs`
/// themselves, matching `render.rs`'s own convention of colocating
/// gameplay-scene colors rather than centralizing every hue here.
pub fn anya() -> Color {
    Color::srgb_u8(0xe8, 0x57, 0x4f)
}
pub fn donna() -> Color {
    Color::srgb_u8(0x65, 0xa9, 0x6b)
}
