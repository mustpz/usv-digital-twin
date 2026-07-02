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

pub const SEAWATER_DENSITY: f32 = 1025.9;
pub const SEAWATER_KINEMATIC_VISCOSITY: f32 = 0.00000119;
pub const GRAVITY: f32 = 9.80665;
pub const ADDED_MASS_COEFFICIENT: f32 = 0.08;
pub const CRITICAL_REYNOLDS_NUMBER: f32 = 500_000.0; // Added readability separator
pub const CRITICAL_FROUDE_NUMBER: f32 = 0.4;
pub const SKIN_FRICTION_COEFFICIENT: f32 = 0.0075;

/// Seawater Bulk Modulus at 15°C (Pa).
pub const SEAWATER_BULK_MODULUS: f32 = 2.34e9;
pub const SEAWATER_BULK_MODULUS_F64: f64 = 2.34e9;

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
    #[inline] // Hint to compiler to inline this allocation at initialization sites
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