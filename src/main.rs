#![feature(adt_const_params, random)]

use clap::{command, Parser};
#[cfg(test)]
pub use tracing_test;

// use truncheon::*;

pub mod ui;
pub mod util;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Options {
}

#[tokio::main]
async fn main() {
    tracing::info!("Welcome to Truncheon.");
    // let options = Options::parse();

    let _ = ui::run().await;
}

