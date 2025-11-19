#![allow(clippy::type_complexity)]

use bevy::prelude::*;

mod action_context;
mod components;
mod plugins;
mod prelude;
mod sequence;

use plugins::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AnimationPlayerPlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Update, update)
        .run();
}

fn update() {}
