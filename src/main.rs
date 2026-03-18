use openaction::OpenActionResult;

mod actions;
mod audio;
mod plugin;
mod render;

#[tokio::main]
async fn main() -> OpenActionResult<()> {
    let log_file = std::fs::File::create(format!(
        "/tmp/reflective-pipewire-{}.log",
        std::process::id()
    )).unwrap();

    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        log_file,
    )
    .unwrap();

    log::info!("Starting reflective-pipewire plugin...");
    plugin::init().await
}
