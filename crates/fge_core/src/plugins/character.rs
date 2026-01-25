use crate::{plugins::input::InputHistory, prelude::*};
use bevy_rapier2d::prelude::*;

mod systems;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Character>()
            .register_type::<CharacterState>()
            .add_systems(Startup, systems::spawn)
            .add_systems(
                FixedUpdate,
                (
                    systems::clear_hitboxes,
                    systems::input_state_transition,
                    systems::update_position,
                    systems::set_collision_boxes,
                    systems::run_state_commands,
                )
                    .chain()
                    .in_set(system_sets::Gameplay),
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
#[require(Position, RigidBody::Dynamic, Transform, GravityScale, InputHistory)]
pub struct Character(#[allow(unused)] fge_models::Character);

impl Character {
    pub fn state(
        &self,
        character_state: &fge_models::CharacterState,
    ) -> Option<&fge_models::State> {
        match character_state {
            fge_models::CharacterState::Standing => self.0.states.get(&"standing".into()),
            fge_models::CharacterState::Crouching => self.0.states.get(&"crouching".into()),
            fge_models::CharacterState::Airborne => self.0.states.get(&"airborne".into()),
            fge_models::CharacterState::Custom(state_id) => self.0.states.get(state_id),
        }
    }
}
