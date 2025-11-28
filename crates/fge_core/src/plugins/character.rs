use crate::prelude::*;
use bevy_rapier2d::prelude::*;

mod systems;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Character>()
            .add_systems(Startup, systems::spawn)
            .add_systems(
                FixedUpdate,
                (
                    systems::clear_hitboxes,
                    systems::update_position,
                    systems::set_collision_boxes,
                    systems::run_state_commands,
                )
                    .chain(),
            );
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub character_data: Character,
    pub health: crate::components::Health,
    pub position: crate::components::Position,
    pub animation_player: crate::plugins::animation_player::components::AnimationPlayer,
    pub state: crate::components::CharacterState,
}

#[derive(Component, Reflect)]
#[require(Position, RigidBody::Dynamic, Transform, GravityScale)]
pub struct Character(#[allow(unused)] fge_models::Character);
