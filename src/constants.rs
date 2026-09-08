use bevy::prelude::*;

// =========================================================================
// --- Static Physical Constants (Reference Standards) ---
// =========================================================================

/// Refractive Index (IOR) of pure water at 20°C.
/// Dual-typed to prevent runtime casting overhead in shader or physical loops.
pub const WATER_REFRACTIVE_INDEX: f32 = 1.333; 
pub const WATER_REFRACTIVE_INDEX_F64: f64 = 1.333; 

/// Standard atmospheric pressure (hPa). 
pub const SEA_LEVEL_PRESSURE: f32 = 1013.25;
pub const SEA_LEVEL_PRESSURE_F64: f64 = 1013.25;

/// Based on FVM analysis for steady flow and low drag.
pub const DRAG_COEFFICIENT: f32 = 0.04; 
pub const IDEAL_FLOW_VELOCITY: f32 = 5.0; 

// =========================================================================
// --- Hydrodynamic & Fluids Architecture Constants ---
// =========================================================================

/// Standard seawater density (kg/m^3) conforming to ITTC standard salinity & 15°C baseline.
pub const SEAWATER_DENSITY: f32 = 1025.0;
pub const SEAWATER_KINEMATIC_VISCOSITY: f32 = 0.00000119;

/// Standard acceleration due to gravity (m/s^2) per WGS 84 / ISO 80000-3 standard.
pub const GRAVITY: f32 = 9.81;

pub const ADDED_MASS_COEFFICIENT: f32 = 0.08;
pub const CRITICAL_REYNOLDS_NUMBER: f32 = 500_000.0;
pub const CRITICAL_FROUDE_NUMBER: f32 = 0.4;
pub const SKIN_FRICTION_COEFFICIENT: f32 = 0.0075;

/// Seawater Bulk Modulus at 15°C (Pa).
pub const SEAWATER_BULK_MODULUS: f32 = 2.34e9;
pub const SEAWATER_BULK_MODULUS_F64: f64 = 2.34e9;

// =========================================================================
// --- 6-DOF Hydrodynamic Coefficients (SNAME Notation) ---
// =========================================================================

// --- Added Mass Derivatives (kg for translational, kg*m^2 for rotational) ---
/// Surge added mass derivative: X_udot (kg)
pub const HYDRO_ADDED_MASS_X_UDOT: f32 = 12.5;
/// Sway added mass derivative: Y_vdot (kg)
pub const HYDRO_ADDED_MASS_Y_VDOT: f32 = 65.0;
/// Heave added mass derivative: Z_wdot (kg)
pub const HYDRO_ADDED_MASS_Z_WDOT: f32 = 110.0;
/// Roll added mass moment of inertia derivative: K_pdot (kg*m^2)
pub const HYDRO_ADDED_MASS_K_PDOT: f32 = 8.2;
/// Pitch added mass moment of inertia derivative: M_qdot (kg*m^2)
pub const HYDRO_ADDED_MASS_M_QDOT: f32 = 24.5;
/// Yaw added mass moment of inertia derivative: N_rdot (kg*m^2)
pub const HYDRO_ADDED_MASS_N_RDOT: f32 = 18.0;

// --- Linear Viscous Damping Coefficients ---
/// Surge linear damping: X_u (N / (m/s))
pub const HYDRO_DAMPING_LINEAR_SURGE: f32 = 45.0;
/// Sway linear damping: Y_v (N / (m/s))
pub const HYDRO_DAMPING_LINEAR_SWAY: f32 = 60.0;
/// Heave linear damping: Z_w (N / (m/s))
pub const HYDRO_DAMPING_LINEAR_HEAVE: f32 = 160.0;
/// Roll linear damping: K_p (N*m / (rad/s))
pub const HYDRO_DAMPING_LINEAR_ROLL: f32 = 28.0;
/// Pitch linear damping: M_q (N*m / (rad/s))
pub const HYDRO_DAMPING_LINEAR_PITCH: f32 = 35.0;
/// Yaw linear damping: N_r (N*m / (rad/s))
pub const HYDRO_DAMPING_LINEAR_YAW: f32 = 22.0;

// --- Quadratic (Cross-Flow / Non-linear) Damping Coefficients ---
/// Surge non-linear drag: X_u|u| (N / (m/s)^2)
pub const HYDRO_DAMPING_QUAD_SURGE: f32 = 24.0;
/// Sway non-linear drag: Y_v|v| (N / (m/s)^2)
pub const HYDRO_DAMPING_QUAD_SWAY: f32 = 135.0;
/// Yaw non-linear drag: N_r|r| (N*m / (rad/s)^2)
pub const HYDRO_DAMPING_QUAD_YAW: f32 = 38.0;

// =========================================================================
// --- Optical & Multispectral Attenuation Constants ---
// =========================================================================

pub const BLUE_ATTENUATION_COEFFICIENT: f32 = 0.015;
pub const RED_ATTENUATION_COEFFICIENT: f32 = 0.35;
pub const WATER_VERDET_CONSTANT: f32 = 0.0134; // rad / (T * m) at 589 nm

// =========================================================================
// --- Biomimetic Octopus-Evasion & Hydrofoil Constants ---
// =========================================================================

pub const BIOMIMETIC_BURST_MULTIPLIER: f32 = 3.5;
pub const THERMAL_DISSIPATION_RATE: f32 = 0.12;

// =========================================================================
// --- Simulation Presets & Enums ---
// =========================================================================

/// Explicitly layout as u8 to reduce memory footprint down to 1 byte.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OceanType {
    Aegean = 0,
    Caribbean = 1,
    Baltic = 2,
}

// =========================================================================
// --- Dynamic Simulation State ---
// =========================================================================

/// Cache-line optimized layout. Fields are ordered by alignment size (f32 -> enum)
/// to prevent memory padding gaps and maximize L1/L2 cache locality during ECS system queries.
#[repr(C)]
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct OceanSettings {
    // 1. Core Hydrodynamic Scalars (4 bytes each)
    pub wave_amplitude: f32,
    pub wave_frequency: f32,
    pub vessel_speed: f32,
    
    // 2. Physical/Chemical Parameters (4 bytes each)
    pub turbidity: f32,
    pub salinity: f32,
    pub temperature: f32,
    
    // 3. Multispectral Sensor Environment (4 bytes each)
    pub surface_lux: f32,
    pub current_depth: f32,
    pub temp_gradient: f32,

    // 4. Discriminant / Layout Boundary (1 byte)
    pub ocean_type: OceanType,
}

impl Default for OceanSettings {
    #[inline]
    fn default() -> Self {
        Self {
            wave_amplitude: 0.6,    
            wave_frequency: 1.0,   
            vessel_speed: 0.0,      
            turbidity: 0.08,        
            salinity: 38.5,        
            temperature: 18.0,     
            surface_lux: 100_000.0,  
            current_depth: 0.0,     
            temp_gradient: 0.0,
            ocean_type: OceanType::Aegean,
        }
    }
}

// =========================================================================
// --- Compile-Time Evaluation Helpers ---
// =========================================================================

impl OceanSettings {
    /// Pure compile-time or runtime calculation of baseline light extinction
    /// using a simplified Beer-Lambert approximation before shader pass.
    #[inline]
    pub const fn compute_static_attenuation(base_coef: f32, turbidity: f32) -> f32 {
        base_coef + turbidity
    }
}

// =========================================================================
// --- Global Maritime Standards & Simulation Protocols ---
// =========================================================================

pub const DNV_VV_COMPLIANCE_VERSION: &str = "2020-10";
pub const FMI_STANDARD_VERSION: &str = "3.0";
pub const IHO_S100_COMPLIANCE_BASELINE: u32 = 100;