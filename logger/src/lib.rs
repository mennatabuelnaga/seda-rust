use std::{io, path::PathBuf};

use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, EnvFilter};

pub fn init<T>(fun: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: FnOnce() -> Result<(), Box<dyn std::error::Error>>,
{
    // todo optional log file from config
    // TODO consider conditional compilation of log file?
    let mut subscriber = tracing_subscriber::registry()
        // TODO load from RUST_LOG that overwrites config if set
        // from default env already loads level from RUST_LOG just need to use config file
        // to decide alt level instead of default to TRACE
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt::Layer::new().with_writer(io::stdout));

    // TODO figure out wasm issues
    // this works for non wasm usage but breaks in wasm.
    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //     let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     dir.pop();
    //     let file_appender = tracing_appender::rolling::hourly(dir,
    // "example.log");     let (non_blocking, _guard) =
    // tracing_appender::non_blocking(file_appender);     let mut file_logger =
    // fmt::Layer::new().with_writer(non_blocking);     file_logger.
    // set_ansi(false);     subscriber.with(file_logger);
    // }

    tracing::subscriber::with_default(subscriber, fun)
}
