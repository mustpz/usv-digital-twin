use bevy::prelude::*;

#[derive(Component)]
pub struct UnmannedSurfaceVehicle {
    pub name: String,
    pub speed: f64,
    
    // --- Multispectral & Stealth Configuration ---
    /// Determines if the active scanning sensors are operational.
    pub multispectral_sensor_active: bool,
    
    /// The calculated optimal color for the hull to match the surroundings (C_cam).
    /// Derived from the weighted average of ambient water and sky colors.
    pub target_camouflage_color: Color,
    
    /// Defines the blending ratio between the original hull texture and the camouflage color.
    /// 0.0 = Fully Visible, 1.0 = Maximum Stealth.
    pub stealth_alpha: f32,
    
    /// Spatial coordinates used to sample environmental data (C_i and d_i).
    /// These points act as virtual "probes" around the hull to 'see' the water.
    pub sampling_points: Vec<Vec3>,
}

impl UnmannedSurfaceVehicle {
    /// Initializes a new USV with default engineering parameters and stealth probes.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            speed: 0.0,
            multispectral_sensor_active: true,
            
            // Defaulting to a neutral gray until the first environmental sample is processed.
            target_camouflage_color: Color::rgb(0.5, 0.5, 0.5), 
            stealth_alpha: 0.0, 
            
            // Local coordinates for environmental sampling (Front, Back, Port, Starboard).
            // These points allow the USV to 'sense' the water color at its boundaries.
            sampling_points: vec![
                Vec3::new(5.0, 0.0, 0.0),  // Bow (Front)
                Vec3::new(-5.0, 0.0, 0.0), // Stern (Back)
                Vec3::new(0.0, 0.0, 5.0),  // Port (Left)
                Vec3::new(0.0, 0.0, -5.0), // Starboard (Right)
            ],
        }
    }

    /// Implements the Mean Color Theory for Adaptive Camouflage.
    /// Formula: C_cam = Σ(C_i * w_i) / Σw_i
    /// 
    /// This function processes raw environmental color data and weights it 
    /// based on importance factors (distance, angle, lighting intensity).
    pub fn calculate_adaptive_color(&mut self, surrounding_colors: Vec<(Color, f32)>) {
        let mut total_weight = 0.0;
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        // Iterating through sampled points (C_i) and their relative weights (w_i).
        for (color, weight) in surrounding_colors {
            r += color.r() * weight;
            g += color.g() * weight;
            b += color.b() * weight;
            total_weight += weight;
        }

        // Apply the weighted average to define the target camouflage signature.
        if total_weight > 0.0 {
            self.target_camouflage_color = Color::rgb(
                r / total_weight,
                g / total_weight,
                b / total_weight,
            );
        }
    }
}