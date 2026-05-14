use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::constants::{OceanSettings, OceanType};
use crate::optics::calculate_visibility_range;
// Importing the USV model to access stealth parameters
use crate::models::UnmannedSurfaceVehicle; 

pub fn update_ui_system(
    mut contexts: EguiContexts,
    mut settings: ResMut<OceanSettings>,
    // We query the USV to control its internal stealth state via UI
    mut usv_query: Query<&mut UnmannedSurfaceVehicle>, 
) {
    egui::Window::new("Operational Command Center")
        .default_open(true)
        .resizable(true)
        .show(contexts.ctx_mut(), |ui: &mut egui::Ui| {
            
            // --- SECTION 1: ENVIRONMENTAL PRESETS ---
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

            // --- SECTION 2: ADAPTIVE STEALTH SYSTEM (NEW) ---
            ui.heading("Signature Management & Stealth");
            ui.label("Control the Adaptive Camouflage System (Active Hull Tinting).");

            if let Ok(mut usv) = usv_query.get_single_mut() {
                // Adaptive Camouflage Intensity (stealth_alpha)
                // This slider allows manual override of the visual signature blending
                ui.add(egui::Slider::new(&mut usv.stealth_alpha, 0.0..=1.0)
                    .text("Stealth Intensity (α)"));

                // Toggle for Multispectral Sensor (Intelligence gathering)
                ui.checkbox(&mut usv.multispectral_sensor_active, "Activate Multispectral Camouflage Sampling");
                
                // Visual Indicator for Calculated Target Color
                ui.horizontal(|ui| {
                    ui.label("Current Target Signature Color:");
                    let c = usv.target_camouflage_color;
                    // Displaying a small color box to show the USV's "thought process"
                    let color_preview = egui::Color32::from_rgb(
                        (c.r() * 255.0) as u8, 
                        (c.g() * 255.0) as u8, 
                        (c.b() * 255.0) as u8
                    );
                    ui.color_edit_button_srgba(&mut color_preview.into());
                });
            }

            ui.separator();

            // --- SECTION 3: HYDRODYNAMICS ---
            ui.heading("Hydrodynamic Sea State");
            ui.add(egui::Slider::new(&mut settings.wave_amplitude, 0.0..=2.5)
                .text("Wave Amplitude (m)"));
            ui.add(egui::Slider::new(&mut settings.wave_frequency, 0.1..=3.0)
                .text("Surface Frequency (Hz)"));

            ui.add_space(10.0);

            // --- SECTION 4: OPTICAL PROPERTIES ---
            ui.heading("Optical & Volumetric Properties");
            ui.add(egui::Slider::new(&mut settings.turbidity, 0.0..=0.3)
                .text("Turbidity Coefficient (κ)"));

            let visibility = calculate_visibility_range(settings.turbidity);
            ui.colored_label(egui::Color32::from_rgb(0, 191, 255), 
                format!("Calculated Visual Range: {:.2} m", visibility));

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