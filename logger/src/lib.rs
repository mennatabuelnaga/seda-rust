use std::io;

use seda_config::CONFIG;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, EnvFilter};

pub fn init() -> Vec<WorkerGuard> {
    let config = CONFIG.blocking_read();
    let config = &config.logging;

    // Grabs from RUST_LOG env var and if not defaults to
    // TRACE for debug, and info for non debug.
    let level_filter = EnvFilter::try_from_default_env().unwrap_or_default();
    #[cfg(debug_assertions)]
    let level_filter = level_filter
        .add_directive("seda_chain_adapters=trace".parse().unwrap())
        .add_directive("seda_p2p_adapters=trace".parse().unwrap())
        .add_directive("seda_cli=trace".parse().unwrap())
        .add_directive("seda_node=trace".parse().unwrap())
        .add_directive("seda_runtime=trace".parse().unwrap());
    #[cfg(not(debug_assertions))]
    let level_filter = level_filter
        .add_directive("seda_chain_adapters=info".parse().unwrap())
        .add_directive("seda_p2p_adapters=info".parse().unwrap())
        .add_directive("seda_cli=info".parse().unwrap())
        .add_directive("seda_node=info".parse().unwrap())
        .add_directive("seda_runtime=info".parse().unwrap());

    let mut guards = Vec::new();

    let (stdout, stdout_guard) = tracing_appender::non_blocking(io::stdout());
    guards.push(stdout_guard);
    let stdout = fmt::Layer::new().with_writer(stdout).pretty().with_thread_ids(true);
    // Logging shows files and line number but only for debug builds.
    #[cfg(not(debug_assertions))]
    let stdout = stdout.with_line_number(false).with_file(false);

    let file_appender = tracing_appender::rolling::daily(&config.log_file_path, "seda_log");
    let (non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);
    guards.push(file_guard);
    let mut file_logger = fmt::Layer::new().with_writer(non_blocking);
    file_logger.set_ansi(false);

    let subscriber = tracing_subscriber::registry()
        .with(level_filter)
        .with(stdout)
        .with(file_logger);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set logger.");

    guards
}
