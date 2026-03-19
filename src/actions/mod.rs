pub mod mute_toggle;
pub mod volume_display;
pub mod volume_down;
pub mod volume_knob;
pub mod volume_up;

use crate::audio::devices;
use crate::render::TitleOpts;
use openaction::Instance;

/// Build TitleOpts from common settings fields.
pub fn title_opts<'a>(title: &'a str, color: &'a str, size: u32, position: &'a str, max_lines: u32, max_chars: u32) -> TitleOpts<'a> {
    TitleOpts { text: title, color, size, position, max_lines, max_chars }
}

/// Sync all action types that are watching a specific device.
/// Called when any action changes a device's state (mute, volume).
pub async fn sync_all_for_device(device_id: &str) {
    mute_toggle::sync_for_device(device_id).await;
    volume_knob::sync_for_device(device_id).await;
    volume_display::sync_for_device(device_id).await;
    volume_up::sync_for_device(device_id).await;
    volume_down::sync_for_device(device_id).await;
}

/// Re-push cached images for all instances without calling wpctl.
pub async fn rerender_all_cached() {
    mute_toggle::rerender_cached().await;
    volume_knob::rerender_cached().await;
    volume_display::rerender_cached().await;
    volume_up::rerender_cached().await;
    volume_down::rerender_cached().await;
}

/// Sync ALL action instances (all types) - called when layout changes
pub async fn sync_all_instances() {
    mute_toggle::sync_all_instances().await;
    volume_knob::sync_all_instances().await;
    volume_display::sync_all_instances().await;
    volume_up::sync_all_instances().await;
    volume_down::sync_all_instances().await;
}

/// Send the list of available audio devices to a Property Inspector.
pub async fn send_device_list(instance: &Instance) {
    let mut all_devices = vec![];
    all_devices.extend(devices::list_sources().await);
    all_devices.extend(devices::list_sinks().await);
    let payload = serde_json::json!({ "devices": all_devices });
    if let Err(e) = instance.send_to_property_inspector(payload).await {
        log::warn!("Failed to send device list: {e}");
    }
}
