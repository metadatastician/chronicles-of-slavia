// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The menu's animated world — moon, ridge, ground, rift, bridge, shrine,
//! birds, mist and fireflies — sitting behind the `bevy_ui` shell (Bevy's UI
//! render phase always draws after 2D world sprites/meshes, so no manual
//! Z-ordering trick is needed for the layering itself).
//!
//! Ported from `docs/design/chronicles-landing-page.html`'s `.world` and its
//! children: irregular CSS `clip-path: polygon(...)` shapes (ridge, ground,
//! path, rift) become hand-built [`Mesh2d`] triangle-fan geometry from the
//! same point lists; everything else is plain sprites/circle meshes.
//! Placements are **hand-approximated** from the mock-up's CSS percentages,
//! not a pixel-exact port — this is decorative, not gameplay geometry.
//! Bridge plank rotations are fixed (hand-authored), not runtime-randomized;
//! firefly positions/phases are deterministically seeded from index, not
//! RNG. Both are explicit v1 simplifications.
//!
//! Shown behind [`crate::state::AppState::Opening`] and `MenuShell`; never
//! during `Playing` — despawned on `OnExit(MenuShell)`, before
//! `crate::render::ZoneAPlugin`'s own scene ever spawns.

use crate::menu::nav::UiSettings;
use crate::menu::{theme, BondFocus};
use bevy::color::Mix;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

/// The reference box the mock-up's CSS percentages are read against —
/// matches `main.rs`'s window resolution.
const WORLD_W: f32 = 1180.0;
const WORLD_H: f32 = 640.0;

/// Converts a CSS-style top-left percentage position (0..100, y down) to a
/// Bevy 2D world position (origin center, y up).
pub(super) fn pct(x_pct: f32, y_pct: f32) -> Vec2 {
    Vec2::new(
        (x_pct / 100.0 - 0.5) * WORLD_W,
        (0.5 - y_pct / 100.0) * WORLD_H,
    )
}

/// Builds triangle-fan [`Mesh2d`] geometry from a CSS `clip-path:
/// polygon(...)` point list (percentages local to `size`, y down) — a
/// centroid fan, which is exact for the convex boxes here (ground, path)
/// and a visually-fine approximation for the star-shaped ones (ridge, rift).
pub(super) fn polygon_mesh(points_pct: &[(f32, f32)], size: Vec2) -> Mesh {
    let local = |x: f32, y: f32| -> [f32; 3] {
        [(x / 100.0 - 0.5) * size.x, (0.5 - y / 100.0) * size.y, 0.0]
    };
    let ring: Vec<[f32; 3]> = points_pct.iter().map(|(x, y)| local(*x, *y)).collect();
    let n = ring.len() as f32;
    let (cx, cy) = ring
        .iter()
        .fold((0.0, 0.0), |(sx, sy), p| (sx + p[0], sy + p[1]));
    let centroid = [cx / n, cy / n, 0.0];

    let mut positions = vec![centroid];
    positions.extend(ring);
    let n = points_pct.len() as u32;
    let mut indices = Vec::with_capacity(n as usize * 3);
    for i in 0..n {
        indices.extend([0, 1 + i, 1 + (i + 1) % n]);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(Indices::U32(indices))
}

/// Spawns one mesh entity as a child of the current builder — a small
/// helper so `spawn` below doesn't repeat the `Mesh2d`/`MeshMaterial2d`
/// bundle shape at every call site.
pub(super) fn mesh_child(
    p: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    shape: Mesh,
    color: Color,
    pos: Vec2,
    z: f32,
) -> Entity {
    p.spawn((
        Mesh2d(meshes.add(shape)),
        MeshMaterial2d(materials.add(color)),
        Transform::from_translation(pos.extend(z)),
    ))
    .id()
}

/// Everything `spawn` creates — despawned wholesale on `OnExit(MenuShell)`.
#[derive(Component)]
pub struct BackgroundRoot;

#[derive(Component)]
pub(super) struct RiftShape;

#[derive(Component)]
pub(super) struct BirdBg(f32);

#[derive(Component)]
pub(super) struct MistLayer(f32);

/// Per-firefly wander parameters, seeded deterministically from spawn
/// index — not RNG, so the layout is stable and reproducible.
#[derive(Component)]
pub(super) struct FireflyPhase {
    seed: f32,
    origin: Vec2,
}

/// `run_if` guard so re-entering `Opening`/`MenuShell` doesn't double-spawn.
pub fn absent(q: Query<(), With<BackgroundRoot>>) -> bool {
    q.is_empty()
}

const RIDGE_POINTS: &[(f32, f32)] = &[
    (0.0, 66.0),
    (10.0, 52.0),
    (19.0, 59.0),
    (29.0, 31.0),
    (40.0, 55.0),
    (50.0, 27.0),
    (61.0, 53.0),
    (72.0, 37.0),
    (82.0, 58.0),
    (91.0, 43.0),
    (100.0, 61.0),
    (100.0, 100.0),
    (0.0, 100.0),
];

const GROUND_POINTS: &[(f32, f32)] = &[
    (0.0, 18.0),
    (12.0, 9.0),
    (26.0, 20.0),
    (39.0, 8.0),
    (52.0, 19.0),
    (67.0, 6.0),
    (81.0, 17.0),
    (100.0, 4.0),
    (100.0, 100.0),
    (0.0, 100.0),
];

const PATH_POINTS: &[(f32, f32)] = &[(42.0, 0.0), (57.0, 0.0), (100.0, 100.0), (0.0, 100.0)];

const RIFT_POINTS: &[(f32, f32)] = &[
    (48.0, 0.0),
    (80.0, 12.0),
    (45.0, 25.0),
    (92.0, 38.0),
    (38.0, 54.0),
    (70.0, 67.0),
    (32.0, 82.0),
    (54.0, 100.0),
    (22.0, 85.0),
    (45.0, 68.0),
    (10.0, 54.0),
    (55.0, 39.0),
    (21.0, 25.0),
];

const SHRINE_ROOF_POINTS: &[(f32, f32)] = &[(50.0, 0.0), (100.0, 100.0), (0.0, 100.0)];

/// Tree canopy blobs: one main + two smaller echoes, offsets in tree-local
/// pixels (ported from the CSS's own `box-shadow` echo offsets).
const CANOPY: [(f32, f32, [u8; 3]); 3] = [
    (0.0, 0.0, [0x29, 0x31, 0x26]),
    (-24.0, 24.0, [0x22, 0x2a, 0x21]),
    (30.0, 18.0, [0x30, 0x36, 0x2a]),
];

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((BackgroundRoot, Transform::default(), Visibility::default()))
        .with_children(|root| {
            // sky-glow: a soft rose haze, flat-rect approximation of the
            // CSS radial-gradient (no post-process pipeline here).
            root.spawn((
                Sprite {
                    color: Color::srgba(220.0 / 255.0, 169.0 / 255.0, 176.0 / 255.0, 0.10),
                    custom_size: Some(Vec2::new(WORLD_W, WORLD_H)),
                    ..default()
                },
                Transform::from_translation(Vec2::ZERO.extend(-50.0)),
            ));

            // moon: base disc + an offset dark disc, giving the CSS's crescent.
            let moon_pos = pct(82.0, 15.0);
            mesh_child(
                root,
                &mut meshes,
                &mut materials,
                Circle::new(43.0).mesh().build(),
                Color::srgb_u8(0xe8, 0xd9, 0xc2),
                moon_pos,
                -45.0,
            );
            mesh_child(
                root,
                &mut meshes,
                &mut materials,
                Circle::new(40.0).mesh().build(),
                Color::srgb_u8(0x25, 0x1b, 0x32),
                moon_pos + Vec2::new(12.0, -4.0),
                -44.0,
            );

            // ridge (back, dimmer) then ridge (front).
            mesh_child(
                root,
                &mut meshes,
                &mut materials,
                polygon_mesh(RIDGE_POINTS, Vec2::new(WORLD_W * 1.1, WORLD_H * 0.35)),
                Color::srgba_u8(0x46, 0x38, 0x4f, 166),
                pct(50.0, 50.5),
                -40.0,
            );
            mesh_child(
                root,
                &mut meshes,
                &mut materials,
                polygon_mesh(RIDGE_POINTS, Vec2::new(WORLD_W * 1.1, WORLD_H * 0.41)),
                Color::srgb_u8(0x2d, 0x27, 0x38),
                pct(50.0, 56.5),
                -39.0,
            );

            // ground and path.
            mesh_child(
                root,
                &mut meshes,
                &mut materials,
                polygon_mesh(GROUND_POINTS, Vec2::new(WORLD_W, WORLD_H * 0.37)),
                Color::srgb_u8(0x4d, 0x3b, 0x35),
                pct(50.0, 81.5),
                -30.0,
            );
            mesh_child(
                root,
                &mut meshes,
                &mut materials,
                polygon_mesh(PATH_POINTS, Vec2::new(WORLD_W * 0.38, WORLD_H * 0.44)),
                Color::srgba_u8(0xab, 0x81, 0x59, 190),
                pct(56.0, 82.0),
                -29.0,
            );

            // trees: trunk rect + three overlapping canopy circles per tree.
            for (x_pct, scale) in [(8.0, 1.25f32), (19.0, 0.86), (94.0, 1.18), (76.0, 0.76)] {
                let base = pct(x_pct, 76.0);
                root.spawn((
                    Sprite {
                        color: Color::srgb_u8(0x17, 0x15, 0x1a),
                        custom_size: Some(Vec2::new(14.0 * scale, 130.0 * scale)),
                        ..default()
                    },
                    Transform::from_translation(
                        (base + Vec2::new(0.0, 65.0 * scale)).extend(-25.0),
                    ),
                ));
                for (dx, dy, [r, g, b]) in CANOPY {
                    mesh_child(
                        root,
                        &mut meshes,
                        &mut materials,
                        Circle::new(41.0 * scale).mesh().build(),
                        Color::srgb_u8(r, g, b),
                        base + Vec2::new(dx * scale, (130.0 + dy) * scale),
                        -24.0,
                    );
                }
            }

            // rift: always faintly present (real gameplay's Rift is
            // separately hidden/awoken by `crate::render` — this is the
            // menu's own ambient seam, matching the mock-up's framing).
            // `RiftShape` sits on the mesh entity itself so `animate_rift`
            // scales the thing actually on screen, not a decoupled sibling.
            root.spawn((
                Mesh2d(meshes.add(polygon_mesh(RIFT_POINTS, Vec2::new(13.0, 275.0)))),
                MeshMaterial2d(materials.add(theme::rift())),
                Transform::from_translation(pct(75.4, 45.5).extend(-3.0)),
                RiftShape,
            ));

            // bridge: 10 rectangle planks at fixed, hand-authored angles
            // (not runtime-randomized).
            let bridge_center = pct(50.0, 71.3);
            const PLANK_ROT: [f32; 10] = [
                -0.10, 0.06, -0.03, 0.09, -0.07, 0.04, -0.05, 0.08, -0.02, 0.05,
            ];
            for (i, rot) in PLANK_ROT.iter().enumerate() {
                let x = -105.0 + i as f32 * 23.0;
                root.spawn((
                    Sprite {
                        color: Color::srgb_u8(0x8a, 0x65, 0x50),
                        custom_size: Some(Vec2::new(23.0, 10.0)),
                        ..default()
                    },
                    Transform::from_translation((bridge_center + Vec2::new(x, 0.0)).extend(-8.0))
                        .with_rotation(Quat::from_rotation_z(*rot)),
                ));
            }

            // shrine: body + triangular roof + a small gold icon.
            let shrine_base = pct(37.1, 65.5);
            root.spawn((
                Sprite {
                    color: Color::srgb_u8(0x8d, 0x72, 0x5e),
                    custom_size: Some(Vec2::new(31.0, 54.0)),
                    ..default()
                },
                Transform::from_translation(shrine_base.extend(-7.0)),
            ));
            mesh_child(
                root,
                &mut meshes,
                &mut materials,
                polygon_mesh(SHRINE_ROOF_POINTS, Vec2::new(46.0, 22.0)),
                Color::srgb_u8(0x51, 0x36, 0x3b),
                shrine_base + Vec2::new(0.0, 38.0),
                -6.0,
            );
            root.spawn((
                Sprite {
                    color: theme::gold(),
                    custom_size: Some(Vec2::new(10.0, 16.0)),
                    ..default()
                },
                Transform::from_translation((shrine_base + Vec2::new(0.0, 10.0)).extend(-5.0)),
            ));

            // birds: 5 small pairs, sine-bobbing (`animate_birds`).
            let birds_centre = pct(47.4, 42.25);
            for i in 0..5 {
                let offset = Vec2::new(-60.0 + i as f32 * 30.0, (i % 2) as f32 * 16.0 - 8.0);
                root.spawn((
                    Sprite {
                        color: Color::srgb(0.82, 0.79, 0.75),
                        custom_size: Some(Vec2::new(18.0, 8.0)),
                        ..default()
                    },
                    Transform::from_translation((birds_centre + offset).extend(-2.0)),
                    BirdBg(i as f32),
                ));
            }

            // mist: two low-alpha bands sliding slowly (`animate_mist`), no
            // real Gaussian blur (no post-process pipeline here).
            for (i, y_off) in [(0.0, 0.0), (1.0, 14.0)] {
                root.spawn((
                    Sprite {
                        color: Color::srgba(0.82, 0.76, 0.80, 0.08),
                        custom_size: Some(Vec2::new(WORLD_W * 1.2, WORLD_H * 0.18)),
                        ..default()
                    },
                    Transform::from_translation(
                        (pct(50.0, 75.5) + Vec2::new(0.0, y_off)).extend(-6.0),
                    ),
                    MistLayer(i),
                ));
            }

            // fireflies: 16 tiny discs, per-index deterministic wander
            // (`animate_fireflies`) — trimmed down from the mock-up's 24.
            for i in 0..16u32 {
                let seed = i as f32 * 0.618_034; // golden-ratio spacing, not RNG
                let x_pct = 12.0 + seed.fract() * 76.0;
                let y_pct = 20.0 + (seed * 1.7).fract() * 55.0;
                let origin = pct(x_pct, y_pct);
                root.spawn((
                    Mesh2d(meshes.add(Circle::new(3.0).mesh().build())),
                    MeshMaterial2d(materials.add(Color::srgb_u8(0xf6, 0xd8, 0x8e))),
                    Transform::from_translation(origin.extend(-1.0)),
                    FireflyPhase { seed, origin },
                ));
            }
        });
}

pub fn despawn(mut commands: Commands, q: Query<Entity, With<BackgroundRoot>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

/// Motion-scale factor honoring `UiSettings::reduced_motion` — its first
/// real consumer.
fn motion_scale(settings: &UiSettings) -> f32 {
    if settings.reduced_motion {
        0.25
    } else {
        1.0
    }
}

pub fn animate_rift(
    time: Res<Time>,
    settings: Res<UiSettings>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut q: Query<(&mut Transform, &MeshMaterial2d<ColorMaterial>), With<RiftShape>>,
) {
    let t = time.elapsed_secs() * motion_scale(&settings);
    // `riftPulse`: scaleX .85->1.25, opacity .65->1 — ported here as a
    // scale wobble plus a colour blend from `rift()` to the hotter
    // `rift_hot()` at the peak, rather than opacity alone.
    let phase = (t / 3.2 * std::f32::consts::TAU).sin() * 0.5 + 0.5;
    for (mut tf, handle) in &mut q {
        tf.scale.x = 0.85 + phase * 0.4;
        if let Some(mat) = materials.get_mut(&handle.0) {
            mat.color = theme::rift().mix(&theme::rift_hot(), phase);
        }
    }
}

pub fn animate_birds(
    time: Res<Time>,
    settings: Res<UiSettings>,
    focus: Res<BondFocus>,
    mut q: Query<(&BirdBg, &mut Transform)>,
) {
    let scale = motion_scale(&settings);
    let dur = match *focus {
        BondFocus::Anya => 0.75, // agitated, matches Anya's stirring gift
        BondFocus::Donna => 6.0, // settled
    };
    let t = time.elapsed_secs() * scale;
    for (bird, mut tf) in &mut q {
        let phase = (t / dur + bird.0 * 0.4) * std::f32::consts::TAU;
        tf.translation.y = phase.sin() * 6.0;
    }
}

pub fn animate_mist(
    time: Res<Time>,
    settings: Res<UiSettings>,
    mut q: Query<(&MistLayer, &mut Transform)>,
) {
    let t = time.elapsed_secs() * motion_scale(&settings);
    for (layer, mut tf) in &mut q {
        let phase = (t / 11.0 + layer.0 * 0.3) * std::f32::consts::TAU;
        tf.translation.x = phase.sin() * 30.0;
    }
}

pub fn animate_fireflies(
    time: Res<Time>,
    settings: Res<UiSettings>,
    mut q: Query<(&FireflyPhase, &mut Transform)>,
) {
    let t = time.elapsed_secs() * motion_scale(&settings);
    for (fp, mut tf) in &mut q {
        let dx = (t / 7.0 + fp.seed * 6.0).sin() * 26.0;
        let dy = (t / 5.3 + fp.seed * 4.0).cos() * 30.0;
        tf.translation = (fp.origin + Vec2::new(dx, dy)).extend(0.0);
    }
}
