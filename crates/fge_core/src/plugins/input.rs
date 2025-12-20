use crate::prelude::*;

mod components;

pub use components::InputHistory;
use fge_input::Inputs;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (populate_history, debug_history).chain());
    }
}

pub fn populate_history(input: Res<ButtonInput<KeyCode>>, query: Query<&mut InputHistory>) {
    for mut input_history in query {
        let inputs = Inputs::from(input.as_ref());
        input_history.push(inputs);
    }
}

pub fn debug_history(query: Query<&InputHistory>) {
    for input_history in query {
        let mut dbg_str = String::new();
        for input in &input_history.history {
            dbg_str.push_str("[");
            if input.to_string() == "<empty>" {
                dbg_str.push_str("5");
            } else {
                dbg_str.push_str(&input.to_string());
            }

            dbg_str.push_str("]");
        }
        // println!("{}", dbg_str);
    }
}
