#[macro_use]
extern crate tracing;

use tracing_subscriber::fmt::time::ChronoUtc;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // create subscriber for collecting log entries
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // .with_env_filter("error,traefik-cert-exporter")
        .with_timer(ChronoUtc::with_format("%F %T.%3f %Z".to_string()))
        .finish()
        .init();

    info!("Starting ...");
}
