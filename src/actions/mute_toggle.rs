use async_trait::async_trait;
use dashmap::DashMap;
use openaction::*;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::audio;
use crate::render;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct MuteToggleSettings {
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

impl Default for MuteToggleSettings {
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

static SETTINGS: LazyLock<DashMap<InstanceId, MuteToggleSettings>> = LazyLock::new(DashMap::new);

pub struct MuteToggleAction;

#[async_trait]
impl Action for MuteToggleAction {
    const UUID: ActionUuid = "de.mircoblitz.reflective-pipewire.mute-toggle";
    type Settings = MuteToggleSettings;

    async fn will_appear(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        let (vol, muted) = audio::get_volume(&settings.device_id).await;
        render_button(instance, vol, muted, settings).await?;
        super::send_device_list(instance).await;
        sync_all_instances().await;
        Ok(())
    }

    async fn will_disappear(&self, instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.remove(&instance.instance_id);
        Ok(())
    }

    async fn did_receive_settings(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        let (vol, muted) = audio::get_volume(&settings.device_id).await;
        render_button(instance, vol, muted, settings).await?;
        super::send_device_list(instance).await;
        Ok(())
    }

    async fn key_up(&self, _instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        audio::toggle_mute(&settings.device_id).await;
        super::sync_all_for_device(&settings.device_id).await;
        Ok(())
    }
}

pub async fn sync_all_instances() {
    for inst in visible_instances(MuteToggleAction::UUID).await {
        let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
        let (vol, muted) = audio::get_volume(&s.device_id).await;
        let _ = render_button(&inst, vol, muted, &s).await;
    }
}

pub async fn sync_for_device(device_id: &str) {
    for inst in visible_instances(MuteToggleAction::UUID).await {
        let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
        if s.device_id == device_id {
            let (vol, muted) = audio::get_volume(device_id).await;
            let _ = render_button(&inst, vol, muted, &s).await;
        }
    }
}

async fn render_button(instance: &Instance, volume: f32, muted: bool, s: &MuteToggleSettings) -> OpenActionResult<()> {
    let (bg, ic) = if s.react_to_state {
        let t = if muted { 0.0 } else { volume.clamp(0.0, 1.0) };
        (render::lerp_color(&s.bg_muted_color, &s.bg_color, t), render::lerp_color(&s.icon_muted_color, &s.icon_color, t))
    } else {
        (s.bg_color.clone(), s.icon_color.clone())
    };
    let title = super::title_opts(&s.title, &s.title_color, s.title_size, &s.title_position);
    let svg = render::mute_button(&bg, &ic, &s.icon, muted, &title);
    instance.set_image(Some(render::svg_to_data_uri(&svg)), None).await
}
