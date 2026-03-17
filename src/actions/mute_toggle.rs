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
    pub bg_color: String,
    pub bg_muted_color: String,
    pub icon_color: String,
    pub icon_muted_color: String,
    /// When true, icon color interpolates between muted and active based on volume
    pub react_to_state: bool,
}

impl Default for MuteToggleSettings {
    fn default() -> Self {
        Self {
            device_id: "@DEFAULT_AUDIO_SOURCE@".to_string(),
            icon: "mic".to_string(),
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
    const UUID: ActionUuid = "com.mircoblitz.reflective-pipewire.mute-toggle";
    type Settings = MuteToggleSettings;

    async fn will_appear(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        let (vol, muted) = audio::get_volume(&settings.device_id).await;
        render_button(instance, vol, muted, settings).await?;
        super::send_device_list(instance).await;
        Ok(())
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
        let (vol, muted) = audio::get_volume(&settings.device_id).await;
        render_button(instance, vol, muted, settings).await?;
        super::send_device_list(instance).await;
        Ok(())
    }

    async fn key_up(
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
    for inst in visible_instances(MuteToggleAction::UUID).await {
        let settings = SETTINGS
            .get(&inst.instance_id)
            .map(|s| s.clone())
            .unwrap_or_default();
        let (vol, muted) = audio::get_volume(&settings.device_id).await;
        let _ = render_button(&inst, vol, muted, &settings).await;
    }
}

pub async fn sync_for_device(device_id: &str) {
    for inst in visible_instances(MuteToggleAction::UUID).await {
        let settings = SETTINGS
            .get(&inst.instance_id)
            .map(|s| s.clone())
            .unwrap_or_default();
        if settings.device_id == device_id {
            let (vol, muted) = audio::get_volume(device_id).await;
            let _ = render_button(&inst, vol, muted, &settings).await;
        }
    }
}

async fn render_button(
    instance: &Instance,
    volume: f32,
    muted: bool,
    settings: &MuteToggleSettings,
) -> OpenActionResult<()> {
    let (bg, ic) = if settings.react_to_state {
        let t = if muted { 0.0 } else { volume.clamp(0.0, 1.0) };
        let bg = render::lerp_color(&settings.bg_muted_color, &settings.bg_color, t);
        let ic = render::lerp_color(&settings.icon_muted_color, &settings.icon_color, t);
        (bg, ic)
    } else {
        (settings.bg_color.clone(), settings.icon_color.clone())
    };
    let svg = render::mute_button(&bg, &ic, &settings.icon, muted);
    instance.set_image(Some(render::svg_to_data_uri(&svg)), None).await
}
