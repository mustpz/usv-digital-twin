use bevy::prelude::*;
use crate::constants::OceanSettings;

/// Calculates the refracted vector using Snell's Law in vector form.
/// 
/// # Arguments
/// * `incident` - Normalized incident ray vector.
/// * `normal` - Normalized surface normal vector.
/// * `n1` - Refractive index of the origin medium (e.g., Air = 1.0).
/// * `n2` - Refractive index of the destination medium (calculated via seawater model).
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

    // If the ray is exiting the medium, invert the normal and swap indices.
    if cos_theta1 < 0.0 {
        cos_theta1 = -cos_theta1;
        n = -n;
        eta = 1.0 / eta;
    }

    // Term under the square root (k) based on Snell's Law derivation
    let k = 1.0 - eta * eta * (1.0 - cos_theta1 * cos_theta1);

    // Check for Total Internal Reflection (TIR)
    if k < 0.0 {
        return None; 
    }

    let refracted = eta * i + (eta * cos_theta1 - k.sqrt()) * n;
    Some(refracted.normalize())
}

/// Calculates the reflection coefficient using Schlick's Approximation of Fresnel Equations.
/// Essential for determining surface glint and water transparency.
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

/// Models light intensity reduction underwater using the Beer-Lambert Law.
/// This determines the "visibility" or "murkiness" of the water.
/// 
/// # Formula: I = I0 * exp(-alpha * distance)
pub fn calculate_beer_lambert_attenuation(
    initial_intensity: Color,
    distance: f32,
    settings: &OceanSettings,
) -> Color {
    // alpha is our absorption coefficient, driven by the UI's turbidity slider
    let alpha = settings.turbidity;
    let attenuation = (-alpha * distance).exp();
    
    // Deconstruct the color to apply attenuation to each channel
    let r = initial_intensity.r() * attenuation;
    let g = initial_intensity.g() * attenuation;
    let b = initial_intensity.b() * attenuation;
    
    Color::rgb(r, g, b)
}

/// Calculates the dynamic refractive index of seawater based on the OceanSettings resource.
/// Accounts for temperature and salinity variations defined in the UI.
pub fn calculate_seawater_index(settings: &OceanSettings) -> f32 {
    let n_base = 1.333; // Reference index for pure water at 20°C
    
    // Empirical corrections: Index increases with salinity and decreases with temperature
    let salinity_correction = (settings.salinity - 35.0) * 0.0002;
    let temp_correction = (settings.temperature - 20.0) * -0.0001;
    
    (n_base + salinity_correction + temp_correction) as f32
}

/// Calculates the light attenuation factor based on depth.
/// Using the Beer-Lambert Law: I = I0 * e^(-k * d)
/// 
/// # Arguments
/// * `depth` - Current depth in meters (m).
/// * `turbidity` - Water clarity factor (0.01 for clear ocean, 0.1+ for murky coastal water).
/// 
/// # Returns
/// A visibility factor between 0.0 and 1.0 (1.0 = fully visible, 0.0 = total darkness).
pub fn calculate_light_attenuation(depth: f32, turbidity: f32) -> f32 {
    let depth_clamped = depth.max(0.0);
    // Exponential decay of light intensity as it travels deeper
    f32::exp(-turbidity * depth_clamped)
}

/// Calculates the UV offset for the seabed to simulate movement (Optical Flow).
/// # Arguments
/// * `velocity` - The speed of the vessel (USV) in m/s.
/// * `time` - Elapsed time in seconds.
pub fn calculate_seabed_uv_offset(velocity: f32, time: f32) -> f32 {
    // Constant factor to scale the texture scrolling speed
    let scroll_speed = 0.1;
    (velocity * time * scroll_speed) % 1.0
}

/// Calculates water clarity color based on depth and refractive indices.
pub fn get_water_clarity_color(_depth: f32, n1: f32, n2: f32) -> Color {
    let intensity = (n1 / n2).powi(2); 
    Color::rgb(0.0, 0.1 * intensity, 0.25 * intensity)
}