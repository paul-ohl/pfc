use bevy::prelude::*;

use crate::{get_single_component, get_single_mut_component, plugins::timer::TimerComponent};

#[derive(Component)]
pub struct ProgressBar;

#[derive(Component)]
pub struct ProgressBarText;

pub(super) fn setup_bar(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(600.0, 20.0)),
            ..default()
        },
        ProgressBar,
        Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
    ));
    commands.spawn((
        Text2d::new("timer"),
        TextFont::default(),
        ProgressBarText,
        Transform::from_translation(Vec3::new(0.0, 225.0, 0.0)),
    ));
}

pub(super) fn update_bar(
    timer_query: Query<(Entity, &mut TimerComponent)>,
    mut bar_query: Query<(Entity, &ProgressBar, &mut Transform, &mut Sprite)>,
    mut text_query: Query<(Entity, &ProgressBarText, &mut Text2d)>,
) {
    let timer = get_single_component!(timer_query);
    let (_, _, mut transform, mut sprite) = get_single_mut_component!(bar_query);
    let (_, _, mut text) = get_single_mut_component!(text_query);

    *text = Text2d::new(format!(
        "Time remaining: {:.2}s",
        timer.1.timer.remaining_secs()
    ));

    let time_remaining = timer.1.timer.fraction_remaining();
    transform.scale.x = time_remaining;
    sprite.color = Color::srgb(1.0 - time_remaining, time_remaining, 0.0);
}
