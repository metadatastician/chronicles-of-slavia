// SPDX-License-Identifier: MPL-2.0
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

use crate::session::{BirdState, Crossing, Session, Settle};
use bevy::prelude::*;

const GROUND_Y: f32 = -140.0; // feet level
const GROVE_X: f32 = -300.0;
const BRIDGE_X: f32 = 0.0; // centre of the gorge
const ENTRANCE_X: f32 = -520.0;
const GORGE_HALF: f32 = 100.0; // water spans BRIDGE_X ± GORGE_HALF
const WADE: f32 = 40.0; // wadeable band width from each bank; beyond = abyss
const GIRL_H: f32 = 54.0;
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

#[derive(Component)]
struct BirdSprite;
#[derive(Component)]
struct RiftSprite;
#[derive(Component)]
struct PipSprite(usize);
/// The trailing hair/ribbon accent behind a girl — owns her id so it can
/// track her position each frame without a parent/child transform.
#[derive(Component)]
struct HairSprite(&'static str);

pub fn run() {
    print_controls();
    App::new()
        .insert_resource(ClearColor(sky(false)))
        .insert_resource(Game {
            session: Session::new(),
            last_beat_count: 0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chronicles of Slavia — Zone A (Bevy · M2.1)".into(),
                resolution: (1180.0, 640.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (input, physics, reactive_rig, sync, announce_progress, quit).chain(),
        )
        .run();
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

fn setup(mut commands: Commands, game: Res<Game>) {
    commands.spawn(Camera2d);

    // lands: forest (left), mountains (right)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.15, 0.34, 0.18),
            custom_size: Some(Vec2::new(560.0, 260.0)),
            ..default()
        },
        Transform::from_xyz(-360.0, GROUND_Y - 40.0, -20.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.30, 0.30, 0.37),
            custom_size: Some(Vec2::new(620.0, 260.0)),
            ..default()
        },
        Transform::from_xyz(360.0, GROUND_Y - 40.0, -20.0),
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
        ));
    }

    // girls — blocks, but M2.2 gives them the verve-scaled reactive rig
    // (lean/bounce/breathe/billow/hair-trail) from `15-character-visual-
    // design.md`, ported from the JS prototype's procedural rig.
    for c in game.session.characters().iter() {
        let id: &'static str = if c.id == "anya" { "anya" } else { "donna" };
        let x = ENTRANCE_X + if id == "anya" { 22.0 } else { -14.0 };
        let y = GROUND_Y + GIRL_H / 2.0;
        let verve = if id == "anya" { 1.0 } else { 0.42 };
        commands.spawn((
            Sprite {
                color: girl_color(id, id == "anya"),
                custom_size: Some(Vec2::new(26.0, GIRL_H)),
                ..default()
            },
            Transform::from_xyz(x, y, 1.0),
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
        ));
        commands.spawn((
            Sprite {
                color: hair_color(id),
                custom_size: Some(Vec2::new(8.0, 20.0)),
                ..default()
            },
            Transform::from_xyz(x, y + GIRL_H * 0.3, 0.9),
            HairSprite(id),
        ));
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
    ));
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
#[allow(clippy::type_complexity)]
fn reactive_rig(
    time: Res<Time>,
    mut girls: Query<(&mut Girl, &mut Transform, &mut Sprite), Without<HairSprite>>,
    mut hair: Query<(&HairSprite, &mut Transform), Without<Girl>>,
) {
    let dt = time.delta_secs();
    for (mut g, mut t, mut sprite) in &mut girls {
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
        let air = !g.on_ground;
        let billow = (if air { vy_norm } else { 0.0 }) * 0.4 * (1.1 - g.verve * 0.4);
        let bw = (1.0 + billow).clamp(0.85, 1.3);

        t.translation.y = g.y + bounce + breathe;
        t.rotation = Quat::from_rotation_z(lean * 0.3);
        t.scale.x = bw;
        sprite.flip_x = g.facing < 0.0;
    }

    for (h, mut t) in &mut hair {
        if let Some((g, gt, _)) = girls.iter().find(|(g, ..)| g.id == h.0) {
            t.translation.x = gt.translation.x + g.hair_x * g.facing;
            t.translation.y = gt.translation.y + GIRL_H * 0.3 - g.hair_y * 8.0;
        }
    }
}

#[allow(clippy::type_complexity)]
fn sync(
    game: Res<Game>,
    mut clear: ResMut<ClearColor>,
    mut q: Query<(
        &mut Sprite,
        Option<&Girl>,
        Option<&BirdSprite>,
        Option<&RiftSprite>,
        Option<&PipSprite>,
    )>,
) {
    clear.0 = sky(game.session.rift_active());
    let active = active_id(&game);
    for (mut s, girl, bird, rift, pip) in &mut q {
        if let Some(g) = girl {
            s.color = girl_color(g.id, g.id == active);
        } else if bird.is_some() {
            s.color = bird_color(game.session.birds);
        } else if rift.is_some() {
            s.color = rift_color(game.session.rift_active());
        } else if let Some(p) = pip {
            s.color = pip_color(pip_on(&game, p.0));
        }
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

fn quit(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
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
fn girl_color(id: &str, active: bool) -> Color {
    let (r, g, b) = if id == "anya" {
        (0.96, 0.55, 0.25)
    } else {
        (0.35, 0.60, 0.90)
    };
    Color::srgba(r, g, b, if active { 1.0 } else { 0.4 })
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
