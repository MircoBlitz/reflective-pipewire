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
    fn default() -> Self { Self::Up }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct VolumeButtonSettings {
    pub device_id: String,
    pub icon: String,
    pub title: String,
    pub bg_color: String,
    pub bg_muted_color: String,
    pub icon_color: String,
    pub icon_muted_color: String,
    pub mode: VolumeButtonMode,
    pub step: u32,
    pub value: u32,
    pub react_to_state: bool,
}

impl Default for VolumeButtonSettings {
    fn default() -> Self {
        Self {
            device_id: "@DEFAULT_AUDIO_SOURCE@".to_string(),
            icon: "mic".to_string(),
            title: String::new(),
            bg_color: "#000000".to_string(),
            bg_muted_color: "#000000".to_string(),
            icon_color: "#22c55e".to_string(),
            icon_muted_color: "#ef4444".to_string(),
            mode: VolumeButtonMode::Up,
            step: 5,
            value: 50,
            react_to_state: false,
        }
    }
}

static SETTINGS: LazyLock<DashMap<InstanceId, VolumeButtonSettings>> = LazyLock::new(DashMap::new);

pub struct VolumeButtonAction;

#[async_trait]
impl Action for VolumeButtonAction {
    const UUID: ActionUuid = "de.mircoblitz.reflective-pipewire.volume-button";
    type Settings = VolumeButtonSettings;

    async fn will_appear(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        render_button(instance, settings).await?;
        super::send_device_list(instance).await;
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
        match settings.mode {
            VolumeButtonMode::Up => {
                audio::adjust_volume(&settings.device_id, &format!("{}%+", settings.step)).await;
            }
            VolumeButtonMode::Down => {
                audio::adjust_volume(&settings.device_id, &format!("{}%-", settings.step)).await;
            }
            VolumeButtonMode::Set => {
                audio::set_volume(&settings.device_id, settings.value as f32 / 100.0).await;
            }
        }
        super::sync_all_for_device(&settings.device_id).await;
        Ok(())
    }
}

pub async fn sync_all_instances() {
    for inst in visible_instances(VolumeButtonAction::UUID).await {
        let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
        if s.react_to_state { let _ = render_button(&inst, &s).await; }
    }
}

pub async fn sync_for_device(device_id: &str) {
    for inst in visible_instances(VolumeButtonAction::UUID).await {
        let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
        if s.device_id == device_id && s.react_to_state {
            let _ = render_button(&inst, &s).await;
        }
    }
}

fn button_label(s: &VolumeButtonSettings) -> String {
    match s.mode {
        VolumeButtonMode::Up => format!("+{}%", s.step),
        VolumeButtonMode::Down => format!("-{}%", s.step),
        VolumeButtonMode::Set => format!("{}%", s.value),
    }
}

async fn render_button(instance: &Instance, s: &VolumeButtonSettings) -> OpenActionResult<()> {
    let label = button_label(s);
    let (bg, ic) = if s.react_to_state {
        let (volume, muted) = audio::get_volume(&s.device_id).await;
        let t = if muted { 0.0 } else { volume.clamp(0.0, 1.0) };
        (render::lerp_color(&s.bg_muted_color, &s.bg_color, t), render::lerp_color(&s.icon_muted_color, &s.icon_color, t))
    } else {
        (s.bg_color.clone(), s.icon_color.clone())
    };
    let svg = render::volume_button(&bg, &ic, &s.icon, &label, &s.title);
    instance.set_image(Some(render::svg_to_data_uri(&svg)), None).await
}
