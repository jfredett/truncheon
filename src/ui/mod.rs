use std::sync::Arc;
use tokio::sync::RwLock;
use std::error::Error;

mod app;
mod widgets;
mod tui;

use app::UI;

use crate::ui::tui::Tui;


pub async fn run() -> Result<(), Box<dyn Error>> {
    let app = Arc::new(RwLock::new(UI::new()));
    let mut tui = Tui::new(app.clone(), 60.0, 100.0).unwrap();
    tui.run().await?;

    Ok(())
}

// async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut Arc<RwLock<UI>>) -> io::Result<bool> {
//     use tracing_subscriber::prelude::*;

//     // Set up the Tracing layer
//     tracing_subscriber::registry()
//         .with(tui_logger::TuiTracingSubscriberLayer)
//         .init();

//     // Initialize the tui-logger widget
//     let _ = init_logger(LevelFilter::Trace);
//     set_default_level(LevelFilter::Trace);


// //     tracing::debug!(target:"truncheon::ui", "Logging to {}", dir.to_str().unwrap());
// //     tracing::debug!(target:"truncheon::ui", "Logging initialized");




//     // we are not okay, this method is bad.
//     return Ok(false);
// }
