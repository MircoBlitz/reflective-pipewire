use openaction::{register_action, run, OpenActionResult};

use crate::actions::mute_toggle::{self, MuteToggleAction};
use crate::actions::volume_display::{self, VolumeDisplayAction};
use crate::actions::volume_down::{self, VolumeDownAction};
use crate::actions::volume_knob::{self, VolumeKnobAction};
use crate::actions::volume_up::{self, VolumeUpAction};
use crate::audio::monitor;

pub async fn init() -> OpenActionResult<()> {
    register_action(MuteToggleAction).await;
    register_action(VolumeKnobAction).await;
    register_action(VolumeDisplayAction).await;
    register_action(VolumeUpAction).await;
    register_action(VolumeDownAction).await;

    // Start PipeWire event monitor — syncs all action instances on audio changes
    let mut rx = monitor::start_monitor().subscribe();
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(_event) => {
                    mute_toggle::sync_all_instances().await;
                    volume_knob::sync_all_instances().await;
                    volume_display::sync_all_instances().await;
                    volume_up::sync_all_instances().await;
                    volume_down::sync_all_instances().await;
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                    log::warn!("Monitor lagged {n} events, syncing now");
                    mute_toggle::sync_all_instances().await;
                    volume_knob::sync_all_instances().await;
                    volume_display::sync_all_instances().await;
                    volume_up::sync_all_instances().await;
                    volume_down::sync_all_instances().await;
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    log::error!("Monitor channel closed");
                    break;
                }
            }
        }
    });

    // Keep-alive: re-render all instances every 2 seconds to prevent OpenDeck UI resets
    tokio::spawn(async {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            mute_toggle::sync_all_instances().await;
            volume_knob::sync_all_instances().await;
            volume_display::sync_all_instances().await;
            volume_up::sync_all_instances().await;
            volume_down::sync_all_instances().await;
        }
    });

    run(std::env::args().collect()).await
}
