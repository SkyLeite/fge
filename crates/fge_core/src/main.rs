use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

mod animation_player;
mod character;
mod sequence;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SpritesheetAnimationPlugin)
        .add_plugins(animation_player::AnimationPlugin)
        .add_systems(Startup, character::spawn)
        .add_systems(Update, update)
        .add_systems(FixedUpdate, character::set_hitboxes)
        .run();
}

fn update() {}
