use bevy::prelude::*;
use crate::optics::core::MultispectralCamouflage;

/// ============================================================================
/// MARITIME OPTICS GRAPHICS & HARDWARE RENDER INTERFACE
/// ============================================================================

/// Dynamically modifies material transparency and roughness to simulate active camouflage.
/// Adjusts the alpha channel and perceptual roughness based on real-time environmental values.
pub fn render_optical_camouflage_system(
    camo_query: Query<(&MultispectralCamouflage, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (camo, material_handle) in camo_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            
            // Lower reflectivity increases roughness to scatter specular reflections realistically
            material.base_color.set_a(camo.visible_reflectivity);
            material.perceptual_roughness = 0.1 + (1.0 - camo.visible_reflectivity) * 0.8;
        }
    }
}

/// Simulates Forward-Looking Infrared (FLIR) thermal imaging pipelines.
/// Maps the calculated heat signature to a linear pseudocolor thermal spectrum when KeyI is held.
pub fn render_thermal_signature_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camo_query: Query<(&MultispectralCamouflage, &Handle<StandardMaterial>)>,
    mut mut_materials: ResMut<Assets<StandardMaterial>>, 
) {
    for (camo, material_handle) in camo_query.iter() {
        if let Some(material) = mut_materials.get_mut(material_handle) {
            let heat_intensity = camo.infrared_signature;

            if keyboard_input.pressed(KeyCode::KeyI) {
                // FLIR Mode: Interpolate color mapping dynamically between cold (Blue) and hot (Red) channels
                material.base_color = Color::rgba_linear(heat_intensity, 0.2, 1.0 - heat_intensity, camo.visible_reflectivity);
                material.emissive = Color::rgba_linear(heat_intensity * 0.5, 0.0, 0.0, 1.0);
            } else {
                // Return to Baseline Optical State when KeyI is released
                material.emissive = Color::rgba_linear(0.0, 0.0, 0.0, 0.0);
                // Note: Baseline color can also be handled via a custom user material component to prevent static overwrite
            }
        }
    }
}

#[derive(Component)]
pub struct StealthHudLabel;

/// High-performance UI HUD System that syncs and renders raw metrics into the Bevy text pipeline.
pub fn draw_stealth_hud_system(
    camo_query: Query<&MultispectralCamouflage>,
    mut text_query: Query<&mut Text, With<StealthHudLabel>>, 
) {
    // get_single() safely checks if there is exactly one monitored USV avatar entity in the scene origin
    if let Ok(camo) = camo_query.get_single() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!(
                "STEALTH METRICS:\n\
                 Optical Visibility: {:.1}%\n\
                 Thermal Signature: {:.1}%\n\
                 Radar Cross Section (RCS): {:.2} m²",
                camo.visible_reflectivity * 100.0,
                camo.infrared_signature * 100.0,
                camo.radar_cross_section
            );
        }
    }
}