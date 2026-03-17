use async_trait::async_trait;
use dashmap::DashMap;
use openaction::*;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::audio;
use crate::render;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct VolumeDisplaySettings {
    pub device_id: String,
    pub icon: String,
    pub bg_color: String,
    pub bg_muted_color: String,
    pub icon_color: String,
    pub icon_muted_color: String,
    pub bar_color: String,
}

impl Default for VolumeDisplaySettings {
    fn default() -> Self {
        Self {
            device_id: "@DEFAULT_AUDIO_SOURCE@".to_string(),
            icon: "mic".to_string(),
            bg_color: "#000000".to_string(),
            bg_muted_color: "#000000".to_string(),
            icon_color: "#22c55e".to_string(),
            icon_muted_color: "#ef4444".to_string(),
            bar_color: "#22c55e".to_string(),
        }
    }
}

static SETTINGS: LazyLock<DashMap<InstanceId, VolumeDisplaySettings>> =
    LazyLock::new(DashMap::new);

pub struct VolumeDisplayAction;

#[async_trait]
impl Action for VolumeDisplayAction {
    const UUID: ActionUuid = "com.mircoblitz.reflective-pipewire.volume-display";
    type Settings = VolumeDisplaySettings;

    async fn will_appear(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        super::send_device_list(instance).await;
        render_display(instance, settings).await
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
        render_display(instance, settings).await
    }
}

pub async fn sync_all_instances() {
    for inst in visible_instances(VolumeDisplayAction::UUID).await {
        let settings = SETTINGS
            .get(&inst.instance_id)
            .map(|s| s.clone())
            .unwrap_or_default();
        if let Err(e) = render_display(&inst, &settings).await {
            log::warn!("volume-display sync failed for {}: {e}", inst.instance_id);
        }
    }
}

pub async fn sync_for_device(device_id: &str) {
    for inst in visible_instances(VolumeDisplayAction::UUID).await {
        let settings = SETTINGS
            .get(&inst.instance_id)
            .map(|s| s.clone())
            .unwrap_or_default();
        if settings.device_id == device_id {
            let _ = render_display(&inst, &settings).await;
        }
    }
}

async fn render_display(
    instance: &Instance,
    settings: &VolumeDisplaySettings,
) -> OpenActionResult<()> {
    let (volume, muted) = audio::get_volume(&settings.device_id).await;
    let bg = if muted { &settings.bg_muted_color } else { &settings.bg_color };
    let ic = if muted { &settings.icon_muted_color } else { &settings.icon_color };
    let svg = render::volume_bar(bg, ic, &settings.bar_color, &settings.icon, volume, muted);
    instance.set_image(Some(render::svg_to_data_uri(&svg)), None).await
}
