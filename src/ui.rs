use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::constants::OceanSettings;
use crate::optics::calculate_secchi_depth; 

/// System responsible for rendering the simulation control panel.
/// It allows real-time manipulation of physical and optical ocean parameters.
pub fn update_ui_system(
    mut contexts: EguiContexts,
    mut settings: ResMut<OceanSettings>,
) {
    // Creating a window labeled "Control Panel"
    egui::Window::new("Simulation Control Panel")
        .default_open(true)
        .show(contexts.ctx_mut(), |ui: &mut egui::Ui| {
            
            ui.heading("Sea State & Hydrodynamics");
            ui.label("Modify the physical behavior of the ocean surface.");
            ui.separator();

            // Adjusts the vertical displacement of the water mesh
            ui.add(egui::Slider::new(&mut settings.wave_amplitude, 0.0..=2.5)
                .text("Wave Height (Amplitude)"));

            // Sets the temporal frequency of the orbital oscillation
            ui.add(egui::Slider::new(&mut settings.wave_frequency, 0.1..=3.0)
                .text("Sea State Frequency"));

            ui.add_space(10.0);

            ui.heading("Optical Properties");
            ui.label("Control how light interacts with the water volume.");
            ui.separator();

            // Directly influences the Beer-Lambert attenuation coefficient
            ui.add(egui::Slider::new(&mut settings.turbidity, 0.0..=0.2)
                .text("Water Turbidity (m⁻¹)"));

            // Dynamic scientific feedback: Display Secchi Depth based on turbidity
            let visibility = calculate_secchi_depth(settings.turbidity);
            ui.colored_label(egui::Color32::from_rgb(0, 191, 255), 
                format!("Estimated Secchi Depth: {:.2} m", visibility));
            
            ui.add_space(5.0);

            // Thermal state affecting both density and optical refraction
            ui.add(egui::Slider::new(&mut settings.temperature, 0.0..=40.0)
                .text("Surface Temperature (°C)"));

            ui.add_space(10.0);

            ui.heading("Navigation Dynamics");
            ui.label("Parameters for the Unmanned Surface Vehicle (USV).");
            ui.separator();

            // Forward velocity of the platform
            ui.add(egui::Slider::new(&mut settings.vessel_speed, 0.0..=20.0)
                .text("Vessel Speed (m/s)"));

            ui.add_space(15.0);
            
            // Interaction footer with reset functionality
            ui.vertical_centered(|ui: &mut egui::Ui| {
                if ui.button("Restore Default Physical Values").clicked() {
                    *settings = OceanSettings::default();
                }
            });
        });
}