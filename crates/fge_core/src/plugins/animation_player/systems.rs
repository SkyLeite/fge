use crate::prelude::*;

pub fn set_animation_frame(
    time: Res<Time>,
    query: Query<(
        &mut super::components::AnimationPlayer,
        &mut super::AnimationTimer,
        &mut CharacterState,
    )>,
) {
    for (mut animation_player, mut timer, mut character_state) in query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let sequence = animation_player.current_sequence();
            if animation_player.animation_frame == sequence.duration() {
                // Animation ended
                match sequence.repetition {
                    fge_models::Repetition::None => character_state.0 = Default::default(),
                    fge_models::Repetition::Loop => animation_player.animation_frame = 1,
                }
            } else {
                animation_player.animation_frame += 1;
            }
        }
    }
}

pub fn set_animation_from_state(
    query: Query<
        (&mut super::components::AnimationPlayer, &CharacterState),
        Changed<CharacterState>,
    >,
) {
    for (mut animation_player, state) in query {
        let animation_id = state.0.to_string();
        animation_player.set_animation(animation_id.into());
    }
}

pub fn set_sprite(query: Query<(&super::components::AnimationPlayer, &mut Sprite)>) {
    for (animation_player, mut sprite) in query {
        let sequence = animation_player.current_sequence();
        let frame = sequence
            .get_frame(animation_player.animation_frame)
            .unwrap();

        let sheet = frame
            .sheet
            .clone()
            .unwrap_or(animation_player.active_animation_id.to_string().into());

        let spritesheet = animation_player.spritesheets.get(&sheet).unwrap();

        sprite.image = spritesheet.image.clone();
        sprite.texture_atlas = Some(TextureAtlas {
            layout: spritesheet.layout.clone(),
            index: frame.cell.0 as usize,
        });
    }
}
