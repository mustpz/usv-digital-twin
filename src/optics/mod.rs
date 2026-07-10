pub mod core;
pub mod render;

use bevy::prelude::*;

/// ============================================================================
/// MARITIME OPTICS & MULTISPECTRAL STEALTH ECOSYSTEM PLUGIN
/// ============================================================================

/// A modular Bevy `Plugin` designed to safely encapsulate, schedule, and execute
/// the entire multi-physical optics simulation and active/passive camouflage pipeline.
pub struct OpticsPlugin;

impl Plugin for OpticsPlugin {
    fn build(&self, app: &mut App) {
        // Explicitly organize the optics pipeline into clean, un-nested execution sets
        app.add_systems(
            Update,
            (
                // PHASE 1: Process real-time physics, microclimate refractions, and biomimetic states.
                core::update_biomimetic_camouflage,
                
                // PHASE 2: Feed the mutated state data directly into rendering / HUD outputs.
                // Chaining individual systems in a flat layout guarantees that Bevy's scheduler
                // enforces hard barriers between data mutations and GPU-adjacent asset updates.
                render::render_optical_camouflage_system,
                render::render_thermal_signature_system,
                render::draw_stealth_hud_system,
            )
                .chain(), // Absolute sequential enforcement: Phase 1 strictly executes BEFORE all Phase 2 systems.
        );
    }
}