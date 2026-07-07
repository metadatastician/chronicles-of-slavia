// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Characters and the signature gift that belongs to Anya and Donna alone.

use serde::Deserialize;

/// The direction an attuned character bends **taxis** — the rate and direction
/// of a living thing's responsive motion (`docs/design/06`).
///
/// Anya *raises* it (quickens, emboldens, dispels panic); Donna *lowers* it
/// (calms, settles). Both act *with* an animal's nature, drawing out its best —
/// this is attunement, not domination (see `Response`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Taxis {
    /// Quicken, embolden, give confidence. (Anya.)
    Raise,
    /// Calm, settle, steady, reassure. (Donna.)
    Lower,
}

/// A character in the world.
///
/// `attunement` is the crux correction to the model: modulating taxis is a
/// **signature gift of Anya and Donna only**, not a property any calm or
/// excitable actor gets. Everyone else has `None`. (Class will later inflect
/// *how* an attuned character's gift manifests; not modelled yet. Enemies who
/// *dominate* animals are a separate category — see `docs/design/06` — and are
/// likewise not modelled in Zone A.)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub attunement: Option<Taxis>,
}
