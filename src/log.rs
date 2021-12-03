use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;

use tracing_subscriber::{fmt::Subscriber, EnvFilter};
/// Set up tracing stuff
pub fn setup(data_dir: &PathBuf) -> WorkerGuard {
    let appender = tracing_appender::rolling::never(data_dir, "neosh.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(appender);
    let subscriber = Subscriber::builder()
        .with_ansi(false)
        .with_env_filter(EnvFilter::new("neosh=debug"))
        .with_writer(non_blocking_appender)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!("Tracing is running");

    guard
}
