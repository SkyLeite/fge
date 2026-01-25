use crate::prelude::*;
use bevy::ecs::system::RunSystemOnce as _;
use fge_models::CharacterID;

pub struct SpawnCharacter(pub CharacterID);

impl Command for SpawnCharacter {
    fn apply(self, world: &mut World) {
        world
            .run_system_once_with(super::systems::spawn, self.0)
            .expect("Failed to spawn character");
    }
}
