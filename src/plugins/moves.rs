use core::f32;

use bevy::{asset::RenderAssetUsages, image::ImageLoaderSettings, prelude::*};

pub struct Moves;

impl Plugin for Moves {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_moves)
            .add_systems(Update, update_moves);
    }
}

#[derive(Debug, Component)]
enum PossibleMoves {
    Rock,
    Paper,
    Scissors,
}

impl PossibleMoves {
    fn get_texture_path(&self) -> &'static str {
        match self {
            PossibleMoves::Rock => "rock.png",
            PossibleMoves::Paper => "paper.png",
            PossibleMoves::Scissors => "scissors.png",
        }
    }
}

#[derive(Debug, Component)]
struct MoveAttack;

fn rock_entity() -> Entity {

}

fn setup_moves(mut commands: Commands, asset_server: Res<AssetServer>) {
    let move_attack = PossibleMoves::Rock;
    let texture_attack = asset_server.load_with_settings(
        move_attack.get_texture_path(),
        |settings: &mut ImageLoaderSettings| settings.asset_usage = RenderAssetUsages::RENDER_WORLD,
    );

    commands.spawn((
        Name::new("Attack Move"),
        MoveAttack,
        Sprite::from_image(texture_attack),
        Transform::from_xyz(-200.0, 0.0, 0.0)
            .with_scale(Vec3::new(0.5, 0.5, 0.5))
            .with_rotation(Quat::from_rotation_z(f32::consts::PI)),
        move_attack,
    ));
}

fn update_moves() {}
