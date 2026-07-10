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
    camo_query: Query<(&MultispectralCamouflage, &Handle<StandardMaterial>), Changed<MultispectralCamouflage>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let flir_active = keyboard_input.pressed(KeyCode::KeyI);

    for (camo, material_handle) in camo_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            
            if flir_active {
                // FLIR Mode: Dynamic spectral shift between cold (Blue) and hot (Red) channels
                let heat = camo.infrared_signature;
                material.base_color = Color::rgba_linear(heat, 0.2, 1.0 - heat, camo.visible_reflectivity);
                material.emissive = Color::rgba_linear(heat * 0.5, 0.0, 0.0, 1.0);
                // Scatter specular highlights intensely during thermal bloom
                material.perceptual_roughness = 0.5; 
            } else {
                // Baseline Optical Active Camouflage State
                // Smoothly lower reflectivity and scale roughness to scatter specular reflections realistically
                material.base_color = Color::rgb(0.9, 0.9, 1.0); // Maintain Industrial Base Gray structural hull
                material.base_color.set_a(camo.visible_reflectivity);
                
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