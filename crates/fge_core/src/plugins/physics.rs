use crate::{plugins::character::Character, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            // .add_systems(FixedUpdate, add_collider_to_characters)
            .add_systems(Startup, create_floor);
    }
}

fn create_floor(mut commands: Commands) {
    commands.spawn((
        Floor,
        Collider::cuboid(500.50, 10.0),
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));
}
