pub mod devices;
pub mod monitor;

use dashmap::DashMap;
use std::sync::LazyLock;
use tokio::process::Command;

// Cache: node.name → current numeric wpctl ID. Invalidated on lookup failure.
static ID_CACHE: LazyLock<DashMap<String, String>> = LazyLock::new(DashMap::new);

/// Resolve a stable node.name to the current numeric WirePlumber ID.
/// Default aliases (@DEFAULT_AUDIO_SINK@ etc.) and already-numeric IDs pass through.
/// Results are cached so pw-dump is only called once per device (or on cache miss).
async fn resolve_wpctl_id(device_id: &str) -> String {
    if device_id.starts_with('@') || device_id.parse::<u64>().is_ok() {
        return device_id.to_string();
    }
    if let Some(cached) = ID_CACHE.get(device_id) {
        return cached.clone();
    }
    let Ok(out) = Command::new("pw-dump").args(["Node"]).output().await else {
        return device_id.to_string();
    };
    let text = String::from_utf8_lossy(&out.stdout);
    let Ok(nodes) = serde_json::from_str::<serde_json::Value>(&text) else {
        return device_id.to_string();
    };
    if let Some(arr) = nodes.as_array() {
        for node in arr {
            let name = node
                .get("info").and_then(|i| i.get("props"))
                .and_then(|p| p.get("node.name"))
                .and_then(|v| v.as_str());
            if name == Some(device_id) {
                if let Some(id) = node.get("id").and_then(|v| v.as_u64()) {
                    let resolved = id.to_string();
                    ID_CACHE.insert(device_id.to_string(), resolved.clone());
                    return resolved;
                }
            }
        }
    }
    log::warn!("Could not resolve node.name '{}' to wpctl ID", device_id);
    device_id.to_string()
}

/// Invalidate the cached wpctl ID for a device (e.g. after device reconnect).
pub fn invalidate_id_cache(device_id: &str) {
    ID_CACHE.remove(device_id);
}

/// Invalidate all cached wpctl IDs (e.g. after any device appears/disappears).
pub fn invalidate_all_id_caches() {
    ID_CACHE.clear();
}

/// Get volume and mute state for a device.
/// Returns (volume 0.0-1.0+, muted).
pub async fn get_volume(device_id: &str) -> (f32, bool) {
    let wpctl_id = resolve_wpctl_id(device_id).await;
    let output = Command::new("wpctl")
        .args(["get-volume", &wpctl_id])
        .output()
        .await;

    match output {
        Ok(out) => {
            let s = String::from_utf8_lossy(&out.stdout);
            let muted = s.contains("[MUTED]");
            let volume = s
                .split_whitespace()
                .nth(1)
                .and_then(|v| v.parse::<f32>().ok())
                .unwrap_or(0.0);
            (volume, muted)
        }
        Err(e) => {
            log::error!("Failed to get volume for {device_id}: {e}");
            (0.0, false)
        }
    }
}

/// Set absolute volume (0.0 - 1.0+).
pub async fn set_volume(device_id: &str, volume: f32) {
    let wpctl_id = resolve_wpctl_id(device_id).await;
    let vol_str = format!("{:.2}", volume.clamp(0.0, 1.5));
    if let Err(e) = Command::new("wpctl")
        .args(["set-volume", &wpctl_id, &vol_str])
        .status()
        .await
    {
        log::error!("Failed to set volume for {device_id}: {e}");
    }
}

/// Adjust volume relatively (e.g. "5%+" or "5%-").
pub async fn adjust_volume(device_id: &str, delta: &str) {
    let (current, _) = get_volume(device_id).await;
    let percent_str = delta.trim_end_matches(|c: char| c == '+' || c == '-');
    let percent = percent_str.trim_end_matches('%').parse::<f32>().unwrap_or(0.0) / 100.0;
    let is_increase = delta.ends_with('+');
    let new_volume = if is_increase {
        current + percent
    } else {
        current - percent
    };
    set_volume(device_id, new_volume).await;
}

/// Toggle mute state.
pub async fn toggle_mute(device_id: &str) {
    let wpctl_id = resolve_wpctl_id(device_id).await;
    if let Err(e) = Command::new("wpctl")
        .args(["set-mute", &wpctl_id, "toggle"])
        .status()
        .await
    {
        log::error!("Failed to toggle mute for {device_id}: {e}");
    }
}

/// Set mute state explicitly.
pub async fn set_mute(device_id: &str, muted: bool) {
    let wpctl_id = resolve_wpctl_id(device_id).await;
    let val = if muted { "1" } else { "0" };
    if let Err(e) = Command::new("wpctl")
        .args(["set-mute", &wpctl_id, val])
        .status()
        .await
    {
        log::error!("Failed to set mute for {device_id}: {e}");
    }
}
