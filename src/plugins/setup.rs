use bevy::prelude::*;

pub struct Setup;

impl Plugin for Setup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Projection::from(OrthographicProjection {
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn update_camera(mut commands: Commands) {}
