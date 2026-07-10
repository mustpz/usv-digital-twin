use bevy::prelude::*;
use bevy::tasks::IoTaskPool; 
use serde::Serialize;
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

/// Data Transfer Object (DTO) representing the core telemetry metrics of the USV.
/// This structure is serialized into JSON format to stream real-time data to backend servers.
#[derive(Serialize, Debug, Clone)]
pub struct UsvTelemetryData {
    pub depth: f32,       // Sensor reading for under-water depth gauge
    pub speed: f32,       // Current velocity of the autonomous vehicle
    pub timestamp: u64,   // Unix timestamp in milliseconds for real-time tracking synchronization
}

/// NEW: Bevy Resource that encapsulates the persistent HTTP configuration.
/// Ensures the system reuses the same connection pool instead of allocating a client every frame.
#[derive(Resource)]
pub struct TelemetryStreamConfig {
    pub client: Client,
    pub api_url: String,
    pub rate_limiter: Timer, // Controls the packet emission frequency (e.g., 5Hz instead of 120Hz)
}

/// Asynchronously customizes, serializes, and transmits the telemetry packet to a remote target API endpoint.
/// Utilizes reqwest for non-blocking network I/O, ensuring the Bevy game loop never experiences frame drops.
pub async fn send_telemetry_packet(
    client: Client, // Moved ownership directly into the async task boundary
    api_url: String, 
    data: UsvTelemetryData
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> { // Send + Sync required for safe multi-threaded task spawning
    
    // 1. Serialize the safe Rust struct into a raw, universal JSON string payload
    let json_payload = serde_json::to_string(&data)?;
    
    // 2. Perform a non-blocking asynchronous HTTP POST request to stream the packet out to the world
    let _response = client.post(&api_url)
        .header("Content-Type", "application/json")
        .body(json_payload.clone())
        .send()
        .await?;

    Ok(json_payload)
}

/// NEW: Bevy ECS System that extracts physical registers from `biomimicry::calculate_biomimetic_evasion_system`
/// and pipes them directly into the asynchronous background network worker pool.
pub fn stream_biomimetic_telemetry_system(
    time: Res<Time>,
    mut config: ResMut<TelemetryStreamConfig>,
    // Reads directly from the same exact components used in your biomimicry system query
    query: Query<(&Transform, &crate::biomimicry::Velocity)>, 
) {
    // Tick the rate limiter timer with delta time; skips execution if the frame interval hasn't finished
    if !config.rate_limiter.tick(time.delta()).just_finished() {
        return;
    }

    let task_pool = IoTaskPool::get();

    for (transform, velocity) in query.iter() {
        // Safe extraction of underwater vehicle depth metrics based on localized coordinate translation
        let current_depth = (-transform.translation.y).max(0.0);
        let current_speed = velocity.0.length();
        
        // Dynamic generation of real-world Unix epoch miliseconds
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

    // Construct the immutable data packet directly inside the frame
let packet = UsvTelemetryData {
    depth: current_depth,
    speed: current_speed,
    timestamp: current_timestamp,
};

let async_client = config.client.clone();
let async_url = config.api_url.clone();

// Spawning the background async thread worker (Zero overhead on main thread)
task_pool.spawn(async move {
    // Serialization happens safely inside the background thread pool boundary, completely off the main thread!
    match send_telemetry_packet(async_client, async_url, packet).await {
        Ok(payload) => {
            trace!(target: "usv_project::telemetry", "Telemetry successfully emitted: {}", payload);
        }
        Err(err) => {
            warn!(target: "usv_project::telemetry", "Telemetry network packet dropped: {}", err);
        }
    }
}).detach();
    }
}