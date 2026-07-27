// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! M2.1 Bevy renderer — free-roam Zone A over `slavia-core` (the real foundation,
//! ADR-0003). Ported from the design validated in the web sketch: the two lands,
//! the tiered gorge water (wade → abyss), the grove birds answering the girls,
//! the Rift. Bridge, shrine and the character rig come in later increments;
//! girls are still blocks here.
//!
//! All rules-facing state and gating goes through [`crate::session::Session`],
//! never `slavia_core::World` directly — `Session` is the sole bridge between
//! renderer and rules core (see `session.rs`'s own doc comment). This renderer
//! owns only what's genuinely its concern: continuous pixel movement and water
//! physics. Beat position is *derived* from that pixel space each frame
//! (`beat_units_for_x`) and fed back into `Session`, so interaction gating
//! (bird approach, bridge crossing, the Rift) is answered by the rules core,
//! not by ad hoc pixel-proximity checks.
//!
//! Lives behind [`AppState::Playing`] (`crate::state`) — [`ZoneAPlugin`] only
//! runs its systems in that state, so the menu (`crate::menu`) can sit in
//! front of it in the same `App`/binary without the two ever colliding. The
//! single `Camera2d` is spawned once, for the app's whole lifetime, in
//! `main.rs` — not here — since both the menu and Zone A render through it.

use crate::session::{BirdState, Crossing, Session, Settle};
use crate::state::AppState;
use bevy::prelude::*;

const GROUND_Y: f32 = -140.0; // feet level
const GROVE_X: f32 = -300.0;
const BRIDGE_X: f32 = 0.0; // centre of the gorge
const ENTRANCE_X: f32 = -520.0;
const GORGE_HALF: f32 = 100.0; // water spans BRIDGE_X ± GORGE_HALF
const WADE: f32 = 40.0; // wadeable band width from each bank; beyond = abyss
/// Was 54.0 for the single flat-block sprite. M2.3's paper-doll rig (below)
/// needs a real pixel budget to read as a figure rather than noise — bumped
/// to roughly the JS prototype's ~92px scale (`15-character-visual-design.md`).
/// Collision/water math below is all parametric on this constant, so the
/// bump is visual-only; it does not change jump feel (that's `JUMP_V_*`,
/// unrelated) or gameplay geometry.
const GIRL_H: f32 = 88.0;
/// Torso/shoulder width reference, matching the JS prototype's own w:h
/// ratio (36:94, `prototype/zone-a/index.html`'s `girl()`).
const GIRL_W: f32 = GIRL_H * 0.383;
const MOVE_SPEED: f32 = 240.0;
const GRAVITY: f32 = 900.0;
/// Jump takeoff speed, verve-scaled (`15-character-visual-design.md`'s
/// reactive-rig table: "Jump height | scaled by verve | springs high | low,
/// heavy"). Was a flat 380.0 for both girls; this restores the spread the
/// design doc already calls for.
const JUMP_V_BASE: f32 = 300.0;
const JUMP_V_VERVE_SCALE: f32 = 110.0;
fn jump_speed(verve: f32) -> f32 {
    JUMP_V_BASE + verve * JUMP_V_VERVE_SCALE
}
/// Reference jump speed, used only to normalize `vy` into a roughly ±1 range
/// for the reactive rig below — not a real per-girl jump height.
const JUMP_V_REF: f32 = 380.0;

// Zone A's later beats (shrine onward) have no dedicated visuals yet in
// M2.1 — these landmarks only need to be monotonic and roughly plausible.
// They exist so the renderer can derive `Session`'s beat-space position
// from the continuous pixel space it actually simulates.
const SHRINE_X: f32 = -150.0;
const RIDGE_X: f32 = 150.0;
const OVERLOOK_X: f32 = 350.0;
const FRACTURE_X: f32 = 550.0;

/// Pixel X for each of Zone A's seven beats, in beat order (`docs/design/02`).
const BEAT_X: [f32; 7] = [
    ENTRANCE_X, GROVE_X, SHRINE_X, BRIDGE_X, RIDGE_X, OVERLOOK_X, FRACTURE_X,
];

/// The beat-unit position (possibly fractional, between two beats) for a
/// pixel X, via piecewise-linear interpolation across `BEAT_X` (monotonic
/// by construction).
fn beat_units_for_x(x: f32) -> f32 {
    let x = x.clamp(BEAT_X[0], BEAT_X[BEAT_X.len() - 1]);
    for i in 0..BEAT_X.len() - 1 {
        let (x0, x1) = (BEAT_X[i], BEAT_X[i + 1]);
        if x <= x1 {
            let t = if x1 > x0 { (x - x0) / (x1 - x0) } else { 0.0 };
            return i as f32 + t;
        }
    }
    (BEAT_X.len() - 1) as f32
}

#[derive(Resource)]
struct Game {
    session: Session,
    /// The last-announced count of Zone A's five understanding-beats
    /// (`Session::beats`), so progress prints once per change, not per frame.
    last_beat_count: usize,
}

#[derive(Component)]
struct Girl {
    id: &'static str,
    x: f32,
    y: f32,
    vy: f32,
    facing: f32,
    on_ground: bool,
    depth: f32,
    /// How energetically the body reacts (`15-character-visual-design.md`):
    /// Anya 1.0, Donna 0.42. Everything below this field is reactive-rig
    /// state (M2.2), ported from the JS prototype's procedural rig
    /// (`prototype/zone-a/index.html`) — ported for *shape*, not exact
    /// numeric parity: the prototype's per-frame canvas units don't map 1:1
    /// onto Bevy's dt-scaled physics, so constants below are re-tuned to
    /// read similarly at this engine's scale, not copied verbatim.
    verve: f32,
    /// `x` last frame, for deriving an eased animation velocity — the
    /// prototype tracks `vx` directly from input; here it's derived from
    /// position so this system doesn't need to duplicate `physics`' input
    /// handling.
    prev_x: f32,
    /// Eased horizontal velocity, animation-only (movement itself is not
    /// affected — this never touches `x`).
    vx: f32,
    /// Walk-cycle phase; advances only while moving.
    walk: f32,
    /// Idle/breathing phase; advances always.
    idle: f32,
    /// Eased 0..1 "is walking" blend, so gait fades in/out instead of
    /// snapping.
    gait: f32,
    /// Trailing hair/ribbon offset (`15`'s "Hair / ribbons" row).
    hair_x: f32,
    hair_y: f32,
}

/// Marks every entity `setup` spawns (except the persistent camera, owned by
/// `main.rs`), so `teardown` can despawn exactly the Zone A scene and nothing
/// else when leaving [`AppState::Playing`].
#[derive(Component)]
struct ZoneAEntity;

#[derive(Component)]
struct BirdSprite;
#[derive(Component)]
struct RiftSprite;
#[derive(Component)]
struct PipSprite(usize);

/// One piece of the paper-doll rig (M2.3): the JS prototype draws each girl
/// as ~20 layered vector-path calls per frame (`prototype/zone-a/index.html`
/// `drawGirl`) — a real, if stylized, costumed figure. Bevy's `Sprite` only
/// draws axis-aligned rectangles, so that look is rebuilt here as a small
/// rig of rectangle sprites (a "paper doll") rather than one flat block —
/// enough to read as a figure in costume, not a 1:1 vector-path port.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Part {
    Head,
    Torso,
    Vest,
    SkirtUpper,
    SkirtLower,
    Apron, // Donna only — Rodopska woven apron over the sukman
    Belt,
    ArmL,
    ArmR,
    HandL,
    HandR,
    BootL,
    BootR,
}

/// A body-part sprite, spawned as a Bevy child of its girl's [`Girl`] entity
/// so the parent's lean/bounce/facing-flip transform composes onto every
/// part automatically (`bevy_transform` propagation) — only each part's own
/// local animation (stride, sway, arm swing, skirt billow) is set by hand.
#[derive(Component)]
struct BodyPart(&'static str, Part);

/// The trailing hair/ribbon accent behind a girl — a child, like [`BodyPart`],
/// but kept as its own component since its trail/lift motion (`hair_x`/
/// `hair_y`) is unlike any other part's animation.
#[derive(Component)]
struct HairSprite(&'static str);

/// Real Zone A gameplay, gated to [`AppState::Playing`]. `main.rs` owns the
/// `App`, the window, and the one persistent `Camera2d`; this plugin owns
/// only the scene and its systems.
pub struct ZoneAPlugin;

impl Plugin for ZoneAPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), (print_controls, setup))
            .add_systems(OnExit(AppState::Playing), teardown)
            .add_systems(
                Update,
                (input, physics, reactive_rig, sync, announce_progress, quit)
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

fn print_controls() {
    println!(
        "\nChronicles of Slavia — Zone A (Bevy · M2.1)\n\
         -------------------------------------------\n\
         A/D or <-> walk    Space jump    Tab switch girl\n\
         E  reach the grove birds (near the grove)\n\
         F  steady the crossing (Donna, at the gorge) / cross it (once steady)\n\
         R  awaken the Rift        Esc quit\n\
         Wade into the gorge water — the black middle is bottomless until\n\
         Donna steadies it. Stir the birds as Anya, settle as Donna; wake\n\
         the Rift and they're disrupted.\n"
    );
}

fn setup(mut commands: Commands) {
    let session = Session::new();

    // lands: forest (left), mountains (right)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.15, 0.34, 0.18),
            custom_size: Some(Vec2::new(560.0, 260.0)),
            ..default()
        },
        Transform::from_xyz(-360.0, GROUND_Y - 40.0, -20.0),
        ZoneAEntity,
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.30, 0.30, 0.37),
            custom_size: Some(Vec2::new(620.0, 260.0)),
            ..default()
        },
        Transform::from_xyz(360.0, GROUND_Y - 40.0, -20.0),
        ZoneAEntity,
    ));

    // ground either side of the gorge
    let left_w = (BRIDGE_X - GORGE_HALF) - (-640.0);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.24, 0.32, 0.20),
            custom_size: Some(Vec2::new(left_w, 120.0)),
            ..default()
        },
        Transform::from_xyz(-640.0 + left_w / 2.0, GROUND_Y - 60.0, -10.0),
        ZoneAEntity,
    ));
    let right_w = 640.0 - (BRIDGE_X + GORGE_HALF);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.24, 0.30, 0.22),
            custom_size: Some(Vec2::new(right_w, 120.0)),
            ..default()
        },
        Transform::from_xyz(
            (BRIDGE_X + GORGE_HALF) + right_w / 2.0,
            GROUND_Y - 60.0,
            -10.0,
        ),
        ZoneAEntity,
    ));

    // water tiers: light wade bands at the banks, dark abyss in the middle
    for side in [-1.0f32, 1.0] {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.30, 0.58, 0.70),
                custom_size: Some(Vec2::new(WADE, 180.0)),
                ..default()
            },
            Transform::from_xyz(
                BRIDGE_X + side * (GORGE_HALF - WADE / 2.0),
                GROUND_Y - 90.0,
                -12.0,
            ),
            ZoneAEntity,
        ));
    }
    let abyss_w = (GORGE_HALF - WADE) * 2.0;
    commands.spawn((
        Sprite {
            color: Color::srgb(0.04, 0.11, 0.19),
            custom_size: Some(Vec2::new(abyss_w, 220.0)),
            ..default()
        },
        Transform::from_xyz(BRIDGE_X, GROUND_Y - 110.0, -12.0),
        ZoneAEntity,
    ));

    // grove birds
    for i in 0..5 {
        commands.spawn((
            Sprite {
                color: bird_color(BirdState::Neutral),
                custom_size: Some(Vec2::new(12.0, 12.0)),
                ..default()
            },
            Transform::from_xyz(
                GROVE_X - 30.0 + i as f32 * 15.0,
                GROUND_Y + 70.0 + (i % 2) as f32 * 16.0,
                0.0,
            ),
            BirdSprite,
            ZoneAEntity,
        ));
    }

    // girls — a paper-doll rig (M2.3): a Girl root (feet-anchor, carries the
    // verve-scaled lean/bounce/facing-flip transform) with body-part sprites
    // as Bevy children, so the group transform composes onto every part for
    // free. Colors ported from the JS prototype's own costume palette
    // (`prototype/zone-a/index.html`), not invented fresh.
    for c in session.characters().iter() {
        let id: &'static str = if c.id == "anya" { "anya" } else { "donna" };
        let x = ENTRANCE_X + if id == "anya" { 22.0 } else { -14.0 };
        let y = GROUND_Y + GIRL_H / 2.0; // physics' own center-referenced y
        let verve = if id == "anya" { 1.0 } else { 0.42 };
        let anya = id == "anya";
        commands
            .spawn((
                Transform::from_xyz(x, y - GIRL_H / 2.0, 1.0),
                Visibility::default(),
                ZoneAEntity,
                Girl {
                    id,
                    x,
                    y,
                    vy: 0.0,
                    facing: 1.0,
                    on_ground: true,
                    depth: 0.0,
                    verve,
                    prev_x: x,
                    vx: 0.0,
                    walk: 0.0,
                    idle: 0.0,
                    gait: 0.0,
                    hair_x: 0.0,
                    hair_y: 0.0,
                },
            ))
            .with_children(|p| {
                let hip_w = GIRL_W * 0.34;
                let sh_w = GIRL_W * 0.33;
                let head_r = GIRL_H * 0.10;

                p.spawn((
                    Sprite {
                        color: skin_color(),
                        custom_size: Some(Vec2::new(head_r * 2.0, head_r * 2.0)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, GIRL_H * 0.86, 0.6),
                    BodyPart(id, Part::Head),
                ));
                p.spawn((
                    Sprite {
                        color: blouse_color(anya),
                        custom_size: Some(Vec2::new(sh_w * 1.8, GIRL_H * 0.22)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, GIRL_H * 0.61, 0.4),
                    BodyPart(id, Part::Torso),
                ));
                p.spawn((
                    Sprite {
                        color: vest_color(anya),
                        custom_size: Some(Vec2::new(sh_w * 1.3, GIRL_H * 0.16)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, GIRL_H * 0.64, 0.5),
                    BodyPart(id, Part::Vest),
                ));
                p.spawn((
                    Sprite {
                        color: skirt_color(anya),
                        custom_size: Some(Vec2::new(hip_w * 2.0, GIRL_H * 0.20)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, GIRL_H * 0.40, 0.35),
                    BodyPart(id, Part::SkirtUpper),
                ));
                p.spawn((
                    Sprite {
                        color: skirt_color(anya),
                        custom_size: Some(Vec2::new(GIRL_W * 0.9, GIRL_H * 0.22)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, GIRL_H * 0.19, 0.35),
                    BodyPart(id, Part::SkirtLower),
                ));
                if !anya {
                    p.spawn((
                        Sprite {
                            color: Color::srgb_u8(0x7a, 0x21, 0x30), // RHODOPE_APRON
                            custom_size: Some(Vec2::new(GIRL_W * 0.55, GIRL_H * 0.30)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, GIRL_H * 0.28, 0.45),
                        BodyPart(id, Part::Apron),
                    ));
                }
                p.spawn((
                    Sprite {
                        color: trim_color(anya),
                        custom_size: Some(Vec2::new(hip_w * 2.0, GIRL_H * 0.035)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, GIRL_H * 0.50, 0.55),
                    BodyPart(id, Part::Belt),
                ));
                for (part, side) in [(Part::ArmL, -1.0f32), (Part::ArmR, 1.0)] {
                    p.spawn((
                        Sprite {
                            color: blouse_color(anya),
                            custom_size: Some(Vec2::new(GIRL_W * 0.14, GIRL_H * 0.22)),
                            ..default()
                        },
                        Transform::from_xyz(side * sh_w * 0.95, GIRL_H * 0.58, 0.3),
                        BodyPart(id, part),
                    ));
                }
                for (part, side) in [(Part::HandL, -1.0f32), (Part::HandR, 1.0)] {
                    p.spawn((
                        Sprite {
                            color: skin_color(),
                            custom_size: Some(Vec2::new(GIRL_W * 0.11, GIRL_W * 0.11)),
                            ..default()
                        },
                        Transform::from_xyz(side * sh_w * 0.95, GIRL_H * 0.47, 0.3),
                        BodyPart(id, part),
                    ));
                }
                for (part, side) in [(Part::BootL, -1.0f32), (Part::BootR, 1.0)] {
                    p.spawn((
                        Sprite {
                            color: Color::srgb_u8(0x3a, 0x26, 0x17), // worn boot leather
                            custom_size: Some(Vec2::new(GIRL_W * 0.30, GIRL_H * 0.09)),
                            ..default()
                        },
                        Transform::from_xyz(side * hip_w * 0.5, GIRL_H * 0.045, 0.3),
                        BodyPart(id, part),
                    ));
                }
                p.spawn((
                    Sprite {
                        color: hair_color(id),
                        custom_size: Some(Vec2::new(GIRL_W * 0.28, GIRL_H * 0.32)),
                        ..default()
                    },
                    Transform::from_xyz(0.0, GIRL_H * 0.78, 0.65),
                    HairSprite(id),
                ));
            });
    }

    // beat pips (stir / settle / rift)
    for i in 0..3 {
        commands.spawn((
            Sprite {
                color: pip_color(false),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Transform::from_xyz(-26.0 + i as f32 * 26.0, 270.0, 2.0),
            PipSprite(i),
            ZoneAEntity,
        ));
    }

    // rift seam (hidden until awoken)
    commands.spawn((
        Sprite {
            color: rift_color(false),
            custom_size: Some(Vec2::new(10.0, 440.0)),
            ..default()
        },
        Transform::from_xyz(BRIDGE_X, 40.0, 5.0),
        RiftSprite,
        ZoneAEntity,
    ));

    commands.insert_resource(Game {
        session,
        last_beat_count: 0,
    });
}

/// Leaving [`AppState::Playing`]: despawn the whole Zone A scene and drop its
/// [`Game`] resource, so re-entering later starts a genuinely fresh
/// [`Session`] rather than resuming stale state.
fn teardown(mut commands: Commands, q: Query<Entity, With<ZoneAEntity>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
    commands.remove_resource::<Game>();
}

fn active_id(game: &Game) -> String {
    game.session.active_id().to_string()
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    if keys.just_pressed(KeyCode::Tab) {
        game.session.toggle_character();
        println!("-> now playing {}", game.session.active_name());
    }
    if keys.just_pressed(KeyCode::KeyE) {
        match game.session.approach_birds() {
            Some(r) => println!(
                "{} reaches out to the birds -> {r:?}",
                game.session.active_name()
            ),
            None => println!("(no birds here — go to the grove)"),
        }
    }
    if keys.just_pressed(KeyCode::KeyF) {
        match game.session.settle_crossing() {
            Settle::Settled => println!("Donna steadies the crossing. It's passable now."),
            Settle::WrongGift => match game.session.cross() {
                Crossing::Crossed => println!("{} crosses.", game.session.active_name()),
                Crossing::Unpassable => println!("(the crossing is still unsteady)"),
                Crossing::NotHere => println!("(nothing to do here)"),
            },
            Settle::NotHere => println!("(nothing to do here)"),
        }
    }
    if keys.just_pressed(KeyCode::KeyR) {
        game.session.awaken_rift();
        game.session.birds = BirdState::Disrupted;
        println!("The Rift awakens. The birds stop answering.");
    }
}

fn physics(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut girls: Query<(&mut Girl, &mut Transform)>,
) {
    let active = active_id(&game);
    let bridge = game.session.crossing_passable();
    let dt = time.delta_secs();
    for (mut g, mut t) in &mut girls {
        if g.id == active {
            let mut dx: f32 = 0.0;
            if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
                dx -= 1.0;
            }
            if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
                dx += 1.0;
            }
            if dx != 0.0 {
                g.facing = dx.signum();
            }
            let d0 = water_at(g.x, bridge).0;
            g.x += dx * MOVE_SPEED * (1.0 - d0 * 0.6) * dt;

            // recompute after moving; clamp out of the abyss
            let (depth, clamp) = water_at(g.x, bridge);
            if let Some(cx) = clamp {
                g.x = cx;
            }
            g.depth = water_at(g.x, bridge).0.max(depth);
            g.x = g.x.clamp(-620.0, 620.0);
            game.session.set_active_pos(beat_units_for_x(g.x));

            let floor = GROUND_Y + GIRL_H / 2.0;
            if g.depth > 0.02 {
                // wade: float at a level set by depth
                let target = floor - g.depth * GIRL_H * 0.7;
                g.y += (target - g.y) * (dt * 7.0).min(1.0);
                g.vy = 0.0;
                g.on_ground = false;
            } else {
                if (keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::ArrowUp))
                    && g.on_ground
                {
                    g.vy = jump_speed(g.verve);
                    g.on_ground = false;
                }
                g.vy -= GRAVITY * dt;
                g.y += g.vy * dt;
                if g.y <= floor {
                    g.y = floor;
                    g.vy = 0.0;
                    g.on_ground = true;
                }
            }
        }
        t.translation.x = g.x;
        t.translation.y = g.y;
    }
}

/// M2.2's verve-scaled reactive rig (`15-character-visual-design.md`): lean
/// into motion, walk-cycle bounce, idle breathing, skirt billow on a fall,
/// and a trailing hair/ribbon accent. Runs after `physics` each frame:
/// `physics` sets the girls' base pose (`translation.x/y` from `g.x/g.y`);
/// this system displaces from that base for render only, so it never
/// affects movement, collision, or the beat position derived from `g.x`.
///
/// Ported for *shape* from the JS prototype's procedural rig, not exact
/// numeric parity — see the `verve` field's doc comment on [`Girl`].
///
/// The `Girl` root gets one group transform (feet-anchor translation, lean
/// rotation, `facing` mirror via `scale.x`) that Bevy's transform hierarchy
/// composes onto every [`BodyPart`]/[`HairSprite`] child automatically —
/// this system only sets each child's own *local* animation (stride, sway,
/// billow, arm swing, hair trail) on top of that shared group pose.
#[allow(clippy::type_complexity)]
fn reactive_rig(
    time: Res<Time>,
    mut girls: Query<(&mut Girl, &mut Transform)>,
    mut parts: Query<(&BodyPart, &mut Transform), Without<Girl>>,
    mut hair: Query<(&HairSprite, &mut Transform), (Without<Girl>, Without<BodyPart>)>,
) {
    let dt = time.delta_secs();
    for (mut g, mut t) in &mut girls {
        // Eased animation velocity, derived from position so this system
        // doesn't need to know which girl is active or duplicate input
        // handling — `physics` already moved `g.x`.
        let raw_vx = if dt > 0.0 { (g.x - g.prev_x) / dt } else { 0.0 };
        g.prev_x = g.x;
        let ease = 0.12 + g.verve * 0.05;
        g.vx += (raw_vx - g.vx) * (dt * 60.0 * ease).min(1.0);
        let vx_norm = (g.vx / MOVE_SPEED).clamp(-1.5, 1.5);
        let vy_norm = (g.vy / JUMP_V_REF).clamp(-1.5, 1.5);

        let moving = g.on_ground && g.vx.abs() > 10.0;
        g.gait += ((moving as u8 as f32) - g.gait) * (dt * 10.0).min(1.0);
        if moving {
            g.walk += dt * (6.0 + vx_norm.abs() * 1.2);
        }
        g.idle += dt * 1.6;

        let flow = 1.2 + g.verve * 1.6;
        let target_hair_x = -vx_norm.abs() * flow;
        g.hair_x += (target_hair_x - g.hair_x) * (dt * 8.0).min(1.0);
        let target_hair_y = (if g.on_ground { 0.0 } else { vy_norm }) * (0.6 + g.verve);
        g.hair_y += (target_hair_y - g.hair_y) * (dt * 8.0).min(1.0);

        let lean = (vx_norm * 0.4 * (0.5 + g.verve)).clamp(-0.3, 0.3);
        let bounce = g.walk.sin().abs() * g.gait * (1.0 + g.verve * 1.3) * 3.0;
        let breathe = g.idle.sin() * 0.6 * (1.0 - g.gait);

        t.translation.x = g.x;
        t.translation.y = g.y - GIRL_H / 2.0 + bounce + breathe;
        // Negated: the JS prototype's `lean` sign assumes canvas's Y-down,
        // clockwise-positive rotation (`ctx.rotate`); Bevy is Y-up,
        // counterclockwise-positive, so the same sign tips the body away
        // from the direction of travel instead of into it.
        t.rotation = Quat::from_rotation_z(-lean * 0.3);
        // Mirrors the whole child rig for free (Bevy composes parent scale
        // into every child's local offset too) — replaces the old
        // `sprite.flip_x`, which only had one flat sprite to flip.
        t.scale.x = g.facing;
    }

    for (part, mut t) in &mut parts {
        let Some((g, ..)) = girls.iter().find(|(g, ..)| g.id == part.0) else {
            continue;
        };
        let vx_norm = (g.vx / MOVE_SPEED).clamp(-1.5, 1.5);
        let vy_norm = (g.vy / JUMP_V_REF).clamp(-1.5, 1.5);
        let air = !g.on_ground;
        let billow = (if air { vy_norm } else { 0.0 }) * 0.4 * (1.1 - g.verve * 0.4);
        let bw = (1.0 + billow).clamp(0.85, 1.3);
        // Sway grows with height off the ground (hem more than waist) —
        // applied per skirt tier below, not as one flat offset.
        //
        // The speed term uses `.abs()`, not signed `vx_norm` — this local
        // offset is authored in "facing-right" space and gets mirrored by
        // the parent's `scale.x = facing` (like the hair trail above). A
        // signed term would get mirrored *twice* (once here, once by
        // `facing`, which share the same sign during sustained movement),
        // landing the hem in front of the girl on one side and correctly
        // behind on the other instead of trailing behind on both.
        let sway = -vx_norm.abs() * GIRL_W * 0.5
            + g.walk.sin() * g.gait * (GIRL_W * 0.18 + g.verve * GIRL_W * 0.25);
        let stride = g.walk.sin() * g.gait;
        let head_bob = g.gait * GIRL_H * 0.02;

        match part.1 {
            Part::Head => t.translation.x = head_bob,
            Part::SkirtUpper => t.translation.x = sway * 0.35,
            Part::SkirtLower | Part::Apron => {
                t.translation.x = sway;
                t.scale.x = bw;
            }
            Part::ArmL | Part::ArmR => {
                let side = if matches!(part.1, Part::ArmL) {
                    -1.0
                } else {
                    1.0
                };
                let swing = if !air && g.gait > 0.08 {
                    (g.walk
                        + if side > 0.0 {
                            std::f32::consts::PI
                        } else {
                            0.0
                        })
                    .sin()
                        * g.gait
                        * (1.0 + g.verve * 1.2)
                } else {
                    0.0
                };
                t.translation.y = GIRL_H * 0.58 - swing * GIRL_H * 0.04;
            }
            Part::HandL | Part::HandR => {
                let side = if matches!(part.1, Part::HandL) {
                    -1.0
                } else {
                    1.0
                };
                let swing = if !air && g.gait > 0.08 {
                    (g.walk
                        + if side > 0.0 {
                            std::f32::consts::PI
                        } else {
                            0.0
                        })
                    .sin()
                        * g.gait
                        * (1.0 + g.verve * 1.2)
                } else {
                    0.0
                };
                t.translation.y = GIRL_H * 0.47 - swing * GIRL_H * 0.04;
            }
            Part::BootL => t.translation.x = -GIRL_W * 0.17 + stride * GIRL_W * 0.22,
            Part::BootR => t.translation.x = GIRL_W * 0.17 - stride * GIRL_W * 0.22,
            Part::Torso | Part::Vest | Part::Belt => {}
        }
    }

    for (h, mut t) in &mut hair {
        if let Some((g, ..)) = girls.iter().find(|(g, ..)| g.id == h.0) {
            t.translation.x = g.hair_x * (GIRL_H * 0.06);
            t.translation.y = GIRL_H * 0.78 - g.hair_y * (GIRL_H * 0.09);
        }
    }
}

#[allow(clippy::type_complexity)]
fn sync(
    game: Res<Game>,
    mut clear: ResMut<ClearColor>,
    mut q: Query<
        (
            &mut Sprite,
            Option<&BirdSprite>,
            Option<&RiftSprite>,
            Option<&PipSprite>,
        ),
        (Without<BodyPart>, Without<HairSprite>),
    >,
    mut parts: Query<(&mut Sprite, &BodyPart), Without<HairSprite>>,
    mut hair: Query<(&mut Sprite, &HairSprite), Without<BodyPart>>,
) {
    clear.0 = sky(game.session.rift_active());
    let active = active_id(&game);
    for (mut s, bird, rift, pip) in &mut q {
        if bird.is_some() {
            s.color = bird_color(game.session.birds);
        } else if rift.is_some() {
            s.color = rift_color(game.session.rift_active());
        } else if let Some(p) = pip {
            s.color = pip_color(pip_on(&game, p.0));
        }
    }
    // The inactive girl's whole rig dims together (was one `girl_color` call
    // on the flat block; now every part/hair sprite keeps its own costume
    // hue and only alpha changes).
    for (mut s, part) in &mut parts {
        s.color = s.color.with_alpha(if part.0 == active { 1.0 } else { 0.4 });
    }
    for (mut s, h) in &mut hair {
        s.color = s.color.with_alpha(if h.0 == active { 1.0 } else { 0.4 });
    }
}

/// Narrate as Zone A's five understanding-beats (`docs/design/00-start-here.md`)
/// come in, once per beat rather than once per frame.
fn announce_progress(mut game: ResMut<Game>) {
    let count = game.session.beats.count();
    if count == game.last_beat_count {
        return;
    }
    game.last_beat_count = count;
    if game.session.beats.all() {
        println!("Two lands, one heart — Zone A's answer is complete.");
    } else if game.session.beats.nature_answered() {
        println!("Nature has answered ({count}/5 understood).");
    } else {
        println!("({count}/5 understood.)");
    }
}

/// Esc returns to the menu (`AppState::MenuShell`), not straight to desktop —
/// matches the mock-up's own "portal" framing and exercises `teardown` on
/// every playthrough, not just at process exit.
fn quit(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(AppState::MenuShell);
    }
}

// --- water: 0 at the banks, wadeable to `WADE` inward, abyss (blocked) beyond ---
fn water_at(x: f32, bridge_stable: bool) -> (f32, Option<f32>) {
    if bridge_stable {
        return (0.0, None);
    }
    let d = x - BRIDGE_X;
    if d.abs() >= GORGE_HALF {
        return (0.0, None); // on land
    }
    let from_bank = GORGE_HALF - d.abs(); // 0 at the bank, grows toward the middle
    if from_bank <= WADE {
        (0.6 * (from_bank / WADE), None) // wade: 0 -> 0.6 (neck)
    } else {
        let sign = if d < 0.0 { -1.0 } else { 1.0 };
        (0.6, Some(BRIDGE_X + sign * (GORGE_HALF - WADE))) // abyss — hold at the edge
    }
}

fn pip_on(game: &Game, i: usize) -> bool {
    match i {
        0 => game.session.beats.anya_stirred,
        1 => game.session.beats.donna_settled,
        2 => game.session.rift_active(),
        _ => false,
    }
}

fn sky(rift: bool) -> Color {
    if rift {
        Color::srgb(0.05, 0.04, 0.09)
    } else {
        Color::srgb(0.10, 0.11, 0.16)
    }
}
fn bird_color(m: BirdState) -> Color {
    match m {
        BirdState::Neutral => Color::srgb(0.62, 0.62, 0.62),
        BirdState::Stirred => Color::srgb(0.96, 0.72, 0.32),
        BirdState::Settled => Color::srgb(0.50, 0.76, 0.96),
        BirdState::Disrupted => Color::srgb(0.30, 0.30, 0.33),
    }
}
/// Costume palette below is taken directly from the JS prototype's own hex
/// constants (`prototype/zone-a/index.html`), not invented fresh — Anya's
/// Hutsul vyshyvanka vs. Donna's Rodopska nosia (`15-character-visual-
/// design.md`).
fn skin_color() -> Color {
    Color::srgb_u8(0xe9, 0xc4, 0xa0) // SKIN
}
fn blouse_color(anya: bool) -> Color {
    if anya {
        Color::srgb_u8(0xf2, 0xed, 0xe0) // LINEN — sorochka
    } else {
        Color::srgb_u8(0xef, 0xe8, 0xda) // RIZA
    }
}
fn vest_color(anya: bool) -> Color {
    if anya {
        Color::srgb_u8(0x8a, 0x5a, 0x2c) // KEPTAR — embroidered sheepskin vest
    } else {
        Color::srgb_u8(0x22, 0x1d, 0x2c) // RHODOPE_SUKMAN — dark bodice strap
    }
}
fn skirt_color(anya: bool) -> Color {
    if anya {
        Color::srgb_u8(0x5e, 0x1d, 0x2a) // HUTSUL_SKIRT — zapaska
    } else {
        Color::srgb_u8(0x22, 0x1d, 0x2c) // RHODOPE_SUKMAN
    }
}
fn trim_color(anya: bool) -> Color {
    if anya {
        Color::srgb_u8(0xb5, 0x20, 0x2a) // EMRED — woven belt
    } else {
        Color::srgb_u8(0xc9, 0x8a, 0x3a) // RHODOPE_STRIPE
    }
}
/// Hair/ribbon accent colors, taken directly from the JS prototype's own
/// palette (`prototype/zone-a/index.html`: `ANYA_HAIR`/`DONNA_HAIR`) rather
/// than invented fresh.
fn hair_color(id: &str) -> Color {
    if id == "anya" {
        Color::srgb_u8(0x6b, 0x3a, 0x1e) // auburn
    } else {
        Color::srgb_u8(0x2c, 0x26, 0x20) // dark — mostly under Donna's headscarf
    }
}
fn rift_color(active: bool) -> Color {
    if active {
        Color::srgba(0.62, 0.86, 1.0, 0.9)
    } else {
        Color::srgba(0.62, 0.86, 1.0, 0.0)
    }
}
fn pip_color(on: bool) -> Color {
    if on {
        Color::srgb(0.96, 0.85, 0.40)
    } else {
        Color::srgb(0.25, 0.25, 0.30)
    }
}
