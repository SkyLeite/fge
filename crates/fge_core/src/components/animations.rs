use fge_models::AnimationID;
use std::collections::HashMap;

use crate::prelude::*;
use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Animations(HashMap<AnimationID, Sequence>);

impl Animations {
    pub fn insert(&mut self, id: AnimationID, sequence: Sequence) {
        self.0.insert(id, sequence);
    }

    pub fn get(&self, id: &AnimationID) -> Option<&Sequence> {
        self.0.get(id)
    }
}
