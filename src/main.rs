#![allow(unused)]

use bevy::{prelude::*, window::WindowResolution};

use pfc::plugins::{MovesPlugin, Players, ProgressBarPlugin, Setup, Timer, TurnPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PFC".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Setup)
        .add_plugins(Players)
        .add_plugins(Timer)
        .add_plugins(MovesPlugin)
        .add_plugins(ProgressBarPlugin)
        .add_plugins(TurnPlugin)
        .run();
}
