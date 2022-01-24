use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt::Subscriber, EnvFilter};

/// Set up log system
///
/// Create file writer and merge it with tracing's subscriber
pub fn setup(data_dir: &Path) -> WorkerGuard {
    // File writer
    let appender = tracing_appender::rolling::never(data_dir, "neosh.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(appender);
    // Subscriber
    let subscriber = Subscriber::builder()
        .with_ansi(false)
        .with_env_filter(EnvFilter::new("neosh=debug"))
        .with_writer(non_blocking_appender)
        .finish();

    // Set up subscriber as global
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!("Tracing is running");

    guard
}

pub mod utils {
    //! Reusable log messages
    use tracing::debug;

    /// Log message for built-in NeoSH commands
    pub fn command(name: &str) {
        debug!("Executing built-in `{}` command", name);
    }
}
