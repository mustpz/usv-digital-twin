use bevy::prelude::*;
use crate::constants::OceanSettings;

/// Calculates the refracted vector using Snell's Law in vector form.
/// 
/// # Arguments
/// * `incident` - Normalized incident ray vector.
/// * `normal` - Normalized surface normal vector.
/// * `n1` - Refractive index of origin medium (Air ≈ 1.0).
/// * `n2` - Refractive index of destination medium (Seawater).
pub fn calculate_snell_refraction(
    incident: Vec3,
    normal: Vec3,
    n1: f32,
    n2: f32,
) -> Option<Vec3> {
    let i = incident.normalize();
    let mut n = normal.normalize();
    
    let mut eta = n1 / n2;
    let mut cos_theta1 = -n.dot(i);

    if cos_theta1 < 0.0 {
        cos_theta1 = -cos_theta1;
        n = -n;
        eta = 1.0 / eta;
    }

    let k = 1.0 - eta * eta * (1.0 - cos_theta1 * cos_theta1);

    if k < 0.0 {
        return None; 
    }

    let refracted = eta * i + (eta * cos_theta1 - k.sqrt()) * n;
    Some(refracted.normalize())
}

/// Calculates the reflection coefficient using Schlick's Approximation of Fresnel Equations.
pub fn calculate_fresnel_reflectance(
    incident: Vec3,
    normal: Vec3,
    n1: f32,
    n2: f32
) -> f32 {
    let i = incident.normalize();
    let n = normal.normalize();

    let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
    let cos_theta = (-n.dot(i)).max(0.0);
    
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

/// Models multispectral light intensity reduction underwater using the Beer-Lambert Law.
/// This implementation uses boosted sensitivity to ensure visible changes in simulation.
/// 
/// # Formula: I(lambda) = I0 * exp(-mu(lambda) * distance * turbidity)
pub fn calculate_beer_lambert_attenuation(
    initial_intensity: Color,
    distance: f32,
    settings: &OceanSettings,
) -> Color {
    // 1. Spectral absorption coefficients (m^-1)
    // Red is absorbed significantly faster than blue in seawater.
    let mu_red = 0.450;   
    let mu_green = 0.050; 
    let mu_blue = 0.020;  

    // 2. Sensitivity Boosting
    // We multiply turbidity and distance to make the exponential decay 
    // visible on a standard 8-bit digital display.
    let turbidity_multiplier = settings.turbidity * 10.0; 
    let effective_dist = distance * 2.0;

    // 3. Channel Attenuation Calculation
    let atten_r = (-(mu_red * turbidity_multiplier * effective_dist)).exp();
    let atten_g = (-(mu_green * turbidity_multiplier * effective_dist)).exp();
    let atten_b = (-(mu_blue * turbidity_multiplier * effective_dist)).exp();
    
    // 4. Color Reconstruction
    Color::rgb(
        initial_intensity.r() * atten_r,
        initial_intensity.g() * atten_g,
        initial_intensity.b() * atten_b,
    )
}

/// Calculates the dynamic refractive index of seawater based on OceanSettings.
pub fn calculate_seawater_index(settings: &OceanSettings) -> f32 {
    let n_base = 1.333; // Pure water at 20°C
    let salinity_correction = (settings.salinity - 35.0) * 0.0002;
    let temp_correction = (settings.temperature - 20.0) * -0.0001;
    
    (n_base + salinity_correction + temp_correction) as f32
}

/// Calculates the Secchi Depth (visual transparency limit) based on turbidity.
pub fn calculate_secchi_depth(turbidity: f32) -> f32 {
    if turbidity <= 0.0001 { return 100.0; }
    1.7 / turbidity
}

/// Calculates the UV offset for the seabed to simulate Optical Flow.
pub fn calculate_seabed_uv_offset(velocity: f32, time: f32) -> f32 {
    let scroll_speed = 0.1;
    (velocity * time * scroll_speed) % 1.0
}