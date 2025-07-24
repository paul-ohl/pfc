use bevy::prelude::Timer as BevyTimer;
use bevy::prelude::*;

pub struct Timer;

impl Plugin for Timer {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_timer)
            .add_systems(Update, update_timer);
    }
}

#[derive(Debug, Component)]
pub struct TimerComponent {
    pub timer: BevyTimer,
    pub step: u32,
}

fn setup_timer(mut commands: Commands) {
    commands.spawn((TimerComponent {
        timer: BevyTimer::from_seconds(3.0, TimerMode::Once),
        step: 0,
    },));
}

fn update_timer(mut timer_query: Query<(Entity, &mut TimerComponent)>, time: Res<Time>) {
    let mut timer = match timer_query.single_mut() {
        Ok(timer) => timer,
        Err(_) => {
            println!("No timer found");
            return;
        }
    };
    if timer.1.timer.tick(time.delta()).just_finished() {
        println!(
            "Timer finished at step: {} ({}s)",
            timer.1.step,
            timer.1.timer.elapsed_secs()
        );
        timer.1.timer =
            BevyTimer::from_seconds(timer.1.timer.elapsed_secs() * 0.80, TimerMode::Once);
        timer.1.step += 1;
    }
}
