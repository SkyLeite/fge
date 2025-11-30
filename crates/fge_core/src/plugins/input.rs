use crate::prelude::*;

mod components;

pub use components::{Input, InputHistory};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, populate_history);
    }
}

pub fn populate_history(input: Res<ButtonInput<KeyCode>>, query: Query<&mut InputHistory>) {
    for mut input_history in query {
        input_history.push(Input::to_bitflags(input.as_ref()));
    }
}
