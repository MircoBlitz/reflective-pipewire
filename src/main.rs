use openaction::OpenActionResult;

mod actions;
mod audio;
mod plugin;
mod render;

#[tokio::main]
async fn main() -> OpenActionResult<()> {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Never,
    )
    .unwrap();

    log::info!("Starting reflective-pipewire plugin...");
    plugin::init().await
}
