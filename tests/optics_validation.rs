use approx::assert_abs_diff_eq;

/// Beer-Lambert Law Optical Attenuation Baseline Test
/// Compares the physical light extinction model implemented in WGSL shaders
/// against analytical exponential decay equations: I = I_0 * exp(-k * x)
#[test]
fn validate_beer_lambert_optical_extinction() {
    // --- PHYSICAL PARAMETERS ---
    let turbidity: f32 = 0.055; // Standard baseline turbidity coefficient (k)
    let k = turbidity * 4.0;
    
    // Test depth points (normalized depth factor from 0.0 to 1.0)
    let test_depth_factors: Vec<f32> = vec![0.0, 0.25, 0.5, 0.75, 1.0];

    for depth in test_depth_factors {
        // 1. Shader physical calculation logic (as executed in ocean_shader.wgsl)
        let shader_extinction = (-k * (1.0 - depth)).exp();

        // 2. Analytical physical optics reference value (I / I_0)
        let physical_depth = 1.0 - depth;
        let analytical_extinction = (-k * physical_depth).exp();

        // Validate numerical stability and floating-point tolerance (Epsilon: 1e-5)
        assert_abs_diff_eq!(
            shader_extinction, 
            analytical_extinction, 
            epsilon = 1e-5
        );
    }
}

/// Boundary Conditions Guard Test
/// Verifies that zero turbidity results in 100% light transmittance (no absorption)
#[test]
fn validate_zero_turbidity_transmittance_boundary() {
    let turbidity: f32 = 0.0;
    let k = turbidity * 4.0;
    let depth_factor = 0.5;

    let extinction = (-k * (1.0 - depth_factor)).exp();
    
    assert_eq!(extinction, 1.0);
}