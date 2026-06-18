pub mod core;
pub mod render;

use bevy::prelude::*;

/// Bevy Plugin to package the entire optics and multispectral stealth pipeline safely
pub struct OpticsPlugin;

impl Plugin for OpticsPlugin {
    fn build(&self, app: &mut App) {
        app
            
            .add_systems(Update, core::update_biomimetic_camouflage)
            
            .add_systems(Update, (
                render::render_optical_camouflage_system,
                render::render_thermal_signature_system,
                render::draw_stealth_hud_system,
            ));
    }
}