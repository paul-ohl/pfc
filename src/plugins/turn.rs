use bevy::prelude::*;

use crate::{get_single_component, get_single_mut_component, plugins::TimerComponent};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_turn)
            .add_systems(Update, update_turn);
    }
}

#[derive(Component)]
pub struct TurnCounter(u32);

#[derive(Component)]
pub struct LastStep(u32);

fn setup_turn(mut commands: Commands) {
    commands.spawn((TurnCounter(0), LastStep(0)));
}

fn update_turn(
    timer_query: Query<(Entity, &mut TimerComponent)>,
    mut turn_counter_query: Query<(Entity, &mut TurnCounter)>,
    mut last_step_query: Query<(Entity, &mut LastStep)>,
) {
    let timer = get_single_component!(timer_query);
    let mut last_step = get_single_mut_component!(last_step_query);

    if timer.1.step != last_step.1.0 {
        let mut turn_counter = get_single_mut_component!(turn_counter_query);
        turn_counter.1.0 += 1;
        println!("Turn: {}", turn_counter.1.0);
        last_step.1.0 = timer.1.step;
    }
}
