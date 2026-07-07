// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Characters and the direction each one bends the rhythm of the world.

use serde::Deserialize;

/// Which way a character bends the rhythm of the world around them.
///
/// From `docs/design/03-living-taxis.md`: Anya stirs, Donna settles. The
/// mechanic keys off this *influence*, never off a character's name, so new
/// characters that stir or settle inherit the grammar for free.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Influence {
    /// Excite, quicken, stir, embolden, agitate. (Anya.)
    Stir,
    /// Calm, slow, settle, quieten, reassure. (Donna.)
    Settle,
}

/// One of the girls the player switches between.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub influence: Influence,
}
