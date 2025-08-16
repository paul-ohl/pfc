#![allow(unused)]

use bevy::prelude::*;

use crate::plugins::{CurrentTurn, MoveIcon, MoveSprites};

pub struct Players;

pub enum PlayerType {
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    player_type: PlayerType,
    health: u32,
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
            health: 10,
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
            health: 10,
        },
    ));
}

fn make_move(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    moves_resource: Res<MoveSprites>,
    existing_moves: Query<Entity, With<MoveIcon>>,
    current_turn_query: Query<(Entity, &mut CurrentTurn)>,
) {
    let left = Transform::from_translation(Vec3::new(-200.0, 40.0, 0.0));
    let right = Transform::from_translation(Vec3::new(200.0, 40.0, 0.0));
    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyA => {
                commands.spawn((moves_resource.rock.clone(), left, MoveIcon));
            }
            KeyCode::KeyS => {
                commands.spawn((moves_resource.paper.clone(), left, MoveIcon));
            }
            KeyCode::KeyD => {
                commands.spawn((moves_resource.scissors.clone(), left, MoveIcon));
            }
            KeyCode::KeyJ => {
                commands.spawn((moves_resource.rock.clone(), right, MoveIcon));
            }
            KeyCode::KeyK => {
                commands.spawn((moves_resource.paper.clone(), right, MoveIcon));
            }
            KeyCode::KeyL => {
                commands.spawn((moves_resource.scissors.clone(), right, MoveIcon));
            }
            KeyCode::KeyC => {
                for entity in existing_moves.iter() {
                    commands.entity(entity).despawn();
                }
            }
            _ => {
                println!("Pressed other key: {:?}", key);
            }
        }
    }
}
