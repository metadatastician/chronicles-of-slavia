// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Credits & Provenance — copy corrected against `LICENSE-INTENT.md`/
//! ADR-0008, not copied verbatim from the mock-up. The mock-up says engine
//! code is MPL-2.0; it is actually AGPL-3.0-or-later, with MPL-2.0 only for
//! specific incorporated components (permitted under MPL-2.0 §3.3).

use super::{body, card, heading};
use crate::menu::fonts::MenuFont;
use bevy::prelude::*;

pub fn build(p: &mut ChildBuilder, font: &MenuFont) {
    heading(p, font, "Credits & Provenance");
    body(
        p,
        font,
        "Chronicles of Slavia is layered, and each layer is licensed for what it is.",
    );
    card(p, font, "Engine & code", "AGPL-3.0-or-later");
    card(
        p,
        font,
        "Content (design notes, lore, writing, worldbuilding, art)",
        "CC-BY-SA-4.0",
    );
    card(
        p,
        font,
        "Names & marks (Chronicles of Slavia, Anya, Donna)",
        "Reserved",
    );
    body(
        p,
        font,
        "Some incorporated components carry MPL-2.0 and keep it - permitted \
         under MPL-2.0 section 3.3 as part of this Larger Work.",
    );
    card(
        p,
        font,
        "Typeface",
        "PT Serif, copyright ParaType Ltd., under the SIL Open Font License 1.1.",
    );
}
