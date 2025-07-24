#![allow(unused)]

use bevy::prelude::*;

pub struct Players;

pub enum PlayerType {
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    player_type: PlayerType,
}

impl Plugin for Players {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players)
            .add_systems(Update, make_move);
    }
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-200.0, 40.0, 0.0),
            ..default()
        },
        Player {
            player_type: PlayerType::Left,
        },
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(200.0, 40.0, 0.0),
            ..default()
        },
        Player {
            player_type: PlayerType::Right,
        },
    ));
}

fn make_move(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyA => {
                println!("Pressed A");
            }
            KeyCode::KeyS => {
                println!("Pressed S");
            }
            KeyCode::KeyD => {
                println!("Pressed D");
            }
            KeyCode::KeyJ => {
                println!("Pressed J");
            }
            KeyCode::KeyK => {
                println!("Pressed K");
            }
            KeyCode::KeyL => {
                println!("Pressed L");
            }
            _ => {
                println!("Pressed other key: {:?}", key);
            }
        }
    }
}
