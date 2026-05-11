#import bevy_pbr::mesh_view_bindings::view

struct OceanMaterial {
    turbidity: f32,
    wave_amplitude: f32,
    wave_frequency: f32,
    time: f32,
    deep_water_color: vec4<f32>,
};

@group(2) @binding(0) var<uniform> material: OceanMaterial;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
};

fn calculate_wave(pos: vec2<f32>, time: f32) -> f32 {
    let amp = material.wave_amplitude;
    let freq = material.wave_frequency * 0.2;
    
    let d1 = dot(pos, vec2<f32>(1.0, 0.2)) * freq + time;
    let d2 = dot(pos, vec2<f32>(-0.5, 0.8)) * (freq * 1.5) + (time * 1.2);
    let d3 = dot(pos, vec2<f32>(0.2, -0.9)) * (freq * 2.0) + (time * 0.8);
    
    let wave = (sin(d1) * amp) + (sin(d2) * (amp * 0.4)) + (cos(d3) * (amp * 0.2));
    return wave;
}

@vertex
fn vertex(@location(0) position: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    var pos = position;
    
    pos.y = calculate_wave(pos.xz, material.time);

    out.world_position = pos; 
    out.clip_position = view.view_proj * vec4<f32>(pos, 1.0);
    
    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = material.deep_water_color.rgb;
    
    let highlight = (input.world_position.y + material.wave_amplitude) / (material.wave_amplitude * 2.5 + 0.1);
  
    let final_color = base_color * (0.4 + highlight * 1.0);
   
    let dist = length(input.world_position.xz) * 0.002;
    let fog_mix = clamp(dist, 0.0, 1.0);
    let final_with_fog = mix(final_color, vec3<f32>(0.4, 0.6, 0.8), fog_mix);

    return vec4<f32>(final_with_fog, 1.0);
}