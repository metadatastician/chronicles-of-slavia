// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Chapter Map — static, matches the mock-up's own honest availability
//! labels (already consistent with `docs/design/08-level1-five-zone-map.md`
//! and `23-level-scope-and-pacing.md`: only Zone A is built).

use super::{card, heading};
use bevy::prelude::*;

pub fn build(p: &mut ChildBuilder) {
    heading(p, "Chapter Map");
    card(
        p,
        "I - The First Tear - Available",
        "Zone A: Border Path. Shrine, birds, unstable bridge and the first Rift interruption.",
    );
    card(
        p,
        "II - Beyond the Overlook - Sealed",
        "In the world design, not yet part of the playable build.",
    );
    card(
        p,
        "The Mirror of Misalignment - Parked",
        "A future corruption that weaponises each heroine's imbalance against the other.",
    );
}
