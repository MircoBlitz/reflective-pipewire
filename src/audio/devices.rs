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

/// Get device name from ID (special handling for defaults)
pub async fn get_device_name(device_id: &str) -> String {
    match device_id {
        "@DEFAULT_AUDIO_SOURCE@" => "Default Source".to_string(),
        "@DEFAULT_AUDIO_SINK@" => "Default Sink".to_string(),
        id => {
            let mut all_devices = list_sources().await;
            all_devices.extend(list_sinks().await);
            all_devices
                .iter()
                .find(|d| d.id == id)
                .map(|d| d.description.clone())
                .unwrap_or_else(|| id.to_string())
        }
    }
}

async fn parse_devices(kind: &str, label: &str) -> Vec<AudioDevice> {
    let output = Command::new("pw-dump")
        .args(["Node"])
        .output()
        .await;

    let Ok(out) = output else {
        log::error!("Failed to run pw-dump for {kind}");
        return vec![];
    };

    let text = String::from_utf8_lossy(&out.stdout);
    log::debug!("pw-dump output length: {} bytes for {kind}", text.len());
    let media_class = if kind == "sinks" { "Audio/Sink" } else { "Audio/Source" };

    // Parse JSON array
    let Ok(nodes) = serde_json::from_str::<serde_json::Value>(&text) else {
        log::error!("Failed to parse pw-dump JSON for {kind}: {}", text.chars().take(200).collect::<String>());
        return vec![];
    };

    let mut devices = vec![];

    log::warn!("JSON parsed for {kind}: is_array={}", nodes.is_array());

    if let Some(nodes_arr) = nodes.as_array() {
        log::warn!("Found {} total nodes in pw-dump for {kind}", nodes_arr.len());
        for (idx, node) in nodes_arr.iter().enumerate() {
            let Some(info) = node.get("info").and_then(|i| i.as_object()) else {
                log::debug!("Node {} has no info", idx);
                continue;
            };
            let Some(props) = info.get("props").and_then(|p| p.as_object()) else {
                log::debug!("Node {} has no props", idx);
                continue;
            };

            // Check media class
            let class = match props.get("media.class").and_then(|v| v.as_str()) {
                Some(c) => c,
                None => {
                    log::debug!("Node {} has no media.class", idx);
                    continue;
                }
            };

            log::debug!("Node {}: media.class={}, looking for {}", idx, class, media_class);
            if !class.contains(media_class) {
                continue;
            }

            // Get ID
            let id = match node.get("id").and_then(|v| v.as_u64()) {
                Some(id) => id.to_string(),
                None => continue,
            };

            // Get nick and description
            let nick = props
                .get("node.nick")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            let description = props
                .get("node.description")
                .and_then(|v| v.as_str())
                .unwrap_or(nick);

            log::debug!("  Added {}: {} ({})", id, nick, class);
            devices.push(AudioDevice {
                id,
                name: nick.to_string(),
                description: description.to_string(),
                kind: label.to_string(),
            });
        }
    }

    log::info!("parse_devices({kind}) found {} devices", devices.len());
    devices
}
