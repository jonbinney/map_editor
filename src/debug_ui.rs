use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::RapierConfiguration;

pub struct DebugUIPlugin;
impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(debug_ui_window);
        app.add_system(simulation_control_window);
    }
}

fn debug_ui_window(mut egui_context: ResMut<EguiContext>, diagnostics: Res<Diagnostics>) {
    egui::Window::new("Statistics").show(egui_context.ctx_mut(), |ui| {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }
        ui.label(format!("FPS: {:3.1}", fps));
    });
}

fn simulation_control_window(
    mut egui_context: ResMut<EguiContext>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    egui::Window::new("Simulation Control").show(egui_context.ctx_mut(), |ui| {
        if rapier_config.physics_pipeline_active {
            if ui.button("Pause").clicked() {
                println!("Pausing physics");
                rapier_config.physics_pipeline_active = false;
            }
        } else {
            if ui.button("Unpause").clicked() {
                println!("Unpausing physics");
                rapier_config.physics_pipeline_active = true;
            }
        }
    });
}
