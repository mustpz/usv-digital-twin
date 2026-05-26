use serde::Serialize;
use reqwest::Client;

/// Data Transfer Object (DTO) representing the core telemetry metrics of the USV (Unmanned Surface Vehicle).
/// This structure is serialized into JSON format to stream real-time data to backend servers or control stations.
#[derive(Serialize, Debug, Clone)]
pub struct UsvTelemetryData {
    pub depth: f32,       // Sensor reading for under-water depth gauge
    pub speed: f32,       // Current velocity of the autonomous vehicle
    pub timestamp: u64,   // Unix timestamp in milliseconds for real-time tracking synchronization
}

/// Asynchronously customizes, serializes, and transmits the telemetry packet to a remote target API endpoint.
/// Utilizes reqwest for non-blocking network I/O, ensuring the Bevy game loop never experiences frame drops.
pub async fn send_telemetry_packet(
    client: &Client, 
    api_url: &str, 
    data: &UsvTelemetryData
) -> Result<String, Box<dyn std::error::Error>> {
    
    // 1. Serialize the safe Rust struct into a raw, universal JSON string payload
    let json_payload = serde_json::to_string(data)?;
    
    // Log the prepared payload to the console for internal development tracking
    println!("📦 [Telemetry Stream] JSON Payload Ready: {}", json_payload);
    
    // 2. Perform a non-blocking asynchronous HTTP POST request to stream the packet out to the world
    // Passes the payload directly as text with a JSON content-type header
    let _response = client.post(api_url)
        .header("Content-Type", "application/json")
        .body(json_payload.clone())
        .send()
        .await?;

    // Return the successfully generated JSON payload back to the system layer if needed
    Ok(json_payload)
}

// Asynchronously fetches real-time environment variables or external control override packets.
/// Utilizes a non-blocking GET request, protecting the siber-physical loop from network latency spikes.
pub async fn fetch_telemetry_data(
    client: &Client, 
    url: &str
) -> Result<String, reqwest::Error> {
    
    println!("📡 [Telemetry Ingress] Fetching real-time environment matrix from: {}", url);
    
    // Non-blocking HTTP GET request to capture the physical/virtual world updates
    let response = client.get(url)
        .send()
        .await?;
        
    // Ensure we parse the response body as text asynchronously without blocking the core executor
    let body_content = response.text().await?;
    
    Ok(body_content)
}