// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The World Book — the four pillars, verbatim from `docs/design/
//! 01-world-principle.md` line 5: "It is memory, medicine, folklore, and
//! moral choice."

use super::{body, card, heading};
use crate::menu::fonts::MenuFont;
use bevy::prelude::*;

pub fn build(p: &mut ChildBuilder, font: &MenuFont) {
    heading(p, font, "The World Book");
    body(
        p,
        font,
        "Slavia is memory, medicine, folklore and moral choice.",
    );
    card(
        p,
        font,
        "Memory",
        "Places retain repeated care, violence, neglect and repair.",
    );
    card(
        p,
        font,
        "Medicine",
        "Remedies are material, ethical and diagnostic - not generic potions.",
    );
    card(
        p,
        font,
        "Folklore",
        "Signs and creatures act according to situated traditions and local nature.",
    );
    card(
        p,
        font,
        "Moral Choice",
        "Decisions leave marks in relationships, landscapes and later possibilities.",
    );
}
