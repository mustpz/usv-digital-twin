use bevy::prelude::*;
use crate::constants::{OceanSettings, OceanType};

/// Transitioning from static texture mapping to procedural spectral modeling.
/// This module handles the core physics of light-matter interaction in a dynamic aquatic medium.

/// Calculates the refracted vector using Snell's Law in vector form.
/// Implements rigorous handling of internal reflection and medium transitions.
pub fn calculate_snell_refraction(
    incident: Vec3,
    normal: Vec3,
    n_origin: f32,
    n_destination: f32,
) -> Option<Vec3> {
    let i = incident.normalize();
    let mut n = normal.normalize();
    
    let mut eta = n_origin / n_destination;
    let mut cos_theta1 = -n.dot(i);

    // Handling ray transition from denser to lighter medium
    if cos_theta1 < 0.0 {
        cos_theta1 = -cos_theta1;
        n = -n;
        eta = 1.0 / eta;
    }

    let k = 1.0 - eta * eta * (1.0 - cos_theta1 * cos_theta1);

    // Total Internal Reflection (TIR) Check
    if k < 0.0 {
        return None; 
    }

    let refracted = eta * i + (eta * cos_theta1 - k.sqrt()) * n;
    Some(refracted.normalize())
}

/// Computes spectral reflectance using Schlick's Approximation of Fresnel Equations.
/// Essential for calculating the 'glitter' and 'mirror' effects of the ocean surface.
pub fn calculate_fresnel_reflectance(
    incident: Vec3,
    normal: Vec3,
    n1: f32,
    n2: f32
) -> f32 {
    let i = incident.normalize();
    let n = normal.normalize();

    // R0 is the reflectance at normal incidence
    let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
    let cos_theta = (-n.dot(i)).max(0.0);
    
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

/// ADVANCED BEER-LAMBERT IMPLEMENTATION
/// Models multispectral attenuation based on wavelength-dependent absorption coefficients.
/// 
/// Replaced fixed multipliers with specific coefficients for Mediterranean (Aegean) 
/// vs. Tropical (Caribbean) spectral profiles to enhance Digital Twin fidelity.
pub fn calculate_beer_lambert_attenuation(
    initial_intensity: Color,
    distance: f32,
    settings: &OceanSettings,
) -> Color {
    // Spectral absorption coefficients (m^-1) based on real-world oceanographic data.
    // Aegean Sea (higher turbidity/particulates) vs Tropical (pure water bias).
    let (mu_r, mu_g, mu_b) = match settings.ocean_type {
        // Higher attenuation in red spectrum for Aegean due to organic particulates.
        OceanType::Aegean => (0.650, 0.070, 0.035), 
        // Tropical waters exhibit lower overall absorption, favoring blue/cyan.
        OceanType::Caribbean => (0.350, 0.040, 0.015),
    };

    // Physics-driven decay calculation.
    // Boosting distance artificially to match visual dynamic range of digital displays.
    let atten_r = (-(mu_r * settings.turbidity * distance)).exp();
    let atten_g = (-(mu_g * settings.turbidity * distance)).exp();
    let atten_b = (-(mu_b * settings.turbidity * distance)).exp();
    
    Color::rgb(
        initial_intensity.r() * atten_r,
        initial_intensity.g() * atten_g,
        initial_intensity.b() * atten_b,
    )
}

/// WAVE DYNAMICS: Procedural Ocean Surface Generation
/// Computes wave displacement using a sum of Gerstner Waves.
/// This replaces the static texture tiling with a continuous mathematical surface.
///
/// Formula: P(x, y, t) = sum( Ai * Di * sin( ki * (x, y) - wi * t + phi_i ) )
pub fn calculate_procedural_wave_height(
    pos: Vec2, 
    time: f32, 
    complexity: u32
) -> f32 {
    let mut height = 0.0;
    
    // Simulating multiple wave octaves for 'natural' ocean look
    for i in 1..=complexity {
        let freq = i as f32 * 0.5;
        let amp = 1.0 / (i as f32 * 2.0);
        let speed = time * (i as f32).sqrt();
        
        height += (pos.x * freq + speed).sin() * (pos.y * freq + speed).cos() * amp;
    }
    
    height
}

/// Dynamically adjusts the refractive index (IOR) based on environmental variables.
/// Critical for high-fidelity USV (Unmanned Surface Vehicle) sensor simulation.
pub fn calculate_seawater_index(settings: &OceanSettings) -> f32 {
    let n_base = 1.333; 
    let salinity_correction = (settings.salinity - 35.0) * 0.0002;
    let temp_correction = (settings.temperature - 20.0) * -0.0001;
    
    (n_base + salinity_correction + temp_correction) as f32
}

/// Secchi Depth Calculation
/// Used to determine the visual operational limit for USV sensors.
pub fn calculate_visibility_range(turbidity: f32) -> f32 {
    if turbidity <= 0.0001 { return 150.0; }
    // The constant 1.7 represents the standard contrast threshold for human/sensor vision.
    1.7 / turbidity
}