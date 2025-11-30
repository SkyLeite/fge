#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

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
        .add_plugins(InputPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
