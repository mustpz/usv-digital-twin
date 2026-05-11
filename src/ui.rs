use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::constants::{OceanSettings, OceanType};
use crate::optics::calculate_visibility_range;

pub fn update_ui_system(
    mut contexts: EguiContexts,
    mut settings: ResMut<OceanSettings>,
) {
    egui::Window::new("Operational Command Center")
        .default_open(true)
        .resizable(true)
        .show(contexts.ctx_mut(), |ui: &mut egui::Ui| {
            
            ui.heading("Geographical & Spectral Presets");
            ui.label("Select operational environment to apply specific spectral attenuation profiles.");
            
ui.horizontal(|ui| {
    if ui.selectable_label(settings.ocean_type == OceanType::Aegean, "Aegean Sea (Med)").clicked() {
        settings.ocean_type = OceanType::Aegean;
    }
    if ui.selectable_label(settings.ocean_type == OceanType::Caribbean, "Caribbean Basin").clicked() {
        settings.ocean_type = OceanType::Caribbean;
    }
});
            
            ui.separator();

            ui.heading("Hydrodynamic Sea State");
            ui.label("Procedural Wave Generation Parameters.");
            
            // Adjusts the vertical displacement of the procedural Gerstner/Sine waves
            ui.add(egui::Slider::new(&mut settings.wave_amplitude, 0.0..=2.5)
                .text("Wave Amplitude (m)"));

            // Temporal frequency influencing the phase shift of surface dynamics
            ui.add(egui::Slider::new(&mut settings.wave_frequency, 0.1..=3.0)
                .text("Surface Frequency (Hz)"));

            ui.add_space(10.0);

            ui.heading("Optical & Volumetric Properties");
            ui.label("Light extinction parameters based on the Beer-Lambert Law.");
            ui.separator();

            // Turbidity directly scales the multispectral absorption coefficients
            ui.add(egui::Slider::new(&mut settings.turbidity, 0.0..=0.3)
                .text("Turbidity Coefficient (κ)"));

            // Display calculated visibility range (Secchi Depth)
            let visibility = calculate_visibility_range(settings.turbidity);
            ui.colored_label(egui::Color32::from_rgb(0, 191, 255), 
                format!("Calculated Visual Range: {:.2} m", visibility));
            
            ui.add_space(5.0);

            // Thermal state influencing refractive index (IOR) calculations
            ui.add(egui::Slider::new(&mut settings.temperature, 0.0..=40.0)
                .text("Surface Temperature (°C)"));
            
            // Salinity control: Critical for high-precision Snell refraction
            ui.add(egui::Slider::new(&mut settings.salinity, 30.0..=45.0)
                .text("Salinity (PSU)"));

            ui.add_space(10.0);

            ui.heading("Platform Navigation");
            ui.label("USV Kinematics for Optical Flow Simulation.");
            ui.separator();

            // Real-time vessel velocity for wake and camouflaging analysis
            ui.add(egui::Slider::new(&mut settings.vessel_speed, 0.0..=30.0)
                .text("Velocity (m/s)"));

            ui.add_space(15.0);
            
            // Global State Reset
            ui.vertical_centered(|ui: &mut egui::Ui| {
                if ui.button("Reset Simulation to Standard Physical Baseline").clicked() {
                    *settings = OceanSettings::default();
                }
            });

            ui.add_space(5.0);
            ui.label(egui::RichText::new("System Status: Operational").small().color(egui::Color32::GREEN));
        });
}