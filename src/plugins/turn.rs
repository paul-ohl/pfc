use bevy::prelude::*;

use crate::{
    get_single_component, get_single_mut_component,
    plugins::timer::TimerComponent,
    utils::types::{Moves, Outcome},
};

#[derive(Component)]
pub struct CurrentTurn {
    pub left_attack: Moves,
    pub right_attack: Moves,
    pub left_defense: Moves,
    pub right_defense: Moves,
    pub left_last_attack: Moves,
    pub right_last_attack: Moves,
}

#[derive(Component)]
pub struct TurnCounter(u32);

#[derive(Component)]
pub struct LastStep(u32);

pub fn setup_turn(mut commands: Commands) {
    commands.spawn((
        TurnCounter(0),
        LastStep(0),
        CurrentTurn {
            left_attack: Moves::NoMove,
            right_attack: Moves::NoMove,
            left_defense: Moves::NoMove,
            right_defense: Moves::NoMove,
            left_last_attack: Moves::NoMove,
            right_last_attack: Moves::NoMove,
        },
    ));
}

pub fn update_turn(
    timer_query: Query<(Entity, &mut TimerComponent)>,
    mut turn_counter_query: Query<(Entity, &mut TurnCounter)>,
    mut last_step_query: Query<(Entity, &mut LastStep)>,
    mut current_turn_query: Query<(Entity, &mut CurrentTurn)>,
) {
    let timer = get_single_component!(timer_query);
    let mut last_step = get_single_mut_component!(last_step_query);

    if timer.1.step != last_step.1.0 {
        let mut turn_counter = get_single_mut_component!(turn_counter_query);
        let mut current_turn = get_single_mut_component!(current_turn_query);
        turn_counter.1.0 += 1;
        last_step.1.0 = timer.1.step;

        let is_left_taking_damage = get_winner(
            current_turn.1.right_last_attack,
            current_turn.1.left_defense,
        );
        let is_right_taking_damage = get_winner(
            current_turn.1.left_last_attack,
            current_turn.1.right_defense,
        );

        if is_left_taking_damage == Outcome::WinB {
            println!("Left player takes damage!");
        } else {
            println!("Left player does not take damage!");
        }

        if is_right_taking_damage == Outcome::WinA {
            println!("Right player takes damage!");
        } else {
            println!("Right player does not take damage!");
        }

        current_turn.1.left_last_attack = current_turn.1.left_attack;
        current_turn.1.right_last_attack = current_turn.1.right_attack;
        current_turn.1.left_attack = Moves::NoMove;
        current_turn.1.right_attack = Moves::NoMove;
    }
}

fn get_winner(left: Moves, right: Moves) -> Outcome {
    match (left as i8 - right as i8).rem_euclid(3) {
        1 => Outcome::WinA,
        2 => Outcome::WinB,
        _ => Outcome::Draw,
    }
}
