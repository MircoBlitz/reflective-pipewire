use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::broadcast;

/// Event sent when any audio device state changes.
#[derive(Clone, Debug)]
pub enum AudioEvent {
    SourceChanged,
    SinkChanged,
}

/// Spawns `pactl subscribe` and broadcasts events on audio state changes.
/// Returns a broadcast sender that actions can subscribe to.
pub fn start_monitor() -> broadcast::Sender<AudioEvent> {
    let (tx, _) = broadcast::channel(64);
    let tx_clone = tx.clone();

    tokio::spawn(async move {
        loop {
            if let Err(e) = run_monitor(&tx_clone).await {
                log::error!("pactl subscribe failed: {e}, restarting in 2s...");
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    });

    tx
}

async fn run_monitor(tx: &broadcast::Sender<AudioEvent>) -> Result<(), String> {
    let mut child = Command::new("pactl")
        .arg("subscribe")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn: {e}"))?;

    let stdout = child.stdout.take().ok_or("no stdout")?;
    let mut reader = BufReader::new(stdout).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        if line.contains(" on source") || line.contains("source-output") {
            let _ = tx.send(AudioEvent::SourceChanged);
        }
        if line.contains(" on sink") || line.contains("sink-input") {
            let _ = tx.send(AudioEvent::SinkChanged);
        }
    }

    Err("pactl subscribe exited".to_string())
}
