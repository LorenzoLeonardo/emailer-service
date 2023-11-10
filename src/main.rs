mod emailer;
mod error;
mod get_profile;
mod interface;
mod shared_object;
mod task_manager;

use error::EmailResult;
use ipc_client::client::shared_object::ObjectDispatcher;
use log::LevelFilter;
use tokio::sync::mpsc::unbounded_channel;

use crate::{
    interface::production::Production, shared_object::EmailerObject, task_manager::TaskManager,
};

pub fn setup_logger(level: LevelFilter) {
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

#[cfg(debug_assertions)]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;

#[tokio::main(flavor = "current_thread")]
async fn main() -> EmailResult<()> {
    setup_logger(LOG_LEVEL);

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
