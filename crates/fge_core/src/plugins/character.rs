use std::path::Path;

use crate::prelude::*;
use crate::sequence::Sequence;
use bevy_spritesheet_animation::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(FixedUpdate, set_hitboxes);
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub character_marker: Character,
    pub health: crate::components::Health,
    pub position: crate::components::Position,
    pub animation_player: crate::plugins::animation_player::AnimationPlayer,
}

#[derive(Component)]
pub struct Character;

pub fn spawn(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d::default());

    let character = fge_models::serde::from_file(Path::new("./data/character.lua")).unwrap();
    let mut animation_atlas = Animations::default();
    let mut spritesheets = Spritesheets::default();

    for (sheet_id, sheet) in character.spritesheets {
        let image = assets.load(sheet.file.clone());

        let spritesheet = Spritesheet::new(&image, sheet.columns as usize, sheet.rows as usize);

        for (id, anim) in &character.animations {
            let mut animation_builder = spritesheet
                .create_animation()
                .set_duration(AnimationDuration::PerFrame(16));

            match anim {
                fge_models::Animation::Sprite(sprite_animation) => {
                    let animation_cells: Vec<(usize, usize)> = sprite_animation
                        .frames
                        .iter()
                        .flat_map(|f| {
                            vec![(f.cell.0 as usize, f.cell.1 as usize); f.duration as usize]
                        })
                        .collect();

                    animation_builder = animation_builder.add_cells(animation_cells);

                    let animation = animation_builder.build();
                    let animation_handle = animations.add(animation);
                    let sequence = Sequence {
                        animation: animation_handle,
                        frames: sprite_animation.frames.clone(),
                    };
                    animation_atlas.insert(id.clone(), sequence);
                }
                fge_models::Animation::Model(model_animation) => todo!(),
            }
        }

        let sprite = spritesheet
            .with_size_hint(sheet.width as u32, sheet.height as u32)
            .sprite(&mut atlas_layouts);

        spritesheets.insert(sheet_id, sprite);
    }

    let animation_player =
        crate::plugins::animation_player::AnimationPlayer::new(animation_atlas, spritesheets);

    commands.spawn(CharacterBundle {
        health: crate::components::Health(character.max_health),
        position: crate::components::Position(Vec2::new(0.0, 0.0)),
        animation_player,
        character_marker: Character {},
    });
}

pub fn set_hitboxes(characters_query: Query<&SpritesheetAnimation, With<Character>>) {
    for animation in characters_query {
        let progress = animation.progress;
    }
}
