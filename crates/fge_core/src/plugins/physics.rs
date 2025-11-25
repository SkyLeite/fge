use crate::prelude::*;
use bevy_rapier2d::prelude::*;

mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, systems::create_floor);
    }
}
