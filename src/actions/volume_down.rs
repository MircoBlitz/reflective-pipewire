use async_trait::async_trait;
use dashmap::DashMap;
use openaction::*;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::audio;
use crate::render;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct VolumeDownSettings {
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
    pub step: u32,
    pub react_to_state: bool,
    pub auto_device_title: bool,
}

impl Default for VolumeDownSettings {
    fn default() -> Self {
        Self {
            device_id: "@DEFAULT_AUDIO_SOURCE@".to_string(),
            icon: "volume_down".to_string(),
            title: String::new(),
            title_color: "#ffffff".to_string(),
            title_size: 14,
            title_position: "top".to_string(),
            bg_color: "#000000".to_string(),
            bg_muted_color: "#000000".to_string(),
            icon_color: "#22c55e".to_string(),
            icon_muted_color: "#ef4444".to_string(),
            step: 5,
            react_to_state: false,
            auto_device_title: true,
        }
    }
}

static SETTINGS: LazyLock<DashMap<InstanceId, VolumeDownSettings>> = LazyLock::new(DashMap::new);

pub struct VolumeDownAction;

#[async_trait]
impl Action for VolumeDownAction {
    const UUID: ActionUuid = "de.mircoblitz.reflective-pipewire.volume-down";
    type Settings = VolumeDownSettings;

    async fn will_appear(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        render_button(instance, settings).await?;
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
        render_button(instance, settings).await?;
        super::send_device_list(instance).await;
        Ok(())
    }

    async fn key_up(&self, _instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        audio::adjust_volume(&settings.device_id, &format!("{}%-", settings.step)).await;
        super::sync_all_for_device(&settings.device_id).await;
        Ok(())
    }
}

pub async fn sync_all_instances() {
    for inst in visible_instances(VolumeDownAction::UUID).await {
        let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
        let _ = render_button(&inst, &s).await;
    }
}

pub async fn sync_for_device(device_id: &str) {
    for inst in visible_instances(VolumeDownAction::UUID).await {
        let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
        if s.device_id == device_id && s.react_to_state {
            let _ = render_button(&inst, &s).await;
        }
    }
}

async fn render_button(instance: &Instance, s: &VolumeDownSettings) -> OpenActionResult<()> {
    let label = format!("-{}%", s.step);
    let (bg, ic) = if s.react_to_state {
        let (volume, muted) = audio::get_volume(&s.device_id).await;
        let t = if muted { 0.0 } else { volume.clamp(0.0, 1.0) };
        (render::lerp_color(&s.bg_muted_color, &s.bg_color, t), render::lerp_color(&s.icon_muted_color, &s.icon_color, t))
    } else {
        (s.bg_color.clone(), s.icon_color.clone())
    };

    let (display_title, title_position) = if s.auto_device_title {
        (audio::devices::get_device_name(&s.device_id).await, "bottom")
    } else {
        (s.title.clone(), s.title_position.as_str())
    };

    let title = super::title_opts(&display_title, &s.title_color, s.title_size, title_position);
    let svg = render::volume_button(&bg, &ic, &s.icon, &label, &title);
    instance.set_image(Some(render::svg_to_data_uri(&svg)), None).await
}
