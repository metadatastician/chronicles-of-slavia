// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! The Bevy renderer (Layer 3) — a deliberately thin skin over [`Session`].
//!
//! It holds only *view* state (sprites, positions) and forwards input. Every
//! rule decision goes through `Session` → `slavia_core`; the renderer never
//! decides the emotional grammar. Art is intentionally schematic (coloured
//! blocks) — this is the M1 spike to make Zone A *felt*, not to dress it.

use crate::session::{BirdState, Session};
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

const FOREST_X: f32 = -300.0;
const CROSSING_X: f32 = 0.0;
const MOUNTAIN_X: f32 = 300.0;
const GROVE_X: f32 = -250.0;
const GIRL_Y: f32 = -40.0;
const MOVE_SPEED: f32 = 260.0;

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
        .add_systems(Update, (input, move_active_girl, sync_view, sync_sky, quit))
        .run();
}

fn print_controls() {
    println!(
        "\nChronicles of Slavia — Zone A (M1 spike)\n\
         ----------------------------------------\n\
         Tab        switch girl (Anya <-> Donna)\n\
         A/D or <-> move the active girl\n\
         E          reach out to the grove birds\n\
         S          settle the crossing   (only Donna can)\n\
         C          cross the crossing     (once it is passable)\n\
         R          awaken the Rift        (the Fracture)\n\
         Esc        quit\n\
         \n\
         Try: E as Anya (stir) -> Tab -> E as Donna (settle) ->\n\
         S as Donna -> Tab -> C as Anya -> R -> E (disrupted).\n\
         The five pips along the top fill as each beat happens.\n"
    );
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // The two lands — forest (Anya's) on the left, mountains (Donna's) on the right.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.15, 0.34, 0.18),
            custom_size: Some(Vec2::new(300.0, 220.0)),
            ..default()
        },
        Transform::from_xyz(FOREST_X, -60.0, -10.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.30, 0.30, 0.37),
            custom_size: Some(Vec2::new(300.0, 220.0)),
            ..default()
        },
        Transform::from_xyz(MOUNTAIN_X, -60.0, -10.0),
    ));

    // The crossing — dim until Donna settles it.
    commands.spawn((
        Sprite {
            color: crossing_color(false),
            custom_size: Some(Vec2::new(170.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(CROSSING_X, GIRL_Y - 22.0, -5.0),
        CrossingSprite,
    ));

    // The Rift — an invisible vertical seam until it awakens.
    commands.spawn((
        Sprite {
            color: rift_color(false),
            custom_size: Some(Vec2::new(12.0, 500.0)),
            ..default()
        },
        Transform::from_xyz(CROSSING_X, 0.0, 6.0),
        RiftSprite,
    ));

    // The grove birds.
    for i in 0..5 {
        commands.spawn((
            Sprite {
                color: bird_color(BirdState::Neutral),
                custom_size: Some(Vec2::new(11.0, 11.0)),
                ..default()
            },
            Transform::from_xyz(GROVE_X + i as f32 * 16.0, 40.0 + (i % 2) as f32 * 14.0, 0.0),
            BirdSprite,
        ));
    }

    // The girls.
    commands.spawn((
        Sprite {
            color: girl_color("anya", true),
            custom_size: Some(Vec2::new(24.0, 42.0)),
            ..default()
        },
        Transform::from_xyz(FOREST_X + 30.0, GIRL_Y, 1.0),
        GirlSprite { id: "anya" },
    ));
    commands.spawn((
        Sprite {
            color: girl_color("donna", false),
            custom_size: Some(Vec2::new(24.0, 42.0)),
            ..default()
        },
        Transform::from_xyz(FOREST_X - 20.0, GIRL_Y, 1.0),
        GirlSprite { id: "donna" },
    ));

    // Beat pips along the top.
    for i in 0..5 {
        commands.spawn((
            Sprite {
                color: pip_color(false),
                custom_size: Some(Vec2::new(24.0, 24.0)),
                ..default()
            },
            Transform::from_xyz(-56.0 + i as f32 * 28.0, 210.0, 2.0),
            BeatPip(i),
        ));
    }
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    let s = &mut game.0;
    if keys.just_pressed(KeyCode::Tab) {
        s.toggle_character();
        println!("-> now playing {}", s.active_name());
    }
    if keys.just_pressed(KeyCode::KeyE) {
        let r = s.approach_birds();
        println!("{} reaches out to the birds -> {:?}", s.active_name(), r);
    }
    if keys.just_pressed(KeyCode::KeyS) {
        if s.settle_crossing() {
            println!("Donna settles the crossing.");
        } else {
            println!(
                "{} cannot settle the crossing (she does not lower taxis).",
                s.active_name()
            );
        }
    }
    if keys.just_pressed(KeyCode::KeyC) {
        match s.cross() {
            Ok(()) => println!("{} crosses.", s.active_name()),
            Err(e) => println!("Cannot cross: {e}."),
        }
    }
    if keys.just_pressed(KeyCode::KeyR) {
        s.awaken_rift();
        println!("The Rift awakens. The birds stop answering normally.");
    }

    // Progress readout after any beat-bearing action.
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

fn move_active_girl(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game: Res<Game>,
    mut girls: Query<(&GirlSprite, &mut Transform)>,
) {
    let active = game.0.active_id();
    let mut dx = 0.0;
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dx -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dx += 1.0;
    }
    if dx == 0.0 {
        return;
    }
    for (girl, mut transform) in &mut girls {
        if girl.id == active {
            transform.translation.x = (transform.translation.x
                + dx * MOVE_SPEED * time.delta_secs())
            .clamp(-420.0, 420.0);
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
        Option<&GirlSprite>,
        Option<&BeatPip>,
    )>,
) {
    let s = &game.0;
    for (mut sprite, bird, crossing, rift, girl, pip) in &mut sprites {
        if bird.is_some() {
            sprite.color = bird_color(s.birds);
        } else if crossing.is_some() {
            sprite.color = crossing_color(s.crossing_passable());
        } else if rift.is_some() {
            sprite.color = rift_color(s.rift_active());
        } else if let Some(girl) = girl {
            sprite.color = girl_color(girl.id, girl.id == s.active_id());
        } else if let Some(pip) = pip {
            sprite.color = pip_color(pip_on(s, pip.0));
        }
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
