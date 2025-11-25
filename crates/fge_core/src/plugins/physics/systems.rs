use crate::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn create_floor(mut commands: Commands) {
    commands.spawn((
        Floor,
        Collider::cuboid(500.50, 10.0),
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));
}
