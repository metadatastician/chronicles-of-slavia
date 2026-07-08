// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! M2.1 Bevy renderer — free-roam Zone A over `slavia-core` (the real foundation,
//! ADR-0003). Ported from the design validated in the web sketch: the two lands,
//! the tiered gorge water (wade → abyss), the grove birds answering the girls,
//! the Rift. Bridge, shrine and the character rig come in later increments;
//! girls are still blocks here. `slavia-core` is the rules brain; movement and
//! water physics live in this renderer.

use bevy::prelude::*;
use slavia_core::{zone_a, Response, World};

const GROUND_Y: f32 = -140.0; // feet level
const GROVE_X: f32 = -300.0;
const BRIDGE_X: f32 = 0.0; // centre of the gorge
const ENTRANCE_X: f32 = -520.0;
const GORGE_HALF: f32 = 100.0; // water spans BRIDGE_X ± GORGE_HALF
const WADE: f32 = 40.0; // wadeable band width from each bank; beyond = abyss
const GIRL_H: f32 = 54.0;
const MOVE_SPEED: f32 = 240.0;
const GRAVITY: f32 = 900.0;
const JUMP_V: f32 = 380.0;

#[derive(Clone, Copy, PartialEq)]
enum BirdMood {
    Neutral,
    Stirred,
    Settled,
    Disrupted,
}

#[derive(Resource)]
struct Game {
    world: World,
    birds: BirdMood,
    seen_stir: bool,
    seen_settle: bool,
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
}

#[derive(Component)]
struct BirdSprite;
#[derive(Component)]
struct RiftSprite;
#[derive(Component)]
struct PipSprite(usize);

pub fn run() {
    print_controls();
    App::new()
        .insert_resource(ClearColor(sky(false)))
        .insert_resource(Game {
            world: World::new(zone_a()),
            birds: BirdMood::Neutral,
            seen_stir: false,
            seen_settle: false,
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
        .add_systems(Update, (input, physics, sync, quit))
        .run();
}

fn print_controls() {
    println!(
        "\nChronicles of Slavia — Zone A (Bevy · M2.1)\n\
         -------------------------------------------\n\
         A/D or <-> walk    Space jump    Tab switch girl\n\
         E  reach the grove birds (near the grove)\n\
         R  awaken the Rift        Esc quit\n\
         Wade into the gorge water — the black middle is bottomless, you can't\n\
         cross it (bridge/shrine come next). Stir the birds as Anya, settle as\n\
         Donna; wake the Rift and they're disrupted.\n"
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
                color: bird_color(BirdMood::Neutral),
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

    // girls (blocks for now — rig in M2.2)
    for c in game.world.spec().characters.iter() {
        let id: &'static str = if c.id == "anya" { "anya" } else { "donna" };
        let x = ENTRANCE_X + if id == "anya" { 22.0 } else { -14.0 };
        let y = GROUND_Y + GIRL_H / 2.0;
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
            },
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
    game.world.active().id.clone()
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>, girls: Query<&Girl>) {
    if keys.just_pressed(KeyCode::Tab) {
        let cur = active_id(&game);
        let other = game
            .world
            .spec()
            .characters
            .iter()
            .map(|c| c.id.clone())
            .find(|i| *i != cur);
        if let Some(o) = other {
            game.world.switch_to(&o);
        }
        println!("-> now playing {}", game.world.active().name);
    }
    if keys.just_pressed(KeyCode::KeyE) {
        let cur = active_id(&game);
        let ax = girls
            .iter()
            .find(|g| g.id == cur)
            .map(|g| g.x)
            .unwrap_or(0.0);
        if (ax - GROVE_X).abs() < 90.0 {
            if let Some(r) = game.world.approach("grove-birds") {
                game.birds = mood_from(&r);
                match r {
                    Response::Stirred => game.seen_stir = true,
                    Response::Settled => game.seen_settle = true,
                    _ => {}
                }
                println!(
                    "{} reaches out to the birds -> {r:?}",
                    game.world.active().name
                );
            }
        } else {
            println!("(no birds here — go to the grove)");
        }
    }
    if keys.just_pressed(KeyCode::KeyR) {
        game.world.awaken_rift();
        game.birds = BirdMood::Disrupted;
        println!("The Rift awakens. The birds stop answering.");
    }
}

fn physics(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game: Res<Game>,
    mut girls: Query<(&mut Girl, &mut Transform)>,
) {
    let active = active_id(&game);
    let bridge = game.world.bridge_stable;
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
                    g.vy = JUMP_V;
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
    clear.0 = sky(game.world.rift_active);
    let active = active_id(&game);
    for (mut s, girl, bird, rift, pip) in &mut q {
        if let Some(g) = girl {
            s.color = girl_color(g.id, g.id == active);
        } else if bird.is_some() {
            s.color = bird_color(game.birds);
        } else if rift.is_some() {
            s.color = rift_color(game.world.rift_active);
        } else if let Some(p) = pip {
            s.color = pip_color(pip_on(&game, p.0));
        }
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

fn mood_from(r: &Response) -> BirdMood {
    match r {
        Response::Stirred => BirdMood::Stirred,
        Response::Settled => BirdMood::Settled,
        Response::Disrupted => BirdMood::Disrupted,
        _ => BirdMood::Neutral,
    }
}

fn pip_on(game: &Game, i: usize) -> bool {
    match i {
        0 => game.seen_stir,
        1 => game.seen_settle,
        2 => game.world.rift_active,
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
fn bird_color(m: BirdMood) -> Color {
    match m {
        BirdMood::Neutral => Color::srgb(0.62, 0.62, 0.62),
        BirdMood::Stirred => Color::srgb(0.96, 0.72, 0.32),
        BirdMood::Settled => Color::srgb(0.50, 0.76, 0.96),
        BirdMood::Disrupted => Color::srgb(0.30, 0.30, 0.33),
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
