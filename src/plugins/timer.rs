use bevy::prelude::Timer as BevyTimer;
use bevy::prelude::*;

use crate::get_single_mut_component;

#[derive(Debug, Component)]
pub struct TimerComponent {
    pub timer: BevyTimer,
    pub step: u32,
}

pub fn setup_timer(mut commands: Commands) {
    commands.spawn((TimerComponent {
        timer: BevyTimer::from_seconds(3.0, TimerMode::Once),
        step: 0,
    },));
}

pub fn update_timer(mut timer_query: Query<(Entity, &mut TimerComponent)>, time: Res<Time>) {
    let mut timer = get_single_mut_component!(timer_query);

    if timer.1.timer.tick(time.delta()).just_finished() {
        println!(
            "Timer finished at step: {} ({}s)",
            timer.1.step,
            timer.1.timer.elapsed_secs()
        );
        timer.1.timer =
            BevyTimer::from_seconds(timer.1.timer.elapsed_secs() * 0.99, TimerMode::Once);
        timer.1.step += 1;
    }
}
