use crate::prelude::*;

pub struct ActionContext<'a, T> {
    pub character_entity: Entity,
    // pub action: &'a fge_models::Action,
    pub data: &'a T,
}
