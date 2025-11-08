use bevy::prelude::*;

#[derive(Component)]
pub struct Health(u32);

#[derive(Component)]
pub struct Position(Vec2);

#[derive(Component)]
pub struct Animations();

#[derive(Bundle)]
pub struct CharacterBundle {
    health: Health,
    position: Position,
}
