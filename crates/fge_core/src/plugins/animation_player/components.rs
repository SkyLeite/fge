use std::time::Duration;

use crate::prelude::*;

use fge_models::AnimationID;

#[derive(Component, Reflect)]
#[require(Sprite, AnimationTimer)]
pub struct AnimationPlayer {
    pub animations: Animations,
    pub spritesheets: Spritesheets,
    pub active_animation_id: AnimationID,
    pub animation_frame: u32,
}

impl AnimationPlayer {
    pub fn new(animations: Animations, spritesheets: Spritesheets) -> Self {
        let active_animation_id: AnimationID = super::DEFAULT_ANIMATION.into();

        Self {
            animations,
            spritesheets,
            active_animation_id,
            animation_frame: 0,
        }
    }

    pub fn current_sequence(&self) -> &Sequence {
        self.animations
            .get(&self.active_animation_id)
            .expect("Invalid active_animation_id")
    }

    /// Changes the currently playing animation.
    pub fn set_animation(&mut self, animation_id: AnimationID) {
        self.active_animation_id = animation_id;
        self.reset();
    }

    /// Resets the animation progress to 0
    pub fn reset(&mut self) {
        self.animation_frame = 0;
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(16), TimerMode::Repeating))
    }
}
