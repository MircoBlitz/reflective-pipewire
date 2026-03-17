pub mod mute_toggle;
pub mod volume_button;
pub mod volume_display;
pub mod volume_knob;

use crate::audio::devices;
use openaction::Instance;

/// Sync all action types that are watching a specific device.
/// Called when any action changes a device's state (mute, volume).
pub async fn sync_all_for_device(device_id: &str) {
    mute_toggle::sync_for_device(device_id).await;
    volume_knob::sync_for_device(device_id).await;
    volume_display::sync_for_device(device_id).await;
    volume_button::sync_for_device(device_id).await;
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
