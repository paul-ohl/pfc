use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

#[derive(Debug, Component)]
pub struct Camera;

pub fn setup_camera(mut commands: Commands) {
    const VIRTUAL_HEIGHT: f32 = 1200.0;

    commands.spawn((
        Camera2d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: VIRTUAL_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
        Camera,
    ));
}

pub fn update_camera(// mut commands: Commands,
    // mut camera_query: Query<(Entity, &Camera)>,
    // mut resize_events: EventReader<WindowResized>,
) {
}
