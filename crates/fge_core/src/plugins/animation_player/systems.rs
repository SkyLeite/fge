use super::AnimationPlayer;
use super::DEFAULT_ANIMATION;
use crate::prelude::*;
use bevy_spritesheet_animation::prelude::*;

pub fn create_spritesheet_animation(
    mut commands: Commands,
    query: Query<
        (Entity, &AnimationPlayer),
        (Added<AnimationPlayer>, Without<SpritesheetAnimation>),
    >,
) {
    for (entity, animation_player) in query {
        let default_sequence = animation_player
            .animations
            .get(&animation_player.active_animation_id)
            .unwrap_or_else(|| panic!("Could not find animation `{DEFAULT_ANIMATION}`"));

        let sprite = animation_player
            .spritesheets
            .first()
            .expect("Could not find a spritesheet to use as the default.")
            .clone();

        let spritesheet_animation = SpritesheetAnimation::new(default_sequence.animation.clone());

        commands
            .entity(entity)
            .insert((spritesheet_animation, sprite));
    }
}

pub fn set_sprite(query: Query<(&AnimationPlayer, &SpritesheetAnimation, &mut Sprite)>) {
    for (animation_player, spritesheet_animation, mut sprite) in query {
        let current_sequence = animation_player.current_sequence();
        let progress = spritesheet_animation.progress;

        let current_frame = current_sequence
            .get_frame(progress.frame as u32)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find frame {} in sequence {:?}",
                    progress.frame, animation_player.active_animation_id
                )
            });

        let spritesheet = animation_player
            .spritesheets
            .get(&current_frame.sheet)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find spritesheet with id `{:?}`",
                    current_frame.sheet
                )
            });

        sprite.image = spritesheet.image.clone();
    }
}
