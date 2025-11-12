use std::collections::HashMap;

use crate::prelude::*;
use fge_models::SpritesheetID;

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
