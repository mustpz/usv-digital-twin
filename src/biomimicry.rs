use bevy::prelude::*;
use std::fmt;

const DENSITY_OF_WATER: f32 = 1025.0;
const DIVERGENT_ACCELERATION_LIMIT: f32 = 500.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlLoopError {
    InvalidThreatVector,
    DivergentAcceleration,
    ContextDriftDetected,
}

impl fmt::Display for ControlLoopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidThreatVector => write!(f, "Threat vector contains NaN or infinite values."),
            Self::DivergentAcceleration => write!(f, "Kinematic acceleration exceeded safety threshold."),
            Self::ContextDriftDetected => write!(f, "State machine drifted from deterministic constraints."),
        }
    }
}

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub enum EvasionMode {
    #[default]
    Idle,
    JetPropulsion,
    InkCloudDecoy,
    HydroelasticSkim,
    ColregHeadOnAlterCourseStarboard,
    ColregGiveWayCrossing,
}

#[derive(Component, Debug, Clone)]
pub struct ThreatVector {
    pub source_id: u32,
    pub direction: Vec3,
    pub distance: f32,
    pub approach_velocity: f32,
    pub severity: f32,
    pub target_heading: Vec3,
}

#[derive(Component, Debug, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug)]
pub struct HullDynamics {
    pub mass: f32,
    pub baseline_drag_area: f32,
}

#[derive(Component, Debug)]
pub struct OctopodEvasionMatrix {
    pub jet_propulsion_force: f32,
    pub ink_decoy_cooldown: f32,
    pub body_morph_drag_coeff: f32,
}

impl ThreatVector {
    #[inline(always)] // Micro-optimization for evaluation loops
    pub fn verify_integrity(&self) -> Result<(), ControlLoopError> {
        if self.direction.is_nan() || self.target_heading.is_nan() || self.distance <= 0.0 {
            return Err(ControlLoopError::InvalidThreatVector);
        }
        Ok(())
    }
}

pub fn calculate_biomimetic_evasion_system(
    mut query: Query<(
        &ThreatVector,
        &mut OctopodEvasionMatrix,
        &HullDynamics,
        &mut Velocity,
        &mut Transform,
    )>,
    current_state: Res<State<EvasionMode>>,
    mut next_state: ResMut<NextState<EvasionMode>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    if dt <= 0.0 { return; }

    let active_state = *current_state.get();

    for (threat, mut matrix, hull, mut velocity, mut transform) in query.iter_mut() {
        // --- STEP 1: VERIFICATION LAYER & CONSTRAINT ENFORCEMENT ---
        if let Err(err) = threat.verify_integrity() {
            error!(target: "agentic_harness::verification", "State validation failed: {}", err);
            if active_state != EvasionMode::Idle {
                next_state.set(EvasionMode::Idle);
            }
            continue;
        }

        let speed = velocity.0.length();
        let mut propulsion_force_magnitude = 0.0;
        let mut target_mode = EvasionMode::Idle;
        let mut escape_direction = Vec3::ZERO;

        // --- STEP 2: DETERMINISTIC CONTROL PATTERN RESOLUTION ---
        if threat.distance < 150.0 && threat.severity > 0.8 {
            // Optimize Vector Normalizations (Cache normalized state to avoid duplicate sqrt calculations)
            let threat_dir_norm = threat.direction.normalize_or_zero();
            let target_heading_norm = threat.target_heading.normalize_or_zero();
            let heading_dot = threat_dir_norm.dot(target_heading_norm);

            if heading_dot < -0.85 {
                // COLREG RULE 14: Head-on Situation (Alter course to starboard)
                target_mode = EvasionMode::ColregHeadOnAlterCourseStarboard;
                matrix.body_morph_drag_coeff = 0.85;
                propulsion_force_magnitude = matrix.jet_propulsion_force * 0.7;
                
                let forward_vector = -threat_dir_norm;
                escape_direction = Vec3::new(forward_vector.z, 0.0, -forward_vector.x).normalize_or_zero();
            } else if heading_dot.abs() <= 0.85 && threat.direction.x > 0.0 {
                // COLREG RULE 15 & 16: Crossing Situation (Give-Way Maneuver)
                target_mode = EvasionMode::ColregGiveWayCrossing;
                matrix.body_morph_drag_coeff = 0.90;
                propulsion_force_magnitude = matrix.jet_propulsion_force * 0.5;
                
                escape_direction = (-target_heading_norm + Vec3::X).normalize_or_zero();
            } else {
                // BIOMIMETIC TACTICAL OVERRIDE: Cephalopod Mantle Constriction (Jet Propulsion)
                target_mode = EvasionMode::JetPropulsion;
                matrix.body_morph_drag_coeff = 0.60;
                propulsion_force_magnitude = matrix.jet_propulsion_force;
                escape_direction = -threat_dir_norm;
            }
        } else {
            matrix.body_morph_drag_coeff = 1.0;
            target_mode = EvasionMode::Idle;
        }

        // --- STEP 3: REACTIVE STATE MACHINE SYNC ---
        if active_state != target_mode {
            info!(
                target: "agentic_harness::state_machine", 
                "Transitioning state registry from {:?} -> {:?}", 
                active_state, target_mode
            );
            next_state.set(target_mode);
        }

        // --- STEP 4: FLUID DYNAMICS & NEWTONIAN INTEGRATION ---
        let propulsion_force = escape_direction * propulsion_force_magnitude;
        let effective_drag_area = hull.baseline_drag_area * matrix.body_morph_drag_coeff;
        
        // Single calculation reuse for speed squared
        let drag_magnitude = 0.5 * DENSITY_OF_WATER * speed.powi(2) * effective_drag_area;
        
        let drag_force = if speed > 0.0 {
            -velocity.0.normalize_or_zero() * drag_magnitude
        } else {
            Vec3::ZERO
        };

        let net_force = propulsion_force + drag_force;
        let acceleration = net_force / hull.mass;

        // Guard against mathematical divergence during deep execution loops
        if acceleration.length() > DIVERGENT_ACCELERATION_LIMIT {
            warn!(target: "agentic_harness::diagnostics", "{}", ControlLoopError::DivergentAcceleration);
            velocity.0 *= 0.5; 
            continue;
        }

        // --- STEP 5: EULER-CROMER KINEMATIC INTEGRATION ---
        velocity.0 += acceleration * dt;
        transform.translation += velocity.0 * dt;
    }
}
