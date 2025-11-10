use std::collections::HashMap;

use crate::sequence::Sequence;
use bevy::prelude::*;
use bevy_spritesheet_animation::{plugin::AnimationSystemSet, prelude::*};
use fge_models::{AnimationID, SpritesheetID};

#[derive(Component)]
#[require(Sprite)]
pub struct AnimationPlayer {
    pub animations: Animations,
    pub spritesheets: Spritesheets,
    active_animation_id: AnimationID,
}

impl AnimationPlayer {
    pub fn new(animations: Animations, spritesheets: Spritesheets) -> Self {
        let active_animation_id: AnimationID = "idle".into();

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

    pub fn set_animation(animation_id: AnimationID) {}
}

#[derive(Default, Component)]
pub struct Animations(HashMap<AnimationID, Sequence>);

impl Animations {
    pub fn insert(&mut self, id: AnimationID, sequence: Sequence) {
        self.0.insert(id, sequence);
    }

    pub fn get(&self, id: &AnimationID) -> Option<&Sequence> {
        self.0.get(&id)
    }
}

#[derive(Default, Component)]
pub struct Spritesheets(HashMap<SpritesheetID, Sprite>);

impl Spritesheets {
    pub fn insert(&mut self, id: SpritesheetID, sprite: Sprite) {
        self.0.insert(id, sprite);
    }

    pub fn get(&self, id: &SpritesheetID) -> Option<&Sprite> {
        self.0.get(&id)
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, create_spritesheet_animation)
            .add_systems(PostUpdate, set_sprite.before(AnimationSystemSet));
    }
}

fn create_spritesheet_animation(
    mut commands: Commands,
    query: Query<
        (Entity, &AnimationPlayer),
        (Added<AnimationPlayer>, Without<SpritesheetAnimation>),
    >,
) {
    for (entity, animation_player) in query {
        println!("Hi!");
        let default_sequence = animation_player
            .animations
            .get(&animation_player.active_animation_id)
            .expect("Could not find animation `idle`");

        let sprite = animation_player
            .spritesheets
            .get(&"idle".into())
            .expect("Could not find spritesheet `idle`")
            .clone();

        let spritesheet_animation = SpritesheetAnimation::new(default_sequence.animation.clone());

        commands
            .entity(entity)
            .insert((spritesheet_animation, sprite));
    }
}

fn set_sprite(query: Query<(&AnimationPlayer, &SpritesheetAnimation, &mut Sprite)>) {
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
