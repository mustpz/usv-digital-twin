#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::forward_io::VertexOutput

struct CamouflageParams {
    target_color: vec4<f32>, 
    stealth_alpha: f32,      
    time: f32,
    base_reflectivity: f32,
    camera_mode: f32,       
    engine_heat: f32,       
};

@group(2) @binding(0) var<uniform> camo_data: CamouflageParams;


@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let view_dir = normalize(view.world_position - input.world_position.xyz);
    let normal = normalize(input.world_normal);

    // --- 1. BASE HULL COLOR & PARAMETERS ---
    let base_hull_color = vec3<f32>(0.9, 0.9, 1.0); 
    let adaptive_color = camo_data.target_color.rgb;
    
    // Compute Schlick's approximation for Fresnel reflection coefficient
    let fresnel = 0.02 + (0.98) * pow(1.0 - max(dot(normal, view_dir), 0.0), 5.0);
    
    // Spatial noise for edge disruption and signature blending
    let noise = sin(input.world_position.x * 2.0 + camo_data.time) * cos(input.world_position.z * 2.0 + camo_data.time * 0.5);
    let final_camo = mix(adaptive_color, adaptive_color * 0.8, noise * 0.2);
    
    // Interpolate between nominal hull paint and adaptive optical camouflage
    var final_color = mix(base_hull_color, final_camo, camo_data.stealth_alpha);
    
    // Apply environmental sky reflections based on surface Fresnel factors
    let sky_color = vec3<f32>(0.5, 0.7, 1.0);
    final_color = mix(final_color, sky_color, fresnel * camo_data.base_reflectivity);

    // =========================================================
    //    HYBRID MULTISPECTRAL SENSOR & SIGNATURE SIMULATION
    // =========================================================
    
    if (camo_data.camera_mode > 0.5) {
        // --- ACTIVE THERMAL (IR) IMAGING CHANNEL ---
        
        // 1. Ambient hull thermal signature (Cold structure/Background radiation)
        let thermal_cold = vec3<f32>(0.05, 0.15, 0.4); 
        
        // 2. Engine exhaust and mechanical friction thermal emission (IR Bloom)
        // Computes the thermal gradient along the longitudinal axis (z-axis)
        let local_z = input.world_position.z; 
        let engine_heat_gradient = smoothstep(-5.0, 5.0, local_z) * camo_data.engine_heat;
        let thermal_hot = mix(vec3<f32>(0.9, 0.4, 0.1), vec3<f32>(1.0, 0.9, 0.8), engine_heat_gradient);

        // Compute unmitigated thermal signature profile
        var thermal_signature = mix(thermal_cold, thermal_hot, engine_heat_gradient);

        // 3. BIOMIMETIC IR SIGNATURE MANAGEMENT & SUPPRESSION
        // When active stealth is enabled, suppress engine thermal bloom and cool down 
        // the hull to match the ambient ocean thermal background (suppressed blackbody radiation)
        thermal_signature = mix(thermal_signature, vec3<f32>(0.02, 0.08, 0.25), camo_data.stealth_alpha);

        final_color = thermal_signature;
    }

    return vec4<f32>(final_color, 1.0);
}