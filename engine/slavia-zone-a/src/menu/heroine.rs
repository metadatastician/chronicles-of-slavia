// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! Anya and Donna's centre-stage figures — visible under every menu panel,
//! matching the mock-up's persistent `.centre` column (not Bond-panel
//! content; the Bond panel's own buttons just set [`BondFocus`]). A fresh,
//! small 4-part figure (head/hair/body/legs), not a reuse of `render.rs`'s
//! ~20-part gameplay rig — that rig's helpers are private and built for a
//! walk/jump physics context this static panel doesn't need. V1 pose is
//! static; `.focused` (lift + scale + glow) is the one real animation.

use crate::menu::background::{mesh_child, pct, polygon_mesh};
use crate::menu::theme;
use bevy::prelude::*;

/// Which heroine the Bond panel currently has focused — drives the
/// mock-up's `.focused` lift/scale/glow here, and (via
/// `background::animate_birds`) the background birds' agitated/settled
/// mood, since Anya's gift raises taxis and Donna's lowers it.
#[derive(Resource, Clone, Copy, PartialEq, Eq, Default)]
pub enum BondFocus {
    #[default]
    Anya,
    Donna,
}

/// Everything `spawn` creates — despawned wholesale on `OnExit(MenuShell)`.
#[derive(Component)]
pub struct HeroineRoot;

#[derive(Component)]
pub(super) struct HeroineFigure {
    who: BondFocus,
    /// Feet-anchor position `spawn` placed this figure at — `apply_focus`
    /// recomputes the lift from this fixed base every time, rather than
    /// accumulating a delta onto a moving `Transform`.
    base: Vec2,
}

#[derive(Component)]
pub(super) struct FocusGlow(BondFocus);

pub fn absent(q: Query<(), With<HeroineRoot>>) -> bool {
    q.is_empty()
}

const BODY_POINTS: &[(f32, f32)] = &[(25.0, 0.0), (75.0, 0.0), (100.0, 100.0), (0.0, 100.0)];

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((HeroineRoot, Transform::default(), Visibility::default()))
        .with_children(|root| {
            for (who, x_pct, primary) in [
                (BondFocus::Anya, 35.0, theme::anya()),
                (BondFocus::Donna, 44.0, theme::donna()),
            ] {
                let base = pct(x_pct, 82.0); // bottom:18% -> near-feet anchor

                // soft focus glow, a sibling (not scaled with the lift) —
                // alpha driven by `apply_focus`.
                root.spawn((
                    Mesh2d(meshes.add(Circle::new(70.0).mesh().build())),
                    MeshMaterial2d(materials.add(theme::gold().with_alpha(0.0))),
                    Transform::from_translation(base.extend(-1.0)),
                    FocusGlow(who),
                ));

                root.spawn((
                    HeroineFigure { who, base },
                    Transform::from_translation(base.extend(1.0)),
                    Visibility::default(),
                ))
                .with_children(|fig| {
                    // legs
                    for side in [-1.0f32, 1.0] {
                        fig.spawn((
                            Sprite {
                                color: Color::srgb_u8(0x2a, 0x24, 0x32),
                                custom_size: Some(Vec2::new(14.0, 53.0)),
                                ..default()
                            },
                            Transform::from_translation(Vec2::new(side * 7.0, 27.0).extend(0.1)),
                        ));
                    }
                    // body: trapezoid, narrower at the shoulders.
                    let body_mesh = mesh_child(
                        fig,
                        &mut meshes,
                        &mut materials,
                        polygon_mesh(BODY_POINTS, Vec2::new(47.0, 114.0)),
                        primary,
                        Vec2::new(0.0, 110.0),
                        0.2,
                    );
                    let _ = body_mesh;
                    // hair (behind the head)
                    fig.spawn((
                        Sprite {
                            color: Color::srgb_u8(0x17, 0x14, 0x1c),
                            custom_size: Some(Vec2::new(44.0, 60.0)),
                            ..default()
                        },
                        Transform::from_translation(Vec2::new(0.0, 175.0).extend(0.15)),
                    ));
                    // head
                    mesh_child(
                        fig,
                        &mut meshes,
                        &mut materials,
                        Circle::new(15.5).mesh().build(),
                        Color::srgb_u8(0xc9, 0x95, 0x77),
                        Vec2::new(0.0, 187.0),
                        0.3,
                    );
                });
            }
        });
}

pub fn despawn(mut commands: Commands, q: Query<Entity, With<HeroineRoot>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

/// Drives the mock-up's `.focused` state: the current [`BondFocus`] lifts +
/// slightly scales that heroine, and brightens her glow.
pub fn apply_focus(
    focus: Res<BondFocus>,
    mut figures: Query<(&HeroineFigure, &mut Transform)>,
    glows: Query<(&FocusGlow, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (h, mut tf) in &mut figures {
        let on = h.who == *focus;
        tf.translation = (h.base + Vec2::new(0.0, if on { 7.0 } else { 0.0 })).extend(1.0);
        tf.scale = Vec3::splat(if on { 1.05 } else { 1.0 });
    }
    for (glow, handle) in &glows {
        if let Some(mat) = materials.get_mut(&handle.0) {
            let alpha = if glow.0 == *focus { 0.16 } else { 0.0 };
            mat.color = theme::gold().with_alpha(alpha);
        }
    }
}
