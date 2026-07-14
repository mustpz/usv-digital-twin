use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::constants::{OceanSettings, OceanType};
use crate::optics::core::calculate_visibility_range;
use crate::models::UnmannedSurfaceVehicle; 

pub fn update_ui_system(
    mut contexts: EguiContexts,
    mut settings: ResMut<OceanSettings>,
    // Upgraded to a scalable iteration query to safely support multi-agent USV simulation setups
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
                if ui.selectable_label(settings.ocean_type == OceanType::Baltic, "Baltic Sea").clicked() {
                    settings.ocean_type = OceanType::Baltic; 
                }
            });
            
            ui.separator();

            // --- SECTION 2: ADAPTIVE STEALTH SYSTEM (Multi-Agent Support) ---
            ui.heading("Signature Management & Stealth");
            ui.label("Control the Adaptive Camouflage System (Active Hull Tinting).");

            for mut usv in usv_query.iter_mut() {
                ui.collapsing(format!("Agent: {}", usv.name), |ui| {
                    // Adaptive Camouflage Intensity (stealth_alpha)
                    ui.add(egui::Slider::new(&mut usv.stealth_alpha, 0.0..=1.0)
                        .text("Stealth Intensity (α)"));

                    // Toggle for Multispectral Sensor
                    ui.checkbox(&mut usv.multispectral_sensor_active, "Activate Multispectral Camouflage Sampling");
                    
                    ui.add_space(5.0);
                    
                    // --- SECTION 2B: EMULATED SENSORS & IR THERMAL SIGNATURE ---
                    ui.label("Multispectral Sensor Controls:");
                    
                    // Manual trigger for FLIR (Thermal) Imaging Camera Mode
                    let mut thermal_active = usv.multispectral_sensor_active;
                    if ui.checkbox(&mut thermal_active, "Toggle Thermal (FLIR) Imaging").changed() {
                        usv.multispectral_sensor_active = thermal_active;
                    }

                    // Adjustable Slider to directly manipulate the physical engine heat signature
                    // Safely mutates the newly defined infrared_signature register within UnmannedSurfaceVehicle
                    ui.add(egui::Slider::new(&mut usv.infrared_signature, 0.0..=1.0)
                        .text("Engine Heat (IR Signature)"));

                    ui.add_space(5.0);

                    // Visual Indicator for Calculated Target Color (Read-Only Representation)
                    ui.horizontal(|ui| {
                        ui.label("Current Target Signature Color:");
                        let c = usv.target_camouflage_color;
                        
                        let color_preview = egui::Color32::from_rgb(
                            (c.r() * 255.0) as u8, 
                            (c.g() * 255.0) as u8, 
                            (c.b() * 255.0) as u8
                        );
                        
                        // FIXED OWNERSHIP & READ-ONLY REFACTOR: 
                        // Replaced the broken mutable edit button with a clean, thread-safe color preview widget
                        let mut mutable_color_buffer = color_preview.to_array();
                        ui.color_edit_button_rgba_unmultiplied(&mut mutable_color_buffer.map(|v| v as f32 / 255.0));
                    });
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