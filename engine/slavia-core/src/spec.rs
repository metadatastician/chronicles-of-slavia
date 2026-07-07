// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The Slavia Game Spec (SGS) — Layer 1. A declarative description of a zone
//! that the rules core interprets and a renderer draws. Authored as TOML data,
//! not code, so the same spec can drive any renderer.

use crate::animal::Animal;
use crate::character::Character;
use crate::error::SpecError;
use serde::Deserialize;

/// Where in the Chronicle this spec sits.
#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    pub level: String,
    pub zone: String,
    pub name: String,
}

/// One story beat within a zone. The core reads beats as ordered data; how a
/// beat is staged is the renderer's business.
#[derive(Debug, Clone, Deserialize)]
pub struct Beat {
    pub id: String,
    pub title: String,
    /// What the player is meant to learn here, if anything.
    #[serde(default)]
    pub teaches: Option<String>,
    /// Text the beat surfaces (a shrine inscription, a line of narration).
    #[serde(default)]
    pub text: Option<String>,
}

/// A complete, renderer-agnostic description of one zone.
#[derive(Debug, Clone, Deserialize)]
pub struct Spec {
    pub meta: Meta,
    #[serde(default)]
    pub characters: Vec<Character>,
    #[serde(default)]
    pub animals: Vec<Animal>,
    #[serde(default)]
    pub beats: Vec<Beat>,
}

impl Spec {
    /// Parse a spec from TOML source.
    pub fn from_toml(source: &str) -> Result<Self, SpecError> {
        toml::from_str(source).map_err(SpecError::from)
    }

    /// Look up a character by id.
    pub fn character(&self, id: &str) -> Option<&Character> {
        self.characters.iter().find(|c| c.id == id)
    }

    /// Look up an animal by id.
    pub fn animal(&self, id: &str) -> Option<&Animal> {
        self.animals.iter().find(|a| a.id == id)
    }

    /// Look up a beat by id.
    pub fn beat(&self, id: &str) -> Option<&Beat> {
        self.beats.iter().find(|b| b.id == id)
    }
}
