use std::collections::HashMap;

use fge_models::{Character, CharacterID};

use crate::prelude::*;

#[derive(Component)]
pub struct CharacterList(pub HashMap<CharacterID, Character>);
