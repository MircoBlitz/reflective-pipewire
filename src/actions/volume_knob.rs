use async_trait::async_trait;
use dashmap::DashMap;
use openaction::*;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::audio;
use crate::render;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct VolumeKnobSettings {
    pub device_id: String,
    pub icon: String,
    pub bg_color: String,
    pub bg_muted_color: String,
    pub icon_color: String,
    pub icon_muted_color: String,
    pub bar_color: String,
    /// Volume change per encoder tick (percentage), default 5
    pub step: u32,
}

impl Default for VolumeKnobSettings {
    fn default() -> Self {
        Self {
            device_id: "@DEFAULT_AUDIO_SOURCE@".to_string(),
            icon: "mic".to_string(),
            bg_color: "#000000".to_string(),
            bg_muted_color: "#000000".to_string(),
            icon_color: "#22c55e".to_string(),
            icon_muted_color: "#ef4444".to_string(),
            bar_color: "#22c55e".to_string(),
            step: 5,
        }
    }
}

static SETTINGS: LazyLock<DashMap<InstanceId, VolumeKnobSettings>> = LazyLock::new(DashMap::new);

pub struct VolumeKnobAction;

#[async_trait]
impl Action for VolumeKnobAction {
    const UUID: ActionUuid = "com.mircoblitz.reflective-pipewire.volume-knob";
    type Settings = VolumeKnobSettings;

    async fn will_appear(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        super::send_device_list(instance).await;
        render_knob(instance, settings).await
    }

    async fn will_disappear(
        &self,
        instance: &Instance,
        _settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        SETTINGS.remove(&instance.instance_id);
        Ok(())
    }

    async fn did_receive_settings(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        super::send_device_list(instance).await;
        render_knob(instance, settings).await
    }

    async fn dial_rotate(
        &self,
        _instance: &Instance,
        settings: &Self::Settings,
        ticks: i16,
        _pressed: bool,
    ) -> OpenActionResult<()> {
        let pct = (settings.step as i32 * ticks.abs() as i32) as u32;
        let delta = if ticks > 0 {
            format!("{pct}%+")
        } else {
            format!("{pct}%-")
        };
        audio::adjust_volume(&settings.device_id, &delta).await;
        super::sync_all_for_device(&settings.device_id).await;
        Ok(())
    }

    async fn dial_up(
        &self,
        _instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        audio::toggle_mute(&settings.device_id).await;
        super::sync_all_for_device(&settings.device_id).await;
        Ok(())
    }
}

pub async fn sync_all_instances() {
    for inst in visible_instances(VolumeKnobAction::UUID).await {
        let settings = SETTINGS
            .get(&inst.instance_id)
            .map(|s| s.clone())
            .unwrap_or_default();
        if let Err(e) = render_knob(&inst, &settings).await {
            log::warn!("volume-knob sync failed for {}: {e}", inst.instance_id);
        }
    }
}

pub async fn sync_for_device(device_id: &str) {
    for inst in visible_instances(VolumeKnobAction::UUID).await {
        let settings = SETTINGS
            .get(&inst.instance_id)
            .map(|s| s.clone())
            .unwrap_or_default();
        if settings.device_id == device_id {
            let _ = render_knob(&inst, &settings).await;
        }
    }
}

async fn render_knob(
    instance: &Instance,
    settings: &VolumeKnobSettings,
) -> OpenActionResult<()> {
    let (volume, muted) = audio::get_volume(&settings.device_id).await;
    let bg = if muted { &settings.bg_muted_color } else { &settings.bg_color };
    let ic = if muted { &settings.icon_muted_color } else { &settings.icon_color };
    let svg = render::volume_bar(bg, ic, &settings.bar_color, &settings.icon, volume, muted);
    instance.set_image(Some(render::svg_to_data_uri(&svg)), None).await
}
