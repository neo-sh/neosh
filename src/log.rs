use tracing_subscriber::{EnvFilter, fmt::Subscriber};
/// Set up tracing stuff
pub fn setup() {
    let subscriber = Subscriber::builder().with_env_filter(EnvFilter::new("neosh=debug")).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::info!("Tracing is running");
}
