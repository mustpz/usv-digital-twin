use bevy::prelude::*;
use crate::constants::{OceanSettings, OceanType};

/// Transitioning from static texture mapping to procedural spectral modeling.
/// This module handles the core physics of light-matter interaction in a dynamic aquatic medium.

/// ============================================================================
/// 1. MULTISPECTRAL CAMOUFLAGE & CEPHALOPOD BIOMIMICRY ECS COMPONENTS
/// ============================================================================

/// ECS Component that encapsulates the active camouflage state across multiple spectrums.
/// Models dynamic reflectivity and thermal/radar signatures for low-observability USV metrics.
#[derive(Component, Debug, Clone, Copy)]
pub struct MultispectralCamouflage {
    /// Visible spectrum reflectivity factor. Range: 0.0 (Perfect active camouflage) to 1.0 (Fully visible).
    pub visible_reflectivity: f32,
    /// Infrared (IR) signature modifier. Scales the thermal emission profile of the propulsion/hull.
    pub infrared_signature: f32,
    /// Radar Cross Section (RCS) multiplier. Models the radar-absorbent properties of the adaptive skin.
    pub radar_cross_section: f32,
}

impl Default for MultispectralCamouflage {
    fn default() -> Self {
        Self {
            visible_reflectivity: 1.0,
            infrared_signature: 1.0,
            radar_cross_section: 1.0,
        }
    }
}

/// ============================================================================
/// 2. PHYSICS & GEOMETRIC OPTICS FUNCTIONS
/// ============================================================================

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
        // Baltic: Extreme high attenuation across the spectrum, catastrophic loss in blue.
        OceanType::Baltic => (0.950, 0.250, 0.450),
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

/// ============================================================================
/// 3. BIOMIMETIC SIMULATION SYSTEMS (BEVY ECS LAYER)
/// ============================================================================

/// Bevy ECS System that dynamically updates the multispectral camouflage matrices.
/// Emulates cephalopod chromatophore and iridophore adaptation velocity mapped against
/// local water turbidity, Secchi depth limits, and spectral attenuation.
pub fn update_biomimetic_camouflage(
    time: Res<Time>,
    ocean_settings: Res<OceanSettings>,
    mut camo_query: Query<(&mut MultispectralCamouflage, &Transform, &crate::biomimicry::OctopodEvasionMatrix)>,
) {
    let visibility_limit = calculate_visibility_range(ocean_settings.turbidity);
    let biological_adaptation_speed = 1.8 * time.delta_seconds();

    for (mut camo, transform, evasion_matrix) in camo_query.iter_mut() {
        let vehicle_depth = (-transform.translation.y).max(0.0);
        
        // BASELINE TARGETS: Environmental baseline matching
        let mut target_visible = if visibility_limit < 10.0 { 0.05 } else { (0.1 + (vehicle_depth * 0.02)).min(0.4) };
        let mut target_ir = if ocean_settings.temperature < 15.0 { 0.3 } else { 0.6 };

        // ============================================================================
        // COLREG & BIOMIMETIC MANEUVER INTEGRATION
        // Adjust camouflage profiles based on the active tactical state machine register
        // ============================================================================
        match evasion_matrix.current_mode {
            // Absolute Tactical Escape: Maximize chromatophore compression during emergency jetting
            crate::biomimicry::EvasionMode::JetPropulsion => {
                target_visible *= 0.5; // Adaptive skin darkens/blends completely to match rapid fluid flow
                target_ir *= 0.4;      // Heat signatures are heavily masked or deflected during peak thrust
            },
            // Active Decoy Deployment: Total disruption of optical and thermal sensor tracking loops
            crate::biomimicry::EvasionMode::InkCloudDecoy => {
                target_visible = 0.01; // Theoretical zero visibility (Complete background blending/aerosol mask)
                target_ir = 0.1;       // Complete thermal dampening
            },
            // COLREG Regulated Maneuvers: Maintain standard adaptive stealth profile to avoid sensor blinding
            crate::biomimicry::EvasionMode::ColregHeadOnAlterCourseStarboard |
            crate::biomimicry::EvasionMode::ColregGiveWayCrossing => {
                target_visible *= 0.8; // Controlled stabilization for secure radar signature feedback
            },
            _ => {} // Idle states use nominal background matching profiles
        }

        // Execute asynchronous biological cell dampening (Interpolation)
        camo.visible_reflectivity += (target_visible - camo.visible_reflectivity) * biological_adaptation_speed;
        camo.infrared_signature += (target_ir - camo.infrared_signature) * biological_adaptation_speed;
        
        // Radar Cross Section dynamically matches ocean wave state (Clutter masking)
        let wave_clutter_factor = calculate_procedural_wave_height(transform.translation.xz(), time.elapsed_seconds(), 3);
        camo.radar_cross_section = (0.2 + (wave_clutter_factor.abs() * 0.1)).clamp(0.1, 0.8);

        // Strict boundary constraints
        camo.visible_reflectivity = camo.visible_reflectivity.clamp(0.0, 1.0);
        camo.infrared_signature = camo.infrared_signature.clamp(0.0, 1.0);
    }
}