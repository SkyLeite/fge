use std::ops::{Deref, DerefMut};

use crate::prelude::*;

#[derive(Reflect, Component, Default)]
pub struct CharacterState(pub fge_models::CharacterState);

impl Deref for CharacterState {
    type Target = fge_models::CharacterState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CharacterState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
