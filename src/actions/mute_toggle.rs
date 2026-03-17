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
    /// Background when active (default black)
    pub bg_color: String,
    /// Background when muted (default black)
    pub bg_muted_color: String,
    /// Icon color when active
    pub icon_color: String,
    /// Icon color when muted
    pub icon_muted_color: String,
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
        super::send_device_list(instance).await;
        let (_vol, muted) = audio::get_volume(&settings.device_id).await;
        render_button(instance, muted, settings).await
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
        let (_vol, muted) = audio::get_volume(&settings.device_id).await;
        render_button(instance, muted, settings).await
    }

    async fn key_up(
        &self,
        _instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        audio::toggle_mute(&settings.device_id).await;
        // Sync all actions watching the same device
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
        let (_vol, muted) = audio::get_volume(&settings.device_id).await;
        if let Err(e) = render_button(&inst, muted, &settings).await {
            log::warn!("mute-toggle sync failed for {}: {e}", inst.instance_id);
        }
    }
}

pub async fn sync_for_device(device_id: &str) {
    let (_vol, muted) = audio::get_volume(device_id).await;
    for inst in visible_instances(MuteToggleAction::UUID).await {
        let settings = SETTINGS
            .get(&inst.instance_id)
            .map(|s| s.clone())
            .unwrap_or_default();
        if settings.device_id == device_id {
            let _ = render_button(&inst, muted, &settings).await;
        }
    }
}

async fn render_button(
    instance: &Instance,
    muted: bool,
    settings: &MuteToggleSettings,
) -> OpenActionResult<()> {
    let bg = if muted { &settings.bg_muted_color } else { &settings.bg_color };
    let ic = if muted { &settings.icon_muted_color } else { &settings.icon_color };
    let svg = render::mute_button(bg, ic, &settings.icon, muted);
    instance.set_image(Some(render::svg_to_data_uri(&svg)), None).await
}
