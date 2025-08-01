use std::fmt;
use tracing_subscriber::{fmt::Subscriber, EnvFilter};

use clap::{command, Parser};
#[cfg(test)]
pub use tracing_test;

use truncheon::*;

pub mod ui;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Options {
    headless: Option<bool>
}

// NOTE: No need to mutation test the main wrapper.
#[tokio::main]
async fn main() {
    tracing::info!("Welcome to Truncheon.");
    let options = Options::parse();


    // Log to a file
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::fs::File::create("truncheon.log").unwrap());
    let subscriber = Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    let _ = ui::run().await;
}

