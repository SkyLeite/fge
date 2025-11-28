use crate::prelude::*;

pub mod components;
mod systems;

pub use components::*;

static DEFAULT_ANIMATION: &str = "standing";

pub struct AnimationPlayerPlugin;

impl Plugin for AnimationPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::AnimationPlayer>()
            .add_systems(FixedFirst, systems::set_animation_frame)
            .add_systems(FixedUpdate, systems::set_sprite);
    }
}
