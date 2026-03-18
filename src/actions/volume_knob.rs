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
    pub title: String,
    pub bg_color: String,
    pub bg_muted_color: String,
    pub icon_color: String,
    pub icon_muted_color: String,
    pub bar_color: String,
    pub step: u32,
    pub react_to_state: bool,
}

impl Default for VolumeKnobSettings {
    fn default() -> Self {
        Self {
            device_id: "@DEFAULT_AUDIO_SOURCE@".to_string(),
            icon: "mic".to_string(),
            title: String::new(),
            bg_color: "#000000".to_string(),
            bg_muted_color: "#000000".to_string(),
            icon_color: "#22c55e".to_string(),
            icon_muted_color: "#ef4444".to_string(),
            bar_color: "#22c55e".to_string(),
            step: 5,
            react_to_state: true,
        }
    }
}

static SETTINGS: LazyLock<DashMap<InstanceId, VolumeKnobSettings>> = LazyLock::new(DashMap::new);

pub struct VolumeKnobAction;

#[async_trait]
impl Action for VolumeKnobAction {
    const UUID: ActionUuid = "de.mircoblitz.reflective-pipewire.volume-knob";
    type Settings = VolumeKnobSettings;

    async fn will_appear(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        render_knob(instance, settings).await?;
        super::send_device_list(instance).await;
        Ok(())
    }

    async fn will_disappear(&self, instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.remove(&instance.instance_id);
        Ok(())
    }

    async fn did_receive_settings(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        render_knob(instance, settings).await?;
        super::send_device_list(instance).await;
        Ok(())
    }

    async fn dial_rotate(&self, _instance: &Instance, settings: &Self::Settings, ticks: i16, _pressed: bool) -> OpenActionResult<()> {
        let pct = (settings.step as i32 * ticks.abs() as i32) as u32;
        let delta = if ticks > 0 { format!("{pct}%+") } else { format!("{pct}%-") };
        audio::adjust_volume(&settings.device_id, &delta).await;
        super::sync_all_for_device(&settings.device_id).await;
        Ok(())
    }

    async fn dial_up(&self, _instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        audio::toggle_mute(&settings.device_id).await;
        super::sync_all_for_device(&settings.device_id).await;
        Ok(())
    }
}

pub async fn sync_all_instances() {
    for inst in visible_instances(VolumeKnobAction::UUID).await {
        let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
        let _ = render_knob(&inst, &s).await;
    }
}

pub async fn sync_for_device(device_id: &str) {
    for inst in visible_instances(VolumeKnobAction::UUID).await {
        let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
        if s.device_id == device_id {
            let _ = render_knob(&inst, &s).await;
        }
    }
}

async fn render_knob(instance: &Instance, s: &VolumeKnobSettings) -> OpenActionResult<()> {
    let (volume, muted) = audio::get_volume(&s.device_id).await;
    let (bg, ic, bar_c) = if s.react_to_state {
        let t = if muted { 0.0 } else { volume.clamp(0.0, 1.0) };
        (
            render::lerp_color(&s.bg_muted_color, &s.bg_color, t),
            render::lerp_color(&s.icon_muted_color, &s.icon_color, t),
            render::lerp_color(&s.icon_muted_color, &s.bar_color, t),
        )
    } else {
        (s.bg_color.clone(), s.icon_color.clone(), s.bar_color.clone())
    };
    let svg = render::volume_bar(&bg, &ic, &bar_c, &s.icon, volume, muted, &s.title);
    instance.set_image(Some(render::svg_to_data_uri(&svg)), None).await
}
