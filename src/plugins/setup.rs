use bevy::prelude::*;

pub struct Setup;

impl Plugin for Setup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
