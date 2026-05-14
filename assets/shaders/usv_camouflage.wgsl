#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::forward_io::VertexOutput

struct CamouflageParams {
    target_color: vec4<f32>, 
    stealth_alpha: f32,      
    time: f32,
    base_reflectivity: f32,
};

@group(2) @binding(0) var<uniform> camo_data: CamouflageParams;


@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let view_dir = normalize(view.world_position - input.world_position.xyz);
    let normal = normalize(input.world_normal);

    // --- 1. BASE HULL COLOR ---
    let base_hull_color = vec3<f32>(0.9, 0.9, 1.0); 

    // --- 2. ADAPTIVE CAMOUFLAGE (The Mean Color Theory) ---
    let adaptive_color = camo_data.target_color.rgb;

    // --- 3. FRESNEL REFLECTION (Optical Realism) ---
    let fresnel = 0.02 + (0.98) * pow(1.0 - max(dot(normal, view_dir), 0.0), 5.0);
    
    // --- 4. DYNAMIC NOISE (Edge Disruption) ---
    let noise = sin(input.world_position.x * 2.0 + camo_data.time) * cos(input.world_position.z * 2.0 + camo_data.time * 0.5);
    
    let final_camo = mix(adaptive_color, adaptive_color * 0.8, noise * 0.2);

    // --- 5. BLENDING (Final Signature Management) ---
    var final_color = mix(base_hull_color, final_camo, camo_data.stealth_alpha);

    // --- 6. ENVIRONMENTAL INTERACTION ---
    let sky_color = vec3<f32>(0.5, 0.7, 1.0);
    final_color = mix(final_color, sky_color, fresnel * camo_data.base_reflectivity);

    return vec4<f32>(final_color, 1.0);
}