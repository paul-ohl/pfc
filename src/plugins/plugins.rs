use bevy::prelude::*;

use crate::plugins::{
    camera::{setup_camera, update_camera},
    moves::{setup_moves, update_moves},
    players::{spawn_players, update_players},
    progress_bar::{setup_bar, update_bar},
    timer::{setup_timer, update_timer},
    turn::{setup_turn, update_turn},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera);
    }
}

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bar)
            .add_systems(Update, update_bar);
    }
}

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_timer)
            .add_systems(Update, update_timer);
    }
}

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_turn)
            .add_systems(Update, update_turn);
    }
}

pub struct MovesPlugin;

impl Plugin for MovesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_moves)
            .add_systems(Update, update_moves);
    }
}

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players)
            .add_systems(Update, update_players);
    }
}
