#![allow(unused)]

use bevy::{prelude::*, window::WindowResolution};

use pfc::plugins::{
    CameraPlugin, MovesPlugin, PlayersPlugin, ProgressBarPlugin, TimerPlugin, TurnPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PFC".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PlayersPlugin)
        .add_plugins(TimerPlugin)
        .add_plugins(MovesPlugin)
        .add_plugins(ProgressBarPlugin)
        .add_plugins(TurnPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
