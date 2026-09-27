// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Failures the core reports loudly rather than swallowing.

use std::fmt;

/// Why a crossing was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossError {
    /// The crossing has not been made passable yet — there is nothing safe to
    /// cross. In Zone A, Donna has not steadied it.
    BridgeUnstable,
}

impl fmt::Display for CrossError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrossError::BridgeUnstable => f.write_str("the crossing has not been made passable"),
        }
    }
}

impl std::error::Error for CrossError {}

/// Why a Slavia Game Spec could not be loaded.
#[derive(Debug)]
pub enum SpecError {
    Parse(toml::de::Error),
    Rooms(String),
}

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => write!(f, "invalid Slavia Game Spec: {error}"),
            Self::Rooms(error) => write!(f, "invalid SGS room topology: {error}"),
        }
    }
}

impl std::error::Error for SpecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Rooms(_) => None,
        }
    }
}

impl From<toml::de::Error> for SpecError {
    fn from(e: toml::de::Error) -> Self {
        Self::Parse(e)
    }
}
