use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPrimaryContextPass},
    egui,
};

use crate::{plugins::input::InputHistory, prelude::*};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, debug_system);
    }
}

fn debug_system(mut contexts: EguiContexts, query: Query<(Entity, &InputHistory)>) -> Result {
    egui::Window::new("Debug").show(contexts.ctx_mut()?, |ui| {
        for (entity, input_history) in query {
            ui.collapsing(format!("Entity {}", entity.index()), |ui| {
                ui.horizontal(|ui| {
                    for input in &input_history.history {
                        let input_str = input.to_string();
                        if input_str == "<empty>" {
                            ui.label("5");
                        } else {
                            ui.label(input_str);
                        }
                    }
                });
            });
        }
    });

    Ok(())
}
