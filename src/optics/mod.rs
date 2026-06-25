pub mod core;
pub mod render;

use bevy::prelude::*;

/// ============================================================================
/// MARITIME OPTICS & MULTISPECTRAL STEALTH ECOSYSTEM PLUGIN
/// ============================================================================

/// A modular Bevy `Plugin` designed to safely encapsulate, schedule, and execute
/// the entire multi-physical optics simulation and active/passive camouflage pipeline.
///
/// # Architectural Design & Scheduling:
/// * **Strict Deterministic Execution (`.chain()`)**: Forces a hard sequential boundary 
///   between data generation (physics/biomimicry) and data consumption (rendering/UI). 
/// * **Prevention of 1-Frame Lag Anomaly**: By chaining systems, we ensure that the 
///   GPU-adjacent render passes read the *current* frame's calculated camouflage states,
///   completely eliminating visual stuttering or micro-interpolation delays on modern GPUs.
/// * **Decoupled Architecture**: Keeps the primary simulation driver (`main.rs`) clean by
///   abstracting the internal systems under a single, atomic registration method.
pub struct OpticsPlugin;

impl Plugin for OpticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // STEP 1: Process real-time physics, microclimate refractions, and biomimetic states.
                core::update_biomimetic_camouflage,
                
                // STEP 2: Feed the mutated state data directly into the hardware-adjacent rendering/HUD systems.
                (
                    render::render_optical_camouflage_system,
                    render::render_thermal_signature_system,
                    render::draw_stealth_hud_system,
                ),
            )
                .chain(), // Explicitly chains Step 1 -> Step 2 to guarantee absolute temporal synchronization.
        );
    }
}