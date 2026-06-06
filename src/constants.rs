use bevy::prelude::*;

// =========================================================================
// --- Static Physical Constants (Reference Standards) ---
// =========================================================================

/// Refractive Index (IOR) of pure water at 20°C.
/// Used as the baseline for dynamic salinity/temperature corrections.
pub const WATER_REFRACTIVE_INDEX: f64 = 1.333; 

/// Standard atmospheric pressure (hPa). 
/// Essential for future implementations of surface wave-air interface physics.
pub const SEA_LEVEL_PRESSURE: f64 = 1013.25;

/// Based on FVM analysis for steady flow and low drag.
pub const DRAG_COEFFICIENT: f32 = 0.04; 
pub const IDEAL_FLOW_VELOCITY: f32 = 5.0; 

// =========================================================================
// --- Hydrodynamic & Fluids Architecture Constants ---
// =========================================================================

/// Standard density of seawater at 15°C and 35 PSU salinity (kg/m^3).
/// Critical for buoyancy, hydrodynamic drag vector calculations, and displacement.
pub const SEAWATER_DENSITY: f32 = 1025.9;

/// Kinematic viscosity of seawater at 15°C (m^2/s).
/// Used in Reynolds number evaluation to scale turbulent boundary layer effects.
pub const SEAWATER_KINEMATIC_VISCOSITY: f32 = 0.00000119;

/// Gravitational acceleration constant (m/s^2).
/// Governs Froude number scaling and Gerstner wave dispersion relations.
pub const GRAVITY: f32 = 9.80665;

/// Added Mass Coefficient for a slender/biomimetic USV hull configuration.
/// Accounted for in transient acceleration phases to simulate fluid entrainment.
pub const ADDED_MASS_COEFFICIENT: f32 = 0.08;

/// Critical Reynolds Number for laminar-to-turbulent boundary layer transition.
/// Triggers non-linear drag scaling and dynamic wake particle instantiation.
pub const CRITICAL_REYNOLDS_NUMBER: f32 = 500000.0;

// =========================================================================
// --- NEW: Advanced Optical & Multispectral Attenuation Constants ---
// =========================================================================

/// Baseline attenuation coefficient for blue spectrum (470nm) in clear waters (m^-1).
/// Dictates premium visibility constraints for camera sensor modeling.
pub const BLUE_ATTENUATION_COEFFICIENT: f32 = 0.015;

/// Baseline attenuation coefficient for red spectrum (650nm) in clear waters (m^-1).
/// Simulates the rapid loss of red chromatic data in the first 5 meters of depth.
pub const RED_ATTENUATION_COEFFICIENT: f32 = 0.35;

/// Faraday Rotation Constant for water-based magneto-optic simulation.
/// Governs the polarization plane rotation vector of light passing through local magnetic anomalies.
pub const WATER_VERDET_CONSTANT: f32 = 0.0134; // rad / (T * m) at 589 nm

// =========================================================================
// --- NEW: Biomimetic Octopus-Evasion & Hydrofoil Constants ---
// =========================================================================

/// Maximum propulsion force multiplier during a biomimetic "jet propulsion" maneuver.
/// Modeled after cephalopod emergency evasion mechanics to simulate instant drag decoupling.
pub const BIOMIMETIC_BURST_MULTIPLIER: f32 = 3.5;

/// Thermal dissipation rate of the vessel hull when operating in stealth mode.
/// Mimics biological thermoregulation to minimize infrared silhouette detection.
pub const THERMAL_DISSIPATION_RATE: f32 = 0.12;

// =========================================================================
// --- Simulation Presets & Enums ---
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OceanType {
    /// Characterized by higher particulate matter and specific spectral absorption.
    Aegean,
    /// High clarity, low turbidity, favoring shorter (blue) wavelengths.
    Caribbean,
    /// NEW: Specific to high-latitude operations with distinct thermal gradient layers.
    Baltic,
}

// =========================================================================
// --- Dynamic Simulation State ---
// =========================================================================

#[derive(Resource, Debug, Clone)]
pub struct OceanSettings {
    /// Selected geographical preset affecting spectral attenuation (Beer-Lambert).
    pub ocean_type: OceanType,

    /// Vertical wave displacement (m). Impacts IR and Visible silhouette distortion.
    pub wave_amplitude: f32,
    
    /// Temporal frequency of wave oscillation (Hz).
    pub wave_frequency: f32,
    
    /// Global turbidity coefficient (m^-1). 
    /// Direct multiplier for multispectral light extinction.
    pub turbidity: f32,
    
    /// Salinity in Practical Salinity Units (PSU). 
    /// Influences the calculated refractive index (IOR).
    pub salinity: f32,
    
    /// Surface water temperature (°C). 
    /// Affects both optical density and thermodynamic sensor modeling.
    pub temperature: f32,
    
    /// Real-time vessel velocity (m/s). 
    /// Used for dynamic wake generation and optical flow calculations.
    pub vessel_speed: f32,

    // --- NEW: Multi-Physical Sensor Additions ---
    
    /// Real-time ambient light lux level at surface interface.
    pub surface_lux: f32,

    /// Depth matrix scalar (m). Calculated dynamically from the vessel's transform.
    pub current_depth: f32,
}

impl Default for OceanSettings {
    /// Default state initialized to Mediterranean/Aegean standards
    /// to provide a balanced baseline for optical attenuation testing.
    fn default() -> Self {
        Self {
            ocean_type: OceanType::Aegean,
            wave_amplitude: 0.6,    // Optimized for procedural Gerstner stability
            wave_frequency: 1.0,   
            turbidity: 0.08,        // Typical Aegean particulate density
            salinity: 38.5,         // Specific salinity for the Mediterranean basin
            temperature: 18.0,     
            vessel_speed: 0.0,      // Initialization state
            surface_lux: 100000.0,  // Standard direct sunlight lumens
            current_depth: 0.0,     // Surface initialization
        }
    }
}