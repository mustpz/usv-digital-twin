use bevy::prelude::*;

// --- Static Physical Constants (Reference Standards) ---

/// Refractive Index (IOR) of pure water at 20°C.
/// Used as the baseline for dynamic salinity/temperature corrections.
pub const WATER_REFRACTIVE_INDEX: f64 = 1.333; 

/// Standard atmospheric pressure (hPa). 
/// Essential for future implementations of surface wave-air interface physics.
pub const SEA_LEVEL_PRESSURE: f64 = 1013.25;

// --- Simulation Presets & Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OceanType {
    /// Characterized by higher particulate matter and specific spectral absorption.
    Aegean,
    /// High clarity, low turbidity, favoring shorter (blue) wavelengths.
    Caribbean,
}

// --- Dynamic Simulation State ---

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
        }
    }
}