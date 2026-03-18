use openaction::{register_action, run, OpenActionResult, async_trait, global_events::{GlobalEventHandler, DeviceDidConnectEvent, set_global_event_handler}};

use crate::actions::mute_toggle::{self, MuteToggleAction};
use crate::actions::volume_display::{self, VolumeDisplayAction};
use crate::actions::volume_down::{self, VolumeDownAction};
use crate::actions::volume_knob::{self, VolumeKnobAction};
use crate::actions::volume_up::{self, VolumeUpAction};
use crate::audio::monitor;

struct GlobalHandler;

#[async_trait]
impl GlobalEventHandler for GlobalHandler {
    async fn device_did_connect(&self, _event: DeviceDidConnectEvent) -> OpenActionResult<()> {
        log::info!("Device connected/layout changed – syncing all instances");
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        mute_toggle::sync_all_instances().await;
        volume_knob::sync_all_instances().await;
        volume_display::sync_all_instances().await;
        volume_up::sync_all_instances().await;
        volume_down::sync_all_instances().await;
        Ok(())
    }
}

static HANDLER: GlobalHandler = GlobalHandler;

pub async fn init() -> OpenActionResult<()> {
    set_global_event_handler(&HANDLER);

    register_action(MuteToggleAction).await;
    register_action(VolumeKnobAction).await;
    register_action(VolumeDisplayAction).await;
    register_action(VolumeUpAction).await;
    register_action(VolumeDownAction).await;

    // Start PipeWire event monitor — syncs all actions on external audio changes
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
                    log::warn!("Monitor lagged {n} events");
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


    run(std::env::args().collect()).await
}
