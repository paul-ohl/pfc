#![allow(unused)]

use bevy::prelude::*;

use pfc::plugins::{Moves, Players, ProgressBarPlugin, Setup, Timer};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Setup)
        .add_plugins(Players)
        .add_plugins(Timer)
        .add_plugins(Moves)
        .add_plugins(ProgressBarPlugin)
        .run();
}
