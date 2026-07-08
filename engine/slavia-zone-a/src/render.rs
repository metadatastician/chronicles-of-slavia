// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The Bevy renderer (Layer 3) — a deliberately thin skin over [`Session`].
//!
//! It maps the pure spatial model in `Session` onto the screen: beats laid out
//! along the path, each girl drawn at her own position, interactions gated by
//! location. It holds only *view* state and forwards input; every rule decision
//! goes through `Session` → `slavia_core`. Art is schematic (coloured blocks) —
//! this is the M1 spike to make Zone A *felt*, not to dress it.

use crate::session::{BirdState, Crossing, Session, Settle};
use bevy::prelude::*;

#[derive(Resource)]
struct Game(Session);

#[derive(Component)]
struct GirlSprite {
    id: &'static str,
}

#[derive(Component)]
struct BirdSprite;

#[derive(Component)]
struct CrossingSprite;

#[derive(Component)]
struct RiftSprite;

#[derive(Component)]
struct BeatPip(usize);

#[derive(Component)]
struct BeatMarker(usize);

const PATH_LEFT: f32 = -380.0;
const PATH_RIGHT: f32 = 380.0;
const GIRL_Y: f32 = -40.0;
const WALK_SPEED: f32 = 1.5; // beats per second

fn beat_x(pos: f32, nbeats: usize) -> f32 {
    let span = (nbeats.saturating_sub(1)).max(1) as f32;
    PATH_LEFT + (pos / span) * (PATH_RIGHT - PATH_LEFT)
}

pub fn run() {
    print_controls();
    App::new()
        .insert_resource(ClearColor(sky(false)))
        .insert_resource(Game(Session::new()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chronicles of Slavia — Zone A (M1 spike)".into(),
                resolution: (900.0, 500.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (walk, input, sync_view, sync_girls, sync_sky, quit))
        .run();
}

fn print_controls() {
    println!(
        "\nChronicles of Slavia — Zone A (M1 spike)\n\
         ----------------------------------------\n\
         A/D or <-> walk the active girl along the path\n\
         Tab        switch girl (Anya <-> Donna)\n\
         E          reach out to the birds  (at the grove)\n\
         S          settle the crossing     (at the bridge, Donna only)\n\
         C          cross                    (at the bridge, once passable)\n\
         R          awaken the Rift          (the Fracture)\n\
         Esc        quit\n\
         \n\
         Walk Anya to the grove and press E (stir); switch to Donna, walk her\n\
         to the grove, E (settle); take Donna to the bridge, S; switch to Anya,\n\
         C to cross; then R and E at the grove (disrupted). Pips fill up top.\n"
    );
}

fn setup(mut commands: Commands, game: Res<Game>) {
    let s = &game.0;
    let n = s.beats_slice().len();
    commands.spawn(Camera2d);

    // The two lands.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.15, 0.34, 0.18),
            custom_size: Some(Vec2::new(380.0, 220.0)),
            ..default()
        },
        Transform::from_xyz(-230.0, -60.0, -10.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.30, 0.30, 0.37),
            custom_size: Some(Vec2::new(380.0, 220.0)),
            ..default()
        },
        Transform::from_xyz(230.0, -60.0, -10.0),
    ));

    // Beat markers along the path.
    for i in 0..n {
        commands.spawn((
            Sprite {
                color: marker_color(false),
                custom_size: Some(Vec2::new(7.0, 28.0)),
                ..default()
            },
            Transform::from_xyz(beat_x(i as f32, n), GIRL_Y - 48.0, -3.0),
            BeatMarker(i),
        ));
    }

    // The crossing and the Rift seam sit at the bridge beat.
    let bridge_i = s
        .beats_slice()
        .iter()
        .position(|b| b.id == "stream-bridge")
        .unwrap_or(3);
    let bridge_x = beat_x(bridge_i as f32, n);
    commands.spawn((
        Sprite {
            color: crossing_color(false),
            custom_size: Some(Vec2::new(120.0, 14.0)),
            ..default()
        },
        Transform::from_xyz(bridge_x, GIRL_Y - 24.0, -5.0),
        CrossingSprite,
    ));
    commands.spawn((
        Sprite {
            color: rift_color(false),
            custom_size: Some(Vec2::new(12.0, 500.0)),
            ..default()
        },
        Transform::from_xyz(bridge_x, 0.0, 6.0),
        RiftSprite,
    ));

    // The grove birds cluster at the grove beat.
    let grove_i = s
        .beats_slice()
        .iter()
        .position(|b| b.id == "bird-grove")
        .unwrap_or(1);
    let grove_x = beat_x(grove_i as f32, n);
    for j in 0..5 {
        commands.spawn((
            Sprite {
                color: bird_color(BirdState::Neutral),
                custom_size: Some(Vec2::new(11.0, 11.0)),
                ..default()
            },
            Transform::from_xyz(
                grove_x - 32.0 + j as f32 * 16.0,
                40.0 + (j % 2) as f32 * 14.0,
                0.0,
            ),
            BirdSprite,
        ));
    }

    // The girls (positions are driven from the Session each frame).
    let start_x = beat_x(0.0, n);
    commands.spawn((
        Sprite {
            color: girl_color("anya", true),
            custom_size: Some(Vec2::new(24.0, 42.0)),
            ..default()
        },
        Transform::from_xyz(start_x, GIRL_Y, 1.0),
        GirlSprite { id: "anya" },
    ));
    commands.spawn((
        Sprite {
            color: girl_color("donna", false),
            custom_size: Some(Vec2::new(24.0, 42.0)),
            ..default()
        },
        Transform::from_xyz(start_x, GIRL_Y, 0.9),
        GirlSprite { id: "donna" },
    ));

    // Beat pips along the top.
    for i in 0..5 {
        commands.spawn((
            Sprite {
                color: pip_color(false),
                custom_size: Some(Vec2::new(22.0, 22.0)),
                ..default()
            },
            Transform::from_xyz(-52.0 + i as f32 * 26.0, 210.0, 2.0),
            BeatPip(i),
        ));
    }
}

fn walk(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut game: ResMut<Game>) {
    let mut dir = 0.0;
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dir -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dir += 1.0;
    }
    if dir == 0.0 {
        return;
    }
    if let Some(i) = game.0.move_active(dir * WALK_SPEED * time.delta_secs()) {
        let beat = &game.0.beats_slice()[i];
        match &beat.text {
            Some(t) => println!("[{}] {}", beat.title, t),
            None => println!("[{}]", beat.title),
        }
    }
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    let s = &mut game.0;
    if keys.just_pressed(KeyCode::Tab) {
        s.toggle_character();
        println!("-> now playing {}", s.active_name());
    }
    if keys.just_pressed(KeyCode::KeyE) {
        match s.approach_birds() {
            Some(r) => println!("{} reaches out to the birds -> {r:?}", s.active_name()),
            None => println!("(no birds here — reach out at the grove)"),
        }
    }
    if keys.just_pressed(KeyCode::KeyS) {
        match s.settle_crossing() {
            Settle::Settled => println!("Donna settles the crossing."),
            Settle::WrongGift => {
                println!(
                    "{} cannot settle it (she does not lower taxis).",
                    s.active_name()
                )
            }
            Settle::NotHere => println!("(no crossing here — settle it at the bridge)"),
        }
    }
    if keys.just_pressed(KeyCode::KeyC) {
        match s.cross() {
            Crossing::Crossed => println!("{} crosses.", s.active_name()),
            Crossing::Unpassable => println!("The crossing is not steady yet."),
            Crossing::NotHere => println!("(nothing to cross here — go to the bridge)"),
        }
    }
    if keys.just_pressed(KeyCode::KeyR) {
        s.awaken_rift();
        println!("The Rift awakens. The birds stop answering normally.");
    }

    let acted = keys.just_pressed(KeyCode::KeyE)
        || keys.just_pressed(KeyCode::KeyC)
        || keys.just_pressed(KeyCode::KeyR);
    if acted {
        println!("   beats witnessed: {}/5", s.beats.count());
        if s.beats.all() {
            println!("   Zone A complete — all five beats witnessed.");
        }
    }
}

#[allow(clippy::type_complexity)]
fn sync_view(
    game: Res<Game>,
    mut sprites: Query<(
        &mut Sprite,
        Option<&BirdSprite>,
        Option<&CrossingSprite>,
        Option<&RiftSprite>,
        Option<&BeatPip>,
        Option<&BeatMarker>,
    )>,
) {
    let s = &game.0;
    for (mut sprite, bird, crossing, rift, pip, marker) in &mut sprites {
        if bird.is_some() {
            sprite.color = bird_color(s.birds);
        } else if crossing.is_some() {
            sprite.color = crossing_color(s.crossing_passable());
        } else if rift.is_some() {
            sprite.color = rift_color(s.rift_active());
        } else if let Some(pip) = pip {
            sprite.color = pip_color(pip_on(s, pip.0));
        } else if let Some(marker) = marker {
            sprite.color = marker_color(s.revealed.get(marker.0).copied().unwrap_or(false));
        }
    }
}

fn sync_girls(game: Res<Game>, mut girls: Query<(&GirlSprite, &mut Transform, &mut Sprite)>) {
    let s = &game.0;
    let n = s.beats_slice().len();
    for (girl, mut transform, mut sprite) in &mut girls {
        transform.translation.x = beat_x(s.pos_of(girl.id), n);
        sprite.color = girl_color(girl.id, girl.id == s.active_id());
    }
}

fn sync_sky(game: Res<Game>, mut clear: ResMut<ClearColor>) {
    clear.0 = sky(game.0.rift_active());
}

fn quit(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

// --- colour helpers ---------------------------------------------------------

fn sky(rift: bool) -> Color {
    if rift {
        Color::srgb(0.05, 0.04, 0.09)
    } else {
        Color::srgb(0.10, 0.11, 0.16)
    }
}

fn bird_color(state: BirdState) -> Color {
    match state {
        BirdState::Neutral => Color::srgb(0.62, 0.62, 0.62),
        BirdState::Stirred => Color::srgb(0.96, 0.72, 0.32),
        BirdState::Settled => Color::srgb(0.50, 0.76, 0.96),
        BirdState::Disrupted => Color::srgb(0.30, 0.30, 0.33),
    }
}

fn crossing_color(passable: bool) -> Color {
    if passable {
        Color::srgb(0.62, 0.46, 0.30)
    } else {
        Color::srgba(0.40, 0.30, 0.20, 0.5)
    }
}

fn rift_color(active: bool) -> Color {
    if active {
        Color::srgba(0.62, 0.86, 1.0, 0.9)
    } else {
        Color::srgba(0.62, 0.86, 1.0, 0.0)
    }
}

fn girl_color(id: &str, active: bool) -> Color {
    let (r, g, b) = if id == "anya" {
        (0.96, 0.55, 0.25)
    } else {
        (0.35, 0.60, 0.90)
    };
    Color::srgba(r, g, b, if active { 1.0 } else { 0.35 })
}

fn pip_color(on: bool) -> Color {
    if on {
        Color::srgb(0.96, 0.85, 0.40)
    } else {
        Color::srgb(0.25, 0.25, 0.30)
    }
}

fn marker_color(revealed: bool) -> Color {
    if revealed {
        Color::srgb(0.70, 0.62, 0.40)
    } else {
        Color::srgb(0.22, 0.22, 0.26)
    }
}

fn pip_on(s: &Session, i: usize) -> bool {
    match i {
        0 => s.beats.anya_stirred,
        1 => s.beats.donna_settled,
        2 => s.beats.nature_answered(),
        3 => s.beats.crossed,
        4 => s.beats.rift_disrupted,
        _ => false,
    }
}
