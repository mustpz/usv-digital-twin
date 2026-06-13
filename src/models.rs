use bevy::prelude::*;

/// Represents the hydrodynamic state of the USV based on Computational Fluid Dynamics (CFD) logic.
/// Derived from Finite Volume Method (FVM) principles to track hull-water interaction.
#[derive(Default, Debug)]
pub struct Hydrodynamics {
    /// Calculated drag force acting on the hull (F_d). 
    /// Aiming for a low coefficient based on steady-state flow simulation results.
    pub current_drag: f32,

    /// Binary state indicating if the laminar flow is maintained.
    /// Steady flow (True) prevents surface disturbances, optimizing both speed and stealth.
    pub is_flow_steady: bool,
}

#[derive(Component)]
pub struct UnmannedSurfaceVehicle {
    pub name: String,
    pub vessel_speed: f32,
    
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

    // --- Propulsion & Hydrodynamics ---
    /// Integrated hydrodynamics module for real-time performance tracking.
    /// Links FVM-based flow analysis with autonomous movement logic.
    pub hydrodynamics: Hydrodynamics,
}

impl UnmannedSurfaceVehicle {
    /// Initializes a new USV with default engineering parameters, stealth probes, and hydrodynamic state.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            vessel_speed: 0.0,
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

            // Initializing hydrodynamics with default steady-flow assumptions.
            hydrodynamics: Hydrodynamics::default(),
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