use std::collections::HashMap;

use crate::prelude::*;
use fge_models::SpritesheetID;

#[derive(Reflect, Default, Component)]
pub struct Spritesheets(HashMap<SpritesheetID, LoadedSpritesheet>);

#[derive(Reflect)]
pub struct LoadedSpritesheet {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl Spritesheets {
    pub fn insert(&mut self, id: SpritesheetID, sprite: LoadedSpritesheet) {
        self.0.insert(id, sprite);
    }

    pub fn get(&self, id: &SpritesheetID) -> Option<&LoadedSpritesheet> {
        self.0.get(id)
    }

    pub fn first(&self) -> Option<&LoadedSpritesheet> {
        self.0.values().next()
    }
}
