// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Anya & Donna — sourced from `docs/design/09-anya-and-donna-backstories.md`
//! (core identity lines) and `06-attunement-and-modifiers.md` (the stir/
//! settle grammar). No animated "focus"/"demonstrate bond" figures — real
//! gameplay already demonstrates the grammar; text only here.

use super::{body, card, heading};
use crate::menu::fonts::MenuFont;
use bevy::prelude::*;

pub fn build(p: &mut ChildBuilder, font: &MenuFont) {
    heading(p, font, "Anya & Donna");
    body(
        p,
        font,
        "Two lands, one heart - each girl can only do her own half of the \
         world's grammar. Neither can do the other's act.",
    );
    card(
        p,
        font,
        "Anya - chaos, movement, courage, quickening",
        "\"It saw her chaos. It needed her chaos.\" She stirs the world; her \
         chaos is not destructive, it is transformative.",
    );
    card(
        p,
        font,
        "Donna - order, care, structure, settling",
        "\"It saw her order. It needed her order.\" She settles the world; her \
         order is not rigid, it is protective.",
    );
}
