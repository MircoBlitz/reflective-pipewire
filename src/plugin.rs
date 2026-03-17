use openaction::{register_action, run, OpenActionResult};

use crate::actions::mute_toggle::{self, MuteToggleAction};
use crate::actions::volume_button::{self, VolumeButtonAction};
use crate::actions::volume_display::{self, VolumeDisplayAction};
use crate::actions::volume_knob::{self, VolumeKnobAction};
use crate::audio::monitor;

pub async fn init() -> OpenActionResult<()> {
    register_action(MuteToggleAction).await;
    register_action(VolumeKnobAction).await;
    register_action(VolumeDisplayAction).await;
    register_action(VolumeButtonAction).await;

    // Start PipeWire event monitor — syncs all action instances on audio changes
    let mut rx = monitor::start_monitor().subscribe();
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(_event) => {
                    // Sync all instances — monitor doesn't know which device changed
                    mute_toggle::sync_all_instances().await;
                    volume_knob::sync_all_instances().await;
                    volume_display::sync_all_instances().await;
                    volume_button::sync_all_instances().await;
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                    log::warn!("Monitor lagged {n} events, syncing now");
                    mute_toggle::sync_all_instances().await;
                    volume_knob::sync_all_instances().await;
                    volume_display::sync_all_instances().await;
                    volume_button::sync_all_instances().await;
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    log::error!("Monitor channel closed");
                    break;
                }
            }
        }
    });

    run(std::env::args().collect()).await
}
