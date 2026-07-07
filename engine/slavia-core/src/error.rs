// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Failures the core reports loudly rather than swallowing.

use std::fmt;

/// Why a crossing was refused. "Donna steadies what Anya crosses."
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossError {
    /// Donna has not steadied the bridge yet — there is nothing safe to cross.
    BridgeUnstable,
    /// The active character is not the one who crosses (only a stirring
    /// character crosses; a settling one steadies).
    NotTheCrosser,
}

impl fmt::Display for CrossError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrossError::BridgeUnstable => f.write_str("the bridge has not been steadied"),
            CrossError::NotTheCrosser => f.write_str("this character steadies, she does not cross"),
        }
    }
}

impl std::error::Error for CrossError {}

/// Why a Slavia Game Spec could not be loaded.
#[derive(Debug)]
pub struct SpecError(pub toml::de::Error);

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid Slavia Game Spec: {}", self.0)
    }
}

impl std::error::Error for SpecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<toml::de::Error> for SpecError {
    fn from(e: toml::de::Error) -> Self {
        SpecError(e)
    }
}
