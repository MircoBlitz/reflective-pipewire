pub mod devices;
pub mod monitor;

use tokio::process::Command;

/// Get volume and mute state for a device.
/// Returns (volume 0.0-1.0+, muted).
pub async fn get_volume(device_id: &str) -> (f32, bool) {
    let output = Command::new("wpctl")
        .args(["get-volume", device_id])
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
    let vol_str = format!("{:.2}", volume.clamp(0.0, 1.0));
    if let Err(e) = Command::new("wpctl")
        .args(["set-volume", device_id, &vol_str])
        .status()
        .await
    {
        log::error!("Failed to set volume for {device_id}: {e}");
    }
}

/// Adjust volume relatively (e.g. "5%+" or "5%-").
pub async fn adjust_volume(device_id: &str, delta: &str) {
    let (current, _) = get_volume(device_id).await;
    let change = delta.trim_matches(|c: char| c.is_alphabetic()).parse::<f32>().unwrap_or(0.0) / 100.0;
    let is_increase = delta.contains('+');
    let new_volume = if is_increase {
        current + change
    } else {
        current - change
    };
    set_volume(device_id, new_volume).await;
}

/// Toggle mute state.
pub async fn toggle_mute(device_id: &str) {
    if let Err(e) = Command::new("wpctl")
        .args(["set-mute", device_id, "toggle"])
        .status()
        .await
    {
        log::error!("Failed to toggle mute for {device_id}: {e}");
    }
}

/// Set mute state explicitly.
pub async fn set_mute(device_id: &str, muted: bool) {
    let val = if muted { "1" } else { "0" };
    if let Err(e) = Command::new("wpctl")
        .args(["set-mute", device_id, val])
        .status()
        .await
    {
        log::error!("Failed to set mute for {device_id}: {e}");
    }
}
