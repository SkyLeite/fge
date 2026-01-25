#![allow(clippy::type_complexity)]

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod action_context;
mod args;
mod components;
mod plugins;
mod prelude;
mod sequence;
pub mod system_sets;

use fge_models::CharacterID;
use plugins::*;

use crate::args::Args;

fn main() {
    let mut app = App::new();

    let args = Args::from_env();
    app.insert_resource(args);

    app.register_type::<components::Position>()
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

fn setup(mut commands: Commands, args: Res<Args>) {
    commands.spawn(Camera2d);

    let game = fge_models::serde::from_file(&args.game_path.join("game.lua")).unwrap();
    commands.spawn(components::CharacterList(game.characters));

    commands.queue(character::commands::SpawnCharacter(CharacterID(
        "akiha".into(),
    )));
}
