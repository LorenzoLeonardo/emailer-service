mod emailer;
mod error;
mod get_profile;
mod interface;
mod shared_object;
mod task_manager;

use tokio::sync::mpsc::unbounded_channel;

use error::EmailResult;
use ipc_client::client::shared_object::ObjectDispatcher;
use ipc_client::ENV_LOGGER;

use crate::{
    interface::production::Production, shared_object::EmailerObject, task_manager::TaskManager,
};

fn setup_logger(level: log::LevelFilter) {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}:{}]: {}",
                chrono::Local::now().format("%H:%M:%S%.9f"),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
        .unwrap_or_else(|e| {
            eprintln!("{:?}", e);
        });
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> EmailResult<()> {
    let level = std::env::var(ENV_LOGGER)
        .map(|var| match var.to_lowercase().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            "off" => log::LevelFilter::Off,
            _ => log::LevelFilter::Info,
        })
        .unwrap_or_else(|_| log::LevelFilter::Info);
    setup_logger(level);

    let version = env!("CARGO_PKG_VERSION");

    log::info!("Starting emailer-service v.{}", version);

    let (tx, rx) = unbounded_channel();

    let mut shared = ObjectDispatcher::new().await.unwrap();
    let interface = Production::new();

    let object = EmailerObject::new(interface, tx);

    shared
        .register_object("applications.email", Box::new(object))
        .await
        .unwrap();

    let _r = shared.spawn().await;

    let mut task = TaskManager::new(rx);

    task.run().await;

    log::info!("Stopping emailer-service v.{}", version);
    Ok(())
}
