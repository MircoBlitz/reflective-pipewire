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
    pub react_to_state: bool,
    pub title: String,
    pub title_enabled: bool,
    pub title_color: String,
    pub title_size: u32,
    pub title_max_lines: u32,
    pub title_max_chars: u32,
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
            react_to_state: true,
            title: String::new(),
            title_enabled: true,
            title_color: "#ffffff".to_string(),
            title_size: 14,
            title_max_lines: 2,
            title_max_chars: 16,
        }
    }
}

static SETTINGS: LazyLock<DashMap<InstanceId, VolumeDisplaySettings>> = LazyLock::new(DashMap::new);
static LAST_IMAGE: LazyLock<DashMap<InstanceId, String>> = LazyLock::new(DashMap::new);

pub struct VolumeDisplayAction;

#[async_trait]
impl Action for VolumeDisplayAction {
    const UUID: ActionUuid = "de.mircoblitz.reflective-pipewire.volume-display";
    type Settings = VolumeDisplaySettings;

    async fn will_appear(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        let id = instance.instance_id.clone();
        tokio::spawn(async move {
            if let Some(inst) = get_instance(id).await {
                let s = SETTINGS.get(&inst.instance_id).map(|s| s.clone()).unwrap_or_default();
                let _ = render_display(&inst, &s).await;
                super::send_device_list(&inst).await;
            }
        });
        tokio::spawn(async {
            for ms in [100u64, 500, 1000] {
                tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
                crate::actions::rerender_all_cached().await;
            }
        });
        Ok(())
    }

    async fn will_disappear(&self, instance: &Instance, _settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.remove(&instance.instance_id);
        LAST_IMAGE.remove(&instance.instance_id);
        Ok(())
    }

    async fn did_receive_settings(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        SETTINGS.insert(instance.instance_id.clone(), settings.clone());
        render_display(instance, settings).await?;
        super::send_device_list(instance).await;
        Ok(())
    }
}

pub async fn rerender_cached() {
    for inst in visible_instances(VolumeDisplayAction::UUID).await {
        if let Some(img) = LAST_IMAGE.get(&inst.instance_id) {
            let _ = inst.set_image(Some(img.clone()), None).await;
        }
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
    let display_title = if !s.title_enabled {
        String::new()
    } else if s.title.is_empty() {
        audio::devices::get_device_name(&s.device_id).await
    } else {
        s.title.clone()
    };
    let title = super::title_opts(&display_title, &s.title_color, s.title_size, "label", s.title_max_lines, s.title_max_chars);
    let svg = render::volume_display(&bg, &ic, volume, muted, &title);
    let data_uri = render::svg_to_data_uri(&svg);
    LAST_IMAGE.insert(instance.instance_id.clone(), data_uri.clone());
    instance.set_image(Some(data_uri), None).await
}
