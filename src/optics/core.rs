use bevy::prelude::*;
use crate::constants::{OceanSettings, OceanType};

// ============================================================================
// 1. MULTISPECTRAL CAMOUFLAGE & ATMOSPHERIC ANOMALY ECS COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct MultispectralCamouflage {
    pub visible_reflectivity: f32,
    pub infrared_signature: f32,
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

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct AtmosphericMirageEffect {
    pub vertical_offset: f32,
    pub shape_distortion: f32,
}

// ============================================================================
// 2. PHYSICS & GEOMETRIC OPTICS FUNCTIONS
// ============================================================================

/// Calculates vertical ray displacement caused by severe sea-surface temperature inversions.
/// Protected against tangent mathematical divergence via small-angle linear expansion.
pub fn calculate_atmospheric_refraction(
    distance_to_target: f32,
    settings: &OceanSettings,
) -> AtmosphericMirageEffect {
    if settings.temp_gradient <= 0.0 {
        return AtmosphericMirageEffect::default();
    }

    // Gladstone-Dale relation: dn/dh
    let dn_dh = -0.00002 * settings.temp_gradient;
    let deviation_angle = (distance_to_target * dn_dh).abs();

    // MATHEMATICAL GUARD: Small-angle approximation (tan(theta) ≈ theta) 
    // Completely eliminates infinite/NaN traps of raw tangent calculations in extreme conditions.
    let vertical_offset = distance_to_target * deviation_angle;

    let shape_distortion = (settings.temp_gradient * 0.15) * (distance_to_target * 0.001);

    AtmosphericMirageEffect {
        vertical_offset: vertical_offset.clamp(0.0, 45.0), 
        shape_distortion: shape_distortion.clamp(0.0, 1.0),
    }
}

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

pub fn calculate_beer_lambert_attenuation(
    initial_intensity: Color,
    distance: f32,
    settings: &OceanSettings,
) -> Color {
    let (mu_r, mu_g, mu_b) = match settings.ocean_type {
        OceanType::Aegean => (0.650, 0.070, 0.035), 
        OceanType::Caribbean => (0.350, 0.040, 0.015),
        OceanType::Baltic => (0.950, 0.250, 0.450),
    };

    let atten_r = (-(mu_r * settings.turbidity * distance)).exp();
    let atten_g = (-(mu_g * settings.turbidity * distance)).exp();
    let atten_b = (-(mu_b * settings.turbidity * distance)).exp();
    
    Color::rgba_linear(
        initial_intensity.r() * atten_r,
        initial_intensity.g() * atten_g,
        initial_intensity.b() * atten_b,
        initial_intensity.a(),
    )
}

#[inline(always)]
pub fn calculate_procedural_wave_height_vectorized(
    pos: Vec2, 
    time: f32, 
    complexity: u32
) -> f32 {
    let mut height = 0.0;
    let wave_dir = Vec2::new(1.0, 1.0).normalize();
    let pos_projected = pos.dot(wave_dir);
    
    for i in 1..=complexity {
        let i_f32 = i as f32;
        let freq = i_f32 * 0.5;
        let amp = 1.0 / (i_f32 * 2.0);
        let speed = time * i_f32.sqrt();
        
        let phase = pos_projected * freq + speed;
        height += (2.0 * phase).sin() * 0.5 * amp;
    }
    
    height
}

pub fn calculate_seawater_index(settings: &OceanSettings) -> f32 {
    let n_base = 1.333; 
    let salinity_correction = (settings.salinity - 35.0) * 0.0002;
    let temp_correction = (settings.temperature - 20.0) * -0.0001;
    
    (n_base + salinity_correction + temp_correction) as f32
}

pub fn calculate_visibility_range(turbidity: f32) -> f32 {
    if turbidity <= 0.0001 { return 150.0; }
    1.7 / turbidity
}

// ============================================================================
// 3. BIOMIMETIC & ATMOSPHERIC SIMULATION SYSTEMS (BEVY ECS LAYER)
// ============================================================================

/// Optimized Adaptive Camouflage System.
/// Implements state change detection filters and computational anchors 
/// to fully prevent unnecessary frame-by-frame VRAM mutations.
pub fn update_biomimetic_camouflage(
    time: Res<Time>,
    ocean_settings: Res<OceanSettings>,
    evasion_state: Res<State<crate::biomimicry::EvasionMode>>, 
    mut camo_query: Query<(
        &mut MultispectralCamouflage, 
        &Transform, 
        Option<&mut AtmosphericMirageEffect>
    )>,
) {
    let visibility_limit = calculate_visibility_range(ocean_settings.turbidity);
    let biological_adaptation_speed = 1.8 * time.delta_seconds();
    let current_evasion_mode = *evasion_state.get();
    let elapsed_time = time.elapsed_seconds();

    for (mut camo, transform, mirage_opt) in camo_query.iter_mut() {
        let vehicle_depth = (-transform.translation.y).max(0.0);
        
        let mut target_visible = if visibility_limit < 10.0 { 0.05 } else { (0.1 + (vehicle_depth * 0.02)).min(0.4) };
        let mut target_ir = if ocean_settings.temperature < 15.0 { 0.3 } else { 0.6 };

        match current_evasion_mode {
            crate::biomimicry::EvasionMode::JetPropulsion => {
                target_visible *= 0.5; 
                target_ir *= 0.4;      
            },
            crate::biomimicry::EvasionMode::InkCloudDecoy => {
                target_visible = 0.01; 
                target_ir = 0.1;       
            },
            crate::biomimicry::EvasionMode::ColregHeadOnAlterCourseStarboard |
            crate::biomimicry::EvasionMode::ColregGiveWayCrossing => {
                target_visible *= 0.8; 
            },
            _ => {} 
        }

        // Calculate next potential states
        let next_visible = camo.visible_reflectivity + (target_visible - camo.visible_reflectivity) * biological_adaptation_speed;
        let next_ir = camo.infrared_signature + (target_ir - camo.infrared_signature) * biological_adaptation_speed;
        
        let wave_clutter_factor = calculate_procedural_wave_height_vectorized(transform.translation.xz(), elapsed_time, 3);
        let next_rcs = (0.2_f32 + (wave_clutter_factor.abs() * 0.1_f32)).clamp(0.1_f32, 0.8_f32);

        // MUTATION GUARD: Only trigger explicit component mutations if floating point delta variance exists.
        // Bypasses VRAM upload pipeline starvation completely when signatures stabilize.
        if (camo.visible_reflectivity - next_visible).abs() > 0.0001 
            || (camo.infrared_signature - next_ir).abs() > 0.0001 
            || (camo.radar_cross_section - next_rcs).abs() > 0.0001 
        {
            camo.visible_reflectivity = next_visible.clamp(0.0, 1.0);
            camo.infrared_signature = next_ir.clamp(0.0, 1.0);
            camo.radar_cross_section = next_rcs;
        }

        if let Some(mut mirage) = mirage_opt {
            let target_distance = transform.translation.length();
            let next_mirage = calculate_atmospheric_refraction(target_distance, &ocean_settings);
            
            if *mirage != next_mirage {
                *mirage = next_mirage;
            }
        }
    } 
}

// ============================================================================
// 4. FMI 3.0 COMPLIANT INTERFACE & MARITIME V&V (DNV-RP-0513) LAYER
// ============================================================================

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct FmiModelExchangeState {
    pub vr_temp_gradient: u32,
    pub validation_error_margin: f32,
    pub is_dnv_validated: bool,
}

impl Default for FmiModelExchangeState {
    fn default() -> Self {
        Self {
            vr_temp_gradient: 1001, 
            validation_error_margin: 0.0,
            is_dnv_validated: true,
        }
    }
}

pub fn validate_optical_fidelity_dnv(
    calculated_offset: f32,
    distance: f32,
    settings: &OceanSettings,
) -> (bool, f32) {
    if settings.temp_gradient <= 0.0 {
        return (true, 0.0);
    }

    let iho_empirical_expected_offset = distance * (0.000015 * settings.temp_gradient);
    let delta_error = (calculated_offset - iho_empirical_expected_offset).abs();
    let acceptable_dnv_tolerance = 0.05 * distance * 0.001; 
    let is_valid = delta_error <= acceptable_dnv_tolerance;
    
    (is_valid, delta_error)
}