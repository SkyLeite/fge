use crate::prelude::*;

mod components;

pub use components::InputHistory;
use fge_input::Inputs;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (populate_history).chain());
    }
}

pub fn populate_history(input: Res<ButtonInput<KeyCode>>, query: Query<&mut InputHistory>) {
    for mut input_history in query {
        let inputs = Inputs::from(input.as_ref());
        input_history.push(inputs);
    }
}
