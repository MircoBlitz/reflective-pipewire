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
    pub title: String,
    pub title_color: String,
    pub title_size: u32,
    pub title_position: String,
    pub bg_color: String,
    pub bg_muted_color: String,
    pub icon_color: String,
    pub icon_muted_color: String,
    pub react_to_state: bool,
}

impl Default for VolumeDisplaySettings {
    fn default() -> Self {
        Self {
            device_id: "@DEFAULT_AUDIO_SOURCE@".to_string(),
            icon: "mic".to_string(),
            title: String::new(),
            title_color: "#ffffff".to_string(),
            title_size: 14,
            title_position: "top".to_string(),
            bg_color: "#000000".to_string(),
            bg_muted_color: "#000000".to_string(),
            icon_color: "#22c55e".to_string(),
            icon_muted_color: "#ef4444".to_string(),
            react_to_state: true,
        }
    }
}

static SETTINGS: LazyLock<DashMap<InstanceId, VolumeDisplaySettings>> = LazyLock::new(DashMap::new);

pub struct VolumeDisplayAction;

#[async_trait]
impl Action for VolumeDisplayAction {
    const UUID: ActionUuid = "de.mircoblitz.reflective-pipewire.volume-display";
    type Settings = VolumeDisplaySettings;

    async fn will_appear(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        render_display(instance, settings).await?;
        super::send_device_list(instance).await;

        tokio::spawn(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            super::sync_all_instances().await;
        });

        Ok(())
    }

    async fn will_disappear(&self, instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.remove(&instance.instance_id);
        Ok(())
    }

    async fn did_receive_settings(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        render_display(instance, settings).await?;
        super::send_device_list(instance).await;
        Ok(())
    }
}

pub async fn sync_all_instances() {
    for inst in visible_instances(VolumeDisplayAction::UUID).await {
        let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
        let _ = render_display(&inst, &s).await;
    }
}

pub async fn sync_for_device(device_id: &str) {
    for inst in visible_instances(VolumeDisplayAction::UUID).await {
        let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
        if s.device_id == device_id {
            let _ = render_display(&inst, &s).await;
        }
    }
}

fn device_label(device_id: &str) -> &str {
    match device_id {
        "@DEFAULT_AUDIO_SOURCE@" => "Default Source",
        "@DEFAULT_AUDIO_SINK@" => "Default Sink",
        other => other,
    }
}

async fn render_display(instance: &Instance, s: &VolumeDisplaySettings) -> OpenActionResult<()> {
    let (volume, muted) = audio::get_volume(&s.device_id).await;
    let (bg, ic) = if s.react_to_state {
        let t = if muted { 0.0 } else { volume.clamp(0.0, 1.0) };
        (
            render::lerp_color(&s.bg_muted_color, &s.bg_color, t),
            render::lerp_color(&s.icon_muted_color, &s.icon_color, t),
        )
    } else {
        (s.bg_color.clone(), s.icon_color.clone())
    };
    let name = device_label(&s.device_id);
    let title = super::title_opts(&s.title, &s.title_color, s.title_size, &s.title_position);
    let svg = render::volume_display(&bg, &ic, volume, muted, name, &title);
    instance.set_image(Some(render::svg_to_data_uri(&svg)), None).await
}
