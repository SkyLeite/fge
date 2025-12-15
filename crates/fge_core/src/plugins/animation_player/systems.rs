use crate::prelude::*;

pub fn set_animation_frame(
    time: Res<Time>,
    query: Query<(
        &mut super::components::AnimationPlayer,
        &mut super::AnimationTimer,
    )>,
) {
    for (mut animation_player, mut timer) in query {
        timer.tick(time.delta());

        if timer.just_finished() {
            let sequence = animation_player.current_sequence();
            if animation_player.animation_frame == sequence.duration() {
                animation_player.animation_frame = 1;
            } else {
                animation_player.animation_frame += 1;
            }
        }
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
