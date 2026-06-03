use bevy::prelude::*;

/// Standard density of seawater measured in kg/m^3.
/// Used as a foundational constant for hydrodynamic drag calculations.
const DENSITY_OF_WATER: f32 = 1025.0;

/// Represents a tracking vector for an incoming localized threat.
#[derive(Component, Debug, Clone)]
pub struct ThreatVector {
    /// Unique identifier for the threat tracking source.
    pub source_id: u32,
    /// Normalized 3D directional vector pointing directly toward the USV.
    pub direction: Vec3,
    /// Current range to the threat target measured in meters.
    pub distance: f32,
    /// Relative closing speed of the incoming threat measured in m/s.
    pub approach_velocity: f32,
    /// Threat evaluation coefficient scaled from 0.0 (low) to 1.0 (critical).
    pub severity: f32,
}

/// Tracks the real-time kinematic velocity of the USV hull.
#[derive(Component, Debug, Default)]
pub struct Velocity(pub Vec3);

/// Contains the fixed physical and hydrodynamic properties of the USV hull.
#[derive(Component, Debug)]
pub struct HullDynamics {
    /// Total mass of the Unmanned Surface Vehicle measured in kilograms.
    pub mass: f32,
    /// Baseline hydrodynamic drag area (Cd * A), representing the drag coefficient 
    /// multiplied by the vehicle's characteristic wetted cross-sectional area.
    pub baseline_drag_area: f32,
}

/// Governs the biomimetic tactical decision-making parameters inspired by cephalopod mechanics.
#[derive(Component, Debug)]
pub struct OctopodEvasionMatrix {
    /// Active tactical state of the biomimetic response system machine.
    pub current_mode: EvasionMode,
    /// Instantaneous peak thrust override applied to the hull during jetting (measured in Newtons).
    pub jet_propulsion_force: f32,
    /// Internal timer tracking the multispectral aerosol/smoke deployment window.
    pub ink_decoy_cooldown: f32,
    /// Dynamic hydrodynamic drag multiplier simulating body constriction/constriction advantages.
    /// Scales from 0.0 (theoretical zero drag) to 1.0 (nominal hull shape).
    pub body_morph_drag_coeff: f32,
}

/// Defines the specific state machine registers for the cephalopod-inspired survival logic.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EvasionMode {
    /// Nominal autonomous cruising; no immediate threat vectors breaching safety perimeters.
    Idle,
    /// High-transient, linear thrust escape maneuver executing maximum hydro-propulsion acceleration.
    JetPropulsion,
    /// Deployment of multispectral camouflage to break optical, infrared, and radar sensor tracking loops.
    InkCloudDecoy,
    /// Reactive pitch/roll stabilization leveraging wave geometry to maximize stealth and radar cross-section indices.
    HydroelasticSkim,
}

/// Bevy ECS system executing real-time threat analysis, biomimetic state transitions, 
/// and deterministic tactical evasion maneuvers fully integrated with fluid dynamics.
pub fn calculate_biomimetic_evasion_system(
    mut query: Query<(
        &ThreatVector,
        &mut OctopodEvasionMatrix,
        &HullDynamics,
        &mut Velocity,
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (threat, mut matrix, hull, mut velocity, mut transform) in query.iter_mut() {
        
        // 1. Tactical State Machine Resolution & Biomimetic Adjustments
        // Automate response if the threat breaches the 150m perimeter and maintains critical severity.
        let (propulsion_force_magnitude, active_mode) = if threat.distance < 150.0 && threat.severity > 0.8 {
            // JetPropulsion Mode: Cephalopods constrict their mantle to reduce surface area.
            // Dynamically drop the drag coefficient multiplier to optimize acceleration (e.g., 40% drag reduction).
            matrix.body_morph_drag_coeff = 0.6;
            (matrix.jet_propulsion_force, EvasionMode::JetPropulsion)
        } else {
            // Idle Mode: Return to nominal hydrodynamic geometry.
            matrix.body_morph_drag_coeff = 1.0;
            (0.0, EvasionMode::Idle)
        };

        matrix.current_mode = active_mode;

        // 2. Compute Propulsion Force Vector
        // Calculate the exact inverse vector relative to the incoming threat trajectory.
        let escape_direction = -threat.direction.normalize_or_zero();
        let propulsion_force = escape_direction * propulsion_force_magnitude;

        // 3. Fluid Dynamics: Hydrodynamic Drag Force Calculation
        // Standard fluid drag equation: F_drag = 0.5 * rho * v^2 * (Cd * A * morph_coefficient)
        let speed = velocity.0.length();
        let effective_drag_area = hull.baseline_drag_area * matrix.body_morph_drag_coeff;
        
        let drag_magnitude = 0.5 * DENSITY_OF_WATER * speed * speed * effective_drag_area;
        let drag_force = if speed > 0.0 {
            // Drag force always acts in the direct opposite direction of the current velocity vector.
            -velocity.0.normalize() * drag_magnitude
        } else {
            Vec3::ZERO
        };

        // 4. Newtonian Mechanics Integration (F_net = F_propulsion + F_drag = m * a)
        let net_force = propulsion_force + drag_force;
        let acceleration = net_force / hull.mass;

        // 5. Kinematic State Update (Euler-Cromer Integration)
        // Update velocity based on net physical acceleration.
        velocity.0 += acceleration * dt;
        
        // Translate the USV hull deterministically through the fluid space.
        transform.translation += velocity.0 * dt;
    }
}
