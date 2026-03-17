use async_trait::async_trait;
use dashmap::DashMap;
use openaction::*;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::audio;
use crate::render;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VolumeButtonMode {
    Up,
    Down,
    Set,
}

impl Default for VolumeButtonMode {
    fn default() -> Self {
        Self::Up
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct VolumeButtonSettings {
    pub device_id: String,
    pub icon: String,
    pub bg_color: String,
    pub bg_muted_color: String,
    pub icon_color: String,
    pub icon_muted_color: String,
    pub mode: VolumeButtonMode,
    /// Step size for up/down (percentage)
    pub step: u32,
    /// Fixed volume for set mode (percentage)
    pub value: u32,
    /// Whether this button reflects mute state via icon color
    pub reflect_state: bool,
}

impl Default for VolumeButtonSettings {
    fn default() -> Self {
        Self {
            device_id: "@DEFAULT_AUDIO_SOURCE@".to_string(),
            icon: "mic".to_string(),
            bg_color: "#000000".to_string(),
            bg_muted_color: "#000000".to_string(),
            icon_color: "#22c55e".to_string(),
            icon_muted_color: "#ef4444".to_string(),
            mode: VolumeButtonMode::Up,
            step: 5,
            value: 50,
            reflect_state: false,
        }
    }
}

static SETTINGS: LazyLock<DashMap<InstanceId, VolumeButtonSettings>> =
    LazyLock::new(DashMap::new);

pub struct VolumeButtonAction;

#[async_trait]
impl Action for VolumeButtonAction {
    const UUID: ActionUuid = "com.mircoblitz.reflective-pipewire.volume-button";
    type Settings = VolumeButtonSettings;

    async fn will_appear(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        super::send_device_list(instance).await;
        render_button(instance, settings).await
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
        render_button(instance, settings).await
    }

    async fn key_up(
        &self,
        _instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        match settings.mode {
            VolumeButtonMode::Up => {
                let delta = format!("{}%+", settings.step);
                audio::adjust_volume(&settings.device_id, &delta).await;
            }
            VolumeButtonMode::Down => {
                let delta = format!("{}%-", settings.step);
                audio::adjust_volume(&settings.device_id, &delta).await;
            }
            VolumeButtonMode::Set => {
                let vol = settings.value as f32 / 100.0;
                audio::set_volume(&settings.device_id, vol).await;
            }
        }
        super::sync_all_for_device(&settings.device_id).await;
        Ok(())
    }
}

pub async fn sync_all_instances() {
    for inst in visible_instances(VolumeButtonAction::UUID).await {
        let settings = SETTINGS
            .get(&inst.instance_id)
            .map(|s| s.clone())
            .unwrap_or_default();
        if let Err(e) = render_button(&inst, &settings).await {
            log::warn!("volume-button sync failed for {}: {e}", inst.instance_id);
        }
    }
}

pub async fn sync_for_device(device_id: &str) {
    for inst in visible_instances(VolumeButtonAction::UUID).await {
        let settings = SETTINGS
            .get(&inst.instance_id)
            .map(|s| s.clone())
            .unwrap_or_default();
        if settings.device_id == device_id && settings.reflect_state {
            let _ = render_button(&inst, &settings).await;
        }
    }
}

fn button_label(settings: &VolumeButtonSettings) -> String {
    match settings.mode {
        VolumeButtonMode::Up => format!("+{}%", settings.step),
        VolumeButtonMode::Down => format!("-{}%", settings.step),
        VolumeButtonMode::Set => format!("{}%", settings.value),
    }
}

async fn render_button(
    instance: &Instance,
    settings: &VolumeButtonSettings,
) -> OpenActionResult<()> {
    let label = button_label(settings);

    let (bg, ic) = if settings.reflect_state {
        let (_vol, muted) = audio::get_volume(&settings.device_id).await;
        let bg = if muted { &settings.bg_muted_color } else { &settings.bg_color };
        let ic = if muted { &settings.icon_muted_color } else { &settings.icon_color };
        (bg.as_str(), ic.as_str())
    } else {
        (settings.bg_color.as_str(), settings.icon_color.as_str())
    };

    let svg = render::volume_button(bg, ic, &settings.icon, &label);
    instance.set_image(Some(render::svg_to_data_uri(&svg)), None).await
}
