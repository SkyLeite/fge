use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPrimaryContextPass},
    egui,
};

use crate::{
    plugins::{character::Character, input::InputHistory},
    prelude::*,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, debug_system);
    }
}

fn debug_system(
    mut contexts: EguiContexts,
    query: Query<(
        Entity,
        &Character,
        &InputHistory,
        &Transform,
    )>,
) -> Result {
    egui::Window::new("Debug").show(contexts.ctx_mut()?, |ui| {
        for (entity, _character, input_history, transform) in query {
            ui.collapsing(format!("Entity {}", entity.index()), |ui| {
                ui.collapsing("Position", |ui| {
                    ui.label(format!("X: {}", transform.translation.x));
                    ui.label(format!("Y: {}", transform.translation.y));
                });

                ui.collapsing("Input History", |ui| {
                    ui.vertical(|ui| {
                        let history = input_history.condensed().into_iter().take(16);
                        for (input, count) in history {
                            let input_str = format!("{} {}", **input, count);
                            let label = egui::widgets::Label::new(input_str);

                            ui.add(label);
                        }
                    });
                });
            });
        }
    });

    Ok(())
}
