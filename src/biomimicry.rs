use bevy::prelude::*;

/// Standard density of seawater measured in kg/m^3.
/// Used as a foundational constant for hydrodynamic drag calculations.
const DENSITY_OF_WATER: f32 = 1025.0;

/// Represents a tracking vector for an incoming localized threat or obstacle.
/// Extended to comply with international maritime safety and COLREG regulations.
#[derive(Component, Debug, Clone)]
pub struct ThreatVector {
    /// Unique identifier for the threat tracking source (e.g., Radar, LiDAR, AIS tracking ID).
    pub source_id: u32,
    /// Normalized 3D directional vector pointing directly toward the USV hull.
    pub direction: Vec3,
    /// Current range to the threat target measured in meters.
    pub distance: f32,
    /// Relative closing speed of the incoming threat measured in m/s.
    pub approach_velocity: f32,
    /// Threat evaluation coefficient scaled from 0.0 (low) to 1.0 (critical).
    pub severity: f32,
    /// Normalized 3D heading vector representing the target vessel's true course through the fluid space.
    pub target_heading: Vec3,
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

/// Governs the tactical and regulatory decision-making parameters integrated with cephalopod mechanics.
#[derive(Component, Debug)]
pub struct OctopodEvasionMatrix {
    /// Active tactical state of the biomimetic and COLREG response system machine.
    pub current_mode: EvasionMode,
    /// Instantaneous peak thrust override applied to the hull during jetting (measured in Newtons).
    pub jet_propulsion_force: f32,
    /// Internal timer tracking the multispectral aerosol/smoke deployment window.
    pub ink_decoy_cooldown: f32,
    /// Dynamic hydrodynamic drag multiplier simulating body constriction advantages.
    /// Scales from 0.0 (theoretical zero drag) to 1.0 (nominal hull shape).
    pub body_morph_drag_coeff: f32,
}

/// Defines the specific state machine registers for survival and international maritime traffic laws.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EvasionMode {
    /// Nominal autonomous cruising; no immediate threat vectors breaching safety perimeters.
    Idle,
    /// High-transient, linear thrust escape maneuver executing maximum hydro-propulsion acceleration.
    JetPropulsion,
    /// Deployment of multispectral camouflage to break optical, infrared, and radar sensor tracking loops.
    InkCloudDecoy,
    /// Reactive pitch/roll stabilization leveraging wave geometry to maximize stealth indices.
    HydroelasticSkim,
    /// COLREG Rule 14 Violation Avoidance: Actively altering course to starboard (right) during a reciprocal head-on profile.
    ColregHeadOnAlterCourseStarboard,
    /// COLREG Rule 15 Give-Way Maneuver: Adjusting kinematics to pass astern of the crossing vessel on the starboard side.
    ColregGiveWayCrossing,
}

/// Bevy ECS system executing real-time threat analysis, biomimetic state transitions, 
/// and COLREG-compliant tactical maneuvers fully integrated with fluid dynamics.
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
        
        let speed = velocity.0.length();
        let mut propulsion_force_magnitude = 0.0;
        let mut active_mode = EvasionMode::Idle;
        let mut escape_direction = Vec3::ZERO;

        // -------------------------------------------------------------------------
        // 1. TACTICAL STATE MACHINE RESOLUTION & REGULATORY COMPLIANCE (COLREG)
        // -------------------------------------------------------------------------
        if threat.distance < 150.0 && threat.severity > 0.8 {
            
            // Vector dot-product evaluation to analyze the orientation profile of the incoming vessel
            let heading_dot = threat.direction.normalize_or_zero().dot(threat.target_heading.normalize_or_zero());

            if heading_dot < -0.85 {
                // COLREG RULE 14: Head-on Situation (Vessels approaching on nearly reciprocal courses)
                // Mandatory Action: Both vessels must alter course to starboard (right) to pass on the port side of each other.
                active_mode = EvasionMode::ColregHeadOnAlterCourseStarboard;
                matrix.body_morph_drag_coeff = 0.85; // Slight hydrodynamic adjustments for turning efficiency
                propulsion_force_magnitude = matrix.jet_propulsion_force * 0.7; // Regulated propulsion force for safe turning radius
                
                // Calculate a perpendicular safe vector bended 90 degrees to the right (Starboard Turn)
                let forward_vector = -threat.direction.normalize_or_zero();
                escape_direction = Vec3::new(forward_vector.z, 0.0, -forward_vector.x).normalize_or_zero();

            } else if heading_dot.abs() <= 0.85 && threat.direction.x > 0.0 {
                // COLREG RULE 15 & 16: Crossing Situation (Vessel approaching from the USV's starboard side)
                // Mandatory Action: The vessel which has the other on its own starboard side must keep out of the way (Give-Way).
                active_mode = EvasionMode::ColregGiveWayCrossing;
                matrix.body_morph_drag_coeff = 0.90;
                propulsion_force_magnitude = matrix.jet_propulsion_force * 0.5; // Reduce speed to safely give way or pass astern
                
                // Vector geometry optimized to steer clear astern of the target crossing path
                let away_from_target_path = -threat.target_heading.normalize_or_zero();
                escape_direction = (away_from_target_path + Vec3::new(1.0, 0.0, 0.0)).normalize_or_zero();

            } else {
                // BIOMIMETIC EVASION: Absolute tactical threat or missile/torpedo vector detected
                // Cephalopods constrict their mantle to minimize cross-sectional surface area.
                active_mode = EvasionMode::JetPropulsion;
                matrix.body_morph_drag_coeff = 0.6; // Maximize constriction (40% drag reduction)
                propulsion_force_magnitude = matrix.jet_propulsion_force;
                
                // Pure inverse tactical vector for maximum geometric separation
                escape_direction = -threat.direction.normalize_or_zero();
            }
        } else {
            // Idle Mode: Maintain nominal hydrodynamic geometry and baseline performance profiles
            matrix.body_morph_drag_coeff = 1.0;
            active_mode = EvasionMode::Idle;
            propulsion_force_magnitude = 0.0;
            escape_direction = Vec3::ZERO;
        }

        matrix.current_mode = active_mode;

        // -------------------------------------------------------------------------
        // 2. COMPUTE PROPULSION FORCE VECTOR
        // -------------------------------------------------------------------------
        let propulsion_force = escape_direction * propulsion_force_magnitude;

        // -------------------------------------------------------------------------
        // 3. FLUID DYNAMICS: HYDRODYNAMIC DRAG FORCE CALCULATION
        // -------------------------------------------------------------------------
        let effective_drag_area = hull.baseline_drag_area * matrix.body_morph_drag_coeff;
        let drag_magnitude = 0.5 * DENSITY_OF_WATER * speed * speed * effective_drag_area;
        
        let drag_force = if speed > 0.0 {
            // Drag force operates in direct vector opposition to the current velocity register
            -velocity.0.normalize() * drag_magnitude
        } else {
            Vec3::ZERO
        };

        // -------------------------------------------------------------------------
        // 4. NEWTONIAN MECHANICS INTEGRATION (F_net = F_propulsion + F_drag = m * a)
        // -------------------------------------------------------------------------
        let net_force = propulsion_force + drag_force;
        let acceleration = net_force / hull.mass;

        // -------------------------------------------------------------------------
        // 5. KINEMATIC STATE UPDATE (Euler-Cromer Integration)
        // -------------------------------------------------------------------------
        velocity.0 += acceleration * dt;
        transform.translation += velocity.0 * dt;
    }
}
