use crate::prelude::*;
use bevy_spritesheet_animation::{plugin::AnimationSystemSet, prelude::*};
use fge_models::AnimationID;

mod systems;

static DEFAULT_ANIMATION: &str = "standing";

#[derive(Component)]
#[require(Sprite)]
pub struct AnimationPlayer {
    pub animations: Animations,
    pub spritesheets: Spritesheets,
    active_animation_id: AnimationID,
}

impl AnimationPlayer {
    pub fn new(animations: Animations, spritesheets: Spritesheets) -> Self {
        let active_animation_id: AnimationID = DEFAULT_ANIMATION.into();

        Self {
            animations,
            spritesheets,
            active_animation_id,
        }
    }

    pub fn current_sequence(&self) -> &Sequence {
        self.animations
            .get(&self.active_animation_id)
            .expect("Invalid active_animation_id")
    }
}

pub struct AnimationPlayerPlugin;

impl Plugin for AnimationPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpritesheetAnimationPlugin)
            .add_systems(FixedUpdate, systems::create_spritesheet_animation)
            .add_systems(PostUpdate, systems::set_sprite.before(AnimationSystemSet));
    }
}
