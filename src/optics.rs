use bevy::prelude::Color;
use bevy::prelude::Vec3;

/// Calculates the refracted vector using Snell's Law in vector form.
/// 
/// # Arguments
/// * `incident` - Normalized incident ray vector.
/// * `normal` - Normalized surface normal vector.
/// * `n1` - Refractive index of the origin medium (e.g., Air = 1.0).
/// * `n2` - Refractive index of the destination medium (e.g., Water = 1.33).
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

    // If the ray is exiting the medium (hitting the back of the surface), 
    // we invert the normal and swap indices.
    if cos_theta1 < 0.0 {
        cos_theta1 = -cos_theta1;
        n = -n;
        eta = 1.0 / eta;
    }

    // Term under the square root (k)
    let k = 1.0 - eta * eta * (1.0 - cos_theta1 * cos_theta1);

    // Check for Total Internal Reflection (TIR)
    if k < 0.0 {
        return None; // No refraction, purely reflection
    }

    let refracted = eta * i + (eta * cos_theta1 - k.sqrt()) * n;
    Some(refracted.normalize())
}

/// Calculates the reflection coefficient using Fresnel Equations (Schlick's Approximation).
/// Useful for determining the intensity of reflected vs. refracted light.
pub fn calculate_fresnel_reflectance(
    incident: Vec3,
    normal: Vec3,
    n1: f32,
    n2: f32
) -> f32 {
    let i = incident.normalize();
    let n = normal.normalize();
    
    // Simplified Schlick's approximation for real-time performance
    let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
    let cos_theta = (-n.dot(i)).max(0.0);
    
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

/// Calculates the dynamic refractive index of seawater based on environmental factors.
/// This implementation uses a simplified empirical model accounting for temperature and salinity.
/// 
/// # Arguments
/// * `temp_c` - Water temperature in Celsius (°C).
/// * `salinity_psu` - Salinity in Practical Salinity Units (standard seawater is ~35.0).
/// 
/// # Returns
/// The calculated refractive index as an f64.
pub fn calculate_seawater_index(temp_c: f32, salinity_psu: f32) -> f64 {
    // Reference index for pure water at 20°C
    let n_base = 1.333;

    // Empirical coefficients (Approximations for simulation purposes)
    let salinity_effect = 0.0002 * (salinity_psu as f64);
    let temperature_effect = -0.0001 * ((temp_c - 20.0) as f64);

    n_base + salinity_effect + temperature_effect
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


pub fn get_water_clarity_color(depth: f32, n1: f32, n2: f32) -> Color {
    let intensity = (n1 / n2).powi(2); 
    Color::rgb(0.0, 0.1 * intensity, 0.25 * intensity)
}