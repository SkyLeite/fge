#![allow(clippy::type_complexity)]

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod action_context;
mod components;
mod plugins;
mod prelude;
mod sequence;
pub mod system_sets;

use plugins::*;

fn main() {
    App::new()
        .register_type::<components::Position>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AnimationPlayerPlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .configure_sets(
            FixedUpdate,
            (
                system_sets::Input,
                system_sets::Gameplay,
                system_sets::Visual,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
