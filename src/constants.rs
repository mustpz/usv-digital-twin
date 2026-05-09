use bevy::prelude::*;

// --- Static Physical Constants  ---

/// Base refractive index for pure water (Reference)
pub const WATER_REFRACTIVE_INDEX: f64 = 1.333; 

/// Standard sea-level atmospheric pressure in hPa
pub const SEA_LEVEL_PRESSURE: f64 = 1013.25;

// --- Dynamic Ocean Settings (Can be changed via UI) ---

#[derive(Resource, Debug, Clone)]
pub struct OceanSettings {
    /// Vertical scale of the waves (meters)
    pub wave_amplitude: f32,
    
    /// Speed of the vessel through the water (m/s)
    pub vessel_speed: f32,
    
    /// Water clarity factor based on Beer-Lambert law (0.0 = clear, higher = murky)
    pub turbidity: f32,
    
    /// Ocean salinity in PSU (Affects optical density)
    pub salinity: f32,
    
    /// Water temperature in Celsius
    pub temperature: f32,
    
    /// Speed of wave propagation
    pub wave_frequency: f32,
}

impl Default for OceanSettings {
    fn default() -> Self {
        Self {
            wave_amplitude: 0.8,    // Default starting value
            vessel_speed: 4.5,     // Default starting value
            turbidity: 0.05,       // Subtle light attenuation
            salinity: 35.0,        // Standard ocean salinity
            temperature: 20.0,     // Average surface temperature
            wave_frequency: 1.2,   // Natural wave rhythm
        }
    }
}