use std::path::Path;

use crate::action_context::ActionContext;
use crate::prelude::*;
use crate::sequence::Sequence;
use bevy::ecs::system::RunSystemOnce;
use bevy_rapier2d::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use fge_models::{Action, Square};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(FixedUpdate, (set_collision_boxes, update_position));
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub character_data: Character,
    pub health: crate::components::Health,
    pub position: crate::components::Position,
    pub animation_player: crate::plugins::animation_player::AnimationPlayer,
    pub state: crate::components::CharacterState,
}

#[derive(Component)]
#[require(Position, RigidBody::Dynamic, Transform, GravityScale)]
pub struct Character(#[allow(unused)] fge_models::Character);

pub fn spawn(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let character = fge_models::serde::from_file(Path::new("./data/character.lua")).unwrap();
    let mut animation_atlas = Animations::default();
    let mut spritesheets = Spritesheets::default();

    for (sheet_id, sheet) in &character.spritesheets {
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
                        default_collision_box: sprite_animation.default_collision_box.clone(),
                        frames: sprite_animation.frames.clone(),
                    };
                    animation_atlas.insert(id.clone(), sequence);
                }
                fge_models::Animation::Model(_model_animation) => todo!(),
            }
        }

        let sprite = spritesheet
            .with_size_hint(sheet.width as u32, sheet.height as u32)
            .sprite(&mut atlas_layouts);

        spritesheets.insert(sheet_id.clone(), sprite);
    }

    let animation_player =
        crate::plugins::animation_player::AnimationPlayer::new(animation_atlas, spritesheets);

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
            &crate::plugins::animation_player::AnimationPlayer,
            &SpritesheetAnimation,
        ),
        With<Character>,
    >,
    mut child_query: Query<&mut Transform, With<CollisionBox>>,
) {
    for (children, player, animation) in characters_query {
        let _progress = animation.progress;

        let sequence = player.current_sequence();

        if let Some(collision_box) = &sequence.default_collision_box {
            for child in children.iter() {
                let mut transform = child_query.get_mut(child).unwrap();
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

pub fn run_state_commands(
    world: &mut World,
    query: Query<(Entity, &mut Character, &CharacterState)>,
) {
    for (entity, character, state) in query.iter() {
        let commands = match &**state {
            fge_models::CharacterState::Standing => character.0.states.get(&"standing".into()),
            fge_models::CharacterState::Crouching => character.0.states.get(&"crouching".into()),
            fge_models::CharacterState::Airborne => character.0.states.get(&"airborne".into()),
            fge_models::CharacterState::Custom(state_id) => character.0.states.get(state_id),
        }
        .map(|s| &s.commands);

        if let Some(commands) = commands {
            for command in commands {
                // TODO: evaluate condition

                run_command(world, entity, command);
                // world.run_system_once_with(run_command, context).unwrap();
            }
        }
    }
}

pub fn set_hitboxes_cmd(In(context): In<ActionContext<'_, Vec<Square>>>) {}

pub fn run_command(world: &mut World, character: Entity, command: &fge_models::Command) {
    match &command.action {
        fge_models::Action::SetState(character_state) => todo!(),
        fge_models::Action::SetAnimation(animation_id) => todo!(),
        fge_models::Action::SetControl(_, _) => todo!(),
        fge_models::Action::SetHitboxes(squares) => {
            let context = ActionContext {
                character_entity: character,
                data: squares,
            };
            world.run_system_once_with(set_hitboxes_cmd, context);
        }
    }
}

pub fn set_hitboxes(
    mut commands: Commands,
    characters_query: Query<
        (
            &Children,
            &crate::plugins::animation_player::AnimationPlayer,
            &SpritesheetAnimation,
        ),
        With<Character>,
    >,
    mut child_query: Query<&mut Transform, With<CollisionBox>>,
) {
    for (children, player, animation) in characters_query {
        let _progress = animation.progress;

        let sequence = player.current_sequence();

        if let Some(collision_box) = &sequence.default_collision_box {
            for child in children.iter() {
                let mut transform = child_query.get_mut(child).unwrap();
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
