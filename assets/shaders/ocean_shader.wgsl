#import bevy_pbr::mesh_view_bindings::view

struct OceanParams {
    turbidity: f32,
    wave_amplitude: f32,
    wave_frequency: f32,
    time: f32,
};

@group(2) @binding(0) var<uniform> material: OceanParams;
@group(2) @binding(1) var<uniform> deep_water_color: vec4<f32>; 
@group(2) @binding(2) var water_normal_texture: texture_2d<f32>;
@group(2) @binding(3) var water_normal_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>, 
    @location(2) crest_factor: f32, 
};

// --- GERSTNER WAVE CALCULATION ---
fn gerstner_wave(pos: vec2<f32>, direction: vec2<f32>, steepness: f32, freq: f32, time: f32) -> vec3<f32> {
    let d = normalize(direction);
    let f = freq * dot(d, pos) + time;
    let a = steepness / freq;

    return vec3<f32>(
        d.x * (a * cos(f)), 
        a * sin(f),         
        d.y * (a * cos(f))
    );
}

@vertex
fn vertex(@location(0) position: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    var final_pos = position;
    
    let time = material.time;
    let amp = material.wave_amplitude;
    let freq = material.wave_frequency * 0.4;
    
    let w1 = gerstner_wave(position.xz, vec2<f32>(1.0, 0.2), 0.3 * amp, freq, time);
    let w2 = gerstner_wave(position.xz, vec2<f32>(-0.7, 0.9), 0.2 * amp, freq * 1.5, time * 1.2);
    let w3 = gerstner_wave(position.xz, vec2<f32>(0.2, -0.8), 0.1 * amp, freq * 2.5, time * 1.8);

    final_pos += w1 + w2 + w3;
    out.crest_factor = (w1.y + w2.y + w3.y) / (amp + 0.01);

    let delta = 0.2;
    let h_x = gerstner_wave(position.xz + vec2<f32>(delta, 0.0), vec2<f32>(1.0, 0.2), 0.3 * amp, freq, time).y;
    let h_z = gerstner_wave(position.xz + vec2<f32>(0.0, delta), vec2<f32>(1.0, 0.2), 0.3 * amp, freq, time).y;
    let wave_normal = normalize(vec3<f32>(final_pos.y - h_x, delta, final_pos.y - h_z));

    out.world_position = final_pos;
    out.world_normal = wave_normal;
    out.clip_position = view.view_proj * vec4<f32>(final_pos, 1.0);
    
    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let view_dir = normalize(view.world_position - input.world_position);
    let sun_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    
    let pixel_dist = length(view.world_position - input.world_position);
    let fade = clamp(1.0 - (pixel_dist / 350.0), 0.0, 1.0);
    
    // Triple Noise Overlay
    let time_val = material.time * 0.03;
    let uv1 = input.world_position.xz * 0.02 + vec2<f32>(time_val * 0.2, time_val * 0.1);
    let uv2 = input.world_position.xz * 0.05 - vec2<f32>(time_val * 0.3, -time_val * 0.2);
    let uv3 = input.world_position.xz * 0.15 + vec2<f32>(0.0, time_val * 0.5);

    let nm1 = textureSample(water_normal_texture, water_normal_sampler, uv1).rgb;
    let nm2 = textureSample(water_normal_texture, water_normal_sampler, uv2).rgb;
    let nm3 = textureSample(water_normal_texture, water_normal_sampler, uv3).rgb;
    
    let combined_nm = normalize((nm1 + nm2 + nm3) - 1.5);
    let final_normal = normalize(input.world_normal + (combined_nm * 0.2 * fade));

   
    let depth_factor = clamp((input.world_position.y + material.wave_amplitude) / (material.wave_amplitude * 2.0 + 0.01), 0.0, 1.0);
    
   
    let k = material.turbidity * 4.0;
    let extinction = exp(-k * (1.0 - depth_factor));
    
  
    let scattering_color = vec3<f32>(0.05, 0.1, 0.05); 
    
    let shallow_color = deep_water_color.rgb + vec3<f32>(0.1, 0.25, 0.3);
    let base_water = mix(deep_water_color.rgb, shallow_color, depth_factor);
    
    
    var water_color = mix(base_water * extinction, scattering_color, material.turbidity * 0.6);
    
    let foam_threshold = 0.65; 
    let foam_mask = clamp((input.crest_factor - foam_threshold) * 5.0, 0.0, 1.0);
    let foam_color = vec3<f32>(0.9, 0.95, 1.0); 
    water_color = mix(water_color, foam_color, foam_mask * (1.0 - material.turbidity));

    let fresnel = 0.02 + (0.98) * pow(1.0 - max(dot(final_normal, view_dir), 0.0), 5.0);
    let half_vec = normalize(sun_dir + view_dir);
    let spec = pow(max(dot(final_normal, half_vec), 0.0), 128.0) * 0.6;

    let sky_color = vec3<f32>(0.5, 0.7, 1.0);
    var final_color = mix(water_color, sky_color, fresnel * 0.5);
   
    final_color += (vec3<f32>(1.0, 0.9, 0.8) * spec * (1.0 - material.turbidity));

    let fog_dist = length(input.world_position.xz) * 0.0008;
    let final_with_fog = mix(final_color, vec3<f32>(0.5, 0.6, 0.7), clamp(fog_dist, 0.0, 1.0));

    return vec4<f32>(final_with_fog, 1.0);
}