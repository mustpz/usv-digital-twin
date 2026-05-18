use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct UsvTelemetryData {
    pub depth: f32,
    pub speed: f32,
    pub timestamp: u64,
}

pub async fn send_telemetry_packet(data: &UsvTelemetryData) -> Result<String, serde_json::Error> {
   
    let json_payload = serde_json::to_string(data)?;
    
    println!("📦 [Telemetry Stream] JSON Payload Ready: {}", json_payload);
    
    Ok(json_payload)
}