use bevy::{asset::RenderAssetUsages, image::ImageLoaderSettings, prelude::*};

pub struct Moves;

impl Plugin for Moves {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_moves)
            .add_systems(Update, update_moves);
    }
}

#[derive(Resource)]
pub struct MoveSprites {
    pub rock: Sprite,
    pub paper: Sprite,
    pub scissors: Sprite,
}

#[derive(Component)]
pub struct MoveIcon;

fn setup_moves(mut commands: Commands, asset_server: Res<AssetServer>) {
    let load_texture = |path: &str| {
        asset_server.load_with_settings(path, |settings: &mut ImageLoaderSettings| {
            settings.asset_usage = RenderAssetUsages::all()
        })
    };
    let rock = load_texture("rock.png");
    let paper = load_texture("paper.png");
    let scissors = load_texture("scissors.png");

    commands.insert_resource(MoveSprites {
        rock: Sprite::from_image(rock),
        paper: Sprite::from_image(paper),
        scissors: Sprite::from_image(scissors),
    });
}

fn update_moves() {}
