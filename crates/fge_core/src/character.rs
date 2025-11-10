use std::{collections::HashMap, path::Path};

use crate::sequence::Sequence;
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use fge_models::{AnimationID, Frame, SpritesheetID};

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Default, Component)]
pub struct Animations(HashMap<AnimationID, Sequence>);

impl Animations {
    pub fn insert(&mut self, id: AnimationID, sequence: Sequence) {
        self.0.insert(id, sequence);
    }

    pub fn get(&self, id: AnimationID) -> Option<&Sequence> {
        self.0.get(&id)
    }
}

#[derive(Default, Component)]
pub struct Spritesheets(HashMap<SpritesheetID, Sprite>);

impl Spritesheets {
    pub fn insert(&mut self, id: SpritesheetID, sprite: Sprite) {
        self.0.insert(id, sprite);
    }

    pub fn get(&self, id: SpritesheetID) -> Option<&Sprite> {
        self.0.get(&id)
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub character_marker: Character,
    pub health: Health,
    pub position: Position,
    pub animations: Animations,
    pub spritesheets: Spritesheets,
    pub sprite: Sprite,
    pub spritesheet_animation: SpritesheetAnimation,
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

    let s = spritesheets.get("idle".into()).unwrap();
    let sequence = animation_atlas.get("standing".into()).unwrap();

    commands.spawn(CharacterBundle {
        health: Health(character.max_health),
        position: Position(Vec2::new(0.0, 0.0)),
        spritesheet_animation: SpritesheetAnimation::new(sequence.animation.clone()),
        animations: animation_atlas,
        sprite: s.clone(),
        spritesheets,
        character_marker: Character {},
    });
}

pub fn set_hitboxes(characters_query: Query<&SpritesheetAnimation, With<Character>>) {
    for animation in characters_query {
        let progress = animation.progress;
    }
}
