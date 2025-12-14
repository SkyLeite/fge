use crate::prelude::*;

pub mod components;
mod systems;

pub use components::*;

static DEFAULT_ANIMATION: &str = "standing";

pub struct AnimationPlayerPlugin;

impl Plugin for AnimationPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::AnimationPlayer>()
            .add_systems(
                FixedUpdate,
                systems::set_animation_frame.in_set(system_sets::Visual),
            )
            .add_systems(FixedUpdate, systems::set_sprite.in_set(system_sets::Visual));
    }
}
