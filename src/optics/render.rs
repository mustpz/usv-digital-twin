use bevy::prelude::*;
use crate::optics::core::MultispectralCamouflage;

/// ============================================================================
/// MARITIME OPTICS GRAPHICS & HARDWARE RENDER INTERFACE
/// ============================================================================

/// Unified Optics Render Engine.
/// Combines optical active camouflage and FLIR thermal imaging into a single pass
/// to prevent resource modification race conditions and eliminate redundant VRAM uploads.
pub fn render_optical_camouflage_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // Added track-state query to detect changes in stealth alpha, infrared, and sensor modes
    camo_query: Query<(&MultispectralCamouflage, &Handle<StandardMaterial>), Changed<MultispectralCamouflage>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Check if the thermal (IR) imaging channel is triggered by the operator
    let flir_active = keyboard_input.pressed(KeyCode::KeyI);

    for (camo, material_handle) in camo_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            
            if flir_active {
                // --- ACTIVE THERMAL (FLIR) IMAGING MODE ---
                // Map the camera_mode to 1.0 (Thermal Active)
                // We pass the parameters utilizing standard material properties as a pipeline bridge:
                // base_color.r -> mapped to engine heat signature
                // base_color.g -> mapped to system camera mode trigger (1.0 = Thermal)
                // base_color.b -> mapped to cold structural signature
                let heat = camo.infrared_signature;
                
                material.base_color = Color::rgba_linear(
                    heat,        // R: Thermal bloom (hot channel)
                    1.0,         // G: Camera mode override flag (1.0 = IR Camera Active)
                    1.0 - heat,  // B: Ambient/cold channel interpolation
                    camo.visible_reflectivity // A: Stealth alpha factor for blackbody suppression
                );
                
                material.emissive = Color::rgba_linear(heat * 0.5, 0.0, 0.0, 1.0);
                material.perceptual_roughness = 0.5; // Scatter specular highlights during thermal bloom
           } else {
                // --- NOMINAL OPTICAL ACTIVE CAMOUFLAGE MODE ---
                // Maintain Industrial Base Gray structural hull on visible light spectrum
                // G channel is set to 0.0 (Normal Visible Light mode flag)
                // A channel represents our adaptive visible reflectivity (stealth_alpha)
                material.base_color = Color::rgba_linear(
                    0.9,                       // R: Base gray
                    0.0,                       // G: Camera mode flag = 0.0 (Normal Visible Light)
                    1.0,                       // B: Base gray
                    camo.visible_reflectivity  // A: Stealth alpha factor
                );

                material.emissive = Color::rgba_linear(0.0, 0.0, 0.0, 0.0);
                material.perceptual_roughness = 0.1 + (1.0 - camo.visible_reflectivity) * 0.8;
            }
        }
    }
}

#[derive(Component)]
pub struct StealthHudLabel;

/// High-performance UI HUD System.
/// Safely mutates the Bevy text pipeline with absolute bounds verification to prevent runtime panics.
pub fn draw_stealth_hud_system(
    camo_query: Query<&MultispectralCamouflage>,
    mut text_query: Query<&mut Text, With<StealthHudLabel>>, 
) {
    if let Ok(camo) = camo_query.get_single() {
        let hud_string = format!(
            "STEALTH METRICS:\n\
             Optical Visibility: {:.1}%\n\
             Thermal Signature: {:.1}%\n\
             Radar Cross Section (RCS): {:.2} m²",
            camo.visible_reflectivity * 100.0,
            camo.infrared_signature * 100.0,
            camo.radar_cross_section
        );

        for mut text in text_query.iter_mut() {
            // BOUNDS VERIFICATION GUARD: Prevent out-of-bounds index panics at runtime
            if let Some(section) = text.sections.get_mut(0) {
                if section.value != hud_string {
                    section.value = hud_string.clone();
                }
            }
        }
    }
}