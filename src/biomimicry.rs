use bevy::prelude::*;

/// Represents a tracking vector for an incoming localized threat.
#[derive(Component, Debug, Clone)]
pub struct ThreatVector {
    pub source_id: u32,               // Unique identifier for the threat tracking source 
    pub direction: Vec3,              // Normalized 3D directional vector pointing toward the USV
    pub distance: f32,                // Current range to the threat target measured in meters
    pub approach_velocity: f32,       // Relative closing speed of the incoming threat (m/s)
    pub severity: f32,                // Threat evaluation coefficient scaled from 0.0 (low) to 1.0 (critical)
}

/// Governs the biomimetic tactical decision-making parameters inspired by cephalopod mechanics.
#[derive(Component, Debug)]
pub struct OctopodEvasionMatrix {
    pub current_mode: EvasionMode,       // Active tactical state of the biomimetic response system
    pub jet_propulsion_force: f32,       // Instantaneous thrust override applied to the hull (measured in Newtons)
    pub ink_decoy_cooldown: f32,         // Internal timer tracking the multispectral aerosol/smoke deployment window
    pub body_morph_drag_coeff: f32,      // Dynamically adjusted hydrodynamic drag multiplier simulating body constriction
}

/// Defines the specific state machine registers for the cephalopod-inspired survival logic.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EvasionMode {
    Idle,             // Nominal autonomous cruising; no immediate threat vectors detected
    JetPropulsion,    // High-transient, linear thrust escape maneuver executing maximum acceleration
    InkCloudDecoy,    // Deployment of multispectral camouflage to break optical and radar sensor tracking loops
    HydroelasticSkim, // Reactive pitch/roll stabilization leveraging wave geometry to maximize stealth index
}

/// Bevy ECS system executing real-time threat analysis and deterministic tactical evasion maneuvers.
pub fn calculate_biomimetic_evasion_system(
    mut query: Query<(&ThreatVector, &mut OctopodEvasionMatrix, &mut Transform)>,
    time: Res<Time>,
) {
    for (threat, mut matrix, mut transform) in query.iter_mut() {
        // Trigger automated response if the threat breaches the perimeter and maintains critical severity
        if threat.distance < 150.0 && threat.severity > 0.8 {
            // Commit state machine to transient high-thrust propulsion mode
            matrix.current_mode = EvasionMode::JetPropulsion;
            
            // Compute the exact inverse vector relative to the incoming threat trajectory
            let escape_direction = -threat.direction.normalize();
            
            // Execute deterministic micro-maneuver across the procedural wave surface
            transform.translation += escape_direction * matrix.jet_propulsion_force * time.delta_seconds();
        } else {
            // Reset to default autonomous pathing state when perimeter clear criteria are met
            matrix.current_mode = EvasionMode::Idle;
        }
    }
}
