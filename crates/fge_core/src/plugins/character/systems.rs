use crate::plugins::input::{Input, InputHistory};
use crate::prelude::*;
use crate::sequence::Sequence;
use crate::{
    action_context::ActionContext, plugins::animation_player::components::AnimationPlayer,
};
use bevy::ecs::system::RunSystemOnce;
use bevy_rapier2d::prelude::*;
use fge_models::{AnimationID, Square};
use std::path::Path;

use super::{Character, CharacterBundle};

pub fn spawn(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let character = fge_models::serde::from_file(Path::new("./data/character.lua")).unwrap();

    // Load spritesheets
    let mut spritesheets = Spritesheets::default();
    for (id, spritesheet) in &character.spritesheets {
        let texture = assets.load(spritesheet.file.clone());
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(
                (spritesheet.width / spritesheet.columns).into(),
                (spritesheet.height / spritesheet.rows).into(),
            ),
            spritesheet.columns.into(),
            spritesheet.rows.into(),
            None,
            None,
        );
        let texture_atlas_layout = atlas_layouts.add(layout);

        spritesheets.insert(
            id.clone(),
            LoadedSpritesheet {
                image: texture,
                layout: texture_atlas_layout,
            },
        );
    }

    // Create animation atlas
    let mut animation_atlas = Animations::default();
    for (id, animation) in &character.animations {
        match animation {
            fge_models::Animation::Sprite(sprite_animation) => {
                let sequence = Sequence {
                    default_collision_box: sprite_animation.default_collision_box.clone(),
                    frames: sprite_animation.frames.clone(),
                    repetition: sprite_animation.repetition,
                };

                animation_atlas.insert(id.clone(), sequence);
            }
            fge_models::Animation::Model(_model_animation) => todo!(),
        }
    }

    let animation_player = crate::plugins::animation_player::components::AnimationPlayer::new(
        animation_atlas,
        spritesheets,
    );

    commands
        .spawn(CharacterBundle {
            health: crate::components::Health(character.max_health),
            position: crate::components::Position::default(),
            animation_player,
            character_data: Character(character),
            state: CharacterState::default(),
        })
        .with_child((
            CollisionBox,
            Collider::cuboid(1.0, 1.0),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
}

pub fn update_position(query: Query<(&mut Transform, &Position), Changed<Position>>) {
    for (mut transform, position) in query {
        transform.translation.x = position.0.x as f32;
        transform.translation.y = position.0.y as f32;
    }
}

pub fn set_collision_boxes(
    mut commands: Commands,
    characters_query: Query<
        (
            &Children,
            &crate::plugins::animation_player::components::AnimationPlayer,
        ),
        With<Character>,
    >,
    mut child_query: Query<&mut Transform, With<CollisionBox>>,
) {
    for (children, player) in characters_query {
        let sequence = player.current_sequence();

        if let Some(collision_box) = &sequence.default_collision_box {
            for child in children.iter() {
                let transform = child_query.get_mut(child);
                if let Ok(mut transform) = transform {
                    transform.translation.x = collision_box.x as f32;
                    transform.translation.y = collision_box.y as f32;

                    commands.entity(child).remove::<Collider>();
                    commands.entity(child).insert(Collider::cuboid(
                        (collision_box.w / 2) as f32,
                        (collision_box.h / 2) as f32,
                    ));
                }
            }
        }
    }
}

pub fn run_state_commands(
    world: &mut World,
    query: &mut QueryState<(Entity, &mut Character, &CharacterState, &AnimationPlayer)>,
) {
    let mut all_commands = vec![];
    for (entity, character, state, animation) in query.iter(world) {
        let progress = animation.animation_frame;
        let commands = match &**state {
            fge_models::CharacterState::Standing => character.0.states.get(&"standing".into()),
            fge_models::CharacterState::Crouching => character.0.states.get(&"crouching".into()),
            fge_models::CharacterState::Airborne => character.0.states.get(&"airborne".into()),
            fge_models::CharacterState::Custom(state_id) => character.0.states.get(state_id),
        }
        .iter()
        .flat_map(|s| &s.commands)
        .filter(|command| {
            if let Some(frames) = &command.frames {
                match frames {
                    fge_models::NumberOrRange::Number(number) => *number == progress,
                    fge_models::NumberOrRange::Range(range) => {
                        range.from <= progress && progress < range.to
                    }
                }
            } else {
                true
            }
        })
        .cloned()
        .collect::<Vec<_>>();

        if !commands.is_empty() {
            all_commands.push((entity, commands));
        }
    }

    for (entity, commands) in all_commands {
        for command in commands {
            run_command(world, entity, &command);
        }
    }
}

pub fn run_command(world: &mut World, character: Entity, command: &fge_models::Command) {
    let context = ActionContext {
        character_entity: character,
    };

    match &command.action {
        fge_models::Action::SetState(character_state) => {
            world
                .run_system_once_with(set_state_cmd, (context, character_state))
                .unwrap();
        }
        fge_models::Action::SetAnimation(animation_id) => {
            world
                .run_system_once_with(set_animation_cmd, (context, animation_id))
                .unwrap();
        }
        fge_models::Action::SetControl(_, _) => todo!(),
        fge_models::Action::SetHitboxes(squares) => {
            world
                .run_system_once_with(set_hitboxes_cmd, (context, squares))
                .unwrap();
        }
    }
}

pub fn clear_hitboxes(
    mut commands: Commands,
    character_query: Query<(Entity, &Children), With<Character>>,
    hitboxes_query: Query<&Hitbox>,
) {
    for (character, children) in character_query {
        for child in children {
            let hitboxes = hitboxes_query.get(*child);

            if hitboxes.is_ok() {
                commands.entity(character).remove_child(*child);
                commands.entity(*child).despawn();
            }
        }
    }
}

pub fn set_hitboxes_cmd(
    (In(context), InRef(squares)): (In<ActionContext>, InRef<Vec<Square>>),
    mut commands: Commands,
    character_query: Query<Entity, With<Character>>,
) {
    // println!("Running set_hitboxes_cmd");
    for character in character_query {
        if character != context.character_entity {
            continue;
        }

        // Add hitboxes from `square`
        let hitboxes = squares.iter().map(|square| {
            (
                Hitbox {},
                Transform::from_xyz(square.x as f32, square.y as f32, 0.0),
                Collider::cuboid((square.w / 2) as f32, (square.h / 2) as f32),
                Sensor,
            )
        });

        for hitbox in hitboxes {
            let id = commands.spawn(hitbox).id();
            commands.entity(character).add_child(id);
        }
    }
}

pub fn set_animation_cmd(
    (In(context), InRef(animation_id)): (In<ActionContext>, InRef<AnimationID>),
    character_query: Query<
        (
            Entity,
            &mut crate::plugins::animation_player::components::AnimationPlayer,
        ),
        With<Character>,
    >,
) {
    // println!("Running set_animation_cmd");
    for (character, mut animation_player) in character_query {
        if character != context.character_entity {
            continue;
        }

        animation_player.set_animation(animation_id.clone());
    }
}

pub fn set_state_cmd(
    (In(context), InRef(character_state)): (In<ActionContext>, InRef<fge_models::CharacterState>),
    character_query: Query<(Entity, &mut CharacterState), With<Character>>,
) {
    // println!("Running set_animation_cmd");
    for (character, mut state) in character_query {
        if character != context.character_entity {
            continue;
        }

        // Set the new state
        state.0 = character_state.clone();
    }
}

pub fn movement(
    query: Query<(
        &Character,
        &mut CharacterState,
        &mut Transform,
        &InputHistory,
    )>,
) {
    for (_character, mut character_state, mut transform, input_history) in query {
        if input_history.just_pressed(Input::F) {
            character_state.0 = fge_models::CharacterState::Custom("walk_forward".into());
            transform.translation.x += 3.0;
        }

        if input_history.pressed(Input::F) {
            transform.translation.x += 3.0;
        }

        if input_history.just_pressed(Input::B) {
            character_state.0 = fge_models::CharacterState::Custom("walk_backward".into());
            transform.translation.x -= 3.0;
            continue;
        }

        if input_history.pressed(Input::B) {
            transform.translation.x -= 3.0;
            continue;
        }

        if input_history.just_released(Input::F) || input_history.just_released(Input::B) {
            character_state.0 = fge_models::CharacterState::Custom("standing".into());
            transform.translation.x -= 3.0;
            continue;
        }
    }
}

pub fn input_state_transition(query: Query<(&Character, &mut CharacterState, &InputHistory)>) {
    for (character, mut character_state, input_history) in query {
        let mut new_state = None;
        if let Some(state) = character.state(&character_state) {
            // Check all cancelable states
            for (state_id, cancel) in &state.cancels {
                let state = character
                    .0
                    .states
                    .get(state_id)
                    .expect("Could not find cancelable state");

                if let Some(input) = &state.input {
                    if let Ok(input) = Input::try_from(input) {
                        if input_history.just_pressed(input) {
                            new_state = Some(fge_models::CharacterState::Custom(state_id.clone()));
                        }
                    }
                }
            }
        }

        if let Some(new_state) = new_state {
            character_state.0 = new_state;
        }
    }
}
