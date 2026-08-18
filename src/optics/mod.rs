pub mod core;
pub mod render;

use bevy::prelude::*;

/// ============================================================================
/// MARITIME OPTICS & MULTISPECTRAL STEALTH ECOSYSTEM PLUGIN
/// ============================================================================

pub struct OpticsPlugin;

impl Plugin for OpticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // PHASE 1: Process real-time physics, microclimate refractions, and biomimetic states.
                core::update_biomimetic_camouflage,
                
                // PHASE 2: Feed the mutated state data directly into rendering / HUD outputs.
                render::render_optical_camouflage_system,
                render::draw_stealth_hud_system,
            )
                .chain(),
        );
    }
}