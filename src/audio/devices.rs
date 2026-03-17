use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub description: String,
    pub kind: String,
}

/// List available sinks (output devices).
pub async fn list_sinks() -> Vec<AudioDevice> {
    parse_devices("sinks", "sink").await
}

/// List available sources (input devices).
pub async fn list_sources() -> Vec<AudioDevice> {
    parse_devices("sources", "source").await
}

async fn parse_devices(kind: &str, label: &str) -> Vec<AudioDevice> {
    let output = Command::new("pactl")
        .args(["list", kind])
        .output()
        .await;

    let Ok(out) = output else {
        log::error!("Failed to list {kind}");
        return vec![];
    };

    let text = String::from_utf8_lossy(&out.stdout);
    let mut devices = vec![];
    let mut current_id = String::new();
    let mut current_name = String::new();
    let mut current_desc = String::new();

    for line in text.lines() {
        let trimmed = line.trim();
        // "Sink #63" or "Source #62"
        if trimmed.starts_with("Sink #") || trimmed.starts_with("Source #") {
            if !current_id.is_empty() {
                devices.push(AudioDevice {
                    id: current_id.clone(),
                    name: current_name.clone(),
                    description: if current_desc.is_empty() {
                        current_name.clone()
                    } else {
                        current_desc.clone()
                    },
                    kind: label.to_string(),
                });
            }
            current_id = trimmed.split('#').nth(1).unwrap_or("").trim().to_string();
            current_name.clear();
            current_desc.clear();
        } else if trimmed.starts_with("Name:") {
            current_name = trimmed.strip_prefix("Name:").unwrap_or("").trim().to_string();
        } else if trimmed.starts_with("Description:") {
            current_desc = trimmed.strip_prefix("Description:").unwrap_or("").trim().to_string();
        }
    }
    // Push last device
    if !current_id.is_empty() {
        devices.push(AudioDevice {
            id: current_id,
            name: current_name.clone(),
            description: if current_desc.is_empty() {
                current_name
            } else {
                current_desc
            },
            kind: label.to_string(),
        });
    }

    devices
}
