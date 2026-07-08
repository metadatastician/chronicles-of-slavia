// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Playable Zone A (milestone M1) — the Bevy renderer over `slavia-core`.
//!
//! Run it:            `cargo run -p slavia-zone-a`
//! Verify logic only: `cargo test -p slavia-zone-a --no-default-features`

mod session;

#[cfg(feature = "render")]
mod render;

fn main() {
    #[cfg(feature = "render")]
    render::run();

    #[cfg(not(feature = "render"))]
    {
        // Built without the renderer (no Bevy). Prove the session still drives.
        let mut s = session::Session::new();
        println!("slavia-zone-a built headless (no `render` feature).");
        println!("Active girl: {}", s.active_name());
        let r = s.approach_birds();
        println!("Anya reaches out to the birds -> {r:?}");
    }
}
