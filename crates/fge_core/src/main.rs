use std::path::Path;

use bevy::prelude::*;
use fge_models;

mod character;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let character = fge_models::serde::from_file(Path::new("./data/character.lua")).unwrap();
}

fn update() {}
