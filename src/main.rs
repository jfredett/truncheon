use tracing_subscriber::{fmt::{format, Subscriber}, EnvFilter};

use clap::{command, Parser};
#[cfg(test)]
pub use tracing_test;

// use truncheon::*;

pub mod ui;
pub mod util;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Options {
    headless: Option<bool>
}

#[tokio::main]
async fn main() {
    tracing::info!("Welcome to Truncheon.");
    // let options = Options::parse();

    // Log to a file
    let (_non_blocking, _guard) = tracing_appender::non_blocking(std::fs::File::create("truncheon.log").unwrap());
    let subscriber = Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .fmt_fields(format::PrettyFields::new())
        .finish();
    _ = tracing::subscriber::set_default(subscriber);
    let _ = ui::run().await;
}

