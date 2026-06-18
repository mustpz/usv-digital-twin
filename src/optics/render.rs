use bevy::prelude::*;
use crate::optics::core::MultispectralCamouflage;

/// Visualizes the optical active camouflage based on reflectivity factors.
pub fn render_optical_camouflage_system(
    camo_query: Query<(&MultispectralCamouflage, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (camo, material_handle) in camo_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color.set_a(camo.visible_reflectivity);
            material.perceptual_roughness = 0.1 + (1.0 - camo.visible_reflectivity) * 0.8;
        }
    }
}

/// Simulates FLIR (Forward-Looking Infrared) thermal imaging when KeyI is triggered.
pub fn render_thermal_signature_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camo_query: Query<(&MultispectralCamouflage, &Handle<StandardMaterial>)>,
    mut mut_materials: ResMut<Assets<StandardMaterial>>, 
) {
    if keyboard_input.pressed(KeyCode::KeyI) {
        
        for (camo, material_handle) in camo_query.iter() {
            
            if let Some(material) = mut_materials.get_mut(material_handle) {
                
                let heat_intensity = camo.infrared_signature;
                
                
                material.base_color = Color::rgb(heat_intensity, 0.2, 1.0 - heat_intensity);
                material.emissive = Color::rgb(heat_intensity * 0.5, 0.0, 0.0);
            }
        }
    }
}

#[derive(Component)]
pub struct StealthHudLabel;

/// UI HUD System to render raw stealth values on screen.
pub fn draw_stealth_hud_system(
    camo_query: Query<&MultispectralCamouflage>,
    mut text_query: Query<&mut Text, With<StealthHudLabel>>, 
) {
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